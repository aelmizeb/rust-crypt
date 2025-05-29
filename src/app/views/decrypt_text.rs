use crate::app::{encryption, TemplateApp};
use egui::{Ui, ComboBox, RichText, TextEdit};
use encryption::{EncryptionMethod, caesar_decrypt, xor_decrypt};


pub fn decrypt_text_view(ui: &mut Ui, app: &mut TemplateApp) {
    ui.heading(RichText::new("🔓 Decrypt Text").size(20.0).strong());
    ui.add_space(10.0);

    ui.group(|ui| {
        ui.vertical(|ui| {
            // Encryption Key
            ui.label(RichText::new("🔑 Encryption Key").strong());
            ui.add(
                TextEdit::singleline(&mut app.encryption_key)
                    .hint_text("Enter the encryption key")
                    .desired_width(ui.available_width()),
            );

            ui.add_space(10.0);

            // Encryption Method
            ui.label(RichText::new("🔒 Encryption Method").strong());
            ComboBox::from_id_salt("decryption_method_select")
                .width(ui.available_width())
                .selected_text(match app.selected_encryption {
                    EncryptionMethod::Caesar => "Caesar Cipher",
                    EncryptionMethod::XOR => "XOR Cipher",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut app.selected_encryption, EncryptionMethod::Caesar, "Caesar Cipher");
                    ui.selectable_value(&mut app.selected_encryption, EncryptionMethod::XOR, "XOR Cipher");
                });

            ui.add_space(10.0);

            // Input Text
            ui.label(RichText::new("📝 Encrypted Input").strong());
            ui.add(
                TextEdit::multiline(&mut app.input_text)
                    .desired_rows(5)
                    .desired_width(ui.available_width()),
            );

            ui.add_space(10.0);

            // Decrypt Button
            if ui.button("🔓 Decrypt").clicked() {
                app.output_text = match app.selected_encryption {
                    EncryptionMethod::Caesar => caesar_decrypt(&app.input_text, &app.encryption_key),
                    EncryptionMethod::XOR => xor_decrypt(&app.input_text, &app.encryption_key),
                };
            }

            ui.add_space(10.0);

            // Output Text
            ui.label(RichText::new("📤 Decrypted Output").strong());
            ui.add(
                TextEdit::multiline(&mut app.output_text)
                    .desired_rows(5)
                    .desired_width(ui.available_width()),
            );
        });
    });
}