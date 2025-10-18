use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Wallpaper {
    pub id: String,
    pub path: PathBuf,
    pub width: u32,
    pub height: u32,
    pub file_size: u64,
    pub wallpaper_type: WallpaperType,
    pub is_favorite: bool,
    pub tags: Vec<String>,
    #[serde(skip)]
    #[allow(dead_code)]
    pub selected: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum WallpaperType {
    Static,      // PNG, JPG, WebP
    Video,       // MP4, WebM
    Live2D,      // Live2D model
    Unknown,
}

impl Wallpaper {
    pub fn from_path(path: PathBuf) -> Self {
        let wallpaper_type = Self::detect_type(&path);
        let id = uuid::Uuid::new_v4().to_string();
        
        Self {
            id,
            path,
            width: 0,
            height: 0,
            file_size: 0,
            wallpaper_type,
            is_favorite: false,
            tags: Vec::new(),
            selected: false,
        }
    }

    fn detect_type(path: &PathBuf) -> WallpaperType {
        let ext = path.extension()
            .and_then(|s| s.to_str())
            .map(|s| s.to_lowercase());

        match ext.as_deref() {
            Some("png") | Some("jpg") | Some("jpeg") | Some("webp") | Some("bmp") => WallpaperType::Static,
            Some("mp4") | Some("webm") | Some("mkv") | Some("avi") => WallpaperType::Video,
            Some("model3") | Some("json") => WallpaperType::Live2D, // Live2D files
            _ => WallpaperType::Unknown,
        }
    }

    #[allow(dead_code)]
    pub fn aspect_ratio(&self) -> f32 {
        if self.height > 0 {
            self.width as f32 / self.height as f32
        } else {
            16.0 / 9.0 // Default
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct BooruImage {
    #[allow(dead_code)]
    pub id: String,
    pub width: u32,
    pub height: u32,
    pub tags: String,
    #[allow(dead_code)]
    pub rating: String,
    #[serde(default)]
    #[allow(dead_code)]
    pub is_nsfw: bool,
    #[allow(dead_code)]
    pub preview_url: String,
    #[allow(dead_code)]
    pub sample_url: String,
    pub file_url: String,
    #[serde(default)]
    pub selected: bool,
}

impl BooruImage {
    #[allow(dead_code)]
    pub fn to_wallpaper(&self, download_path: PathBuf) -> Wallpaper {
        Wallpaper {
            id: self.id.clone(),
            path: download_path,
            width: self.width,
            height: self.height,
            file_size: 0,
            wallpaper_type: WallpaperType::Static,
            is_favorite: false,
            tags: self.tags.split_whitespace().map(String::from).collect(),
            selected: false,
        }
    }
}
