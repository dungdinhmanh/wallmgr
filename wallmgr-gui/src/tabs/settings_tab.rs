use eframe::egui;
use crate::models::{AppSettings, ImageSource};

pub struct SettingsTab {
    settings: AppSettings,
    status_message: String,
}

impl SettingsTab {
    pub fn new() -> Self {
        Self {
            settings: AppSettings::load(),
            status_message: String::new(),
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("⚙ Settings");
            ui.separator();

            // Folders
            ui.group(|ui| {
                ui.label(egui::RichText::new("Folders").strong());
                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Download folder:");
                    ui.label(self.settings.download_folder.display().to_string());
                    if ui.button("📁 Browse").clicked() {
                        // TODO: File dialog
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Local folder:");
                    ui.label(self.settings.local_folder.display().to_string());
                    if ui.button("📁 Browse").clicked() {
                        // TODO: File dialog
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Favorites folder:");
                    ui.label(self.settings.favorites_folder.display().to_string());
                    if ui.button("📁 Browse").clicked() {
                        // TODO: File dialog
                    }
                });
            });

            ui.add_space(10.0);

            // Online sources - Variety style checkbox list
            ui.group(|ui| {
                ui.label(egui::RichText::new("Image Sources").strong());
                ui.label(egui::RichText::new("Select which sources to search").small().weak());
                ui.separator();

                let all_sources = ImageSource::all_sources();
                let mut enabled_set: std::collections::HashSet<String> = self.settings.enabled_sources.iter().cloned().collect();
                
                for source in all_sources {
                    let mut is_enabled = enabled_set.contains(&source.id);
                    let label_text = if source.is_nsfw {
                        format!("{}  ({}) [NSFW]", source.name, source.base_url)
                    } else {
                        format!("{}  ({})", source.name, source.base_url)
                    };
                    
                    ui.horizontal(|ui| {
                        if ui.checkbox(&mut is_enabled, &label_text).changed() {
                            if is_enabled {
                                enabled_set.insert(source.id.clone());
                            } else {
                                enabled_set.remove(&source.id);
                            }
                            self.settings.enabled_sources = enabled_set.iter().cloned().collect();
                        }
                    });
                }

                ui.add_space(5.0);
                ui.separator();
                
                ui.checkbox(&mut self.settings.allow_nsfw, "Show NSFW content (if enabled sources have it)");

                ui.horizontal(|ui| {
                    ui.label("Items per page:");
                    ui.add(egui::Slider::new(&mut self.settings.items_per_page, 10..=100));
                });
            });

            ui.add_space(10.0);

            // Display
            ui.group(|ui| {
                ui.label(egui::RichText::new("Display").strong());
                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Thumbnail size:");
                    ui.add(egui::Slider::new(&mut self.settings.thumbnail_size, 100.0..=400.0));
                });

                ui.horizontal(|ui| {
                    ui.label("Columns:");
                    ui.add(egui::Slider::new(&mut self.settings.columns, 2..=10));
                });
            });

            ui.add_space(10.0);

            // Wallpaper
            ui.group(|ui| {
                ui.label(egui::RichText::new("Wallpaper").strong());
                ui.separator();

                ui.checkbox(&mut self.settings.auto_set_wallpaper, "Auto-set downloaded wallpapers");

                ui.horizontal(|ui| {
                    ui.label("Display mode:");
                    egui::ComboBox::from_label("")
                        .selected_text(&self.settings.display_mode)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.settings.display_mode, "fill".to_string(), "Fill");
                            ui.selectable_value(&mut self.settings.display_mode, "fit".to_string(), "Fit");
                            ui.selectable_value(&mut self.settings.display_mode, "stretch".to_string(), "Stretch");
                            ui.selectable_value(&mut self.settings.display_mode, "center".to_string(), "Center");
                            ui.selectable_value(&mut self.settings.display_mode, "tile".to_string(), "Tile");
                        });
                });
            });

            ui.add_space(20.0);

            // Save button
            ui.horizontal(|ui| {
                if ui.button("💾 Save Settings").clicked() {
                    match self.settings.save() {
                        Ok(_) => self.status_message = "Settings saved successfully!".to_string(),
                        Err(e) => self.status_message = format!("Error saving: {}", e),
                    }
                }

                if !self.status_message.is_empty() {
                    ui.label(&self.status_message);
                }
            });
        });
    }
}
