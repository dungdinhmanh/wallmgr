use crate::booru::{BooruClient, BooruConnector, parse_rating};
use async_trait::async_trait;
use serde::Deserialize;
use wallmgr_core::error::{Error, Result};
use wallmgr_core::types::BooruImage;

const API_BASE: &str = "https://yande.re";

#[derive(Debug, Deserialize)]
struct YanderePost {
    id: i64,
    #[serde(default)]
    file_url: Option<String>,
    #[serde(default)]
    sample_url: Option<String>,
    #[serde(default)]
    preview_url: Option<String>,
    width: u32,
    height: u32,
    #[serde(default)]
    tags: String,
    rating: String,
    #[serde(default)]
    score: Option<i32>,
    #[serde(default)]
    author: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct YandereTag {
    name: String,
    count: i64,
}

pub struct YandereConnector {
    client: BooruClient,
}

impl YandereConnector {
    pub fn new() -> Self {
        Self {
            client: BooruClient::new(),
        }
    }
}

impl Default for YandereConnector {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl BooruConnector for YandereConnector {
    fn name(&self) -> &str {
        "yandere"
    }

    async fn search(&self, tags: &[String], limit: u32, page: u32) -> Result<Vec<BooruImage>> {
        let tags_str = tags.join(" ");
        let url = format!(
            "{}/post.json?tags={}&limit={}&page={}",
            API_BASE,
            urlencoding::encode(&tags_str),
            limit.min(100),
            page
        );

        let response = self.client.client()
            .get(&url)
            .send()
            .await
            .map_err(|e| Error::Renderer(format!("Yande.re API error: {}", e)))?;

        if !response.status().is_success() {
            return Err(Error::Renderer(format!(
                "Yande.re API returned status: {}",
                response.status()
            )));
        }

        let posts: Vec<YanderePost> = response
            .json()
            .await
            .map_err(|e| Error::Renderer(format!("Failed to parse Yande.re response: {}", e)))?;

        let images = posts
            .into_iter()
            .filter_map(|post| {
                let file_url = post.file_url?;
                let is_nsfw = post.rating != "s";

                Some(BooruImage {
                    id: post.id.to_string(),
                    source: "yandere".to_string(),
                    file_url,
                    preview_url: post.preview_url,
                    sample_url: post.sample_url,
                    width: post.width,
                    height: post.height,
                    tags: post.tags.split_whitespace().map(|s| s.to_string()).collect(),
                    rating: parse_rating(&post.rating),
                    score: post.score,
                    author: post.author,
                    is_nsfw,
                })
            })
            .collect();

        Ok(images)
    }

    async fn autocomplete_tags(&self, prefix: &str, limit: u32) -> Result<Vec<String>> {
        let url = format!(
            "{}/tag.json?name={}&limit={}&order=count",
            API_BASE,
            urlencoding::encode(prefix),
            limit.min(20)
        );

        let response = self.client.client()
            .get(&url)
            .send()
            .await
            .map_err(|e| Error::Renderer(format!("Yande.re autocomplete error: {}", e)))?;

        if !response.status().is_success() {
            return Ok(Vec::new());
        }

        let tags: Vec<YandereTag> = response
            .json()
            .await
            .unwrap_or_default();

        Ok(tags.into_iter().map(|t| t.name).collect())
    }

    async fn get_image(&self, id: &str) -> Result<BooruImage> {
        let url = format!("{}/post.json?tags=id:{}", API_BASE, id);

        let response = self.client.client()
            .get(&url)
            .send()
            .await
            .map_err(|e| Error::Renderer(format!("Yande.re API error: {}", e)))?;

        if !response.status().is_success() {
            return Err(Error::NotFound(format!("Post {} not found", id)));
        }

        let mut posts: Vec<YanderePost> = response
            .json()
            .await
            .map_err(|e| Error::Renderer(format!("Failed to parse Yande.re response: {}", e)))?;

        let post = posts.pop()
            .ok_or_else(|| Error::NotFound(format!("Post {} not found", id)))?;

        let file_url = post.file_url
            .ok_or_else(|| Error::NotFound("No file URL found".to_string()))?;
        
        let is_nsfw = post.rating != "s";

        Ok(BooruImage {
            id: post.id.to_string(),
            source: "yandere".to_string(),
            file_url,
            preview_url: post.preview_url,
            sample_url: post.sample_url,
            width: post.width,
            height: post.height,
            tags: post.tags.split_whitespace().map(|s| s.to_string()).collect(),
            rating: parse_rating(&post.rating),
            score: post.score,
            author: post.author,
            is_nsfw,
        })
    }
}
