use eframe::egui;
use poll_promise::Promise;
use crate::models::{BooruImage, AppSettings, ImageSource};

pub struct OnlineTab {
    // Search
    search_query: String,
    
    // Results
    images: Vec<BooruImage>,
    promise: Option<Promise<Result<Vec<BooruImage>, String>>>,
    
    // Suggestions
    tag_suggestions: Vec<String>,
    
    // State
    settings: AppSettings,
    is_loading: bool,
    status_message: String,
    active_sources: Vec<String>, // Currently searching from these sources
}

impl OnlineTab {
    pub fn new() -> Self {
        let settings = AppSettings::load();

        Self {
            search_query: String::new(),
            images: Vec::new(),
            promise: None,
            tag_suggestions: Self::default_tags(),
            settings,
            is_loading: false,
            status_message: "Enter tags and click Search".to_string(),
            active_sources: Vec::new(),
        }
    }

    fn default_tags() -> Vec<String> {
        vec![
            "landscape".to_string(),
            "anime".to_string(),
            "nature".to_string(),
            "4k".to_string(),
            "wallpaper".to_string(),
            "scenery".to_string(),
            "mountain".to_string(),
            "ocean".to_string(),
        ]
    }

    pub fn show(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        // Reload settings in case they changed
        self.settings = AppSettings::load();
        
        // Check promise
        if let Some(promise) = &self.promise {
            if let Some(result) = promise.ready() {
                self.is_loading = false;
                match result {
                    Ok(images) => {
                        self.status_message = format!("Found {} images from: {}", images.len(), self.active_sources.join(", "));
                        self.images = images.clone();
                    }
                    Err(e) => {
                        self.status_message = format!("Error: {}", e);
                    }
                }
                self.promise = None;
            }
        }

        // Title - centered
        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            ui.heading("🌐 Online Wallpaper Search");
            ui.add_space(10.0);
        });

        // Search bar - centered
        ui.vertical_centered(|ui| {
            ui.horizontal(|ui| {
                ui.add_space(50.0);
                ui.label("🔍");
                let search_field = egui::TextEdit::singleline(&mut self.search_query)
                    .hint_text("Enter tags... (e.g., landscape anime)")
                    .desired_width(400.0);
                
                let response = ui.add(search_field);
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.search();
                }

                if ui.button("Search").clicked() {
                    self.search();
                }

                if self.is_loading {
                    ui.spinner();
                }
            });
        });

        ui.add_space(10.0);

        // Trending tags - centered
        ui.vertical_centered(|ui| {
            ui.label(egui::RichText::new("Trending:").small().weak());
            ui.add_space(5.0);
            
            let tags = self.tag_suggestions.clone();
            ui.horizontal_wrapped(|ui| {
                for tag in &tags {
                    if ui.button(tag).clicked() {
                        self.search_query = tag.clone();
                        self.search();
                    }
                }
            });
        });

        ui.add_space(10.0);
        ui.separator();

        // Status message
        ui.vertical_centered(|ui| {
            if !self.active_sources.is_empty() {
                ui.label(egui::RichText::new(
                    format!("Searching: {}", self.active_sources.join(", "))
                ).small().weak());
            }
            ui.label(&self.status_message);
        });

        ui.add_space(10.0);

        // Results grid
        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                if self.images.is_empty() {
                    ui.vertical_centered(|ui| {
                        ui.add_space(50.0);
                        ui.label("No images yet. Try searching!");
                    });
                } else {
                    self.show_image_grid(ui, ctx);
                }
            });
    }

    fn show_image_grid(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        let thumbnail_size = egui::vec2(200.0, 150.0);
        let spacing = 10.0;
        let available_width = ui.available_width();
        let columns = ((available_width + spacing) / (thumbnail_size.x + spacing)).floor() as usize;
        let columns = columns.max(1);

        egui::Grid::new("image_grid")
            .spacing([spacing, spacing])
            .show(ui, |ui| {
                for (i, image) in self.images.iter_mut().enumerate() {
                    if i > 0 && i % columns == 0 {
                        ui.end_row();
                    }

                    ui.vertical(|ui| {
                        let button = egui::Button::new(
                            egui::RichText::new(&format!("{}x{}", image.width, image.height))
                                .size(10.0)
                        )
                        .min_size(thumbnail_size);

                        let response = ui.add(button);

                        if response.clicked() {
                            image.selected = !image.selected;
                        }

                        if image.selected {
                            ui.label(egui::RichText::new("✓ Selected").color(egui::Color32::GREEN));
                        }

                        // Context menu
                        response.context_menu(|ui| {
                            if ui.button("📋 Copy URL").clicked() {
                                ui.output_mut(|o| o.copied_text = image.file_url.clone());
                                ui.close_menu();
                            }
                            if ui.button("🌐 Open in browser").clicked() {
                                let _ = webbrowser::open(&image.file_url);
                                ui.close_menu();
                            }
                            if ui.button("💾 Download").clicked() {
                                // TODO: Download implementation
                                ui.close_menu();
                            }
                            if ui.button("🖼 Set as wallpaper").clicked() {
                                // TODO: Set wallpaper implementation
                                ui.close_menu();
                            }
                        });

                        // Tags preview
                        ui.label(
                            egui::RichText::new(
                                if image.tags.len() > 30 {
                                    format!("{}...", &image.tags[..30])
                                } else {
                                    image.tags.clone()
                                }
                            )
                            .size(9.0)
                            .weak(),
                        );
                    });
                }
            });
    }

    fn search(&mut self) {
        if self.search_query.trim().is_empty() {
            self.status_message = "Please enter search tags".to_string();
            return;
        }

        // Get enabled sources
        let all_sources = ImageSource::all_sources();
        let enabled_sources: Vec<ImageSource> = all_sources
            .into_iter()
            .filter(|s| self.settings.enabled_sources.contains(&s.id))
            .collect();

        if enabled_sources.is_empty() {
            self.status_message = "No sources enabled. Go to Settings to enable sources.".to_string();
            return;
        }

        self.active_sources = enabled_sources.iter().map(|s| s.name.clone()).collect();
        self.is_loading = true;
        self.status_message = format!("Searching {} sources...", enabled_sources.len());
        self.images.clear();

        let search_query = self.search_query.clone();
        let limit = self.settings.items_per_page;

        // Create promises for each source
        let promise = Promise::spawn_thread("multi_source_search", move || {
            let mut all_images = Vec::new();
            
            // Use blocking reqwest client for thread
            let client = reqwest::blocking::Client::new();
            
            for source in enabled_sources {
                let url = source.api_url(&search_query, 1, limit as u32);
                
                match client.get(&url).send() {
                    Ok(response) => {
                        if let Ok(json) = response.json::<serde_json::Value>() {
                            if let Ok(images) = Self::parse_response(&source, json) {
                                all_images.extend(images);
                            }
                        }
                    }
                    Err(_) => continue,
                }
            }

            Ok(all_images)
        });

        self.promise = Some(promise);
    }

    fn parse_response(source: &ImageSource, json: serde_json::Value) -> Result<Vec<BooruImage>, String> {
        use crate::models::EngineType;

        match source.engine {
            EngineType::Moebooru => Self::parse_moebooru(json),
            EngineType::Danbooru2 => Self::parse_danbooru2(json),
            EngineType::Gelbooru02 => Self::parse_gelbooru02(json),
            EngineType::Zerochan => Self::parse_zerochan(json),
            EngineType::AnimePictures => Self::parse_anime_pictures(json),
        }
    }

    fn parse_moebooru(json: serde_json::Value) -> Result<Vec<BooruImage>, String> {
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

    fn parse_danbooru2(json: serde_json::Value) -> Result<Vec<BooruImage>, String> {
        let items = json.as_array().ok_or("Invalid response")?;
        
        Ok(items.iter().filter_map(|item| {
            let rating = item["rating"].as_str().unwrap_or("g");
            Some(BooruImage {
                id: item["id"].as_u64()?.to_string(),
                width: item["image_width"].as_u64()? as u32,
                height: item["image_height"].as_u64()? as u32,
                tags: item["tag_string"].as_str()?.to_string(),
                rating: rating.to_string(),
                is_nsfw: rating != "g" && rating != "s",
                preview_url: item["preview_file_url"].as_str().unwrap_or("").to_string(),
                sample_url: item["large_file_url"].as_str()
                    .or(item["file_url"].as_str()).unwrap_or("").to_string(),
                file_url: item["file_url"].as_str().unwrap_or("").to_string(),
                selected: false,
            })
        }).collect())
    }

    fn parse_gelbooru02(json: serde_json::Value) -> Result<Vec<BooruImage>, String> {
        let posts = json["post"].as_array()
            .or_else(|| json.as_array())
            .ok_or("Invalid response")?;
        
        Ok(posts.iter().filter_map(|post| {
            let rating = post["rating"].as_str().unwrap_or("general");
            Some(BooruImage {
                id: post["id"].as_u64()?.to_string(),
                width: post["width"].as_u64()? as u32,
                height: post["height"].as_u64()? as u32,
                tags: post["tags"].as_str().unwrap_or("").to_string(),
                rating: rating.to_string(),
                is_nsfw: rating != "general" && rating != "safe",
                preview_url: post["preview_url"].as_str().unwrap_or("").to_string(),
                sample_url: post["sample_url"].as_str()
                    .or(post["file_url"].as_str()).unwrap_or("").to_string(),
                file_url: post["file_url"].as_str().unwrap_or("").to_string(),
                selected: false,
            })
        }).collect())
    }

    fn parse_zerochan(json: serde_json::Value) -> Result<Vec<BooruImage>, String> {
        let items = json.as_array().ok_or("Invalid response")?;
        
        Ok(items.iter().filter_map(|item| {
            Some(BooruImage {
                id: item["id"].as_str()?.to_string(),
                width: item["width"].as_u64().unwrap_or(1920) as u32,
                height: item["height"].as_u64().unwrap_or(1080) as u32,
                tags: item["tag"].as_str().unwrap_or("").replace(", ", " "),
                rating: "s".to_string(),
                is_nsfw: false,
                preview_url: item["thumbnail"].as_str()?.to_string(),
                sample_url: item["image"].as_str().unwrap_or(item["thumbnail"].as_str()?).to_string(),
                file_url: item["full"].as_str().unwrap_or(item["image"].as_str()?).to_string(),
                selected: false,
            })
        }).collect())
    }

    fn parse_anime_pictures(json: serde_json::Value) -> Result<Vec<BooruImage>, String> {
        let posts = json["posts"].as_array().ok_or("Invalid response")?;
        
        Ok(posts.iter().filter_map(|post| {
            let id = post["id"].as_u64()?.to_string();
            let md5 = post["md5"].as_str()?;
            let ext = post["ext"].as_str()?.trim_start_matches('.');
            let width = post["width"].as_u64()? as u32;
            let height = post["height"].as_u64()? as u32;
            
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
