use crate::TemplateApp;
use egui::Ui;

pub fn encrypt_text_view(ui: &mut Ui, app: &mut TemplateApp) {
    ui.label("🔒 Encrypt Text");

    ui.horizontal(|ui| {
        ui.label("Encryption Key:");
        ui.text_edit_singleline(&mut app.encryption_key);
    });

    ui.label("Input:");
    ui.text_edit_multiline(&mut app.input_text);

    if ui.button("Encrypt").clicked() {
        app.output_text = format!("Encrypted({})", app.input_text);
    }

    ui.label("Output:");
    ui.text_edit_multiline(&mut app.output_text);
}
