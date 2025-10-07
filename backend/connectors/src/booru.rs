use wallmgr_core::error::Result;
use wallmgr_core::types::{BooruImage, Rating};
use async_trait::async_trait;

#[async_trait]
pub trait BooruConnector: Send + Sync {
    /// Get connector name
    fn name(&self) -> &str;

    /// Search images with tags
    async fn search(&self, tags: &[String], limit: u32, page: u32) -> Result<Vec<BooruImage>>;

    /// Get tag autocomplete suggestions
    async fn autocomplete_tags(&self, prefix: &str, limit: u32) -> Result<Vec<String>>;

    /// Get image details by ID
    async fn get_image(&self, id: &str) -> Result<BooruImage>;
}

pub struct BooruClient {
    client: reqwest::Client,
}

impl BooruClient {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .user_agent("Wallmgr/1.0")
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap();

        Self { client }
    }

    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }
}

impl Default for BooruClient {
    fn default() -> Self {
        Self::new()
    }
}

pub fn parse_rating(rating: &str) -> Rating {
    match rating.to_lowercase().as_str() {
        "s" | "safe" | "general" => Rating::Safe,
        "q" | "questionable" => Rating::Questionable,
        "e" | "explicit" => Rating::Explicit,
        _ => Rating::Safe,
    }
}
