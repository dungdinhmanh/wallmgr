use wallmgr_core::types::BooruImage;

/// Wallpaper search filters optimized for desktop backgrounds
#[derive(Debug, Clone)]
pub struct WallpaperSearchFilter {
    /// Minimum width in pixels (default: 1920 for Full HD)
    pub min_width: u32,
    
    /// Minimum height in pixels (default: 1080 for Full HD)
    pub min_height: u32,
    
    /// Minimum aspect ratio (default: 1.3 for landscape)
    /// 4:3 = 1.33, 16:9 = 1.77, 21:9 = 2.33
    pub aspect_ratio_min: f64,
    
    /// Maximum aspect ratio (default: 2.4 to exclude ultra-ultrawide)
    pub aspect_ratio_max: f64,
    
    /// Maximum aspect ratio for portrait detection (default: 1.0)
    /// Anything below this is considered portrait and will be filtered out
    pub portrait_threshold: f64,
    
    /// Allow NSFW content (default: false)
    pub allow_nsfw: bool,
}

impl Default for WallpaperSearchFilter {
    fn default() -> Self {
        Self {
            min_width: 1920,
            min_height: 1080,
            aspect_ratio_min: 1.3,   // Slightly wider than 4:3
            aspect_ratio_max: 2.4,   // Just beyond 21:9
            portrait_threshold: 1.0, // Anything <= 1.0 is portrait/square
            allow_nsfw: false,
        }
    }
}

impl WallpaperSearchFilter {
    /// Create filter for 1080p landscape wallpapers (most common)
    pub fn hd_landscape() -> Self {
        Self::default()
    }
    
    /// Create filter for 1440p (2K) landscape wallpapers
    pub fn qhd_landscape() -> Self {
        Self {
            min_width: 2560,
            min_height: 1440,
            ..Self::default()
        }
    }
    
    /// Create filter for 4K landscape wallpapers
    pub fn uhd_landscape() -> Self {
        Self {
            min_width: 3840,
            min_height: 2160,
            ..Self::default()
        }
    }
    
    /// Create filter for ultrawide (21:9) wallpapers
    pub fn ultrawide() -> Self {
        Self {
            min_width: 2560,
            min_height: 1080,
            aspect_ratio_min: 2.0,
            aspect_ratio_max: 2.5,
            ..Self::default()
        }
    }
    
    /// Create filter allowing all aspect ratios (portrait + landscape)
    pub fn any_orientation() -> Self {
        Self {
            aspect_ratio_min: 0.0,
            aspect_ratio_max: 10.0,
            portrait_threshold: 0.0,
            ..Self::default()
        }
    }
    
    /// Check if an image passes this filter
    pub fn matches(&self, image: &BooruImage) -> bool {
        // Check dimensions
        if image.width < self.min_width || image.height < self.min_height {
            return false;
        }
        
        // Calculate aspect ratio
        let aspect_ratio = image.width as f64 / image.height as f64;
        
        // Check if portrait (reject if below threshold)
        if aspect_ratio <= self.portrait_threshold {
            return false;
        }
        
        // Check aspect ratio range
        if aspect_ratio < self.aspect_ratio_min || aspect_ratio > self.aspect_ratio_max {
            return false;
        }
        
        // Check NSFW setting
        if !self.allow_nsfw && image.is_nsfw {
            return false;
        }
        
        true
    }
    
    /// Filter a list of images
    pub fn filter(&self, images: Vec<BooruImage>) -> Vec<BooruImage> {
        images.into_iter()
            .filter(|img| self.matches(img))
            .collect()
    }
    
    /// Get human-readable description of filter
    pub fn description(&self) -> String {
        format!(
            "{}x{}, aspect ratio {:.2}-{:.2}, {}",
            self.min_width,
            self.min_height,
            self.aspect_ratio_min,
            self.aspect_ratio_max,
            if self.allow_nsfw { "NSFW allowed" } else { "SFW only" }
        )
    }
}

