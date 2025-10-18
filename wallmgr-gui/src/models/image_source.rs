use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EngineType {
    Moebooru,      // konachan, yande.re
    Danbooru2,     // danbooru.donmai.us
    Gelbooru02,    // gelbooru, safebooru, rule34
    Zerochan,      // zerochan.net
    AnimePictures, // anime-pictures.net
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImageSource {
    pub id: String,
    pub name: String,
    pub base_url: String,
    pub engine: EngineType,
    pub enabled: bool,
    pub is_nsfw: bool,
    pub max_limit: u32,
}

impl ImageSource {
    pub fn all_sources() -> Vec<Self> {
        vec![
            // Moebooru sites
            Self {
                id: "konachan".to_string(),
                name: "Konachan".to_string(),
                base_url: "https://konachan.com".to_string(),
                engine: EngineType::Moebooru,
                enabled: true,
                is_nsfw: false,
                max_limit: 100,
            },
            Self {
                id: "yandere".to_string(),
                name: "yande.re".to_string(),
                base_url: "https://yande.re".to_string(),
                engine: EngineType::Moebooru,
                enabled: true,
                is_nsfw: true,
                max_limit: 100,
            },
            // Danbooru 2.0
            Self {
                id: "danbooru".to_string(),
                name: "Danbooru".to_string(),
                base_url: "https://danbooru.donmai.us".to_string(),
                engine: EngineType::Danbooru2,
                enabled: true,
                is_nsfw: true,
                max_limit: 200,
            },
            // Gelbooru 0.2 - SFW only
            Self {
                id: "safebooru".to_string(),
                name: "Safebooru".to_string(),
                base_url: "https://safebooru.org".to_string(),
                engine: EngineType::Gelbooru02,
                enabled: true,
                is_nsfw: false,
                max_limit: 100,
            },
            // Gelbooru 0.2 - NSFW
            Self {
                id: "gelbooru".to_string(),
                name: "Gelbooru".to_string(),
                base_url: "https://gelbooru.com".to_string(),
                engine: EngineType::Gelbooru02,
                enabled: false, // Disabled by default (NSFW)
                is_nsfw: true,
                max_limit: 100,
            },
            Self {
                id: "rule34".to_string(),
                name: "Rule34".to_string(),
                base_url: "https://api.rule34.xxx".to_string(),
                engine: EngineType::Gelbooru02,
                enabled: false, // Disabled by default (NSFW)
                is_nsfw: true,
                max_limit: 100,
            },
            // Custom engines
            Self {
                id: "zerochan".to_string(),
                name: "Zerochan".to_string(),
                base_url: "https://www.zerochan.net".to_string(),
                engine: EngineType::Zerochan,
                enabled: false,
                is_nsfw: false,
                max_limit: 100,
            },
            Self {
                id: "anime_pictures".to_string(),
                name: "Anime-Pictures".to_string(),
                base_url: "https://api.anime-pictures.net".to_string(),
                engine: EngineType::AnimePictures,
                enabled: false,
                is_nsfw: false,
                max_limit: 30,
            },
        ]
    }

    pub fn api_url(&self, tags: &str, page: u32, limit: u32) -> String {
        match self.engine {
            EngineType::Moebooru => {
                format!(
                    "{}/post.json?tags={}&page={}&limit={}",
                    self.base_url,
                    urlencoding::encode(tags),
                    page,
                    limit.min(self.max_limit)
                )
            }
            EngineType::Danbooru2 => {
                format!(
                    "{}/posts.json?tags={}&page={}&limit={}",
                    self.base_url,
                    urlencoding::encode(tags),
                    page,
                    limit.min(self.max_limit)
                )
            }
            EngineType::Gelbooru02 => {
                let pid = if page > 0 { page - 1 } else { 0 };
                format!(
                    "{}/index.php?page=dapi&s=post&q=index&json=1&tags={}&pid={}&limit={}",
                    self.base_url,
                    urlencoding::encode(tags),
                    pid,
                    limit.min(self.max_limit)
                )
            }
            EngineType::Zerochan => {
                format!(
                    "{}/{}?json&p={}",
                    self.base_url,
                    urlencoding::encode(tags),
                    page
                )
            }
            EngineType::AnimePictures => {
                let page_idx = if page > 0 { page - 1 } else { 0 };
                format!(
                    "{}/api/v3/posts?page={}&posts_per_page={}&lang=en",
                    self.base_url, page_idx, limit.min(self.max_limit)
                )
            }
        }
    }
}
