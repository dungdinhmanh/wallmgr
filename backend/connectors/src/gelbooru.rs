use crate::booru::{BooruClient, BooruConnector, parse_rating};
use async_trait::async_trait;
use serde::Deserialize;
use wallmgr_core::error::{Error, Result};
use wallmgr_core::types::BooruImage;

const API_BASE: &str = "https://gelbooru.com";

#[derive(Debug, Deserialize)]
struct GelbooruPost {
    id: i64,
    #[serde(default)]
    file_url: Option<String>,
    width: u32,
    height: u32,
    #[serde(default)]
    tags: String,
    rating: String,
    #[serde(default)]
    score: Option<i32>,
    #[serde(default)]
    owner: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GelbooruResponse {
    #[serde(rename = "post", default)]
    posts: Vec<GelbooruPost>,
}

pub struct GelbooruConnector {
    client: BooruClient,
}

impl GelbooruConnector {
    pub fn new() -> Self {
        Self {
            client: BooruClient::new(),
        }
    }
}

impl Default for GelbooruConnector {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl BooruConnector for GelbooruConnector {
    fn name(&self) -> &str {
        "gelbooru"
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
            .map_err(|e| Error::Renderer(format!("Gelbooru API error: {}", e)))?;

        if !response.status().is_success() {
            return Err(Error::Renderer(format!(
                "Gelbooru API returned status: {}",
                response.status()
            )));
        }

        let wrapper: GelbooruResponse = response
            .json()
            .await
            .map_err(|e| Error::Renderer(format!("Failed to parse Gelbooru response: {}", e)))?;

        let images = wrapper.posts
            .into_iter()
            .filter_map(|post| {
                let file_url = post.file_url?;

                Some(BooruImage {
                    id: post.id.to_string(),
                    source: "gelbooru".to_string(),
                    file_url,
                    preview_url: None,
                    sample_url: None,
                    width: post.width,
                    height: post.height,
                    tags: post.tags.split_whitespace().map(|s| s.to_string()).collect(),
                    rating: parse_rating(&post.rating),
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
            .map_err(|e| Error::Renderer(format!("Gelbooru autocomplete error: {}", e)))?;

        if !response.status().is_success() {
            return Ok(Vec::new());
        }

        #[derive(Deserialize)]
        struct TagWrapper {
            tag: Vec<Tag>,
        }

        #[derive(Deserialize)]
        struct Tag {
            name: String,
        }

        let wrapper: TagWrapper = response
            .json()
            .await
            .unwrap_or(TagWrapper { tag: Vec::new() });

        Ok(wrapper.tag.into_iter().map(|t| t.name).collect())
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
            .map_err(|e| Error::Renderer(format!("Gelbooru API error: {}", e)))?;

        if !response.status().is_success() {
            return Err(Error::NotFound(format!("Post {} not found", id)));
        }

        let wrapper: GelbooruResponse = response
            .json()
            .await
            .map_err(|e| Error::Renderer(format!("Failed to parse Gelbooru response: {}", e)))?;

        let post = wrapper.posts.into_iter().next()
            .ok_or_else(|| Error::NotFound(format!("Post {} not found", id)))?;

        let file_url = post.file_url
            .ok_or_else(|| Error::NotFound("No file URL found".to_string()))?;

        Ok(BooruImage {
            id: post.id.to_string(),
            source: "gelbooru".to_string(),
            file_url,
            preview_url: None,
            sample_url: None,
            width: post.width,
            height: post.height,
            tags: post.tags.split_whitespace().map(|s| s.to_string()).collect(),
            rating: parse_rating(&post.rating),
            score: post.score,
            author: post.owner,
        })
    }
}
