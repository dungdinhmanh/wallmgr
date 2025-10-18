use crate::renderer::RendererTrait;
use wallmgr_core::error::{Error, Result};
use std::path::Path;
use std::process::{Child, Command, Stdio};

/// Spine renderer - Renders standalone Spine animations
/// Uses spine-runtime or custom OpenGL renderer
pub struct SpineRenderer {
    process: Option<Child>,
    current_path: Option<String>,
    renderer_path: Option<String>,
}

impl SpineRenderer {
    pub fn new() -> Self {
        // Try to find spine renderer binary
        let renderer_path = Self::find_spine_renderer();

        Self {
            process: None,
            current_path: None,
            renderer_path,
        }
    }

    fn find_spine_renderer() -> Option<String> {
        // Check for various Spine runtime implementations
        let binaries = vec![
            "spine-wallpaper",      // Custom wallpaper renderer
            "spine-player",         // Official Spine player
            "spine-runtime",        // Generic runtime
        ];

        for binary in binaries {
            if let Ok(output) = Command::new("which").arg(binary).output() {
                if output.status.success() {
                    return String::from_utf8(output.stdout)
                        .ok()
                        .map(|s| s.trim().to_string());
                }
            }
        }

        // Check common install paths
        let paths = vec![
            "/usr/bin/spine-wallpaper",
            "/usr/local/bin/spine-wallpaper",
            "~/.local/bin/spine-wallpaper",
        ];

        for path in paths {
            let expanded = shellexpand::tilde(path);
            if std::path::Path::new(expanded.as_ref()).exists() {
                return Some(expanded.to_string());
            }
        }

        None
    }

    /// Detect if directory contains Spine files
    pub fn is_spine_directory(path: &Path) -> bool {
        if !path.is_dir() {
            return false;
        }

        // Must have .skel file
        let has_skel = std::fs::read_dir(path)
            .ok()
            .and_then(|entries| {
                Some(entries.filter_map(|e| e.ok()).any(|entry| {
                    entry.path().extension()
                        .and_then(|s| s.to_str())
                        .map(|s| s == "skel")
                        .unwrap_or(false)
                }))
            })
            .unwrap_or(false);

        // Must NOT have project.json (that would be WallpaperEngine)
        let has_project = path.join("project.json").exists();

        has_skel && !has_project
    }

    /// Find .skel file in directory
    fn find_skel_file(dir: &Path) -> Option<std::path::PathBuf> {
        std::fs::read_dir(dir).ok()?.find_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()?.to_str()? == "skel" {
                Some(path)
            } else {
                None
            }
        })
    }
}

impl Default for SpineRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl RendererTrait for SpineRenderer {
    fn name(&self) -> &str {
        "spine"
    }

    fn is_available(&self) -> bool {
        // TODO: Spine renderer is coming soon - temporarily disabled
        // self.renderer_path.is_some()
        false
    }

    #[allow(unused_variables)]
    fn start(&mut self, path: &Path, monitor: Option<&str>) -> Result<()> {
        // Spine renderer is coming soon - currently disabled
        return Err(Error::Renderer(
            "Spine animation support is coming soon! Currently under development.".to_string()
        ));

        #[allow(unreachable_code)]
        {
        let renderer = self
            .renderer_path
            .as_ref()
            .ok_or_else(|| Error::Renderer("Spine renderer not found".to_string()))?;

        // Stop any existing process
        self.stop()?;

        // Determine project directory and .skel file
        let (project_dir, skel_file) = if path.is_dir() {
            let skel = Self::find_skel_file(path)
                .ok_or_else(|| Error::NotFound("No .skel file found".to_string()))?;
            (path.to_path_buf(), skel)
        } else if path.extension().and_then(|s| s.to_str()) == Some("skel") {
            let parent = path.parent()
                .ok_or_else(|| Error::InvalidPath("Invalid path".to_string()))?;
            (parent.to_path_buf(), path.to_path_buf())
        } else {
            return Err(Error::InvalidPath("Not a valid Spine path".to_string()));
        };

        let mut cmd = Command::new(renderer);

        // Arguments vary based on renderer
        // This is a generic approach - adjust based on actual renderer
        cmd.arg("--skeleton")
            .arg(&skel_file)
            .arg("--dir")
            .arg(&project_dir)
            .arg("--fullscreen")
            .arg("--fps")
            .arg("30")
            .arg("--loop");

        // Monitor selection
        if let Some(mon) = monitor {
            cmd.arg("--monitor").arg(mon);
        }

        // Detect display server
        if std::env::var("WAYLAND_DISPLAY").is_ok() {
            cmd.arg("--wayland");
        } else {
            cmd.arg("--x11");
        }

        cmd.stdout(Stdio::null()).stderr(Stdio::null());

        let child = cmd.spawn().map_err(|e| {
            Error::Renderer(format!("Failed to start Spine renderer: {}", e))
        })?;

        self.process = Some(child);
        self.current_path = Some(project_dir.to_string_lossy().to_string());

        Ok(())
        } // End of unreachable_code block
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

impl Drop for SpineRenderer {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

// Note: You'll need to create or integrate a Spine runtime
// Options:
// 1. Use spine-cpp runtime: https://github.com/EsotericSoftware/spine-runtimes/tree/4.2/spine-cpp
// 2. Create custom OpenGL/Vulkan renderer
// 3. Use existing spine-player if available
// 4. Fork and modify a Spine renderer for wallpaper use
