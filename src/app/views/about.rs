use egui::{RichText, Ui};

pub fn about_view(ui: &mut Ui) {
    let version = env!("CARGO_PKG_VERSION");

    ui.heading(RichText::new("📂 About This App").size(20.0).strong());
    ui.label(RichText::new(format!("Version: {}", version)).monospace().small());
    ui.separator();
    ui.add_space(10.0);

    ui.group(|ui| {
        ui.vertical(|ui| {
            ui.label(RichText::new("🔐 Purpose").strong());
            ui.label("This application allows you to securely encrypt and decrypt text using simple cipher methods:");
            ui.add_space(4.0);
            ui.label("• Caesar Cipher – A basic letter-shifting algorithm.");
            ui.label("• XOR Cipher – Applies bitwise XOR using a repeating key.");

            ui.add_space(10.0);
            ui.label(RichText::new("🛠 Features").strong());
            ui.label("• Select encryption method");
            ui.label("• Enter a custom key");
            ui.label("• Encrypt or decrypt text with a single click");
            ui.label("• View input/output in real-time");

            ui.add_space(10.0);
            ui.label(RichText::new("💻 Technology").strong());
            ui.label("• Built with Rust and egui");
            ui.label("• Created as a simple demonstration of interactive GUI and cipher logic");
        });
    });
}
