pub mod caesar;
pub mod xor;
pub mod custom;

pub use caesar::{caesar_encrypt, caesar_decrypt, caesar_encrypt_file};
pub use xor::{xor_encrypt, xor_decrypt, xor_encrypt_file};
pub use custom::{custom_encrypt, custom_encrypt_file, custom_decrypt, custom_decrypt_file};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EncryptionMethod {
    Caesar,
    XOR,
    Custom,
}