use eframe::egui;
use crate::models::{Wallpaper, WallpaperType, AppSettings};
use std::path::PathBuf;

pub struct LocalTab {
    wallpapers: Vec<Wallpaper>,
    current_folder: PathBuf,
    selected_index: Option<usize>,
    settings: AppSettings,
    loading: bool,
    style_mode: String,
    folder_name: String,
}

impl LocalTab {
    pub fn new() -> Self {
        let settings = AppSettings::load();
        let current_folder = settings.local_folder.clone();
        let folder_name = current_folder.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Pictures")
            .to_string();
        
        let mut tab = Self {
            wallpapers: Vec::new(),
            current_folder,
            selected_index: None,
            style_mode: settings.display_mode.clone(),
            folder_name,
            settings,
            loading: false,
        };
        
        tab.load_folder();
        tab
    }

    fn load_folder(&mut self) {
        self.wallpapers.clear();
        self.loading = true;
        
        if let Ok(entries) = std::fs::read_dir(&self.current_folder) {
            for entry in entries.flatten() {
                if let Ok(meta) = entry.metadata() {
                    if meta.is_file() {
                        let mut wp = Wallpaper::from_path(entry.path());
                        if matches!(wp.wallpaper_type, WallpaperType::Static | WallpaperType::Video) {
                            wp.file_size = meta.len();
                            self.wallpapers.push(wp);
                        }
                    }
                }
            }
        }
        
        self.loading = false;
    }

