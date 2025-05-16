use crate::app::{TemplateApp, EncryptionMethod};
use egui::{Ui, ComboBox};

pub fn encrypt_text_view(ui: &mut Ui, app: &mut TemplateApp) {
    ui.label("🔒 Encrypt Text");

    ui.horizontal(|ui| {
        ui.label("Encryption Key:");
        ui.text_edit_singleline(&mut app.encryption_key);
    });

    ui.horizontal(|ui| {
        ui.label("Encryption Method");
        ComboBox::from_id_salt("encryption_method_select")
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

    if ui.button("Encrypt").clicked() {
        app.output_text = match app.selected_encryption {
            EncryptionMethod::Caesar => caesar_encrypt(&app.input_text, &app.encryption_key),
            EncryptionMethod::XOR => xor_encrypt(&app.input_text, &app.encryption_key),
        };
    }

    ui.label("Output:");
    ui.text_edit_multiline(&mut app.output_text);
}

// Simple Caesar cipher encryption shifting letters by key length (mod 26)
fn caesar_encrypt(text: &str, key: &str) -> String {
    let shift = key.len() as u8 % 26;
    text.chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                (((c as u8 - b'a' + shift) % 26) + b'a') as char
            } else if c.is_ascii_uppercase() {
                (((c as u8 - b'A' + shift) % 26) + b'A') as char
            } else {
                c
            }
        })
        .collect()
}

// XOR cipher encryption with repeating key bytes (output as hex string)
fn xor_encrypt(text: &str, key: &str) -> String {
    if key.is_empty() {
        return "Error: key is empty".to_owned();
    }
    let key_bytes = key.as_bytes();
    text.as_bytes()
        .iter()
        .enumerate()
        .map(|(i, &b)| b ^ key_bytes[i % key_bytes.len()])
        .map(|b| format!("{:02x}", b))
        .collect()
}
