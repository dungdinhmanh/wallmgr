use wallmgr_core::error::Result;
use std::path::Path;

pub trait RendererTrait: Send + Sync {
    /// Get renderer name
    fn name(&self) -> &str;

    /// Check if renderer is available
    fn is_available(&self) -> bool;

    /// Start rendering wallpaper
    fn start(&mut self, path: &Path, monitor: Option<&str>) -> Result<()>;

    /// Stop rendering
    fn stop(&mut self) -> Result<()>;

    /// Check if currently running
    fn is_running(&self) -> bool;
}

pub enum Renderer {
    Video(crate::video::VideoRenderer),
    Spine(crate::spine::SpineRenderer),
    WallpaperEngine(crate::wallpaper_engine::WallpaperEngineRenderer),
}

impl Renderer {
    pub fn get_trait_mut(&mut self) -> &mut dyn RendererTrait {
        match self {
            Renderer::Video(r) => r,
            Renderer::Spine(r) => r,
            Renderer::WallpaperEngine(r) => r,
        }
    }
}
