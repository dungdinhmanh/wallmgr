use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::error::{Error, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub data_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub thumbnails_dir: PathBuf,
    pub database_path: PathBuf,
    pub api: ApiConfig,
    pub renderer: RendererConfig,
    pub sources: SourcesConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub host: String,
    pub port: u16,
    pub max_connections: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RendererConfig {
    pub video_fps: u32,
    pub hardware_accel: bool,
    pub mpv_options: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourcesConfig {
    pub enable_danbooru: bool,
    pub enable_yandere: bool,
    pub enable_safebooru: bool,
    pub enable_gelbooru: bool,
    pub enable_pixiv: bool,
    pub pixiv_refresh_token: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        let data_dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("wallmgr");

        let cache_dir = dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("wallmgr");

        let thumbnails_dir = cache_dir.join("thumbnails");
        let database_path = data_dir.join("wallmgr.db");

        Self {
            data_dir,
            cache_dir,
            thumbnails_dir,
            database_path,
            api: ApiConfig::default(),
            renderer: RendererConfig::default(),
            sources: SourcesConfig::default(),
        }
    }
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 9527,
            max_connections: 100,
        }
    }
}

impl Default for RendererConfig {
    fn default() -> Self {
        Self {
            video_fps: 30,
            hardware_accel: true,
            mpv_options: vec![
                "--loop".to_string(),
                "--no-audio".to_string(),
                "--hwdec=auto".to_string(),
            ],
        }
    }
}

impl Default for SourcesConfig {
    fn default() -> Self {
        Self {
            enable_danbooru: true,
            enable_yandere: true,
            enable_safebooru: true,
            enable_gelbooru: true,
            enable_pixiv: false,
            pixiv_refresh_token: None,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| Error::Config("Cannot find config directory".to_string()))?
            .join("wallmgr");

        Self::load_from_path(&config_dir.join("config.toml"))
    }

    pub fn load_from_path<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        let config_path = path.as_ref();

        if config_path.exists() {
            let content = std::fs::read_to_string(config_path)?;
            let config: Config = toml::from_str(&content)
                .map_err(|e| Error::Config(format!("Failed to parse config: {}", e)))?;
            Ok(config)
        } else {
            let config = Self::default();
            let config_path_str = config_path.display().to_string();
            if let Some(parent) = config_path.parent() {
                std::fs::create_dir_all(parent).map_err(|e| {
                    Error::Config(format!("Cannot create config dir {}: {}", config_path_str, e))
                })?;
            }
            config.save_to_path(config_path)?;
            Ok(config)
        }
    }

    pub async fn load_from_path_async<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        let config_path = path.as_ref();

        if config_path.exists() {
            let content = tokio::fs::read_to_string(config_path).await
                .map_err(|e| Error::Config(format!("Cannot read config file: {}", e)))?;
            let config: Config = toml::from_str(&content)
                .map_err(|e| Error::Config(format!("Failed to parse config: {}", e)))?;
            Ok(config)
        } else {
            let config = Self::default();
            let config_path_str = config_path.display().to_string();
            if let Some(parent) = config_path.parent() {
                tokio::fs::create_dir_all(parent).await
                    .map_err(|e| Error::Config(format!("Cannot create config dir {}: {}", config_path_str, e)))?;
            }
            config.save_to_path_async(config_path).await?;
            Ok(config)
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| Error::Config("Cannot find config directory".to_string()))?
            .join("wallmgr");

        Self::save_dir(&config_dir, self)
    }

    pub fn save_to_path<P: AsRef<std::path::Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref().parent()
            .ok_or_else(|| Error::Config("Invalid config path".to_string()))?;

        Self::save_dir(path, self)
    }

    pub async fn save_to_path_async<P: AsRef<std::path::Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();
        let content = toml::to_string_pretty(self)
            .map_err(|e| Error::Config(format!("Failed to serialize config: {}", e)))?;

        tokio::fs::write(path, content).await
            .map_err(|e| Error::Config(format!("Cannot write config file: {}", e)))?;

        Ok(())
    }

    fn save_dir(dir: &std::path::Path, config: &Config) -> Result<()> {
        std::fs::create_dir_all(dir)?;

        let config_path = dir.join("config.toml");
        let content = toml::to_string_pretty(config)
            .map_err(|e| Error::Config(format!("Failed to serialize config: {}", e)))?;

        std::fs::write(&config_path, content)?;
        Ok(())
    }

    pub fn ensure_directories(&self) -> Result<()> {
        std::fs::create_dir_all(&self.data_dir)?;
        std::fs::create_dir_all(&self.cache_dir)?;
        std::fs::create_dir_all(&self.thumbnails_dir)?;

        if let Some(parent) = self.database_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        Ok(())
    }

    pub async fn ensure_directories_async(&self) -> Result<()> {
        tokio::fs::create_dir_all(&self.data_dir).await
            .map_err(|e| Error::Config(format!("Cannot create data dir: {}", e)))?;
        tokio::fs::create_dir_all(&self.cache_dir).await
            .map_err(|e| Error::Config(format!("Cannot create cache dir: {}", e)))?;
        tokio::fs::create_dir_all(&self.thumbnails_dir).await
            .map_err(|e| Error::Config(format!("Cannot create thumbnails dir: {}", e)))?;
        Ok(())
    }
}
