use crate::adapter::AdapterTrait;
use crate::detector::check_command_available;
use wallmgr_core::error::{Error, Result};
use wallmgr_core::types::{DisplayMode, Monitor};
use std::path::Path;
use std::process::Command;

/// feh adapter - lightweight and fast
pub struct FehAdapter;

impl FehAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl AdapterTrait for FehAdapter {
    fn name(&self) -> &str {
        "feh"
    }

    fn is_available(&self) -> bool {
        check_command_available("feh")
    }

    fn set_wallpaper(&self, path: &Path, monitor: Option<&str>, mode: DisplayMode) -> Result<()> {
        if !self.is_available() {
            return Err(Error::AdapterUnavailable("feh not found".to_string()));
        }

        let mode_arg = match mode {
            DisplayMode::Fill => "--bg-fill",
            DisplayMode::Fit => "--bg-max",
            DisplayMode::Stretch => "--bg-scale",
            DisplayMode::Center => "--bg-center",
            DisplayMode::Tile => "--bg-tile",
        };

        let mut cmd = Command::new("feh");
        cmd.arg(mode_arg);

        if let Some(mon) = monitor {
            // feh supports multi-monitor via xinerama
            cmd.arg("--xinerama-index").arg(mon);
        }

        cmd.arg(path);

        let output = cmd.output()?;
        if !output.status.success() {
            return Err(Error::Renderer(format!(
                "feh failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        Ok(())
    }

    fn list_monitors(&self) -> Result<Vec<Monitor>> {
        // Use xrandr to get monitor info
        if !check_command_available("xrandr") {
            return Ok(Vec::new());
        }

        let output = Command::new("xrandr").arg("--query").output()?;
        let stdout = String::from_utf8_lossy(&output.stdout);

        let mut monitors = Vec::new();
        for line in stdout.lines() {
            if line.contains(" connected") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    let name = parts[0].to_string();
                    let primary = line.contains("primary");

                    // Parse resolution (e.g., "1920x1080+0+0")
                    if let Some(res_part) = parts.iter().find(|p| p.contains('x') && p.contains('+')) {
                        if let Some((res, pos)) = res_part.split_once('+') {
                            if let Some((w, h)) = res.split_once('x') {
                                if let (Ok(width), Ok(height)) = (w.parse::<u32>(), h.parse::<u32>()) {
                                    let coords: Vec<&str> = pos.split('+').collect();
                                    let x = coords.get(0).and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);
                                    let y = coords.get(1).and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);

                                    monitors.push(Monitor {
                                        name,
                                        width,
                                        height,
                                        x,
                                        y,
                                        primary,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(monitors)
    }

    fn stop(&self) -> Result<()> {
        // feh doesn't run as a daemon, so nothing to stop
        Ok(())
    }
}

/// nitrogen adapter - good for X11, has GUI
pub struct NitrogenAdapter;

impl NitrogenAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl AdapterTrait for NitrogenAdapter {
    fn name(&self) -> &str {
        "nitrogen"
    }

    fn is_available(&self) -> bool {
        check_command_available("nitrogen")
    }

    fn set_wallpaper(&self, path: &Path, _monitor: Option<&str>, mode: DisplayMode) -> Result<()> {
        if !self.is_available() {
            return Err(Error::AdapterUnavailable("nitrogen not found".to_string()));
        }

        let mode_arg = match mode {
            DisplayMode::Fill => "--set-zoom-fill",
            DisplayMode::Fit => "--set-auto",
            DisplayMode::Stretch => "--set-scaled",
            DisplayMode::Center => "--set-centered",
            DisplayMode::Tile => "--set-tiled",
        };

        let output = Command::new("nitrogen")
            .arg(mode_arg)
            .arg(path)
            .output()?;

        if !output.status.success() {
            return Err(Error::Renderer(format!(
                "nitrogen failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        Ok(())
    }

    fn list_monitors(&self) -> Result<Vec<Monitor>> {
        // Use xrandr
        if !check_command_available("xrandr") {
            return Ok(Vec::new());
        }

        let output = Command::new("xrandr").arg("--query").output()?;
        let stdout = String::from_utf8_lossy(&output.stdout);

        let mut monitors = Vec::new();
        for line in stdout.lines() {
            if line.contains(" connected") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    let name = parts[0].to_string();
                    let primary = line.contains("primary");

                    if let Some(res_part) = parts.iter().find(|p| p.contains('x') && p.contains('+')) {
                        if let Some((res, pos)) = res_part.split_once('+') {
                            if let Some((w, h)) = res.split_once('x') {
                                if let (Ok(width), Ok(height)) = (w.parse::<u32>(), h.parse::<u32>()) {
                                    let coords: Vec<&str> = pos.split('+').collect();
                                    let x = coords.get(0).and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);
                                    let y = coords.get(1).and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);

                                    monitors.push(Monitor {
                                        name,
                                        width,
                                        height,
                                        x,
                                        y,
                                        primary,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(monitors)
    }

    fn stop(&self) -> Result<()> {
        Ok(())
    }
}

/// xwallpaper adapter - minimal X11 wallpaper setter
pub struct XWallpaperAdapter;

impl XWallpaperAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl AdapterTrait for XWallpaperAdapter {
    fn name(&self) -> &str {
        "xwallpaper"
    }

    fn is_available(&self) -> bool {
        check_command_available("xwallpaper")
    }

    fn set_wallpaper(&self, path: &Path, monitor: Option<&str>, mode: DisplayMode) -> Result<()> {
        if !self.is_available() {
            return Err(Error::AdapterUnavailable("xwallpaper not found".to_string()));
        }

        let mode_arg = match mode {
            DisplayMode::Fill => "--zoom",
            DisplayMode::Fit => "--maximize",
            DisplayMode::Stretch => "--stretch",
            DisplayMode::Center => "--center",
            DisplayMode::Tile => "--tile",
        };

        let mut cmd = Command::new("xwallpaper");

        if let Some(mon) = monitor {
            cmd.arg("--output").arg(mon);
        }

        cmd.arg(mode_arg).arg(path);

        let output = cmd.output()?;
        if !output.status.success() {
            return Err(Error::Renderer(format!(
                "xwallpaper failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        Ok(())
    }

    fn list_monitors(&self) -> Result<Vec<Monitor>> {
        if !check_command_available("xrandr") {
            return Ok(Vec::new());
        }

        let output = Command::new("xrandr").arg("--query").output()?;
        let stdout = String::from_utf8_lossy(&output.stdout);

        let mut monitors = Vec::new();
        for line in stdout.lines() {
            if line.contains(" connected") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    let name = parts[0].to_string();
                    let primary = line.contains("primary");

                    if let Some(res_part) = parts.iter().find(|p| p.contains('x') && p.contains('+')) {
                        if let Some((res, pos)) = res_part.split_once('+') {
                            if let Some((w, h)) = res.split_once('x') {
                                if let (Ok(width), Ok(height)) = (w.parse::<u32>(), h.parse::<u32>()) {
                                    let coords: Vec<&str> = pos.split('+').collect();
                                    let x = coords.get(0).and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);
                                    let y = coords.get(1).and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);

                                    monitors.push(Monitor {
                                        name,
                                        width,
                                        height,
                                        x,
                                        y,
                                        primary,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(monitors)
    }

    fn stop(&self) -> Result<()> {
        Ok(())
    }
}
