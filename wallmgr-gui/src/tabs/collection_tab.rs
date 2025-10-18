use eframe::egui;
use crate::models::{Wallpaper, AppSettings};

#[derive(PartialEq)]
enum CollectionType {
    Static,
    Video,
    Live2D,
}

pub struct CollectionTab {
    current_type: CollectionType,
    #[allow(dead_code)]
    wallpapers: Vec<Wallpaper>,
    #[allow(dead_code)]
    settings: AppSettings,
}

impl CollectionTab {
    pub fn new() -> Self {
        Self {
            current_type: CollectionType::Static,
            wallpapers: Vec::new(),
            settings: AppSettings::load(),
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        // Sub-tabs
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.current_type, CollectionType::Static, "🖼️ Static");
            ui.selectable_value(&mut self.current_type, CollectionType::Video, "🎬 Video");
            ui.selectable_value(&mut self.current_type, CollectionType::Live2D, "✨ Live2D");
        });

        ui.separator();

        match self.current_type {
            CollectionType::Static => self.show_static(ui),
            CollectionType::Video => self.show_video(ui),
            CollectionType::Live2D => self.show_live2d(ui),
        }
    }

    fn show_static(&mut self, ui: &mut egui::Ui) {
        ui.centered_and_justified(|ui| {
            ui.vertical(|ui| {
                ui.heading("🖼️ Official Static Wallpapers");
                ui.add_space(10.0);
                ui.label("High-quality official wallpapers collection");
                ui.label("Coming soon...");
            });
        });
    }

    fn show_video(&mut self, ui: &mut egui::Ui) {
        ui.centered_and_justified(|ui| {
            ui.vertical(|ui| {
                ui.heading("🎬 Official Video Wallpapers");
                ui.add_space(10.0);
                ui.label("Animated wallpapers from official sources");
                ui.label("Coming soon...");
            });
        });
    }

    fn show_live2d(&mut self, ui: &mut egui::Ui) {
        ui.centered_and_justified(|ui| {
            ui.vertical(|ui| {
                ui.heading("✨ Live2D Wallpapers");
                ui.add_space(10.0);
                ui.label("Interactive Live2D character wallpapers");
                ui.label("🚧 Coming soon...");
            });
        });
    }
}
