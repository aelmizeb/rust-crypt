use eframe::egui;
use serde::{Deserialize, Serialize};

mod views;

use views::{
    decrypt_file::decrypt_file_view,
    decrypt_text::decrypt_text_view,
    encrypt_file::encrypt_file_view,
    encrypt_text::encrypt_text_view,
    about::about_view,
    settings::settings_view,
};

#[derive(serde::Deserialize, serde::Serialize, PartialEq)]
enum CurrentView {
    EncryptText,
    DecryptText,
    EncryptFile,
    DecryptFile,
    About,
    Settings,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    input_text: String,
    output_text: String,
    encryption_key: String,
    current_view: CurrentView,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            input_text: "".to_owned(),
            output_text: "".to_owned(),
            encryption_key: "defaultkey".to_owned(),
            current_view: CurrentView::EncryptText,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        // Set the window title based on current view
        let title = match self.current_view {
            CurrentView::EncryptText => "🔐 Encrypt Text - Rust Crypt",
            CurrentView::DecryptText => "🔓 Decrypt Text - Rust Crypt",
            CurrentView::EncryptFile => "🗄️ Encrypt File - Rust Crypt",
            CurrentView::DecryptFile => "📂 Decrypt File - Rust Crypt",
            CurrentView::About => "📖 About - Rust Crypt",
            CurrentView::Settings => "⚙️ Settings - Rust Crypt",
        };

        ctx.send_viewport_cmd(egui::ViewportCommand::Title(title.to_owned()));

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                ui.menu_button("Menu", |ui| {
                    if ui.button("Encrypt Text").clicked() {
                        self.current_view = CurrentView::EncryptText;
                        ui.close_menu();
                    }

                    if ui.button("Decrypt Text").clicked() {
                        self.current_view = CurrentView::DecryptText;
                        ui.close_menu();
                    }

                    if ui.button("Encrypt File").clicked() {
                        self.current_view = CurrentView::EncryptFile;
                        ui.close_menu();
                    }

                    if ui.button("Decrypt File").clicked() {
                        self.current_view = CurrentView::DecryptFile;
                        ui.close_menu();
                    }

                    if ui.button("About").clicked() {
                        self.current_view = CurrentView::About;
                        ui.close_menu();
                    }

                    if ui.button("Settings").clicked() {
                        self.current_view = CurrentView::Settings;
                        ui.close_menu();
                    }

                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_view {
                CurrentView::EncryptText => encrypt_text_view(ui, self),
                CurrentView::DecryptText => decrypt_text_view(ui, self),
                CurrentView::EncryptFile => encrypt_file_view(ui),
                CurrentView::DecryptFile => decrypt_file_view(ui),
                CurrentView::About => about_view(ui),
                CurrentView::Settings => settings_view(ui),
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

use chrono::Datelike;

fn powered_by(ui: &mut egui::Ui) {
    let current_year = chrono::Utc::now().year();

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Developed by ");
        ui.hyperlink_to("Abdellatif EL MIZEB", "https://github.com/aelmizeb");
        ui.label(" ");
        ui.label(format!("{}", current_year));
        ui.label(". Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
