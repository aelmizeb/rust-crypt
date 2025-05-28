use crate::app::{encryption, TemplateApp};
use egui::{Ui, ComboBox};
use encryption::{EncryptionMethod, caesar_decrypt, xor_decrypt};

pub fn decrypt_text_view(ui: &mut Ui, app: &mut TemplateApp) {
    ui.label("🔓 Decrypt Text");

    ui.horizontal(|ui| {
        ui.label("Encryption Key:");
        ui.text_edit_singleline(&mut app.encryption_key);
    });

    ui.horizontal(|ui| {
        ui.label("Encryption Method");
        ComboBox::from_id_salt("decryption_method_select")
            .selected_text(match app.selected_encryption {
                EncryptionMethod::Caesar => "Caesar Cipher",
                EncryptionMethod::XOR => "XOR Cipher",
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.selected_encryption, EncryptionMethod::Caesar, "Caesar Cipher");
                ui.selectable_value(&mut app.selected_encryption, EncryptionMethod::XOR, "XOR Cipher");
            });
    });

    ui.label("Input:");
    ui.text_edit_multiline(&mut app.input_text);

    if ui.button("Decrypt").clicked() {
        app.output_text = match app.selected_encryption {
            EncryptionMethod::Caesar => caesar_decrypt(&app.input_text, &app.encryption_key),
            EncryptionMethod::XOR => xor_decrypt(&app.input_text, &app.encryption_key),
        };
    }

    ui.label("Output:");
    ui.text_edit_multiline(&mut app.output_text);
}
