use crate::renderer::RendererTrait;
use wallmgr_core::error::{Error, Result};
use std::path::Path;
use std::process::{Child, Command, Stdio};

/// Video renderer using mpv
/// Supports: MP4, WebM, MKV, AVI, GIF
pub struct VideoRenderer {
    process: Option<Child>,
    current_path: Option<String>,
}

impl VideoRenderer {
    pub fn new() -> Self {
        Self {
            process: None,
            current_path: None,
        }
    }

    fn get_window_id(monitor: Option<&str>) -> Result<Option<String>> {
        // For X11, we need to get or create a window ID
        // This is a simplified version - in practice you'd want to:
        // 1. Check if running on X11 or Wayland
        // 2. For X11: create a fullscreen window and get its ID
        // 3. For Wayland: use layer-shell protocol

        if std::env::var("WAYLAND_DISPLAY").is_ok() {
            // On Wayland, mpv can handle fullscreen directly
            Ok(None)
        } else {
            // On X11, we'll let mpv create its own window in fullscreen mode
            Ok(None)
        }
    }
}

impl Default for VideoRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl RendererTrait for VideoRenderer {
    fn name(&self) -> &str {
        "mpv-video"
    }

    fn is_available(&self) -> bool {
        Command::new("which")
            .arg("mpv")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    fn start(&mut self, path: &Path, monitor: Option<&str>) -> Result<()> {
        if !self.is_available() {
            return Err(Error::Renderer("mpv not found".to_string()));
        }

        // Stop any existing process
        self.stop()?;

        let mut cmd = Command::new("mpv");

        // Basic video playback options
        cmd.arg("--loop")
            .arg("--no-audio")
            .arg("--hwdec=auto")
            .arg("--vo=gpu")
            .arg("--profile=low-latency")
            .arg("--fps=30")
            .arg("--no-osc")
            .arg("--no-osd-bar")
            .arg("--no-input-default-bindings")
            .arg("--input-conf=/dev/null")
            .arg("--no-border");

        // Fullscreen mode
        if std::env::var("WAYLAND_DISPLAY").is_ok() {
            // Wayland fullscreen
            cmd.arg("--fs");
            if let Some(mon) = monitor {
                cmd.arg(format!("--fs-screen={}", mon));
            }
        } else {
            // X11 fullscreen
            cmd.arg("--fs");
            if let Some(mon) = monitor {
                cmd.arg(format!("--screen={}", mon));
            }
        }

        // Background mode (try to keep window behind others)
        cmd.arg("--ontop=no")
            .arg("--no-keepaspect-window");

        cmd.arg(path)
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        let child = cmd
            .spawn()
            .map_err(|e| Error::Renderer(format!("Failed to start mpv: {}", e)))?;

        self.process = Some(child);
        self.current_path = Some(path.to_string_lossy().to_string());

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
        if let Some(child) = &self.process {
            // Check if process is still alive
            // This is a simplified check
            true
        } else {
            false
        }
    }
}

impl Drop for VideoRenderer {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}
