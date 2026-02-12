use std::fmt;

#[derive(Debug)]
pub enum SteganoError {
    IoError(std::io::Error),
    ImageError(image::ImageError),
    AudioError(hound::Error),
    EncryptionError(String),
    DecryptionError(String),
    EmbedError(String),
    ExtractError(String),
    InvalidKey(String),
    InvalidData(String),
}

impl fmt::Display for SteganoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SteganoError::IoError(e) => write!(f, "IO error: {}", e),
            SteganoError::ImageError(e) => write!(f, "Image error: {}", e),
            SteganoError::AudioError(e) => write!(f, "Audio error: {}", e),
            SteganoError::EncryptionError(msg) => write!(f, "Encryption error: {}", msg),
            SteganoError::DecryptionError(msg) => write!(f, "Decryption error: {}", msg),
            SteganoError::EmbedError(msg) => write!(f, "Embed error: {}", msg),
            SteganoError::ExtractError(msg) => write!(f, "Extract error: {}", msg),
            SteganoError::InvalidKey(msg) => write!(f, "Invalid key: {}", msg),
            SteganoError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
        }
    }
}

impl std::error::Error for SteganoError {}

impl From<std::io::Error> for SteganoError {
    fn from(err: std::io::Error) -> Self {
        SteganoError::IoError(err)
    }
}

impl From<image::ImageError> for SteganoError {
    fn from(err: image::ImageError) -> Self {
        SteganoError::ImageError(err)
    }
}

impl From<hound::Error> for SteganoError {
    fn from(err: hound::Error) -> Self {
        SteganoError::AudioError(err)
    }
}

pub type Result<T> = std::result::Result<T, SteganoError>;