    pub fn show(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        // Top section: Wallpaper preview grid
        ui.group(|ui| {
            ui.set_min_height(300.0);
            
            if self.loading {
                ui.centered_and_justified(|ui| {
                    ui.spinner();
                });
            } else if self.wallpapers.is_empty() {
                ui.centered_and_justified(|ui| {
                    ui.label("No wallpapers found in this folder");
                });
            } else {
                egui::ScrollArea::vertical().max_height(280.0).show(ui, |ui| {
                    self.show_preview_grid(ui);
                });
            }
        });

        ui.add_space(10.0);

        // Bottom section: Settings and controls
        ui.horizontal(|ui| {
            // Left column: Folder and Style
            ui.vertical(|ui| {
                ui.set_min_width(300.0);
                
                // Folder dropdown
                ui.horizontal(|ui| {
                    ui.label("Folder:");
                    egui::ComboBox::from_id_salt("folder")
                        .selected_text(&self.folder_name)
                        .width(180.0)
                        .show_ui(ui, |ui| {
                            if ui.selectable_label(false, "desktop-base").clicked() {
                                // TODO: Change folder
                            }
                            if ui.selectable_label(false, "Pictures").clicked() {
                                // TODO: Change folder
                            }
                            if ui.selectable_label(false, "Downloads").clicked() {
                                // TODO: Change folder
                            }
                        });
                });

                ui.add_space(5.0);

                // Style dropdown
                ui.horizontal(|ui| {
                    ui.label("Style:");
                    egui::ComboBox::from_id_salt("style")
                        .selected_text(&self.style_mode)
                        .width(180.0)
                        .show_ui(ui, |ui| {
                            if ui.selectable_value(&mut self.style_mode, "Zoomed".to_string(), "Zoomed").clicked() {
                                self.settings.display_mode = "zoomed".to_string();
                            }
                            if ui.selectable_value(&mut self.style_mode, "Scaled".to_string(), "Scaled").clicked() {
                                self.settings.display_mode = "scaled".to_string();
                            }
                            if ui.selectable_value(&mut self.style_mode, "Stretched".to_string(), "Stretched").clicked() {
                                self.settings.display_mode = "stretched".to_string();
                            }
                            if ui.selectable_value(&mut self.style_mode, "Centered".to_string(), "Centered").clicked() {
                                self.settings.display_mode = "centered".to_string();
                            }
                            if ui.selectable_value(&mut self.style_mode, "Tiled".to_string(), "Tiled").clicked() {
                                self.settings.display_mode = "tiled".to_string();
                            }
                            if ui.selectable_value(&mut self.style_mode, "Spanned".to_string(), "Spanned").clicked() {
                                self.settings.display_mode = "spanned".to_string();
                            }
                        });
                });

                ui.add_space(5.0);

                // Color options with color pickers
                ui.horizontal(|ui| {
                    ui.label("Color:");
                    egui::ComboBox::from_id_salt("color_type")
                        .selected_text("Horizontal gradient")
                        .width(120.0)
                        .show_ui(ui, |ui| {
                            let _ = ui.selectable_label(false, "Solid color");
                            let _ = ui.selectable_label(true, "Horizontal gradient");
                            let _ = ui.selectable_label(false, "Vertical gradient");
                        });
                    
                    // Color picker squares (left and right)
                    ui.add_space(5.0);
                    
                    // Left color (color1)
                    let color1 = egui::Color32::from_rgb(
                        self.settings.background_color1[0],
                        self.settings.background_color1[1],
                        self.settings.background_color1[2],
                    );
                    let (rect1, response1) = ui.allocate_exact_size(
                        egui::vec2(30.0, 25.0),
                        egui::Sense::click(),
                    );
                    ui.painter().rect_filled(rect1, 3.0, color1);
                    ui.painter().rect_stroke(rect1, 3.0, egui::Stroke::new(1.0, egui::Color32::GRAY));
                    
                    if response1.clicked() {
                        // TODO: Open color picker
                    }
                    
                    ui.add_space(5.0);
                    
                    // Right color (color2)
                    let color2 = egui::Color32::from_rgb(
                        self.settings.background_color2[0],
                        self.settings.background_color2[1],
                        self.settings.background_color2[2],
                    );
                    let (rect2, response2) = ui.allocate_exact_size(
                        egui::vec2(30.0, 25.0),
                        egui::Sense::click(),
                    );
                    ui.painter().rect_filled(rect2, 3.0, color2);
                    ui.painter().rect_stroke(rect2, 3.0, egui::Stroke::new(1.0, egui::Color32::GRAY));
                    
                    if response2.clicked() {
                        // TODO: Open color picker
                    }
                });
            });

            ui.add_space(20.0);

            // Right column: Checkboxes and options
            ui.vertical(|ui| {
                ui.checkbox(&mut self.settings.apply_all_workspaces, "Apply to all workspaces");
                
                ui.add_space(5.0);
                
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.settings.auto_change, "Change the background");
                    ui.label("in minutes:");
                    ui.add(egui::DragValue::new(&mut self.settings.change_interval)
                        .speed(1.0)
                        .range(1..=1440));
                });

                ui.add_space(5.0);
                
                ui.checkbox(&mut self.settings.random_order, "Random Order");
            });
        });

        ui.add_space(10.0);

        // Bottom buttons (Help and Close/Apply)
        ui.horizontal(|ui| {
            if ui.button("❓ Help").clicked() {
                // TODO: Show help dialog
            }
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("✖ Close").clicked() {
                    // Save settings
                    let _ = self.settings.save();
                }
                
                if ui.button("✓ Apply").clicked() {
                    // Apply wallpaper
                    let _ = self.settings.save();
                }
            });
        });
    }

    fn show_preview_grid(&mut self, ui: &mut egui::Ui) {
        let available_width = ui.available_width();
        let thumb_width = 150.0;
        let thumb_height = 100.0;
        let spacing = 8.0;
        let cols = ((available_width + spacing) / (thumb_width + spacing)).floor() as usize;
        let cols = cols.max(4).min(6); // Between 4-6 columns like in the image

        let rows = (self.wallpapers.len() + cols - 1) / cols;
        
        for row in 0..rows {
            ui.horizontal(|ui| {
                for col in 0..cols {
                    let idx = row * cols + col;
                    if idx >= self.wallpapers.len() {
                        break;
                    }

                    let is_selected = Some(idx) == self.selected_index;
                    
                    // Thumbnail with border for selected
                    let (rect, response) = ui.allocate_exact_size(
                        egui::vec2(thumb_width, thumb_height),
                        egui::Sense::click(),
                    );

                    // Border for selected item (blue like in image)
                    if is_selected {
                        ui.painter().rect_stroke(
                            rect.expand(3.0),
                            5.0,
                            egui::Stroke::new(3.0, egui::Color32::from_rgb(52, 101, 164)),
                        );
                    }

                    // Background
                    ui.painter().rect_filled(rect, 3.0, egui::Color32::from_gray(45));
                    
                    // Placeholder text (centered)
                    let text = if let Some(name) = self.wallpapers[idx].path.file_name().and_then(|n| n.to_str()) {
                        name.chars().take(20).collect()
                    } else {
                        "Wallpaper".to_string()
                    };
                    
                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        text,
                        egui::FontId::proportional(10.0),
                        egui::Color32::GRAY,
                    );

                    // Handle click
                    if response.clicked() {
                        self.selected_index = Some(idx);
                    }

                    // Context menu
                    response.context_menu(|ui| {
                        if ui.button("Set as Wallpaper").clicked() {
                            ui.close_menu();
                        }
                        if ui.button("Add to Favorites").clicked() {
                            ui.close_menu();
                        }
                    });
                }
            });
            ui.add_space(spacing);
        }
    }
}
