use std::env;
use std::process::Command;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    X11,
    Wayland,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DesktopEnvironment {
    Gnome,
    Kde,
    Xfce,
    LxQt,
    Cinnamon,
    Mate,
    Hyprland,
    Sway,
    I3,
    Niri,
    Openbox,
    Unknown,
}

pub fn detect_environment() -> Option<Environment> {
    // Check XDG_SESSION_TYPE first
    if let Ok(session_type) = env::var("XDG_SESSION_TYPE") {
        match session_type.to_lowercase().as_str() {
            "wayland" => return Some(Environment::Wayland),
            "x11" => return Some(Environment::X11),
            _ => {}
        }
    }

    // Check WAYLAND_DISPLAY
    if env::var("WAYLAND_DISPLAY").is_ok() {
        return Some(Environment::Wayland);
    }

    // Check DISPLAY for X11
    if env::var("DISPLAY").is_ok() {
        return Some(Environment::X11);
    }

    None
}

pub fn detect_desktop_environment() -> DesktopEnvironment {
    // Check for specific WM/DE environment variables
    if let Ok(desktop) = env::var("XDG_CURRENT_DESKTOP") {
        let desktop_lower = desktop.to_lowercase();
        if desktop_lower.contains("gnome") {
            return DesktopEnvironment::Gnome;
        } else if desktop_lower.contains("kde") {
            return DesktopEnvironment::Kde;
        } else if desktop_lower.contains("xfce") {
            return DesktopEnvironment::Xfce;
        } else if desktop_lower.contains("lxqt") {
            return DesktopEnvironment::LxQt;
        } else if desktop_lower.contains("cinnamon") {
            return DesktopEnvironment::Cinnamon;
        } else if desktop_lower.contains("mate") {
            return DesktopEnvironment::Mate;
        }
    }

    // Check for Hyprland
    if env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok() {
        return DesktopEnvironment::Hyprland;
    }

    // Check for Sway
    if env::var("SWAYSOCK").is_ok() {
        return DesktopEnvironment::Sway;
    }

    // Check for i3
    if env::var("I3SOCK").is_ok() {
        return DesktopEnvironment::I3;
    }

    // Try to detect by running processes
    if is_process_running("hyprland") {
        return DesktopEnvironment::Hyprland;
    }
    if is_process_running("sway") {
        return DesktopEnvironment::Sway;
    }
    if is_process_running("i3") {
        return DesktopEnvironment::I3;
    }
    if is_process_running("niri") {
        return DesktopEnvironment::Niri;
    }
    if is_process_running("openbox") {
        return DesktopEnvironment::Openbox;
    }

    DesktopEnvironment::Unknown
}

fn is_process_running(process_name: &str) -> bool {
    Command::new("pgrep")
        .arg("-x")
        .arg(process_name)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

pub fn check_command_available(command: &str) -> bool {
    Command::new("which")
        .arg(command)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_environment() {
        let env = detect_environment();
        println!("Detected environment: {:?}", env);
    }

    #[test]
    fn test_detect_de() {
        let de = detect_desktop_environment();
        println!("Detected DE: {:?}", de);
    }
}
