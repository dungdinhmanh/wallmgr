use crate::booru::{BooruClient, BooruConnector, parse_rating};
use wallmgr_core::error::{Error, Result};
use wallmgr_core::types::BooruImage;
use async_trait::async_trait;
use serde::Deserialize;

/// Konachan connector - Specialized for desktop wallpapers
/// API: https://konachan.net/help/api
pub struct KonachanConnector {
    client: BooruClient,
    base_url: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct KonachanPost {
    id: u64,
    tags: String,
    created_at: u64,
    creator_id: u64,
    author: String,
    source: Option<String>,
    score: i32,
    md5: String,
    file_size: u64,
    file_url: String,
    preview_url: String,
    sample_url: Option<String>,
    width: u32,
    height: u32,
    rating: String,
}

impl KonachanConnector {
    pub fn new() -> Self {
        Self {
            client: BooruClient::new(),
            base_url: "https://konachan.net".to_string(),
        }
    }
    
    pub fn with_mirror(mirror_url: &str) -> Self {
        Self {
            client: BooruClient::new(),
            base_url: mirror_url.to_string(),
        }
    }
    
    fn post_to_image(&self, post: KonachanPost) -> BooruImage {
        let is_nsfw = post.rating != "s";
        
        BooruImage {
            id: post.id.to_string(),
            source: "konachan".to_string(),
            file_url: post.file_url.clone(),
            preview_url: Some(post.preview_url),
            sample_url: post.sample_url.or(Some(post.file_url)),
            width: post.width,
            height: post.height,
            tags: post.tags.split_whitespace()
                .map(|s| s.to_string())
                .collect(),
            rating: parse_rating(&post.rating),
            score: Some(post.score),
            author: Some(post.author),
            is_nsfw,
        }
    }
}

impl Default for KonachanConnector {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl BooruConnector for KonachanConnector {
    fn name(&self) -> &str {
        "konachan"
    }
    
    async fn search(&self, tags: &[String], limit: u32, page: u32) -> Result<Vec<BooruImage>> {
        let tags_str = tags.join(" ");
        
        let url = format!(
            "{}/post.json?tags={}&limit={}&page={}",
            self.base_url,
            urlencoding::encode(&tags_str),
            limit.min(100), // Konachan max: 100
            page
        );
        
        let response = self.client.client()
            .get(&url)
            .send()
            .await
            .map_err(|e| Error::Network(e.to_string()))?;
        
        if !response.status().is_success() {
            return Err(Error::Api(format!(
                "Konachan API error: {}",
                response.status()
            )));
        }
        
        let posts: Vec<KonachanPost> = response
            .json()
            .await
            .map_err(|e| Error::Parse(e.to_string()))?;
        
        Ok(posts.into_iter()
            .map(|p| self.post_to_image(p))
            .collect())
    }
    
    async fn autocomplete_tags(&self, prefix: &str, limit: u32) -> Result<Vec<String>> {
        let url = format!(
            "{}/tag.json?order=count&name={}*&limit={}",
            self.base_url,
            urlencoding::encode(prefix),
            limit.min(20)
        );
        
        let response = self.client.client()
            .get(&url)
            .send()
            .await
            .map_err(|e| Error::Network(e.to_string()))?;
        
        #[derive(Deserialize)]
        struct Tag {
            name: String,
        }
        
        let tags: Vec<Tag> = response
            .json()
            .await
            .map_err(|e| Error::Parse(e.to_string()))?;
        
        Ok(tags.into_iter()
            .map(|t| t.name)
            .collect())
    }
    
    async fn get_image(&self, id: &str) -> Result<BooruImage> {
        let url = format!("{}/post.json?tags=id:{}", self.base_url, id);
        
        let response = self.client.client()
            .get(&url)
            .send()
            .await
            .map_err(|e| Error::Network(e.to_string()))?;
        
        let posts: Vec<KonachanPost> = response
            .json()
            .await
            .map_err(|e| Error::Parse(e.to_string()))?;
        
        posts.into_iter()
            .next()
            .map(|p| self.post_to_image(p))
            .ok_or_else(|| Error::NotFound(format!("Image {} not found", id)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_konachan_search() {
        let connector = KonachanConnector::new();
        
        // Search for landscape wallpapers
        let results = connector.search(
            &["landscape".to_string(), "rating:safe".to_string()],
            10,
            1
        ).await;
        
        assert!(results.is_ok());
        let images = results.unwrap();
        assert!(!images.is_empty());
        
        // Check first image has required fields
        if let Some(img) = images.first() {
            assert!(!img.id.is_empty());
            assert!(!img.file_url.is_empty());
            assert!(img.width > 0);
            assert!(img.height > 0);
        }
    }
    
    #[tokio::test]
    async fn test_konachan_autocomplete() {
        let connector = KonachanConnector::new();
        
        let tags = connector.autocomplete_tags("lands", 5).await;
        
        assert!(tags.is_ok());
        let tag_list = tags.unwrap();
        assert!(!tag_list.is_empty());
    }
}
