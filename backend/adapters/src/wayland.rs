use crate::adapter::AdapterTrait;
use crate::detector::check_command_available;
use wallmgr_core::error::{Error, Result};
use wallmgr_core::types::{DisplayMode, Monitor};
use std::path::Path;
use std::process::Command;

/// swww adapter - Wayland wallpaper daemon with transitions
pub struct SwwwAdapter;

impl SwwwAdapter {
    pub fn new() -> Self {
        Self
    }

    fn ensure_daemon(&self) -> Result<()> {
        // Check if swww-daemon is running
        let status = Command::new("pgrep")
            .arg("-x")
            .arg("swww-daemon")
            .status()?;

        if !status.success() {
            // Start daemon
            Command::new("swww-daemon")
                .spawn()
                .map_err(|e| Error::Renderer(format!("Failed to start swww-daemon: {}", e)))?;

            // Wait a bit for daemon to start
            std::thread::sleep(std::time::Duration::from_millis(500));
        }

        Ok(())
    }
}

impl AdapterTrait for SwwwAdapter {
    fn name(&self) -> &str {
        "swww"
    }

    fn is_available(&self) -> bool {
        check_command_available("swww")
    }

    fn set_wallpaper(&self, path: &Path, monitor: Option<&str>, mode: DisplayMode) -> Result<()> {
        if !self.is_available() {
            return Err(Error::AdapterUnavailable("swww not found".to_string()));
        }

        self.ensure_daemon()?;

        let resize_mode = match mode {
            DisplayMode::Fill => "crop",
            DisplayMode::Fit => "fit",
            DisplayMode::Stretch => "no",
            DisplayMode::Center => "fit",
            DisplayMode::Tile => "no",
        };

        let mut cmd = Command::new("swww");
        cmd.arg("img");

        if let Some(mon) = monitor {
            cmd.arg("--outputs").arg(mon);
        }

        cmd.arg("--resize")
            .arg(resize_mode)
            .arg("--transition-type")
            .arg("fade")
            .arg("--transition-duration")
            .arg("1")
            .arg(path);

        let output = cmd.output()?;
        if !output.status.success() {
            return Err(Error::Renderer(format!(
                "swww failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        Ok(())
    }

    fn list_monitors(&self) -> Result<Vec<Monitor>> {
        if !self.is_available() {
            return Ok(Vec::new());
        }

        self.ensure_daemon()?;

        let output = Command::new("swww").arg("query").output()?;

        if !output.status.success() {
            return Ok(Vec::new());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut monitors = Vec::new();

        for (idx, line) in stdout.lines().enumerate() {
            // swww query output: "eDP-1: 1920x1080"
            if let Some((name, res)) = line.split_once(':') {
                let name = name.trim().to_string();
                if let Some((w, h)) = res.trim().split_once('x') {
                    if let (Ok(width), Ok(height)) = (w.parse::<u32>(), h.parse::<u32>()) {
                        monitors.push(Monitor {
                            name,
                            width,
                            height,
                            x: idx as i32 * width as i32,
                            y: 0,
                            primary: idx == 0,
                        });
                    }
                }
            }
        }

        Ok(monitors)
    }

    fn stop(&self) -> Result<()> {
        // Kill swww-daemon
        let _ = Command::new("pkill")
            .arg("-x")
            .arg("swww-daemon")
            .status();

        Ok(())
    }
}

/// hyprpaper adapter - Hyprland's wallpaper utility
pub struct HyprpaperAdapter;

impl HyprpaperAdapter {
    pub fn new() -> Self {
        Self
    }

    fn ensure_daemon(&self) -> Result<()> {
        let status = Command::new("pgrep")
            .arg("-x")
            .arg("hyprpaper")
            .status()?;

        if !status.success() {
            Command::new("hyprpaper")
                .spawn()
                .map_err(|e| Error::Renderer(format!("Failed to start hyprpaper: {}", e)))?;

            std::thread::sleep(std::time::Duration::from_millis(500));
        }

        Ok(())
    }

    fn send_command(&self, cmd: &str) -> Result<()> {
        let output = Command::new("hyprctl")
            .arg("hyprpaper")
            .arg(cmd)
            .output()?;

        if !output.status.success() {
            return Err(Error::Renderer(format!(
                "hyprpaper command failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        Ok(())
    }
}

impl AdapterTrait for HyprpaperAdapter {
    fn name(&self) -> &str {
        "hyprpaper"
    }

    fn is_available(&self) -> bool {
        check_command_available("hyprpaper") && check_command_available("hyprctl")
    }

    fn set_wallpaper(&self, path: &Path, monitor: Option<&str>, _mode: DisplayMode) -> Result<()> {
        if !self.is_available() {
            return Err(Error::AdapterUnavailable("hyprpaper not found".to_string()));
        }

        self.ensure_daemon()?;

        let path_str = path.to_string_lossy();

        // Preload image
        self.send_command(&format!("preload {}", path_str))?;

        // Set wallpaper
        if let Some(mon) = monitor {
            self.send_command(&format!("wallpaper {},{}", mon, path_str))?;
        } else {
            // Set for all monitors
            self.send_command(&format!("wallpaper ,{}", path_str))?;
        }

        Ok(())
    }

    fn list_monitors(&self) -> Result<Vec<Monitor>> {
        if !check_command_available("hyprctl") {
            return Ok(Vec::new());
        }

        let output = Command::new("hyprctl")
            .arg("monitors")
            .arg("-j")
            .output()?;

        if !output.status.success() {
            return Ok(Vec::new());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let monitors_json: serde_json::Value = serde_json::from_str(&stdout)
            .map_err(|e| Error::Renderer(format!("Failed to parse hyprctl output: {}", e)))?;

        let mut monitors = Vec::new();

        if let Some(arr) = monitors_json.as_array() {
            for mon in arr {
                if let (Some(name), Some(width), Some(height), Some(x), Some(y)) = (
                    mon["name"].as_str(),
                    mon["width"].as_u64(),
                    mon["height"].as_u64(),
                    mon["x"].as_i64(),
                    mon["y"].as_i64(),
                ) {
                    monitors.push(Monitor {
                        name: name.to_string(),
                        width: width as u32,
                        height: height as u32,
                        x: x as i32,
                        y: y as i32,
                        primary: mon["focused"].as_bool().unwrap_or(false),
                    });
                }
            }
        }

        Ok(monitors)
    }

    fn stop(&self) -> Result<()> {
        let _ = Command::new("pkill")
            .arg("-x")
            .arg("hyprpaper")
            .status();

        Ok(())
    }
}

/// swaybg adapter - Sway's background utility
pub struct SwaybgAdapter;

impl SwaybgAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl AdapterTrait for SwaybgAdapter {
    fn name(&self) -> &str {
        "swaybg"
    }

    fn is_available(&self) -> bool {
        check_command_available("swaybg")
    }

    fn set_wallpaper(&self, path: &Path, monitor: Option<&str>, mode: DisplayMode) -> Result<()> {
        if !self.is_available() {
            return Err(Error::AdapterUnavailable("swaybg not found".to_string()));
        }

        // Kill existing swaybg instances
        let _ = Command::new("pkill")
            .arg("-x")
            .arg("swaybg")
            .status();

        let mode_arg = match mode {
            DisplayMode::Fill => "fill",
            DisplayMode::Fit => "fit",
            DisplayMode::Stretch => "stretch",
            DisplayMode::Center => "center",
            DisplayMode::Tile => "tile",
        };

        let mut cmd = Command::new("swaybg");

        if let Some(mon) = monitor {
            cmd.arg("-o").arg(mon);
        }

        cmd.arg("-i")
            .arg(path)
            .arg("-m")
            .arg(mode_arg)
            .spawn()
            .map_err(|e| Error::Renderer(format!("Failed to start swaybg: {}", e)))?;

        Ok(())
    }

    fn list_monitors(&self) -> Result<Vec<Monitor>> {
        if !check_command_available("swaymsg") {
            return Ok(Vec::new());
        }

        let output = Command::new("swaymsg")
            .arg("-t")
            .arg("get_outputs")
            .arg("-r")
            .output()?;

        if !output.status.success() {
            return Ok(Vec::new());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let outputs: serde_json::Value = serde_json::from_str(&stdout)
            .map_err(|e| Error::Renderer(format!("Failed to parse swaymsg output: {}", e)))?;

        let mut monitors = Vec::new();

        if let Some(arr) = outputs.as_array() {
            for output in arr {
                if let (Some(name), Some(rect)) = (output["name"].as_str(), output["rect"].as_object()) {
                    if let (Some(width), Some(height), Some(x), Some(y)) = (
                        rect["width"].as_u64(),
                        rect["height"].as_u64(),
                        rect["x"].as_i64(),
                        rect["y"].as_i64(),
                    ) {
                        monitors.push(Monitor {
                            name: name.to_string(),
                            width: width as u32,
                            height: height as u32,
                            x: x as i32,
                            y: y as i32,
                            primary: output["focused"].as_bool().unwrap_or(false),
                        });
                    }
                }
            }
        }

        Ok(monitors)
    }

    fn stop(&self) -> Result<()> {
        let _ = Command::new("pkill")
            .arg("-x")
            .arg("swaybg")
            .status();

        Ok(())
    }
}
