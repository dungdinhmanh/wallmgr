use crate::booru::{BooruClient, BooruConnector};
use wallmgr_core::error::{Error, Result};
use wallmgr_core::types::{BooruImage, Rating};
use async_trait::async_trait;
use serde::Deserialize;

/// WallHaven connector - Premium wallpaper site
/// API: https://wallhaven.cc/help/api
/// Note: Requires API key for NSFW access
pub struct WallHavenConnector {
    client: BooruClient,
    base_url: String,
    api_key: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct WallHavenResponse {
    data: Vec<WallHavenData>,
    meta: WallHavenMeta,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct WallHavenData {
    id: String,
    url: String,
    short_url: String,
    views: u64,
    favorites: u64,
    source: Option<String>,
    purity: String, // "sfw", "sketchy", "nsfw"
    category: String, // "general", "anime", "people"
    dimension_x: u32,
    dimension_y: u32,
    resolution: String,
    ratio: String,
    file_size: u64,
    file_type: String,
    created_at: String,
    colors: Vec<String>,
    path: String,
    thumbs: WallHavenThumbs,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct WallHavenThumbs {
    large: String,
    original: String,
    small: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct WallHavenMeta {
    current_page: u32,
    last_page: u32,
    per_page: u32,
    total: u64,
}

impl WallHavenConnector {
    pub fn new() -> Self {
        Self {
            client: BooruClient::new(),
            base_url: "https://wallhaven.cc/api/v1".to_string(),
            api_key: None,
        }
    }
    
    pub fn with_api_key(api_key: impl Into<String>) -> Self {
        Self {
            client: BooruClient::new(),
            base_url: "https://wallhaven.cc/api/v1".to_string(),
            api_key: Some(api_key.into()),
        }
    }
    
    fn data_to_image(&self, data: WallHavenData) -> BooruImage {
        // Parse purity to rating
        let (rating, is_nsfw) = match data.purity.as_str() {
            "sfw" => (Rating::Safe, false),
            "sketchy" => (Rating::Questionable, true),
            "nsfw" => (Rating::Explicit, true),
            _ => (Rating::Safe, false),
        };
        
        // Extract tags from colors and category
        let mut tags = vec![data.category.clone()];
        tags.extend(data.colors.iter().cloned());
        
        BooruImage {
            id: data.id.clone(),
            source: "wallhaven".to_string(),
            file_url: data.path,
            preview_url: Some(data.thumbs.small),
            sample_url: Some(data.thumbs.large),
            width: data.dimension_x,
            height: data.dimension_y,
            tags,
            rating,
            score: Some(data.favorites as i32),
            author: None,
            is_nsfw,
        }
    }
}

impl Default for WallHavenConnector {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl BooruConnector for WallHavenConnector {
    fn name(&self) -> &str {
        "wallhaven"
    }
    
    async fn search(&self, tags: &[String], limit: u32, page: u32) -> Result<Vec<BooruImage>> {
        // WallHaven uses "q" parameter for search query
        let query = tags.join(" ");
        
        let mut url = format!(
            "{}/search?q={}&page={}",
            self.base_url,
            urlencoding::encode(&query),
            page
        );
        
        // Add API key if available (required for NSFW)
        if let Some(key) = &self.api_key {
            url.push_str(&format!("&apikey={}", key));
            url.push_str("&purity=111"); // sfw+sketchy+nsfw
        } else {
            url.push_str("&purity=100"); // sfw only without API key
        }
        
        // Add categories (general, anime, people)
        url.push_str("&categories=111");
        
        // Sorting options: date_added, relevance, random, views, favorites
        url.push_str("&sorting=relevance");
        
        // Aspect ratios: landscape, portrait
        url.push_str("&ratios=landscape");
        
        let response = self.client.client()
            .get(&url)
            .send()
            .await
            .map_err(|e| Error::Network(e.to_string()))?;
        
        if !response.status().is_success() {
            return Err(Error::Api(format!(
                "WallHaven API error: {}",
                response.status()
            )));
        }
        
        let wallhaven_response: WallHavenResponse = response
            .json()
            .await
            .map_err(|e| Error::Parse(e.to_string()))?;
        
        let images: Vec<BooruImage> = wallhaven_response.data
            .into_iter()
            .take(limit as usize)
            .map(|d| self.data_to_image(d))
            .collect();
        
        Ok(images)
    }
    
    async fn autocomplete_tags(&self, prefix: &str, _limit: u32) -> Result<Vec<String>> {
        // WallHaven doesn't have traditional tag autocomplete
        // Return common categories and colors
        let suggestions = vec![
            "general", "anime", "people",
            "landscape", "abstract", "nature",
            "city", "space", "fantasy",
        ];
        
        Ok(suggestions.iter()
            .filter(|s| s.starts_with(prefix))
            .map(|s| s.to_string())
            .collect())
    }
    
    async fn get_image(&self, id: &str) -> Result<BooruImage> {
        let mut url = format!("{}/w/{}", self.base_url, id);
        
        if let Some(key) = &self.api_key {
            url.push_str(&format!("?apikey={}", key));
        }
        
        let response = self.client.client()
            .get(&url)
            .send()
            .await
            .map_err(|e| Error::Network(e.to_string()))?;
        
        if !response.status().is_success() {
            return Err(Error::NotFound(format!(
                "WallHaven image {} not found",
                id
            )));
        }
        
        #[derive(Deserialize)]
        struct SingleResponse {
            data: WallHavenData,
        }
        
        let single: SingleResponse = response
            .json()
            .await
            .map_err(|e| Error::Parse(e.to_string()))?;
        
        Ok(self.data_to_image(single.data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_wallhaven_search() {
        let connector = WallHavenConnector::new();
        
        // Search for landscape nature wallpapers
        let results = connector.search(
            &["nature".to_string()],
            10,
            1
        ).await;
        
        assert!(results.is_ok());
        let images = results.unwrap();
        assert!(!images.is_empty());
        
        // All should be landscape
        for img in images {
            let aspect_ratio = img.width as f64 / img.height as f64;
            assert!(aspect_ratio > 1.0, "Should be landscape orientation");
        }
    }
    
    #[tokio::test]
    async fn test_wallhaven_autocomplete() {
        let connector = WallHavenConnector::new();
        
        let tags = connector.autocomplete_tags("land", 5).await;
        
        assert!(tags.is_ok());
        let tag_list = tags.unwrap();
        assert!(tag_list.contains(&"landscape".to_string()));
    }
}
