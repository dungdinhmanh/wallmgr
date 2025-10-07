use clap::{Parser, Subcommand};
use std::path::PathBuf;
use colored::*;
use reqwest::Client;

#[derive(Parser)]
#[command(name = "wallmgr")]
#[command(about = "Wallmgr wallpaper manager CLI")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add wallpaper to library
    Add {
        /// Path to wallpaper file/directory
        path: PathBuf,
        /// Tags to add (comma-separated)
        #[arg(short, long)]
        tags: Option<String>,
    },

    /// Set wallpaper
    Set {
        /// Wallpaper path or ID
        wallpaper: String,
        /// Monitor to set wallpaper on (optional)
        #[arg(short, long)]
        monitor: Option<String>,
    },

    /// List wallpapers
    List {
        /// Filter by type (image, video, spine, wallpaper_engine)
        #[arg(short, long)]
        r#type: Option<String>,
        /// Limit results
        #[arg(short, long, default_value = "50")]
        limit: usize,
    },

    /// Remove wallpaper
    Remove {
        /// Wallpaper ID
        id: String,
    },

    /// Search for wallpapers
    Search {
        /// Search query/tags
        query: Option<String>,
        /// Tags to search for (comma-separated)
        #[arg(short, long)]
        tags: Option<String>,
        /// Source booru (danbooru, yandere, safebooru, gelbooru)
        #[arg(short, long)]
        source: Option<String>,
        /// Limit results
        #[arg(short, long, default_value = "20")]
        limit: u32,
    },

    /// Get current status
    Status,

    /// Interact with daemon
    Daemon {
        #[command(subcommand)]
        daemon_cmd: DaemonCommands,
    },
}

#[derive(Subcommand)]
enum DaemonCommands {
    /// Start daemon
    Start,
    /// Stop daemon
    Stop,
    /// Restart daemon
    Restart,
    /// Get daemon status
    Status,
}

struct WallmgrClient {
    client: Client,
    base_url: String,
}

