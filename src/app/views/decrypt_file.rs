use crate::app::{encryption, TemplateApp};
use egui::{Ui, ComboBox, RichText, TextEdit};
use rfd::FileDialog;
use std::fs;
use encryption::{EncryptionMethod, caesar_decrypt, xor_decrypt, custom_decrypt_file};

pub fn decrypt_file_view(ui: &mut Ui, app: &mut TemplateApp) {
    ui.heading(RichText::new("📂 Decrypt File").size(20.0).strong());
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
            ComboBox::from_id_salt("file_decryption_method_select")
                .width(ui.available_width())
                .selected_text(match app.selected_encryption {
                    EncryptionMethod::Caesar => "Caesar Cipher",
                    EncryptionMethod::XOR => "XOR Cipher",
                    EncryptionMethod::Custom => "Custom Script",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut app.selected_encryption, EncryptionMethod::Caesar, "Caesar Cipher");
                    ui.selectable_value(&mut app.selected_encryption, EncryptionMethod::XOR, "XOR Cipher");
                    ui.selectable_value(&mut app.selected_encryption, EncryptionMethod::Custom, "Custom Script");
                });

            ui.add_space(10.0);

            // Show custom script input only if custom encryption selected
            if app.selected_encryption == EncryptionMethod::Custom {
                ui.label(RichText::new("✍️ Custom Decryption Script (Rhai)").strong());
                ui.add(
                    TextEdit::multiline(&mut app.custom_script)
                        .desired_rows(10)
                        .desired_width(ui.available_width())
                        .hint_text("Enter your Rhai script here.\nUse variables 'text' for input and 'key' for the encryption key."),
                );
                ui.add_space(10.0);
            }

            // File Selection Button
            if ui.button("📁 Select File to Decrypt").clicked() {
                if let Some(path) = FileDialog::new().pick_file() {
                    if let Ok(data) = fs::read_to_string(&path) {
                        let decrypted = match app.selected_encryption {
                            EncryptionMethod::Caesar => caesar_decrypt(&data, &app.encryption_key),
                            EncryptionMethod::XOR => xor_decrypt(&data, &app.encryption_key),
                            EncryptionMethod::Custom => {
                                custom_decrypt_file(&data, &app.encryption_key, &app.custom_script)
                            }
                        };

                        let output_path = path.with_file_name(format!(
                            "decrypted_{}",
                            path.file_name().unwrap().to_string_lossy()
                        ));

                        if let Err(err) = fs::write(&output_path, decrypted) {
                            app.output_text = format!("❌ Failed to write output: {}", err);
                        } else {
                            app.output_text = format!("✅ Decryption successful.\nSaved to:\n{}", output_path.display());
                        }
                    } else {
                        app.output_text = "❌ Failed to read the selected file.".to_owned();
                    }
                }
            }

            ui.add_space(10.0);

            // Status Output
            ui.label(RichText::new("📄 Status").strong());
            ui.add(
                TextEdit::multiline(&mut app.output_text)
                    .desired_rows(4)
                    .desired_width(ui.available_width()),
            );
        });
    });
}
