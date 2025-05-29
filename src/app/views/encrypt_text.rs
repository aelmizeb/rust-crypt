use crate::app::{encryption, TemplateApp};
use egui::{Ui, ComboBox, RichText, TextEdit};
use encryption::{EncryptionMethod, caesar_encrypt, xor_encrypt};

pub fn encrypt_text_view(ui: &mut Ui, app: &mut TemplateApp) {
    ui.heading(RichText::new("🔒 Encrypt Text").size(20.0).strong());
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
            ComboBox::from_id_salt("encryption_method_select")
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
            ui.label(RichText::new("📝 Text to Encrypt").strong());
            ui.add(
                TextEdit::multiline(&mut app.input_text)
                    .desired_rows(5)
                    .desired_width(ui.available_width()),
            );

            ui.add_space(10.0);

            // Encrypt Button
            if ui.button("🔒 Encrypt").clicked() {
                app.output_text = match app.selected_encryption {
                    EncryptionMethod::Caesar => caesar_encrypt(&app.input_text, &app.encryption_key),
                    EncryptionMethod::XOR => xor_encrypt(&app.input_text, &app.encryption_key),
                };
            }

            ui.add_space(10.0);

            // Output Text
            ui.label(RichText::new("📤 Encrypted Output").strong());
            ui.add(
                TextEdit::multiline(&mut app.output_text)
                    .desired_rows(5)
                    .desired_width(ui.available_width()),
            );
        });
    });
}