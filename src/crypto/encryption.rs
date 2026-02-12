use aes_gcm::{
    aead::Aead,
    Aes256Gcm, Key, KeyInit, Nonce,
};
use rand::{rng, RngCore};
use crate::error::{Result, SteganoError};

pub struct CryptoManager {
    aes_key: [u8; 32],
}

impl CryptoManager {
    /// Create a new instance with a random key
    pub fn new() -> Self {
        let mut key = [0u8; 32];
        rng().fill_bytes(&mut key);
        Self { aes_key: key }
    }
    
    /// Create with an existing key
    pub fn with_key(key: [u8; 32]) -> Self {
        Self { aes_key: key }
    }

    /// Get a copy of the current key
    pub fn get_key(&self) -> [u8; 32] {
        self.aes_key
    }

    /// Encrypt plaintext using AES-256-GCM
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>> {
        let key = Key::<Aes256Gcm>::from_slice(&self.aes_key);
        let cipher = Aes256Gcm::new(key);
        
        // Generate a random nonce
        let mut nonce_bytes = [0u8; 12];
        rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        // Encrypt the data
        let encrypted = cipher
            .encrypt(nonce, plaintext.as_ref())
            .map_err(|e| SteganoError::EncryptionError(format!("AES encryption failed: {}", e)))?;
        
        // Prepend nonce to ciphertext for later decryption
        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&encrypted);
        
        // Include a length prefix for easier extraction
        let len = result.len() as u32;
        let len_bytes = len.to_le_bytes();
        let mut final_result = len_bytes.to_vec();
        final_result.extend_from_slice(&result);
        
        Ok(final_result)
    }

    /// Decrypt ciphertext using AES-256-GCM
    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        // Need at least 4 bytes for length + 12 bytes for nonce + 1 byte for data
        if ciphertext.len() < 17 {
            return Err(SteganoError::DecryptionError(
                "Invalid ciphertext: too short".to_string()
            ));
        }
        
        // Extract the length prefix
        let len_bytes: [u8; 4] = ciphertext[0..4]
            .try_into()
            .map_err(|_| SteganoError::DecryptionError("Failed to parse length".to_string()))?;
        let expected_len = u32::from_le_bytes(len_bytes) as usize;
        
        // Validate length
        if expected_len != ciphertext.len() - 4 {
            return Err(SteganoError::DecryptionError(
                "Corrupted data: length mismatch".to_string()
            ));
        }
        
        // Extract the actual data
        let actual_data = &ciphertext[4..];
        
        // Extract the nonce from the first 12 bytes
        let nonce = Nonce::from_slice(&actual_data[0..12]);
        
        let key = Key::<Aes256Gcm>::from_slice(&self.aes_key);
        let cipher = Aes256Gcm::new(key);
        
        // Decrypt using the extracted nonce and the rest of the ciphertext
        cipher
            .decrypt(nonce, &actual_data[12..])
            .map_err(|e| SteganoError::DecryptionError(format!("AES decryption failed: {}", e)))
    }
}

impl Default for CryptoManager {
    fn default() -> Self {
        Self::new()
    }
}
