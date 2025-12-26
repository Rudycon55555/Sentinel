// Sentinel/src/Backend/Cryptography.rs

//! Sentinel Cryptography Module
//!
//! Provides:
//! - Argon2id hashing (with developer-provided salt)
//! - SHA256 hashing
//! - AES256 encryption/decryption (developer-provided keys)

use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, PasswordHash};
use sha2::{Sha256, Digest};
use aes_gcm::{Aes256Gcm, Key, Nonce}; 
use aes_gcm::aead::{Aead, NewAead};

/// Hash a password using Argon2id with a developer-provided salt.
pub fn hash_argon2id(password: &str, salt: &str) -> Result<String, String> {
    let salt = SaltString::b64_encode(salt.as_bytes())
        .map_err(|e| e.to_string())?;

    let argon = Argon2::default();
    let hash = argon.hash_password(password.as_bytes(), &salt)
        .map_err(|e| e.to_string())?;

    Ok(hash.to_string())
}

/// Verify an Argon2id hash.
pub fn verify_argon2id(password: &str, hash: &str) -> bool {
    if let Ok(parsed) = PasswordHash::new(hash) {
        Argon2::default().verify_password(password.as_bytes(), &parsed).is_ok()
    } else {
        false
    }
}

/// Hash using SHA256.
pub fn hash_sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Encrypt data using AES256 (developer provides 32-byte key).
pub fn encrypt_aes256(key: &[u8; 32], nonce: &[u8; 12], plaintext: &str) -> Result<Vec<u8>, String> {
    let cipher = Aes256Gcm::new(Key::from_slice(key));
    cipher.encrypt(Nonce::from_slice(nonce), plaintext.as_bytes())
        .map_err(|e| e.to_string())
}

/// Decrypt AES256 data.
pub fn decrypt_aes256(key: &[u8; 32], nonce: &[u8; 12], ciphertext: &[u8]) -> Result<String, String> {
    let cipher = Aes256Gcm::new(Key::from_slice(key));
    let decrypted = cipher.decrypt(Nonce::from_slice(nonce), ciphertext)
        .map_err(|e| e.to_string())?;

    String::from_utf8(decrypted).map_err(|e| e.to_string())
}