impl WallmgrClient {
    fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: "http://127.0.0.1:9527/api".to_string(),
        }
    }

    async fn health_check(&self) -> Result<(), Box<dyn std::error::Error>> {
        let resp: serde_json::Value = self.client
            .get(&format!("{}/health", self.base_url))
            .send()
            .await?
            .json()
            .await?;

        if resp["status"] == "healthy" {
            println!("{} Daemon is running", "✓".green());
            Ok(())
        } else {
            println!("{} Daemon is not responding", "✗".red());
            Err("Daemon not healthy".into())
        }
    }

    async fn add_wallpaper(&self, path: PathBuf, tags: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
        if !path.exists() {
            return Err(format!("Path does not exist: {}", path.display()).into());
        }

        let tags_vec: Option<Vec<String>> = tags.map(|t| t.split(',').map(|s| s.trim().to_string()).collect());

        let request = serde_json::json!({
            "path": path.to_string_lossy(),
            "tags": tags_vec
        });

        let resp = self.client
            .post(&format!("{}/wallpapers/add", self.base_url))
            .json(&request)
            .send()
            .await?;

        if resp.status().is_success() {
            println!("{} Wallpaper added successfully", "✓".green());
        } else {
            let error: serde_json::Value = resp.json().await?;
            println!("{} Failed to add wallpaper: {}", "✗".red(), error);
        }

        Ok(())
    }

    async fn set_wallpaper(&self, wallpaper: &str, monitor: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
        let request = serde_json::json!({
            "wallpaper_id": if wallpaper.chars().next().map_or(false, |c| c.is_ascii_digit()) {
                // If starts with digit, it's an ID
                wallpaper.parse::<uuid::Uuid>()?
            } else {
                // Otherwise, assume it's a path (TODO: handle this properly)
                return Err("ID based setting not implemented yet".into());
            },
            "monitor": monitor
        });

        let resp = self.client
            .post(&format!("{}/wallpapers/set", self.base_url))
            .json(&request)
            .send()
            .await?;

        if resp.status().is_success() {
            println!("{} Wallpaper set successfully", "✓".green());
        } else {
            let error: serde_json::Value = resp.json().await?;
            println!("{} Failed to set wallpaper: {}", "✗".red(), error);
        }

        Ok(())
    }

    async fn list_wallpapers(&self, r#type: Option<&str>, limit: usize) -> Result<(), Box<dyn std::error::Error>> {
        let mut url = format!("{}?limit={}", self.base_url, limit);
        if let Some(t) = r#type {
            url.push_str(&format!("&type={}", t));
        }

        let resp: Vec<serde_json::Value> = self.client
            .get(&url)
            .send()
            .await?
            .json()
            .await?;

        println!("{}\n", "Wallpapers:".bold());
        for wallpaper in resp {
            let id = wallpaper["id"].as_str().unwrap_or("unknown");
            let filename = wallpaper["filename"].as_str().unwrap_or("unknown");
            let wtype = wallpaper["wallpaper_type"].as_str().unwrap_or("unknown");
            let size = wallpaper["size"].as_u64().unwrap_or(0);

            println!("{} {} ({}) - {}",
                id.blue(),
                filename,
                wtype.yellow(),
                human_bytes::human_bytes(size as f64)
            );

            if let Some(tags) = wallpaper["tags"].as_array() {
                if !tags.is_empty() {
                    let tags_str: Vec<_> = tags.iter()
                        .filter_map(|t| t.as_str())
                        .collect();
                    println!("  Tags: {}", tags_str.join(", ").cyan());
                }
            }

            println!();
        }

        Ok(())
    }

    async fn remove_wallpaper(&self, id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let resp = self.client
            .delete(&format!("{}/wallpapers/{}", self.base_url, id))
            .send()
            .await?;

        if resp.status().is_success() {
            println!("{} Wallpaper removed successfully", "✓".green());
        } else {
            println!("{} Failed to remove wallpaper", "✗".red());
        }

        Ok(())
    }

    async fn search_booru(&self, tags: &[String], source: Option<&str>, limit: u32) -> Result<(), Box<dyn std::error::Error>> {
        let mut sources = vec!["danbooru", "yandere", "safebooru", "gelbooru"];
        if let Some(s) = source {
            sources = vec![s];
        }

        let request = serde_json::json!({
            "tags": tags,
            "sources": sources,
            "limit": limit
        });

        let resp: serde_json::Value = self.client
            .post(&format!("{}/search", self.base_url))
            .json(&request)
            .send()
            .await?
            .json()
            .await?;

        println!("{}\n", "Search Results:".bold());

        if let Some(images) = resp["images"].as_array() {
            for image in images {
                let id = image["id"].as_str().unwrap_or("unknown");
                let source = image["source"].as_str().unwrap_or("unknown");
                let url = image["url"].as_str().unwrap_or("unknown");
                let width = image["width"].as_u64().unwrap_or(0);
                let height = image["height"].as_u64().unwrap_or(0);

                println!("{} {} {}x{} ({})",
                    id.blue(),
                    source.yellow(),
                    width,
                    height,
                    url
                );

                if let Some(tags) = image["tags"].as_array() {
                    if !tags.is_empty() {
                        let tags_str: Vec<_> = tags.iter()
                            .take(5) // Show first 5 tags
                            .filter_map(|t| t.as_str())
                            .collect();
                        println!("  Tags: {}", tags_str.join(", ").cyan());
                    }
                }

                println!();
            }
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();
    let client = WallmgrClient::new();

    match cli.command {
        Commands::Add { path, tags } => {
            client.health_check().await?;
            client.add_wallpaper(path, tags.as_deref()).await?;
        }

        Commands::Set { wallpaper, monitor } => {
            client.health_check().await?;
            client.set_wallpaper(&wallpaper, monitor.as_deref()).await?;
        }

        Commands::List { r#type, limit } => {
            client.health_check().await?;
            client.list_wallpapers(r#type.as_deref(), limit).await?;
        }

        Commands::Remove { id } => {
            client.health_check().await?;
            client.remove_wallpaper(&id).await?;
        }

        Commands::Search { query, tags, source, limit } => {
            client.health_check().await?;

            let search_tags: Vec<String> = if let Some(tag_str) = tags {
                tag_str.split(',').map(|s| s.trim().to_string()).collect()
            } else if let Some(q) = query {
                vec![q]
            } else {
                return Err("Must specify either --tags or query".into());
            };

            client.search_booru(&search_tags, source.as_deref(), limit).await?;
        }

        Commands::Status => {
            let result = client.health_check().await;
            if result.is_ok() {
                // Could show more info here
                println!("Daemon is running and responsive");
            }
        }

        Commands::Daemon { daemon_cmd } => {
            match daemon_cmd {
                DaemonCommands::Start => {
                    println!("Starting wallmgr daemon...");
                    // TODO: systemctl integration
                    println!("{} Use: systemctl --user start wallmgr", "Note:".yellow());
                }
                DaemonCommands::Stop => {
                    println!("Stopping wallmgr daemon...");
                    println!("{} Use: systemctl --user stop wallmgr", "Note:".yellow());
                }
                DaemonCommands::Restart => {
                    println!("Restarting wallmgr daemon...");
                    println!("{} Use: systemctl --user restart wallmgr", "Note:".yellow());
                }
                DaemonCommands::Status => {
                    println!("Checking daemon status...");
                    println!("{} Use: systemctl --user status wallmgr", "Note:".yellow());
                }
            }
        }
    }

    Ok(())
}