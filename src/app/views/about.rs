use egui::Ui;

pub fn about_view(ui: &mut Ui) {
    ui.label("📂 About This App");
    ui.separator();
    ui.label("🔐 Text Encryption/Decryption Tool");
    ui.label("This application allows you to securely encrypt and decrypt text using simple cipher methods:");
    ui.label("• Caesar Cipher – A basic letter-shifting algorithm.");
    ui.label("• XOR Cipher – Applies bitwise XOR using a repeating key.");
    ui.add_space(8.0);
    ui.label("🛠 Features:");
    ui.label("• Select encryption method");
    ui.label("• Enter a custom key");
    ui.label("• Encrypt or decrypt text with a single click");
    ui.label("• View input/output in real-time");
    ui.add_space(8.0);
    ui.label("📦 Built with Rust and egui.");
    ui.label("💡 Created as a simple demonstration of interactive GUI and cipher logic.");
}
