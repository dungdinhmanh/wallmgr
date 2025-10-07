use std::path::PathBuf;
use axum::{
    extract::{Path, Query, Multipart},
    http::StatusCode,
    response::Json,
    Extension,
};
use serde::{Deserialize, Serialize};
use tokio::fs;
use uuid::Uuid;
use wallmgr_core::{
    error::{Error, Result},
    Config, Database,
};
use wallmgr_adapters::Adapter;
use wallmgr_renderers::Renderer;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct AddWallpaperRequest {
    pub path: String,
    pub tags: Option<Vec<String>>,
    pub source: Option<String>,
    pub source_url: Option<String>,
}

#[derive(Deserialize)]
pub struct SetWallpaperRequest {
    pub wallpaper_id: Uuid,
    pub monitor: Option<String>,
}

#[derive(Deserialize)]
pub struct SearchRequest {
    pub tags: Vec<String>,
    pub sources: Vec<String>,
    pub limit: Option<u32>,
    pub rating: Option<String>,
}

#[derive(Serialize)]
pub struct WallpaperResponse {
    pub id: String,
    pub filename: String,
    pub wallpaper_type: String,
    pub path: String,
    pub width: u32,
    pub height: u32,
    pub size: u64,
    pub hash: String,
    pub tags: Vec<String>,
    pub created_at: String,
    pub source: Option<String>,
    pub source_url: Option<String>,
    pub thumbnail_path: Option<String>,
}

#[derive(Serialize)]
pub struct SearchResult {
    pub images: Vec<SearchedImage>,
    pub has_more: bool,
}

#[derive(Serialize)]
pub struct SearchedImage {
    pub id: String,
    pub source: String,
    pub url: String,
    pub preview_url: Option<String>,
    pub width: u32,
    pub height: u32,
    pub tags: Vec<String>,
    pub score: Option<i32>,
    pub author: Option<String>,
}

#[derive(Serialize)]
pub struct MonitorResponse {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub x: i32,
    pub y: i32,
    pub primary: bool,
}

#[derive(Serialize)]
pub struct TagResponse {
    pub name: String,
    pub count: i64,
}

// Convert database wallpaper to API response
fn wallpaper_to_response(wallpaper: &wallmgr_core::types::Wallpaper) -> WallpaperResponse {
    WallpaperResponse {
        id: wallpaper.id.to_string(),
        filename: wallpaper.filename.clone(),
        wallpaper_type: match wallpaper.wallpaper_type {
            wallmgr_core::types::WallpaperType::Image => "image".to_string(),
            wallmgr_core::types::WallpaperType::Video => "video".to_string(),
            wallmgr_core::types::WallpaperType::Spine => "spine".to_string(),
            wallmgr_core::types::WallpaperType::WallpaperEngine => "wallpaper_engine".to_string(),
        },
        path: wallpaper.path.clone(),
        width: wallpaper.width,
        height: wallpaper.height,
        size: wallpaper.size,
        hash: wallpaper.hash.clone(),
        tags: wallpaper.tags.clone(),
        created_at: wallpaper.created_at.to_rfc3339(),
        source: wallpaper.source.clone(),
        source_url: wallpaper.source_url.clone(),
        thumbnail_path: wallpaper.thumbnail_path.clone(),
    }
}

pub async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now().to_rfc3339(),
    }))
}

pub async fn list_wallpapers(
    Query(params): Query<std::collections::HashMap<String, String>>,
    Extension(state): Extension<AppState>,
) -> Result<Json<Vec<WallpaperResponse>>, StatusCode> {
    let mut db = state.database.write().await;

    // Parse filter_type parameter
    let filter_type = params.get("type").and_then(|ftype| match ftype.as_str() {
        "image" => Some(wallmgr_core::types::WallpaperType::Image),
        "video" => Some(wallmgr_core::types::WallpaperType::Video),
        "spine" => Some(wallmgr_core::types::WallpaperType::Spine),
        "wallpaper_engine" => Some(wallmgr_core::types::WallpaperType::WallpaperEngine),
        _ => None,
    });

    let wallpapers = db
        .list_wallpapers(filter_type)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let responses = wallpapers
        .into_iter()
        .map(|w| wallpaper_to_response(&w))
        .collect();

    Ok(Json(responses))
}

