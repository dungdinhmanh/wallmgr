use crate::booru::{BooruClient, BooruConnector, parse_rating};
use async_trait::async_trait;
use serde::Deserialize;
use wallmgr_core::error::{Error, Result};
use wallmgr_core::types::{BooruImage, Rating};

const API_BASE: &str = "https://safebooru.org";

#[derive(Debug, Deserialize)]
struct SafebooruPost {
    id: i64,
    #[serde(default)]
    image: Option<String>,
    width: u32,
    height: u32,
    #[serde(default)]
    tags: String,
    #[serde(default)]
    score: Option<i32>,
    #[serde(default)]
    owner: Option<String>,
}

pub struct SafebooruConnector {
    client: BooruClient,
}

impl SafebooruConnector {
    pub fn new() -> Self {
        Self {
            client: BooruClient::new(),
        }
    }
}

impl Default for SafebooruConnector {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl BooruConnector for SafebooruConnector {
    fn name(&self) -> &str {
        "safebooru"
    }

    async fn search(&self, tags: &[String], limit: u32, page: u32) -> Result<Vec<BooruImage>> {
        let tags_str = tags.join(" ");
        let url = format!(
            "{}/index.php?page=dapi&s=post&q=index&json=1&tags={}&limit={}&pid={}",
            API_BASE,
            urlencoding::encode(&tags_str),
            limit.min(100),
            page
        );

        let response = self.client.client()
            .get(&url)
            .send()
            .await
            .map_err(|e| Error::Renderer(format!("Safebooru API error: {}", e)))?;

        if !response.status().is_success() {
            return Err(Error::Renderer(format!(
                "Safebooru API returned status: {}",
                response.status()
            )));
        }

        let posts: Vec<SafebooruPost> = response
            .json()
            .await
            .map_err(|e| Error::Renderer(format!("Failed to parse Safebooru response: {}", e)))?;

        let images = posts
            .into_iter()
            .filter_map(|post| {
                let image = post.image?;
                let file_url = format!("https://safebooru.org/images/{}", image);

                Some(BooruImage {
                    id: post.id.to_string(),
                    source: "safebooru".to_string(),
                    file_url,
                    preview_url: None,
                    sample_url: None,
                    width: post.width,
                    height: post.height,
                    tags: post.tags.split_whitespace().map(|s| s.to_string()).collect(),
                    rating: Rating::Safe, // Safebooru only has safe content
                    score: post.score,
                    author: post.owner,
                })
            })
            .collect();

        Ok(images)
    }

    async fn autocomplete_tags(&self, prefix: &str, limit: u32) -> Result<Vec<String>> {
        let url = format!(
            "{}/index.php?page=dapi&s=tag&q=index&json=1&name_pattern={}%&limit={}",
            API_BASE,
            urlencoding::encode(prefix),
            limit.min(20)
        );

        let response = self.client.client()
            .get(&url)
            .send()
            .await
            .map_err(|e| Error::Renderer(format!("Safebooru autocomplete error: {}", e)))?;

        if !response.status().is_success() {
            return Ok(Vec::new());
        }

        #[derive(Deserialize)]
        struct Tag {
            name: String,
        }

        let tags: Vec<Tag> = response
            .json()
            .await
            .unwrap_or_default();

        Ok(tags.into_iter().map(|t| t.name).collect())
    }

    async fn get_image(&self, id: &str) -> Result<BooruImage> {
        let url = format!(
            "{}/index.php?page=dapi&s=post&q=index&json=1&id={}",
            API_BASE, id
        );

        let response = self.client.client()
            .get(&url)
            .send()
            .await
            .map_err(|e| Error::Renderer(format!("Safebooru API error: {}", e)))?;

        if !response.status().is_success() {
            return Err(Error::NotFound(format!("Post {} not found", id)));
        }

        let mut posts: Vec<SafebooruPost> = response
            .json()
            .await
            .map_err(|e| Error::Renderer(format!("Failed to parse Safebooru response: {}", e)))?;

        let post = posts.pop()
            .ok_or_else(|| Error::NotFound(format!("Post {} not found", id)))?;

        let image = post.image
            .ok_or_else(|| Error::NotFound("No image found".to_string()))?;

        let file_url = format!("https://safebooru.org/images/{}", image);

        Ok(BooruImage {
            id: post.id.to_string(),
            source: "safebooru".to_string(),
            file_url,
            preview_url: None,
            sample_url: None,
            width: post.width,
            height: post.height,
            tags: post.tags.split_whitespace().map(|s| s.to_string()).collect(),
            rating: Rating::Safe,
            score: post.score,
            author: post.owner,
        })
    }
}
