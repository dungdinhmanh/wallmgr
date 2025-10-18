use eframe::egui;
use poll_promise::Promise;
use crate::models::{BooruImage, BooruProvider, AppSettings};

pub struct OnlineTab {
    // Search
    search_query: String,
    current_provider: BooruProvider,
    allow_nsfw: bool,
    
    // Results
    images: Vec<BooruImage>,
    promise: Option<Promise<Result<Vec<BooruImage>, String>>>,
    
    // Suggestions
    tag_suggestions: Vec<String>,
    
    // State
    settings: AppSettings,
    is_loading: bool,
    status_message: String,
}

impl OnlineTab {
    pub fn new() -> Self {
        let settings = AppSettings::load();
        let current_provider = match settings.default_provider.as_str() {
            "yandere" => BooruProvider::Yandere,
            "danbooru" => BooruProvider::Danbooru,
            "gelbooru" => BooruProvider::Gelbooru,
            "wallhaven" => BooruProvider::WallHaven,
            _ => BooruProvider::Konachan,
        };

        Self {
            search_query: String::new(),
            current_provider,
            allow_nsfw: settings.allow_nsfw,
            images: Vec::new(),
            promise: None,
            tag_suggestions: Self::default_tags(),
            settings,
            is_loading: false,
            status_message: "Enter tags and click Search".to_string(),
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
        // Check promise
        if let Some(promise) = &self.promise {
            if let Some(result) = promise.ready() {
                self.is_loading = false;
                match result {
                    Ok(images) => {
                        self.status_message = format!("Found {} images", images.len());
                        self.images = images.clone();
                    }
                    Err(e) => {
                        self.status_message = format!("Error: {}", e);
                    }
                }
                self.promise = None;
            }
        }

        // Provider selector
        ui.horizontal(|ui| {
            ui.label("Provider:");
            let old_provider = self.current_provider.clone();
            egui::ComboBox::from_id_salt("provider")
                .selected_text(self.current_provider.name())
                .show_ui(ui, |ui| {
                    for provider in BooruProvider::all() {
                        ui.selectable_value(&mut self.current_provider, provider.clone(), provider.name());
                    }
                });

            // Auto-search when provider changes
            if old_provider != self.current_provider && !self.search_query.is_empty() {
                self.search();
            }

            ui.separator();
            ui.checkbox(&mut self.allow_nsfw, "NSFW");
        });

        ui.add_space(10.0);

        // Search bar
        ui.vertical_centered(|ui| {
            ui.horizontal(|ui| {
                ui.label("🔍");
                let response = ui.add(
                    egui::TextEdit::singleline(&mut self.search_query)
                        .desired_width(400.0)
                        .hint_text("Enter tags (space-separated)")
                );

                if ui.button("Search").clicked() || (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) {
                    self.search();
                }

                if self.is_loading {
                    ui.spinner();
                }
            });

            ui.label(&self.status_message);
        });

        ui.add_space(5.0);

        // Tag suggestions
        ui.horizontal_wrapped(|ui| {
            ui.label("Trending:");
            for tag in &self.tag_suggestions {
                if ui.small_button(tag).clicked() {
                    if !self.search_query.is_empty() {
                        self.search_query.push(' ');
                    }
                    self.search_query.push_str(tag);
                }
            }
        });

        ui.separator();

        // Image grid
        if self.images.is_empty() && !self.is_loading {
            ui.centered_and_justified(|ui| {
                ui.vertical(|ui| {
                    ui.heading("No images");
                    ui.label("Enter tags and click Search");
                    ui.label("💡 Try: landscape, anime, nature, 4k");
                });
            });
        } else {
            egui::ScrollArea::vertical().show(ui, |ui| {
                self.show_grid(ui);
            });
        }

        ctx.request_repaint();
    }

    fn search(&mut self) {
        if self.search_query.trim().is_empty() {
            // Show trending if no query
            self.status_message = "Showing trending...".to_string();
        }

        let tags = self.search_query.clone();
        let provider = self.current_provider.clone();
        let nsfw = self.allow_nsfw;
        let limit = self.settings.items_per_page;

        self.is_loading = true;
        self.status_message = format!("Searching {}...", provider.name());

        let promise = Promise::spawn_thread("fetch_images", move || {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(Self::fetch_images(provider, tags, nsfw, limit))
        });

        self.promise = Some(promise);
    }

    async fn fetch_images(
        provider: BooruProvider,
        tags: String,
        nsfw: bool,
        limit: usize,
    ) -> Result<Vec<BooruImage>, String> {
        let client = reqwest::Client::new();
        let mut url = provider.api_url().to_string();

        // Build params
        let mut params = vec![
            ("tags", tags),
            ("limit", limit.to_string()),
        ];

        if !nsfw {
            params.push(("rating", "safe".to_string()));
        }

        let query = params.iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        url = if url.contains('?') {
            format!("{}&{}", url, query)
        } else {
            format!("{}?{}", url, query)
        };

        let response = client.get(&url)
            .header("User-Agent", "Wallmgr/1.0")
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("HTTP {}", response.status()));
        }

        let json: serde_json::Value = response.json()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))?;

        provider.map_response(json)
    }

    fn show_grid(&mut self, ui: &mut egui::Ui) {
        let available_width = ui.available_width();
        let thumb_size = self.settings.thumbnail_size;
        let spacing = 10.0;
        let cols = ((available_width + spacing) / (thumb_size + spacing)).floor() as usize;
        let cols = cols.max(1);

        ui.columns(cols, |columns| {
            for (idx, img) in self.images.iter_mut().enumerate() {
                let col = idx % cols;
                columns[col].group(|ui| {
                    // Checkbox
                    ui.checkbox(&mut img.selected, "");

                    // Thumbnail placeholder
                    let (rect, response) = ui.allocate_exact_size(
                        egui::vec2(thumb_size, thumb_size * 0.75),
                        egui::Sense::click(),
                    );

                    // Background
                    ui.painter().rect_filled(rect, 5.0, egui::Color32::from_gray(40));
                    
                    // Resolution text (centered)
                    let text_pos = rect.center();
                    ui.painter().text(
                        text_pos,
                        egui::Align2::CENTER_CENTER,
                        format!("{}x{}", img.width, img.height),
                        egui::FontId::proportional(14.0),
                        egui::Color32::WHITE,
                    );

                    // Tags (truncated and centered)
                    let tags = img.tags.chars().take(40).collect::<String>();
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        ui.label(egui::RichText::new(tags).size(9.0).color(egui::Color32::GRAY));
                    });

                    // Context menu
                    response.context_menu(|ui| {
                        if ui.button("📥 Download").clicked() {
                            ui.close_menu();
                        }
                        if ui.button("⭐ Add to Favorites").clicked() {
                            ui.close_menu();
                        }
                        if ui.button("📋 Copy URL").clicked() {
                            ui.output_mut(|o| o.copied_text = img.file_url.clone());
                            ui.close_menu();
                        }
                    });
                });
            }
        });
    }
}