pub async fn get_wallpaper(
    Path(id): Path<String>,
    Extension(state): Extension<AppState>,
) -> Result<Json<WallpaperResponse>, StatusCode> {
    let wallpaper_id = Uuid::parse_str(&id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let mut db = state.database.write().await;

    // Get wallpaper info
    let mut wallpaper = db
        .get_wallpaper(&wallpaper_id)
        .map_err(|_| StatusCode::NOT_FOUND)?;

    // Get tags
    wallpaper.tags = db
        .get_wallpaper_tags(&wallpaper_id)
        .unwrap_or_default();

    let response = wallpaper_to_response(&wallpaper);
    Ok(Json(response))
}

pub async fn add_wallpaper(
    Json(request): Json<AddWallpaperRequest>,
    Extension(state): Extension<AppState>,
) -> Result<StatusCode, StatusCode> {
    // Validate path
    let path = PathBuf::from(&request.path);
    if !path.exists() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Get file metadata
    let metadata = fs::metadata(&path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !metadata.is_file() && !path.is_dir() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Determine wallpaper type
    let wallpaper_type = wallmgr_core::types::WallpaperType::from_path(&request.path)
        .ok_or(StatusCode::BAD_REQUEST)?;

    // Generate hash
    use sha2::{Sha256, Digest};
    let file_content = fs::read(&path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let hash = format!("{:x}", Sha256::digest(&file_content));

    // Get dimensions based on type
    let (width, height) = match wallpaper_type {
        wallmgr_core::types::WallpaperType::Image => {
            // Use image crate for static images
            tokio::task::spawn_blocking(move || {
                image::image_dimensions(&path)
            })
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .map_err(|_| StatusCode::BAD_REQUEST)?
        }
        wallmgr_core::types::WallpaperType::Video |
        wallmgr_core::types::WallpaperType::WallpaperEngine |
        wallmgr_core::types::WallpaperType::Spine => {
            // TODO: Add proper video/GIF dimension detection using ffmpeg
            // For now use sensible defaults
            (1920, 1080)
        }
    };

    // Create wallpaper object
    let wallpaper = wallmgr_core::types::Wallpaper {
        id: Uuid::new_v4(),
        path: request.path.clone(),
        filename: path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string(),
        wallpaper_type,
        width,
        height,
        size: metadata.len(),
        hash,
        source: request.source,
        source_url: request.source_url,
        tags: request.tags.unwrap_or_default(),
        thumbnail_path: None,
        created_at: chrono::Utc::now(),
        modified_at: chrono::Utc::now(),
    };

    let mut db = state.database.write().await;
    db.add_wallpaper(&wallpaper)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}

pub async fn set_wallpaper(
    Json(request): Json<SetWallpaperRequest>,
    Extension(state): Extension<AppState>,
) -> Result<StatusCode, StatusCode> {
    let mut db = state.database.write().await;

    // Get wallpaper info
    let wallpaper = db
        .get_wallpaper(&request.wallpaper_id)
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let mut adapter_lock = state.adapter.write().await;

    // Initialize adapter if needed
    if adapter_lock.is_none() {
        // TODO: Detect and select appropriate adapter
        todo!("Adapter selection");
    }

    let adapter = adapter_lock.as_ref().unwrap();

    // Create renderer based on wallpaper type
    let mut renderer_lock = state.renderer.write().await;

    if renderer_lock.is_none() {
        let renderer = match wallpaper.wallpaper_type {
            wallmgr_core::types::WallpaperType::Video |
            wallmgr_core::types::WallpaperType::Image => {
                use wallmgr_renderers::{Renderer, VideoRenderer};
                Some(Renderer::Video(VideoRenderer::new()))
            },
            wallmgr_core::types::WallpaperType::Spine => {
                use wallmgr_renderers::{Renderer, SpineRenderer};
                Some(Renderer::Spine(SpineRenderer::new()))
            },
            wallmgr_core::types::WallpaperType::WallpaperEngine => {
                use wallmgr_renderers::{Renderer, WallpaperEngineRenderer};
                Some(Renderer::WallpaperEngine(WallpaperEngineRenderer::new()))
            },
        };

        *renderer_lock = renderer;
    }

    if let Some(renderer) = renderer_lock.as_mut() {
        renderer
            .get_trait_mut()
            .start(std::path::Path::new(&wallpaper.path), request.monitor.as_deref())
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(StatusCode::OK)
}

pub async fn delete_wallpaper(
    Path(id): Path<String>,
    Extension(state): Extension<AppState>,
) -> Result<StatusCode, StatusCode> {
    let wallpaper_id = Uuid::parse_str(&id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let mut db = state.database.write().await;
    db.delete_wallpaper(&wallpaper_id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn search_booru(
    Json(request): Json<SearchRequest>,
    Extension(state): Extension<AppState>,
) -> Result<Json<SearchResult>, StatusCode> {
    let limit = request.limit.unwrap_or(20).min(100);
    let mut all_images = Vec::new();

    // Search each requested source
    for source in request.sources {
        let connector = match source.as_str() {
            "danbooru" => &state.booru_clients.danbooru,
            "yandere" => &state.booru_clients.yandere,
            "safebooru" => &state.booru_clients.safebooru,
            "gelbooru" => &state.booru_clients.gelbooru,
            _ => continue,
        };

        match connector.search(&request.tags, limit, 1).await {
            Ok(images) => {
                for img in images {
                    all_images.push(SearchedImage {
                        id: img.id,
                        source: source.clone(),
                        url: img.file_url,
                        preview_url: img.preview_url,
                        width: img.width,
                        height: img.height,
                        tags: img.tags,
                        score: img.score,
                        author: img.author,
                    });
                }
            }
            Err(_) => continue, // Skip failed sources
        }
    }

    // Sort by score if available
    all_images.sort_by(|a, b| {
        match (a.score, b.score) {
            (Some(a_score), Some(b_score)) => b_score.cmp(&a_score),
            (Some(_), None) => std::cmp::Ordering::Greater,
            (None, Some(_)) => std::cmp::Ordering::Less,
            _ => std::cmp::Ordering::Equal,
        }
    });

    let has_more = all_images.len() > limit as usize;
    all_images.truncate(limit as usize);

    Ok(Json(SearchResult {
        images: all_images,
        has_more,
    }))
}

pub async fn download_image(
    Json(request): Json<serde_json::Value>,
    Extension(state): Extension<AppState>,
) -> Result<StatusCode, StatusCode> {
    // TODO: Implement download and add to library
    // This would require:
    // 1. Download image to temp
    // 2. Move to library directory
    // 3. Add to database
    // 4. Generate thumbnail

    Ok(StatusCode::NOT_IMPLEMENTED)
}

pub async fn autocomplete_tags(
    Query(params): Query<std::collections::HashMap<String, String>>,
    Extension(state): Extension<AppState>,
) -> Result<Json<Vec<String>>, StatusCode> {
    let prefix = params
        .get("query")
        .map(|s| s.as_str())
        .unwrap_or("");

    let mut db = state.database.write().await;
    let tags = db
        .search_tags(prefix, 20)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let tag_names = tags.into_iter().map(|t| t.name).collect();
    Ok(Json(tag_names))
}

pub async fn search_tags(
    Query(params): Query<std::collections::HashMap<String, String>>,
    Extension(state): Extension<AppState>,
) -> Result<Json<Vec<TagResponse>>, StatusCode> {
    let query = params
        .get("query")
        .map(|s| s.as_str())
        .unwrap_or("");

    let mut db = state.database.write().await;
    let tags = db
        .search_tags(query, 50)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let responses = tags
        .into_iter()
        .map(|t| TagResponse {
            name: t.name,
            count: t.count,
        })
        .collect();

    Ok(Json(responses))
}

pub async fn list_monitors(
    Extension(state): Extension<AppState>,
) -> Result<Json<Vec<MonitorResponse>>, StatusCode> {
    let adapter_lock = state.adapter.read().await;

    if let Some(adapter) = &*adapter_lock {
        let monitors = adapter
            .list_monitors()
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let responses = monitors
            .into_iter()
            .map(|m| MonitorResponse {
                name: m.name,
                width: m.width,
                height: m.height,
                x: m.x,
                y: m.y,
                primary: m.primary,
            })
            .collect();

        Ok(Json(responses))
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
