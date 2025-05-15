use egui::{Ui, Visuals};

pub fn settings_view(ui: &mut Ui) {
    ui.heading("⚙️ Settings");
    ui.separator();

    ui.label("Theme preference:");

    // Buttons for theme selection
    ui.horizontal(|ui| {
        if ui.button("🌙 Dark").clicked() {
            ui.ctx().set_visuals(Visuals::dark());
        }
        if ui.button("☀️ Light").clicked() {
            ui.ctx().set_visuals(Visuals::light());
        }
        if ui.button("🖥 System").clicked() {
            ui.ctx().set_visuals(Visuals::default()); // system / default theme
        }
    });
}
