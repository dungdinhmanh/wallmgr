use crate::models::BooruImage;
use serde_json::Value;

#[derive(Clone, Debug, PartialEq)]
pub enum BooruProvider {
    Konachan,
    Yandere,
    Danbooru,
    Zerochan,
    AnimePictures,
    Gelbooru,
    #[allow(dead_code)]
    WallHaven,
}

impl BooruProvider {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Konachan,
            Self::Yandere,
            Self::Danbooru,
            Self::Zerochan,
            Self::AnimePictures,
            // Gelbooru removed - 401 unauthorized errors
            // WallHaven removed - use anime-focused sites only
        ]
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Konachan => "Konachan",
            Self::Yandere => "yande.re",
            Self::Danbooru => "Danbooru",
            Self::Zerochan => "Zerochan",
            Self::AnimePictures => "Anime-Pictures",
            Self::Gelbooru => "Gelbooru",
            Self::WallHaven => "WallHaven",
        }
    }

    pub fn api_url(&self) -> &str {
        match self {
            Self::Konachan => "https://konachan.net/post.json",
            Self::Yandere => "https://yande.re/post.json",
            Self::Danbooru => "https://danbooru.donmai.us/posts.json",
            Self::Zerochan => "https://www.zerochan.net",
            Self::AnimePictures => "https://api.anime-pictures.net/api/v3/posts",
            Self::Gelbooru => "https://gelbooru.com/index.php?page=dapi&s=post&q=index&json=1",
            Self::WallHaven => "https://wallhaven.cc/api/v1/search",
        }
    }

    #[allow(dead_code)]
    pub fn tag_api_url(&self) -> Option<String> {
        match self {
            Self::Konachan => Some("https://konachan.net/tag.json?order=count&limit=10".to_string()),
            Self::Yandere => Some("https://yande.re/tag.json?order=count&limit=10".to_string()),
            Self::Danbooru => Some("https://danbooru.donmai.us/tags.json?search[order]=count&limit=10".to_string()),
            Self::Gelbooru => Some("https://gelbooru.com/index.php?page=dapi&s=tag&q=index&json=1&orderby=count&limit=10".to_string()),
            _ => None,
        }
    }

    pub fn map_response(&self, json: Value) -> Result<Vec<BooruImage>, String> {
        match self {
            Self::Konachan | Self::Yandere => Self::map_moebooru(json),
            Self::Danbooru => Self::map_danbooru(json),
            Self::Zerochan => Self::map_zerochan(json),
            Self::AnimePictures => Self::map_anime_pictures(json),
            Self::Gelbooru => Self::map_gelbooru(json),
            Self::WallHaven => Self::map_wallhaven(json),
        }
    }

    fn map_moebooru(json: Value) -> Result<Vec<BooruImage>, String> {
        let items = json.as_array().ok_or("Invalid response")?;
        
        Ok(items.iter().filter_map(|item| {
            Some(BooruImage {
                id: item["id"].as_u64()?.to_string(),
                width: item["width"].as_u64()? as u32,
                height: item["height"].as_u64()? as u32,
                tags: item["tags"].as_str()?.to_string(),
                rating: item["rating"].as_str()?.to_string(),
                is_nsfw: item["rating"].as_str()? != "s",
                preview_url: item["preview_url"].as_str()?.to_string(),
                sample_url: item["sample_url"].as_str()
                    .or(item["file_url"].as_str())?.to_string(),
                file_url: item["file_url"].as_str()?.to_string(),
                selected: false,
            })
        }).collect())
    }

    fn map_danbooru(json: Value) -> Result<Vec<BooruImage>, String> {
        let items = json.as_array().ok_or("Invalid response")?;
        
        Ok(items.iter().filter_map(|item| {
            Some(BooruImage {
                id: item["id"].as_u64()?.to_string(),
                width: item["image_width"].as_u64()? as u32,
                height: item["image_height"].as_u64()? as u32,
                tags: item["tag_string"].as_str()?.to_string(),
                rating: item["rating"].as_str()?.to_string(),
                is_nsfw: item["rating"].as_str()? != "s",
                preview_url: item["preview_file_url"].as_str()?.to_string(),
                sample_url: item["file_url"].as_str()
                    .or(item["large_file_url"].as_str())?.to_string(),
                file_url: item["large_file_url"].as_str()?.to_string(),
                selected: false,
            })
        }).collect())
    }

    fn map_gelbooru(json: Value) -> Result<Vec<BooruImage>, String> {
        let items = json["post"].as_array().ok_or("Invalid response")?;
        
        Ok(items.iter().filter_map(|item| {
            let rating = item["rating"].as_str()?.replace("general", "s");
            Some(BooruImage {
                id: item["id"].as_u64()?.to_string(),
                width: item["width"].as_u64()? as u32,
                height: item["height"].as_u64()? as u32,
                tags: item["tags"].as_str()?.to_string(),
                rating: rating.clone(),
                is_nsfw: rating != "s",
                preview_url: item["preview_url"].as_str()?.to_string(),
                sample_url: item["sample_url"].as_str()
                    .or(item["file_url"].as_str())?.to_string(),
                file_url: item["file_url"].as_str()?.to_string(),
                selected: false,
            })
        }).collect())
    }

    fn map_wallhaven(json: Value) -> Result<Vec<BooruImage>, String> {
        let items = json["data"].as_array().ok_or("Invalid response")?;
        
        Ok(items.iter().filter_map(|item| {
            let purity = item["purity"].as_str()?;
            Some(BooruImage {
                id: item["id"].as_str()?.to_string(),
                width: item["dimension_x"].as_u64()? as u32,
                height: item["dimension_y"].as_u64()? as u32,
                tags: item["tags"].as_array()?
                    .iter()
                    .filter_map(|t| t["name"].as_str())
                    .collect::<Vec<_>>()
                    .join(" "),
                rating: if purity == "sfw" { "s" } else { "q" }.to_string(),
                is_nsfw: purity != "sfw",
                preview_url: item["thumbs"]["small"].as_str()?.to_string(),
                sample_url: item["path"].as_str()?.to_string(),
                file_url: item["path"].as_str()?.to_string(),
                selected: false,
            })
        }).collect())
    }

    fn map_zerochan(json: Value) -> Result<Vec<BooruImage>, String> {
        // Zerochan returns simple JSON array
        let items = json.as_array().ok_or("Invalid Zerochan response")?;
        
        Ok(items.iter().filter_map(|item| {
            Some(BooruImage {
                id: item["id"].as_str()?.to_string(),
                width: item["width"].as_u64().unwrap_or(1920) as u32,
                height: item["height"].as_u64().unwrap_or(1080) as u32,
                tags: item["tag"].as_str().unwrap_or("").replace(", ", " "),
                rating: "s".to_string(), // Zerochan is generally safe
                is_nsfw: false,
                preview_url: item["thumbnail"].as_str()?.to_string(),
                sample_url: item["image"].as_str().unwrap_or(item["thumbnail"].as_str()?).to_string(),
                file_url: item["full"].as_str().unwrap_or(item["image"].as_str()?).to_string(),
                selected: false,
            })
        }).collect())
    }

    fn map_anime_pictures(json: Value) -> Result<Vec<BooruImage>, String> {
        // Anime-Pictures uses "posts" array
        let posts = json["posts"].as_array().ok_or("Invalid Anime-Pictures response")?;
        
        Ok(posts.iter().filter_map(|post| {
            let id = post["id"].as_u64()?.to_string();
            let md5 = post["md5"].as_str()?;
            let ext = post["ext"].as_str()?.trim_start_matches('.');
            let width = post["width"].as_u64()? as u32;
            let height = post["height"].as_u64()? as u32;
            
            // Build URLs like imgbrd-grabber does
            let md5_part = format!("{}/{}", &md5[0..3], md5);
            let preview_ext = if post["have_alpha"].as_bool().unwrap_or(false) { "png" } else { "jpg" };
            
            Some(BooruImage {
                id,
                width,
                height,
                tags: post["tags_string"].as_str().unwrap_or("").to_string(),
                rating: "s".to_string(),
                is_nsfw: post["adult"].as_bool().unwrap_or(false),
                preview_url: format!("https://opreviews.anime-pictures.net/{}_sp.{}", md5_part, preview_ext),
                sample_url: format!("https://opreviews.anime-pictures.net/{}_bp.{}", md5_part, preview_ext),
                file_url: format!("https://oimages.anime-pictures.net/{}.{}", md5_part, ext),
                selected: false,
            })
        }).collect())
    }
}
