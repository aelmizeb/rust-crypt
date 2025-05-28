use crate::app::{encryption, TemplateApp};
use egui::{Ui, ComboBox};
use rfd::FileDialog;
use std::fs;
use encryption::{EncryptionMethod, caesar_encrypt_file, xor_encrypt_file};

pub fn encrypt_file_view(ui: &mut Ui, app: &mut TemplateApp) {
    ui.label("📁 Encrypt File");

    ui.horizontal(|ui| {
        ui.label("Encryption Key:");
        ui.text_edit_singleline(&mut app.encryption_key);
    });

    ui.horizontal(|ui| {
        ui.label("Encryption Method");
        ComboBox::from_id_salt("file_encryption_method_select")
            .selected_text(match app.selected_encryption {
                EncryptionMethod::Caesar => "Caesar Cipher",
                EncryptionMethod::XOR => "XOR Cipher",
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.selected_encryption, EncryptionMethod::Caesar, "Caesar Cipher");
                ui.selectable_value(&mut app.selected_encryption, EncryptionMethod::XOR, "XOR Cipher");
            });
    });

    if ui.button("Select File to Encrypt").clicked() {
        if let Some(path) = FileDialog::new().pick_file() {
            if let Ok(data) = fs::read_to_string(&path) {
                let encrypted = match app.selected_encryption {
                    EncryptionMethod::Caesar => caesar_encrypt_file(&data, &app.encryption_key),
                    EncryptionMethod::XOR => xor_encrypt_file(&data, &app.encryption_key),
                };

                let output_path = path.with_file_name(format!(
                    "encrypted_{}",
                    path.file_name().unwrap().to_string_lossy()
                ));

                if let Err(err) = fs::write(&output_path, encrypted) {
                    app.output_text = format!("Failed to write output: {}", err);
                } else {
                    app.output_text = format!("Encryption successful. Saved to:\n{}", output_path.display());
                }
            } else {
                app.output_text = "Failed to read the selected file.".to_owned();
            }
        }
    }

    ui.label("Status:");
    ui.text_edit_multiline(&mut app.output_text);
}
