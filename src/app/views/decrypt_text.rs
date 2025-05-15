use crate::TemplateApp;
use egui::Ui;

pub fn decrypt_text_view(ui: &mut Ui, app: &mut TemplateApp) {
    ui.label("🔓 Decrypt Text");

    ui.horizontal(|ui| {
        ui.label("Decryption Key:");
        ui.text_edit_singleline(&mut app.encryption_key);
    });

    ui.label("Encrypted Input:");
    ui.text_edit_multiline(&mut app.input_text);

    if ui.button("Decrypt").clicked() {
        app.output_text = format!("Decrypted({})", app.input_text);
    }

    ui.label("Decrypted Output:");
    ui.text_edit_multiline(&mut app.output_text);
}
