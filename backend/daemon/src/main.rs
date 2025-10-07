use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tokio::signal::unix::{signal, SignalKind};
use tracing::{info, error, warn};
use wallmgr_api::{AppState, create_router};
use wallmgr_core::{Config, Database};
use wallmgr_adapters::detector::{detect_environment, detect_desktop_environment, check_command_available};
use wallmgr_adapters::{Adapter, AdapterTrait};

#[derive(Parser)]
#[command(name = "wallmgr-daemon")]
#[command(about = "Wallmgr wallpaper daemon")]
struct Cli {
    #[arg(short, long, default_value = "127.0.0.1")]
    host: String,

    #[arg(short, long, default_value = "9527")]
    port: u16,

    #[arg(short, long)]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Run,
}

async fn run_daemon(cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting Wallmgr daemon");

    // Load configuration
    let config = if let Some(config_path) = &cli.config {
        tokio::task::spawn_blocking(move || Config::load_from_path(config_path.as_path()))
            .await??
    } else {
        tokio::task::spawn_blocking(Config::load)
            .await??
    };

    info!("Config loaded");
    config.ensure_directories_async().await?;

    // Initialize database
    let database = Database::new(&config.database_path)?;

    // Detect and initialize adapter
    let adapter = select_adapter().await?;
    if let Some(adapter) = &adapter {
        info!("Using adapter: {}", adapter.name());
    } else {
        warn!("No suitable adapter found");
    }

    // Create app state
    let state = AppState::new(config, database);
    *state.adapter.write().await = adapter;

    // Start HTTP server
    let app = create_router(state);
    let addr = format!("{}:{}", cli.host, cli.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("Listening on {}", addr);

    // Graceful shutdown handler
    let mut sigint = signal(SignalKind::interrupt())?;
    let mut sigterm = signal(SignalKind::terminate())?;

    let server = axum::serve(listener, app);
    tokio::select! {
        result = server => {
            if let Err(e) = result {
                error!("Server error: {}", e);
            }
        }
        _ = sigint.recv() => {
            info!("Received SIGINT, shutting down");
        }
        _ = sigterm.recv() => {
            info!("Received SIGTERM, shutting down");
        }
    }

    info!("Daemon stopped");
    Ok(())
}

async fn select_adapter() -> Result<Option<wallmgr_adapters::Adapter>, Box<dyn std::error::Error>> {
    use wallmgr_adapters::x11::FehAdapter;
    use wallmgr_adapters::x11::NitrogenAdapter;
    use wallmgr_adapters::x11::XWallpaperAdapter;
    use wallmgr_adapters::wayland::SwwwAdapter;
    use wallmgr_adapters::wayland::HyprpaperAdapter;
    use wallmgr_adapters::wayland::SwaybgAdapter;
    use wallmgr_adapters::desktop::GnomeAdapter;
    use wallmgr_adapters::desktop::KdeAdapter;
    use wallmgr_adapters::desktop::XfceAdapter;

    let environment = detect_environment();
    let de = detect_desktop_environment();

    info!("Detected environment: {:?}", environment);
    info!("Detected DE: {:?}", de);

    // Desktop Environment adapters (highest priority)
    match de {
        wallmgr_adapters::detector::DesktopEnvironment::Gnome if check_command_available("gsettings") => {
            return Ok(Some(wallmgr_adapters::Adapter::Gnome(GnomeAdapter::new())));
        }
        wallmgr_adapters::detector::DesktopEnvironment::Kde if check_command_available("qdbus") => {
            return Ok(Some(wallmgr_adapters::Adapter::Kde(KdeAdapter::new())));
        }
        wallmgr_adapters::detector::DesktopEnvironment::Xfce if check_command_available("xfconf-query") => {
            return Ok(Some(wallmgr_adapters::Adapter::Xfce(XfceAdapter::new())));
        }
        _ => {}
    }

    // Wayland compositor specific
    if let wallmgr_adapters::detector::Environment::Wayland = environment {
        if check_command_available("swww") && SwwwAdapter::new().is_available() {
            return Ok(Some(wallmgr_adapters::Adapter::Swww(SwwwAdapter::new())));
        }
        if check_command_available("hyprpaper") {
            return Ok(Some(wallmgr_adapters::Adapter::Hyprpaper(HyprpaperAdapter::new())));
        }
        if check_command_available("swaybg") {
            return Ok(Some(wallmgr_adapters::Adapter::Swaybg(SwaybgAdapter::new())));
        }
    }

    // X11 fallbacks
    if check_command_available("nitrogen") {
        return Ok(Some(wallmgr_adapters::Adapter::Nitrogen(NitrogenAdapter::new())));
    }
    if check_command_available("feh") {
        return Ok(Some(wallmgr_adapters::Adapter::Feh(FehAdapter::new())));
    }
    if check_command_available("xwallpaper") {
        return Ok(Some(wallmgr_adapters::Adapter::XWallpaper(XWallpaperAdapter::new())));
    }

    warn!("No suitable wallpaper adapter found");
    Ok(None)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    match cli.command.unwrap_or(Commands::Run) {
        Commands::Run => {
            run_daemon(&cli).await?;
        }
    }

    Ok(())
}