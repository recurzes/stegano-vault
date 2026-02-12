//! SteganoVault - A secure steganography tool with encryption
//! 
//! This library provides modular steganography and encryption capabilities
//! for hiding encrypted data within image and audio files.

pub mod error;
pub mod crypto;
pub mod steganography;
pub mod cli;

// Re-export commonly used types
pub use error::{Result, SteganoError};
pub use crypto::{CryptoManager, KeyManager};
pub use steganography::{Steganography, ImageSteganography, AudioSteganography};
