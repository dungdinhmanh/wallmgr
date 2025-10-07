use wallmgr_core::types::{DisplayMode, Monitor};
use wallmgr_core::error::Result;
use std::path::Path;

/// Trait for wallpaper adapters
pub trait AdapterTrait: Send + Sync {
    /// Get adapter name
    fn name(&self) -> &str;

    /// Check if adapter is available on system
    fn is_available(&self) -> bool;

    /// Set wallpaper for specific monitor (None = all monitors)
    fn set_wallpaper(&self, path: &Path, monitor: Option<&str>, mode: DisplayMode) -> Result<()>;

    /// Get list of available monitors
    fn list_monitors(&self) -> Result<Vec<Monitor>>;

    /// Stop any running wallpaper processes
    fn stop(&self) -> Result<()>;
}

pub enum Adapter {
    // X11 adapters
    Feh(crate::x11::FehAdapter),
    Nitrogen(crate::x11::NitrogenAdapter),
    XWallpaper(crate::x11::XWallpaperAdapter),

    // Wayland adapters
    Swww(crate::wayland::SwwwAdapter),
    Hyprpaper(crate::wayland::HyprpaperAdapter),
    Swaybg(crate::wayland::SwaybgAdapter),

    // Desktop environment adapters
    Gnome(crate::desktop::GnomeAdapter),
    Kde(crate::desktop::KdeAdapter),
    Xfce(crate::desktop::XfceAdapter),
}

impl Adapter {
    pub fn get_trait(&self) -> &dyn AdapterTrait {
        match self {
            Adapter::Feh(a) => a,
            Adapter::Nitrogen(a) => a,
            Adapter::XWallpaper(a) => a,
            Adapter::Swww(a) => a,
            Adapter::Hyprpaper(a) => a,
            Adapter::Swaybg(a) => a,
            Adapter::Gnome(a) => a,
            Adapter::Kde(a) => a,
            Adapter::Xfce(a) => a,
        }
    }
}