/// Helper to construct booru query with size hints
pub fn build_landscape_query(base_tags: &[String], filter: &WallpaperSearchFilter) -> Vec<String> {
    let mut query = base_tags.to_vec();
    
    // Add size-related tags for boorus that support them
    // Note: Not all boorus support these, but they help when available
    match (filter.min_width, filter.min_height) {
        (w, h) if w >= 3840 && h >= 2160 => {
            query.push("absurdres".to_string());
        }
        (w, h) if w >= 1920 && h >= 1080 => {
            query.push("highres".to_string());
        }
        _ => {}
    }
    
    // Add orientation hint (for boorus that support it)
    if filter.aspect_ratio_min > 1.3 {
        // Some boorus support "landscape" tag
        // query.push("landscape".to_string());
    }
    
    // Add SFW/NSFW filter (handled per-booru in connectors)
    
    query
}

#[cfg(test)]
mod tests {
    use super::*;
    use wallmgr_core::types::Rating;
    
    fn create_test_image(width: u32, height: u32, is_nsfw: bool) -> BooruImage {
        BooruImage {
            id: "test".to_string(),
            source: "test".to_string(),
            file_url: "https://example.com/test.jpg".to_string(),
            preview_url: None,
            sample_url: None,
            width,
            height,
            tags: vec![],
            rating: if is_nsfw { Rating::Explicit } else { Rating::Safe },
            score: None,
            author: None,
            is_nsfw,
        }
    }
    
    #[test]
    fn test_hd_landscape_filter() {
        let filter = WallpaperSearchFilter::hd_landscape();
        
        // Should pass: 1920x1080 landscape
        assert!(filter.matches(&create_test_image(1920, 1080, false)));
        
        // Should pass: 2560x1440 landscape
        assert!(filter.matches(&create_test_image(2560, 1440, false)));
        
        // Should fail: too small
        assert!(!filter.matches(&create_test_image(1280, 720, false)));
        
        // Should fail: portrait
        assert!(!filter.matches(&create_test_image(1080, 1920, false)));
        
        // Should fail: NSFW
        assert!(!filter.matches(&create_test_image(1920, 1080, true)));
    }
    
    #[test]
    fn test_aspect_ratio_filtering() {
        let filter = WallpaperSearchFilter::default();
        
        // 16:9 = 1.77 (should pass)
        assert!(filter.matches(&create_test_image(1920, 1080, false)));
        
        // 21:9 = 2.33 (should pass)
        assert!(filter.matches(&create_test_image(2560, 1080, false)));
        
        // 4:3 = 1.33 (should pass, just above threshold)
        assert!(filter.matches(&create_test_image(1920, 1440, false)));
        
        // 1:1 square (should fail)
        assert!(!filter.matches(&create_test_image(1920, 1920, false)));
        
        // 9:16 portrait (should fail)
        assert!(!filter.matches(&create_test_image(1080, 1920, false)));
    }
    
    #[test]
    fn test_ultrawide_filter() {
        let filter = WallpaperSearchFilter::ultrawide();
        
        // Should pass: 21:9
        assert!(filter.matches(&create_test_image(2560, 1080, false)));
        
        // Should pass: 21:10
        assert!(filter.matches(&create_test_image(3440, 1440, false)));
        
        // Should fail: 16:9 (too narrow for ultrawide)
        assert!(!filter.matches(&create_test_image(1920, 1080, false)));
    }
    
    #[test]
    fn test_filter_list() {
        let filter = WallpaperSearchFilter::hd_landscape();
        
        let images = vec![
            create_test_image(1920, 1080, false), // Pass
            create_test_image(1280, 720, false),  // Fail: too small
            create_test_image(2560, 1440, false), // Pass
            create_test_image(1080, 1920, false), // Fail: portrait
            create_test_image(1920, 1080, true),  // Fail: NSFW
        ];
        
        let filtered = filter.filter(images);
        assert_eq!(filtered.len(), 2);
    }
}
