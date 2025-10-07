use crate::adapter::AdapterTrait;
use crate::detector::check_command_available;
use wallmgr_core::error::{Error, Result};
use wallmgr_core::types::{DisplayMode, Monitor};
use std::path::Path;
use std::process::Command;

/// GNOME adapter using gsettings
pub struct GnomeAdapter;

impl GnomeAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl AdapterTrait for GnomeAdapter {
    fn name(&self) -> &str {
        "gnome"
    }

    fn is_available(&self) -> bool {
        check_command_available("gsettings")
    }

    fn set_wallpaper(&self, path: &Path, _monitor: Option<&str>, mode: DisplayMode) -> Result<()> {
        if !self.is_available() {
            return Err(Error::AdapterUnavailable("gsettings not found".to_string()));
        }

        let uri = format!("file://{}", path.display());

        // Set wallpaper
        let output = Command::new("gsettings")
            .arg("set")
            .arg("org.gnome.desktop.background")
            .arg("picture-uri")
            .arg(&uri)
            .output()?;

        if !output.status.success() {
            return Err(Error::Renderer(format!(
                "gsettings failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        // Set dark mode wallpaper as well
        let _ = Command::new("gsettings")
            .arg("set")
            .arg("org.gnome.desktop.background")
            .arg("picture-uri-dark")
            .arg(&uri)
            .output();

        // Set picture options
        let picture_option = match mode {
            DisplayMode::Fill => "zoom",
            DisplayMode::Fit => "scaled",
            DisplayMode::Stretch => "stretched",
            DisplayMode::Center => "centered",
            DisplayMode::Tile => "wallpaper",
        };

        let _ = Command::new("gsettings")
            .arg("set")
            .arg("org.gnome.desktop.background")
            .arg("picture-options")
            .arg(picture_option)
            .output();

        Ok(())
    }

    fn list_monitors(&self) -> Result<Vec<Monitor>> {
        // GNOME doesn't provide easy CLI access to monitor info
        // Use xrandr if available
        if check_command_available("xrandr") {
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
        } else {
            Ok(Vec::new())
        }
    }

    fn stop(&self) -> Result<()> {
        Ok(())
    }
}

/// KDE adapter using qdbus
pub struct KdeAdapter;

impl KdeAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl AdapterTrait for KdeAdapter {
    fn name(&self) -> &str {
        "kde"
    }

    fn is_available(&self) -> bool {
        check_command_available("qdbus") || check_command_available("qdbus-qt5")
    }

    fn set_wallpaper(&self, path: &Path, _monitor: Option<&str>, _mode: DisplayMode) -> Result<()> {
        let qdbus_cmd = if check_command_available("qdbus") {
            "qdbus"
        } else if check_command_available("qdbus-qt5") {
            "qdbus-qt5"
        } else {
            return Err(Error::AdapterUnavailable("qdbus not found".to_string()));
        };

        let path_str = path.to_string_lossy();

        // KDE Plasma 5/6 uses different methods
        // Try Plasma 6 first
        let script = format!(
            r#"
            const allDesktops = desktops();
            for (const desktop of allDesktops) {{
                desktop.currentConfigGroup = ["Wallpaper", "org.kde.image", "General"];
                desktop.writeConfig("Image", "file://{}");
            }}
            "#,
            path_str
        );

        let output = Command::new(qdbus_cmd)
            .arg("org.kde.plasmashell")
            .arg("/PlasmaShell")
            .arg("org.kde.PlasmaShell.evaluateScript")
            .arg(script)
            .output()?;

        if !output.status.success() {
            return Err(Error::Renderer(format!(
                "qdbus failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        Ok(())
    }

    fn list_monitors(&self) -> Result<Vec<Monitor>> {
        if check_command_available("xrandr") {
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
        } else {
            Ok(Vec::new())
        }
    }

    fn stop(&self) -> Result<()> {
        Ok(())
    }
}

/// XFCE adapter using xfconf-query
pub struct XfceAdapter;

impl XfceAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl AdapterTrait for XfceAdapter {
    fn name(&self) -> &str {
        "xfce"
    }

    fn is_available(&self) -> bool {
        check_command_available("xfconf-query")
    }

    fn set_wallpaper(&self, path: &Path, monitor: Option<&str>, _mode: DisplayMode) -> Result<()> {
        if !self.is_available() {
            return Err(Error::AdapterUnavailable("xfconf-query not found".to_string()));
        }

        let path_str = path.to_string_lossy();

        // Get list of properties
        let output = Command::new("xfconf-query")
            .arg("-c")
            .arg("xfce4-desktop")
            .arg("-l")
            .output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        // Find backdrop properties
        let backdrop_props: Vec<&str> = stdout
            .lines()
            .filter(|line| line.contains("/backdrop/screen0/") && line.ends_with("/last-image"))
            .collect();

        if backdrop_props.is_empty() {
            return Err(Error::Renderer("No XFCE backdrop properties found".to_string()));
        }

        // Set wallpaper for each property (or specific monitor)
        for prop in backdrop_props {
            if let Some(mon_name) = monitor {
                // Only set for specific monitor
                if !prop.contains(mon_name) {
                    continue;
                }
            }

            let output = Command::new("xfconf-query")
                .arg("-c")
                .arg("xfce4-desktop")
                .arg("-p")
                .arg(prop)
                .arg("-s")
                .arg(path_str.as_ref())
                .output()?;

            if !output.status.success() {
                return Err(Error::Renderer(format!(
                    "xfconf-query failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                )));
            }
        }

        Ok(())
    }

    fn list_monitors(&self) -> Result<Vec<Monitor>> {
        if check_command_available("xrandr") {
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
        } else {
            Ok(Vec::new())
        }
    }

    fn stop(&self) -> Result<()> {
        Ok(())
    }
}
