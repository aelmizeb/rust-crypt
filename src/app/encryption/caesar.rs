pub fn caesar_decrypt(text: &str, key: &str) -> String {
    let shift = key.len() as u8 % 26;
    text.chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                (((26 + c as u8 - b'a' - shift) % 26) + b'a') as char
            } else if c.is_ascii_uppercase() {
                (((26 + c as u8 - b'A' - shift) % 26) + b'A') as char
            } else {
                c
            }
        })
        .collect()
}

pub fn caesar_encrypt(text: &str, key: &str) -> String {
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

pub fn caesar_encrypt_file(input: &str, key: &str) -> String {
    caesar_encrypt(input, key)
}