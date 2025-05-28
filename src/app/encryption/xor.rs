pub fn xor_decrypt(hex: &str, key: &str) -> String {
    if key.is_empty() {
        return "Error: key is empty".to_owned();
    }

    let key_bytes = key.as_bytes();
    let bytes = (0..hex.len())
        .step_by(2)
        .filter_map(|i| u8::from_str_radix(&hex[i..i + 2], 16).ok())
        .collect::<Vec<u8>>();

    bytes
        .iter()
        .enumerate()
        .map(|(i, &b)| b ^ key_bytes[i % key_bytes.len()])
        .map(|b| b as char)
        .collect()
}

pub fn xor_encrypt(text: &str, key: &str) -> String {
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

pub fn xor_encrypt_file(input: &str, key: &str) -> String {
    xor_encrypt(input, key)
}