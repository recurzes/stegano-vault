use crate::error::Result;

/// Trait for steganography operations
/// Implement this trait to add new steganography methods
pub trait Steganography {
    /// Embed data into a carrier file
    fn embed(&self, carrier_path: &str, data: &[u8], output_path: &str) -> Result<()>;
    
    /// Extract hidden data from a carrier file
    fn extract(&self, carrier_path: &str) -> Result<Vec<u8>>;
    
    /// Check if a carrier file can hold the given amount of data
    fn can_embed(&self, carrier_path: &str, data_size: usize) -> Result<bool>;
}
