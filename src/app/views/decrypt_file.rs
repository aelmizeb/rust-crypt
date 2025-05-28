use crate::app::{encryption, TemplateApp};
use egui::{Ui, ComboBox};
use rfd::FileDialog;
use std::fs;
use encryption::{EncryptionMethod, caesar_decrypt, xor_decrypt};

pub fn decrypt_file_view(ui: &mut Ui, app: &mut TemplateApp) {
    ui.label("📂 Decrypt File");

    ui.horizontal(|ui| {
        ui.label("Encryption Key:");
        ui.text_edit_singleline(&mut app.encryption_key);
    });

    ui.horizontal(|ui| {
        ui.label("Encryption Method");
        ComboBox::from_id_salt("file_decryption_method_select")
            .selected_text(match app.selected_encryption {
                EncryptionMethod::Caesar => "Caesar Cipher",
                EncryptionMethod::XOR => "XOR Cipher",
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.selected_encryption, EncryptionMethod::Caesar, "Caesar Cipher");
                ui.selectable_value(&mut app.selected_encryption, EncryptionMethod::XOR, "XOR Cipher");
            });
    });

    if ui.button("Select File to Decrypt").clicked() {
        if let Some(path) = FileDialog::new().pick_file() {
            if let Ok(data) = fs::read_to_string(&path) {
                let decrypted = match app.selected_encryption {
                    EncryptionMethod::Caesar => caesar_decrypt(&data, &app.encryption_key),
                    EncryptionMethod::XOR => xor_decrypt(&data, &app.encryption_key),
                };

                let output_path = path.with_file_name(format!(
                    "decrypted_{}",
                    path.file_name().unwrap().to_string_lossy()
                ));

                if let Err(err) = fs::write(&output_path, decrypted) {
                    app.output_text = format!("Failed to write output: {}", err);
                } else {
                    app.output_text = format!("Decryption successful. Saved to:\n{}", output_path.display());
                }
            } else {
                app.output_text = "Failed to read the selected file.".to_owned();
            }
        }
    }

    ui.label("Status:");
    ui.text_edit_multiline(&mut app.output_text);
}
