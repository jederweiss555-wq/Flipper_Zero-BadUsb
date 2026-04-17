use base64::{Engine as _, engine::general_purpose::STANDARD};

const XOR_KEY: &[u8] = b"KryptoPilot2024!SecretXorKey#MVP";

pub fn encrypt(plaintext: &str) -> String {
    let bytes = plaintext.as_bytes();
    let encrypted: Vec<u8> = bytes
        .iter()
        .enumerate()
        .map(|(i, b)| b ^ XOR_KEY[i % XOR_KEY.len()])
        .collect();
    STANDARD.encode(encrypted)
}

pub fn decrypt(ciphertext: &str) -> String {
    let bytes = match STANDARD.decode(ciphertext) {
        Ok(b) => b,
        Err(_) => return String::new(),
    };
    let decrypted: Vec<u8> = bytes
        .iter()
        .enumerate()
        .map(|(i, b)| b ^ XOR_KEY[i % XOR_KEY.len()])
        .collect();
    String::from_utf8(decrypted).unwrap_or_default()
}
