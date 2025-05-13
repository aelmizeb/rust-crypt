/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,
    #[serde(skip)] value: f32,

    // New fields
    input_text: String,
    output_text: String,
    encryption_key: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            input_text: "".to_owned(),
            output_text: "".to_owned(),
            encryption_key: "defaultkey".to_owned(),
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

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("Menu", |ui| {
                        if ui.button("Encrypt Text").clicked() {
                            self.output_text = format!("Encrypted({})", self.input_text); // Dummy logic
                        }

                        if ui.button("Decrypt Text").clicked() {
                            self.output_text = format!("Decrypted({})", self.input_text); // Dummy logic
                        }

                        if ui.button("Settings").clicked() {
                            // future settings logic
                        }

                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🔐 Rust Crypt");

            ui.horizontal(|ui| {
                ui.label("Encryption Key:");
                ui.text_edit_singleline(&mut self.encryption_key);
            });

            ui.label("Input:");
            ui.text_edit_multiline(&mut self.input_text);

            ui.horizontal(|ui| {
                if ui.button("🔒 Encrypt").clicked() {
                    self.output_text = format!("Encrypted({})", self.input_text); // Replace with real encryption logic
                }
                if ui.button("🔓 Decrypt").clicked() {
                    self.output_text = format!("Decrypted({})", self.input_text); // Replace with real decryption logic
                }
            });

            ui.separator();
            ui.label("Output:");
            ui.text_edit_multiline(&mut self.output_text);

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
