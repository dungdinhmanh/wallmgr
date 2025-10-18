use crate::renderer::RendererTrait;
use wallmgr_core::error::{Error, Result};
use std::path::Path;
use std::process::{Child, Command, Stdio};

/// Wallpaper Engine renderer using linux-wallpaperengine
pub struct WallpaperEngineRenderer {
    process: Option<Child>,
    current_path: Option<String>,
    linux_we_path: Option<String>,
}

impl WallpaperEngineRenderer {
    pub fn new() -> Self {
        // Try to find linux-wallpaperengine binary
        let linux_we_path = Self::find_linux_wallpaperengine();

        Self {
            process: None,
            current_path: None,
            linux_we_path,
        }
    }

    fn find_linux_wallpaperengine() -> Option<String> {
        // Check common installation paths
        let paths = vec![
            "/usr/bin/linux-wallpaperengine",
            "/usr/local/bin/linux-wallpaperengine",
            "~/.local/bin/linux-wallpaperengine",
        ];

        for path in paths {
            let expanded = shellexpand::tilde(path);
            if std::path::Path::new(expanded.as_ref()).exists() {
                return Some(expanded.to_string());
            }
        }

        // Try to find via which
        Command::new("which")
            .arg("linux-wallpaperengine")
            .output()
            .ok()
            .and_then(|output| {
                if output.status.success() {
                    String::from_utf8(output.stdout)
                        .ok()
                        .map(|s| s.trim().to_string())
                } else {
                    None
                }
            })
    }

    /// Detect if path is a Wallpaper Engine project
    /// WE projects ALWAYS have project.json
    pub fn is_wallpaper_engine_project(path: &Path) -> bool {
        if path.is_dir() {
            // Must have project.json
            path.join("project.json").exists()
        } else if path.is_file() {
            // If pointing to project.json directly
            if path.file_name().and_then(|s| s.to_str()) == Some("project.json") {
                return true;
            }
            // Check parent directory
            if let Some(parent) = path.parent() {
                parent.join("project.json").exists()
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl Default for WallpaperEngineRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl RendererTrait for WallpaperEngineRenderer {
    fn name(&self) -> &str {
        "linux-wallpaperengine"
    }

    fn is_available(&self) -> bool {
        self.linux_we_path.is_some()
    }

    fn start(&mut self, path: &Path, monitor: Option<&str>) -> Result<()> {
        let linux_we_path = self
            .linux_we_path
            .as_ref()
            .ok_or_else(|| Error::Renderer("linux-wallpaperengine not found".to_string()))?
            .clone();

        // Stop any existing process
        self.stop()?;

        let mut cmd = Command::new(&linux_we_path);

        // Determine the project directory
        let project_dir = if path.is_dir() {
            path.to_path_buf()
        } else if let Some(parent) = path.parent() {
            parent.to_path_buf()
        } else {
            return Err(Error::InvalidPath(
                "Cannot determine project directory".to_string(),
            ));
        };

        // Basic arguments for linux-wallpaperengine
        cmd.arg("--dir")
            .arg(&project_dir)
            .arg("--silent")
            .arg("--noautomute")
            .arg("--fps")
            .arg("30");

        // Screen selection
        if let Some(mon) = monitor {
            cmd.arg("--screen-root").arg(mon);
        } else {
            cmd.arg("--screen-root").arg("all");
        }

        // Detect display server
        if std::env::var("WAYLAND_DISPLAY").is_ok() {
            // Currently linux-wallpaperengine primarily supports X11
            // For Wayland, we might need additional configuration or fallback
            tracing::warn!("linux-wallpaperengine has limited Wayland support");
        }

        cmd.stdout(Stdio::null()).stderr(Stdio::null());

        let child = cmd.spawn().map_err(|e| {
            Error::Renderer(format!("Failed to start linux-wallpaperengine: {}", e))
        })?;

        self.process = Some(child);
        self.current_path = Some(project_dir.to_string_lossy().to_string());

        Ok(())
    }

    fn stop(&mut self) -> Result<()> {
        if let Some(mut child) = self.process.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
        self.current_path = None;
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.process.is_some()
    }
}

impl Drop for WallpaperEngineRenderer {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}
