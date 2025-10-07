use axum::{
    middleware,
    routing::{get, post, delete, put, get_service},
    Router,
    Extension,
};
use tower_http::services::ServeDir;
use std::sync::Arc;
use std::path::PathBuf;
use crate::state::AppState;
use crate::handlers::*;
use crate::websocket::ws_handler;

/// Configure file server for thumbnails and assets
pub fn file_routes() -> Router<Extension<AppState>> {
    Router::new()
        .nest_service("/thumbnails", ServeDir::new("thumbnails"))
        .nest_service("/assets", ServeDir::new("assets"))
}

/// Main router configuration
pub fn create_router(state: AppState) -> Router {
    let api_routes = Router::new()
        // Wallpaper management
        .route("/wallpapers", get(list_wallpapers))
        .route("/wallpapers/:id", get(get_wallpaper))
        .route("/wallpapers/:id", delete(delete_wallpaper))
        .route("/wallpapers/add", post(add_wallpaper))
        .route("/wallpapers/set", post(set_wallpaper))

        // Search and download
        .route("/search", post(search_booru))
        .route("/search/download", post(download_image))

        // Tags
        .route("/tags/autocomplete", get(autocomplete_tags))
        .route("/tags/search", get(search_tags))

        // System info
        .route("/monitors", get(list_monitors))
        .route("/health", get(health_check))

        // WebSocket
        .route("/ws", get(ws_handler));

    let app = Router::new()
        // API v1 routes
        .nest("/api/v1", api_routes)

        // Backward compatibility for /api routes
        .nest("/api", Router::new()
            .route("/wallpapers", get(list_wallpapers))
            .route("/wallpapers/:id", get(get_wallpaper))
            .route("/wallpapers/:id", delete(delete_wallpaper))
            .route("/wallpapers/add", post(add_wallpaper))
            .route("/wallpapers/set", post(set_wallpaper))
            .route("/search", post(search_booru))
            .route("/search/download", post(download_image))
            .route("/tags/autocomplete", get(autocomplete_tags))
            .route("/tags/search", get(search_tags))
            .route("/monitors", get(list_monitors))
            .route("/health", get(health_check))
            .route("/ws", get(ws_handler))
        )

        // File serving
        .nest_service("/files", file_routes())

        .layer(Extension(state));

    app
}