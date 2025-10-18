use crate::booru::{BooruClient, BooruConnector, parse_rating};
use async_trait::async_trait;
use serde::Deserialize;
use wallmgr_core::error::{Error, Result};
use wallmgr_core::types::BooruImage;

const API_BASE: &str = "https://danbooru.donmai.us";

#[derive(Debug, Deserialize)]
struct DanbooruPost {
    id: i64,
    #[serde(default)]
    file_url: Option<String>,
    #[serde(default)]
    large_file_url: Option<String>,
    #[serde(default)]
    preview_file_url: Option<String>,
    image_width: u32,
    image_height: u32,
    #[serde(default)]
    tag_string: String,
    rating: String,
    #[serde(default)]
    score: Option<i32>,
    #[serde(default)]
    tag_string_artist: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct DanbooruTag {
    name: String,
    post_count: i64,
}

pub struct DanbooruConnector {
    client: BooruClient,
}

impl DanbooruConnector {
    pub fn new() -> Self {
        Self {
            client: BooruClient::new(),
        }
    }
}

impl Default for DanbooruConnector {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl BooruConnector for DanbooruConnector {
    fn name(&self) -> &str {
        "danbooru"
    }

    async fn search(&self, tags: &[String], limit: u32, page: u32) -> Result<Vec<BooruImage>> {
        let tags_str = tags.join(" ");
        let url = format!(
            "{}/posts.json?tags={}&limit={}&page={}",
            API_BASE,
            urlencoding::encode(&tags_str),
            limit.min(200),
            page
        );

        let response = self.client.client()
            .get(&url)
            .send()
            .await
            .map_err(|e| Error::Renderer(format!("Danbooru API error: {}", e)))?;

        if !response.status().is_success() {
            return Err(Error::Renderer(format!(
                "Danbooru API returned status: {}",
                response.status()
            )));
        }

        let posts: Vec<DanbooruPost> = response
            .json()
            .await
            .map_err(|e| Error::Renderer(format!("Failed to parse Danbooru response: {}", e)))?;

        let images = posts
            .into_iter()
            .filter_map(|post| {
                let file_url = post.file_url.clone().or_else(|| post.large_file_url.clone())?;
                let sample_url = post.large_file_url.clone();
                let is_nsfw = post.rating != "s";

                Some(BooruImage {
                    id: post.id.to_string(),
                    source: "danbooru".to_string(),
                    file_url,
                    preview_url: post.preview_file_url.clone(),
                    sample_url,
                    width: post.image_width,
                    height: post.image_height,
                    tags: post.tag_string.split_whitespace().map(|s| s.to_string()).collect(),
                    rating: parse_rating(&post.rating),
                    score: post.score,
                    author: if !post.tag_string_artist.is_empty() {
                        Some(post.tag_string_artist)
                    } else {
                        None
                    },
                    is_nsfw,
                })
            })
            .collect();

        Ok(images)
    }

    async fn autocomplete_tags(&self, prefix: &str, limit: u32) -> Result<Vec<String>> {
        let url = format!(
            "{}/autocomplete.json?search[query]={}&search[type]=tag_query&limit={}",
            API_BASE,
            urlencoding::encode(prefix),
            limit.min(20)
        );

        let response = self.client.client()
            .get(&url)
            .send()
            .await
            .map_err(|e| Error::Renderer(format!("Danbooru autocomplete error: {}", e)))?;

        if !response.status().is_success() {
            return Ok(Vec::new());
        }

        #[derive(Deserialize)]
        struct AutocompleteResult {
            value: String,
        }

        let results: Vec<AutocompleteResult> = response
            .json()
            .await
            .unwrap_or_default();

        Ok(results.into_iter().map(|r| r.value).collect())
    }

    async fn get_image(&self, id: &str) -> Result<BooruImage> {
        let url = format!("{}/posts/{}.json", API_BASE, id);

        let response = self.client.client()
            .get(&url)
            .send()
            .await
            .map_err(|e| Error::Renderer(format!("Danbooru API error: {}", e)))?;

        if !response.status().is_success() {
            return Err(Error::NotFound(format!("Post {} not found", id)));
        }

        let post: DanbooruPost = response
            .json()
            .await
            .map_err(|e| Error::Renderer(format!("Failed to parse Danbooru response: {}", e)))?;

        let file_url = post.file_url.clone()
            .or_else(|| post.large_file_url.clone())
            .ok_or_else(|| Error::NotFound("No file URL found".to_string()))?;
        
        let sample_url = post.large_file_url.clone();
        let is_nsfw = post.rating != "s";

        Ok(BooruImage {
            id: post.id.to_string(),
            source: "danbooru".to_string(),
            file_url,
            preview_url: post.preview_file_url,
            sample_url,
            width: post.image_width,
            height: post.image_height,
            tags: post.tag_string.split_whitespace().map(|s| s.to_string()).collect(),
            rating: parse_rating(&post.rating),
            score: post.score,
            author: if !post.tag_string_artist.is_empty() {
                Some(post.tag_string_artist)
            } else {
                None
            },
            is_nsfw,
        })
    }
}
