// Wallmgr GUI - Modern Wallpaper Manager
// Features: Local browsing, Collection management, Online sources (booru)

mod tabs;
mod components;
mod models;
mod utils;

use eframe::egui;
use tabs::{LocalTab, CollectionTab, OnlineTab, SettingsTab};

fn main() -> eframe::Result<()> {
    env_logger::init();
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 900.0])
            .with_min_inner_size([1000.0, 600.0])
            .with_title("Wallmgr - Wallpaper Manager"),
        ..Default::default()
    };

    eframe::run_native(
        "wallmgr-gui",
        options,
        Box::new(|cc| Ok(Box::new(WallmgrApp::new(cc)))),
    )
}

#[derive(PartialEq)]
enum Tab {
    Local,
    Collection,
    Online,
    Settings,
}

struct WallmgrApp {
    current_tab: Tab,
    local_tab: LocalTab,
    collection_tab: CollectionTab,
    online_tab: OnlineTab,
    settings_tab: SettingsTab,
}

impl WallmgrApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Configure fonts and style
        let mut style = (*cc.egui_ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(8.0, 8.0);
        style.interaction.selectable_labels = false; // Disable text selection
        cc.egui_ctx.set_style(style);

        Self {
            current_tab: Tab::Local,
            local_tab: LocalTab::new(),
            collection_tab: CollectionTab::new(),
            online_tab: OnlineTab::new(),
            settings_tab: SettingsTab::new(),
        }
    }
}

impl eframe::App for WallmgrApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Disable text selection globally
        ctx.style_mut(|style| {
            style.interaction.selectable_labels = false;
        });

        // Top menu bar with tabs
        egui::TopBottomPanel::top("tabs").show(ctx, |ui| {
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                // Logo/Title on the left
                ui.add_space(10.0);
                ui.label(egui::RichText::new("🎨 Wallmgr").size(16.0).strong());
                
                ui.add_space(30.0);
                
                // 3 main tabs centered
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    let tab_height = 34.0;
                    let tab_width = 160.0;
                    let font_size = 14.0;
                    
                    if ui.add_sized(
                        [tab_width, tab_height],
                        egui::SelectableLabel::new(
                            self.current_tab == Tab::Local,
                            egui::RichText::new("📁 Local Wallpapers").size(font_size)
                        )
                    ).clicked() {
                        self.current_tab = Tab::Local;
                    }
                    
                    if ui.add_sized(
                        [tab_width, tab_height],
                        egui::SelectableLabel::new(
                            self.current_tab == Tab::Collection,
                            egui::RichText::new("🏛️ Official").size(font_size)
                        )
                    ).clicked() {
                        self.current_tab = Tab::Collection;
                    }
                    
                    if ui.add_sized(
                        [tab_width, tab_height],
                        egui::SelectableLabel::new(
                            self.current_tab == Tab::Online,
                            egui::RichText::new("🌐 Online Sources").size(font_size)
                        )
                    ).clicked() {
                        self.current_tab = Tab::Online;
                    }
                });
                
                // Settings tab pushed to the right
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(10.0);
                    let tab_height = 34.0;
                    let tab_width = 120.0;
                    let font_size = 14.0;
                    
                    if ui.add_sized(
                        [tab_width, tab_height],
                        egui::SelectableLabel::new(
                            self.current_tab == Tab::Settings,
                            egui::RichText::new("⚙ Settings").size(font_size)
                        )
                    ).clicked() {
                        self.current_tab = Tab::Settings;
                    }
                });
            });
            ui.add_space(5.0);
        });

        // Main content area
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_tab {
                Tab::Local => self.local_tab.show(ui, ctx),
                Tab::Collection => self.collection_tab.show(ui, ctx),
                Tab::Online => self.online_tab.show(ui, ctx),
                Tab::Settings => self.settings_tab.show(ui, ctx),
            }
        });
    }
}
