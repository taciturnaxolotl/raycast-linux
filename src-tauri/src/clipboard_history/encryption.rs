use crate::error::AppError;
use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};

const KEYRING_SERVICE: &str = "dev.byteatatime.raycast";
const KEYRING_USERNAME: &str = "clipboard_history_key";

pub fn get_encryption_key() -> Result<[u8; 32], AppError> {
    let entry = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USERNAME)?;
    match entry.get_password() {
        Ok(hex_key) => {
            let key_bytes =
                hex::decode(hex_key).map_err(|e| AppError::ClipboardHistory(e.to_string()))?;
            Ok(key_bytes.try_into().unwrap())
        }
        Err(keyring::Error::NoEntry) => {
            let new_key: [u8; 32] = rand::random();
            let hex_key = hex::encode(new_key);
            entry.set_password(&hex_key)?;
            Ok(new_key)
        }
        Err(e) => Err(e.into()),
    }
}

pub fn encrypt(data: &str, key: &[u8; 32]) -> Result<String, AppError> {
    let cipher = Aes256Gcm::new(key.into());
    let nonce_bytes: [u8; 12] = rand::random();
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ciphertext = cipher
        .encrypt(nonce, data.as_bytes())
        .map_err(|e| AppError::ClipboardHistory(e.to_string()))?;

    let mut result = nonce_bytes.to_vec();
    result.extend_from_slice(&ciphertext);
    Ok(hex::encode(result))
}

pub fn decrypt(hex_data: &str, key: &[u8; 32]) -> Result<String, AppError> {
    let data = hex::decode(hex_data).map_err(|e| AppError::ClipboardHistory(e.to_string()))?;
    if data.len() < 12 {
        return Err(AppError::ClipboardHistory("Invalid encrypted data".into()));
    }
    let (nonce_bytes, ciphertext) = data.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);
    let cipher = Aes256Gcm::new(key.into());
    let decrypted_bytes = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| AppError::ClipboardHistory(e.to_string()))?;
    String::from_utf8(decrypted_bytes).map_err(|e| AppError::ClipboardHistory(e.to_string()))
}
