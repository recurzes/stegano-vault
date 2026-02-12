use std::fs;
use std::path::Path;
use crate::error::{Result, SteganoError};
use super::encryption::CryptoManager;

pub struct KeyManager;

impl KeyManager {
    /// Load an existing key from a file or create a new one if it doesn't exist
    pub fn load_or_create(keyfile_path: &str) -> Result<CryptoManager> {
        let path = Path::new(keyfile_path);
        
        if path.exists() {
            Self::load(keyfile_path)
        } else {
            Self::create(keyfile_path)
        }
    }

    /// Load an existing key from a file
    pub fn load(keyfile_path: &str) -> Result<CryptoManager> {
        let key_data = fs::read(keyfile_path)?;
        
        if key_data.len() != 32 {
            return Err(SteganoError::InvalidKey(
                format!("Invalid key size: expected 32 bytes, got {}", key_data.len())
            ));
        }
        
        let mut key = [0u8; 32];
        key.copy_from_slice(&key_data);
        
        Ok(CryptoManager::with_key(key))
    }

    /// Create a new key and save it to a file
    pub fn create(keyfile_path: &str) -> Result<CryptoManager> {
        let crypto_manager = CryptoManager::new();
        fs::write(keyfile_path, &crypto_manager.get_key())?;
        Ok(crypto_manager)
    }

    /// Save a key to a file
    pub fn save(crypto_manager: &CryptoManager, keyfile_path: &str) -> Result<()> {
        fs::write(keyfile_path, &crypto_manager.get_key())?;
        Ok(())
    }
}
