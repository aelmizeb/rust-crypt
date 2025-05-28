pub mod caesar;
pub mod xor;

pub use caesar::{caesar_encrypt, caesar_decrypt, caesar_encrypt_file};
pub use xor::{xor_encrypt, xor_decrypt, xor_encrypt_file};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EncryptionMethod {
    Caesar,
    XOR,
}