use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppSettings {
    // Folders
    pub download_folder: PathBuf,
    pub local_folder: PathBuf,
    pub favorites_folder: PathBuf,
    
    // Online sources
    pub enabled_sources: Vec<String>, // Source IDs that are enabled
    pub allow_nsfw: bool,
    pub items_per_page: usize,
    
    // Wallpaper
    pub auto_set_wallpaper: bool,
    pub display_mode: String, // "zoomed", "scaled", "stretched", "centered", "tiled", "spanned"
    pub apply_all_workspaces: bool,
    pub auto_change: bool,
    pub change_interval: u32, // minutes
    pub random_order: bool,
    pub background_color1: [u8; 3], // RGB for gradient start
    pub background_color2: [u8; 3], // RGB for gradient end
    
    // UI
    pub thumbnail_size: f32,
    pub columns: usize,
}

impl Default for AppSettings {
    fn default() -> Self {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        
        Self {
            download_folder: home.join("Pictures").join("Wallpapers"),
            local_folder: home.join("Pictures"),
            favorites_folder: home.join("Pictures").join("Favorites"),
            enabled_sources: vec!["konachan".to_string(), "yandere".to_string(), "danbooru".to_string(), "safebooru".to_string()],
            allow_nsfw: false,
            items_per_page: 20,
            auto_set_wallpaper: false,
            display_mode: "zoomed".to_string(),
            apply_all_workspaces: true,
            auto_change: false,
            change_interval: 10,
            random_order: false,
            background_color1: [64, 64, 64],
            background_color2: [32, 32, 32],
            thumbnail_size: 200.0,
            columns: 5,
        }
    }
}

impl AppSettings {
    pub fn load() -> Self {
        let config_path = Self::config_path();
        
        if let Ok(contents) = std::fs::read_to_string(&config_path) {
            if let Ok(settings) = serde_json::from_str(&contents) {
                return settings;
            }
        }
        
        Self::default()
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::config_path();
        
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(&config_path, json)?;
        
        Ok(())
    }

    fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("wallmgr")
            .join("settings.json")
    }
}
