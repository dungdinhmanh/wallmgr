// Standalone connector test tool
// Compile with: rustc test_connector.rs --edition 2021
// Or add to Cargo.toml as a binary

use std::error::Error;

// Mock types for testing
#[derive(Debug)]
struct BooruImage {
    url: String,
    width: u32,
    height: u32,
    tags: Vec<String>,
    is_nsfw: bool,
}

#[derive(Debug)]
struct WallpaperSearchFilter {
    min_width: u32,
    min_height: u32,
    aspect_ratio_min: f32,
    aspect_ratio_max: f32,
    allow_nsfw: bool,
}

impl WallpaperSearchFilter {
    fn hd_landscape() -> Self {
        Self {
            min_width: 1920,
            min_height: 1080,
            aspect_ratio_min: 1.3,
            aspect_ratio_max: 2.4,
            allow_nsfw: false,
        }
    }

    fn matches(&self, image: &BooruImage) -> bool {
        // Check dimensions
        if image.width < self.min_width || image.height < self.min_height {
            return false;
        }

        // Calculate aspect ratio
        let aspect_ratio = image.width as f32 / image.height as f32;

        // Check if portrait or square (reject)
        if aspect_ratio <= 1.0 {
            return false;
        }

        // Check aspect ratio range
        if aspect_ratio < self.aspect_ratio_min || aspect_ratio > self.aspect_ratio_max {
            return false;
        }

        // Check NSFW
        if !self.allow_nsfw && image.is_nsfw {
            return false;
        }

        true
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== Wallmgr Connector Test Tool ===\n");

    // Test cases
    let test_images = vec![
        BooruImage {
            url: "test1.jpg".to_string(),
            width: 1920,
            height: 1080,
            tags: vec!["landscape".to_string()],
            is_nsfw: false,
        },
        BooruImage {
            url: "test2.jpg".to_string(),
            width: 2560,
            height: 1440,
            tags: vec!["nature".to_string()],
            is_nsfw: false,
        },
        BooruImage {
            url: "test3.jpg".to_string(),
            width: 1080,
            height: 1920,
            tags: vec!["portrait".to_string()],
            is_nsfw: false,
        },
        BooruImage {
            url: "test4.jpg".to_string(),
            width: 3840,
            height: 2160,
            tags: vec!["4k".to_string()],
            is_nsfw: false,
        },
    ];

    let filter = WallpaperSearchFilter::hd_landscape();

    println!("Filter Settings:");
    println!("  Min Resolution: {}x{}", filter.min_width, filter.min_height);
    println!("  Aspect Ratio: {:.2} - {:.2}", filter.aspect_ratio_min, filter.aspect_ratio_max);
    println!("  Allow NSFW: {}\n", filter.allow_nsfw);

    println!("Testing images:");
    for image in &test_images {
        let matches = filter.matches(image);
        let aspect_ratio = image.width as f32 / image.height as f32;
        let status = if matches { "✓ PASS" } else { "✗ REJECT" };
        
        println!("  {} - {}x{} (AR: {:.2}) - {:?}",
            status,
            image.width,
            image.height,
            aspect_ratio,
            image.url
        );
    }

    println!("\n=== Test Complete ===");
    println!("\nNote: For live API testing, you need to:");
    println!("  1. Re-enable backend/api in Cargo.toml");
    println!("  2. Refactor Database to use tokio-rusqlite (async)");
    println!("  3. Start API server: cargo run --bin wallmgr-api");
    println!("  4. Then use: wallmgr search --tags landscape --source konachan");

    Ok(())
}
