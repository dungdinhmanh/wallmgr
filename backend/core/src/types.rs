use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallpaper {
    pub id: Uuid,
    pub path: String,
    pub filename: String,
    pub wallpaper_type: WallpaperType,
    pub width: u32,
    pub height: u32,
    pub size: u64,
    pub hash: String,
    pub source: Option<String>,
    pub source_url: Option<String>,
    pub tags: Vec<String>,
    pub thumbnail_path: Option<String>,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum WallpaperType {
    Image,
    Video,
    Spine,           // Standalone Spine animations (.skel, .atlas, .png)
    WallpaperEngine, // Full Wallpaper Engine projects (project.json)
}

impl WallpaperType {
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "jpg" | "jpeg" | "png" | "webp" | "bmp" => Some(Self::Image),
            "mp4" | "webm" | "mkv" | "avi" | "gif" => Some(Self::Video),
            "skel" | "atlas" => Some(Self::Spine),
            _ => None,
        }
    }

    pub fn from_path(path: &str) -> Option<Self> {
        let path_obj = std::path::Path::new(path);

        // Check for Wallpaper Engine project (has project.json)
        if path.ends_with("project.json") {
            return Some(Self::WallpaperEngine);
        }

        if path_obj.is_dir() {
            if path_obj.join("project.json").exists() {
                return Some(Self::WallpaperEngine);
            }
        } else if let Some(parent) = path_obj.parent() {
            // If pointing to file, check parent directory
            if parent.join("project.json").exists() {
                return Some(Self::WallpaperEngine);
            }
        }

        // Check for standalone Spine animation
        // Must have .skel file AND NO project.json
        if let Some(parent) = if path_obj.is_dir() {
            Some(path_obj)
        } else {
            path_obj.parent()
        } {
            let has_skel = std::fs::read_dir(parent).ok()?.any(|e| {
                if let Ok(entry) = e {
                    entry.path().extension()
                        .and_then(|s| s.to_str())
                        .map(|s| s == "skel")
                        .unwrap_or(false)
                } else {
                    false
                }
            });

            let has_project = parent.join("project.json").exists();

            // Spine ONLY if has .skel AND no project.json
            if has_skel && !has_project {
                return Some(Self::Spine);
            }
        }

        // Fallback to extension-based detection
        if let Some(ext) = path_obj.extension().and_then(|s| s.to_str()) {
            Self::from_extension(ext)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub category: Option<String>,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Monitor {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub x: i32,
    pub y: i32,
    pub primary: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WallpaperConfig {
    pub monitor: Option<String>, // None = all monitors
    pub wallpaper_id: Uuid,
    pub mode: DisplayMode,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DisplayMode {
    Fill,
    Fit,
    Stretch,
    Center,
    Tile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    pub tags: Vec<String>,
    pub sources: Vec<String>,
    pub limit: Option<u32>,
    pub rating: Option<Rating>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Rating {
    Safe,
    Questionable,
    Explicit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BooruImage {
    pub id: String,
    pub source: String,
    pub file_url: String,
    pub preview_url: Option<String>,
    pub sample_url: Option<String>,
    pub width: u32,
    pub height: u32,
    pub tags: Vec<String>,
    pub rating: Rating,
    pub score: Option<i32>,
    pub author: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message.into()),
        }
    }
}
