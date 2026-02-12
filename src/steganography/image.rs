use image::{GenericImage, GenericImageView};
use crate::error::{Result, SteganoError};
use crate::steganography::traits::Steganography;

pub struct ImageSteganography;

impl ImageSteganography {
    pub fn new() -> Self {
        ImageSteganography
    }

    fn calculate_capacity(width: u32, height: u32) -> usize {
        // Each pixel has 3 color channels (RGB), each can hold 1 bit
        // Subtract 32 bits for length prefix
        let total_bits = (width as usize) * (height as usize) * 3;
        (total_bits - 32) / 8
    }
}

impl Steganography for ImageSteganography {
    fn embed(&self, carrier_path: &str, data: &[u8], output_path: &str) -> Result<()> {
        let mut img = image::open(carrier_path)?;
        let (width, _height) = img.dimensions();
        
        // Check capacity
        if !self.can_embed(carrier_path, data.len())? {
            return Err(SteganoError::EmbedError(
                format!("Image too small to embed {} bytes of data", data.len())
            ));
        }
        
        // Embed length (4 bytes = 32 bits)
        let len_bytes = (data.len() as u32).to_le_bytes();
        let mut bit_count = 0;
        
        // Embed length bits
        for byte_idx in 0..4 {
            for bit_idx in 0..8 {
                let x = bit_count % width;
                let y = bit_count / width;
                let color_idx = (bit_count % 3) as usize;
                
                let bit = (len_bytes[byte_idx] >> bit_idx) & 1;
                
                let mut pixel = img.get_pixel(x, y);
                pixel[color_idx] = (pixel[color_idx] & 0xFE) | bit;
                img.put_pixel(x, y, pixel);
                
                bit_count += 1;
            }
        }
        
        // Embed the actual data
        for byte_idx in 0..data.len() {
            for bit_idx in 0..8 {
                let x = bit_count % width;
                let y = bit_count / width;
                let color_idx = (bit_count % 3) as usize;
                
                let bit = (data[byte_idx] >> bit_idx) & 1;
                
                let mut pixel = img.get_pixel(x, y);
                pixel[color_idx] = (pixel[color_idx] & 0xFE) | bit;
                img.put_pixel(x, y, pixel);
                
                bit_count += 1;
            }
        }

        img.save(output_path)?;
        Ok(())
    }

    fn extract(&self, carrier_path: &str) -> Result<Vec<u8>> {
        let img = image::open(carrier_path)?;
        let (width, height) = img.dimensions();
        
        // Extract the data length (4 bytes = 32 bits)
        let mut len_bytes = [0u8; 4];
        let mut bit_count = 0;
        
        for byte_idx in 0..4 {
            let mut byte = 0u8;
            for bit_idx in 0..8 {
                let x = bit_count % width;
                let y = bit_count / width;
                let color_idx = (bit_count % 3) as usize;
                
                let pixel = img.get_pixel(x, y);
                let bit = pixel[color_idx] & 1;
                
                byte |= bit << bit_idx;
                bit_count += 1;
            }
            len_bytes[byte_idx] = byte;
        }
        
        let data_len = u32::from_le_bytes(len_bytes) as usize;

        // Validate data length
        if data_len < 16 {
            return Err(SteganoError::ExtractError(
                "Invalid data length: too small for encrypted content".to_string()
            ));
        }
        
        let capacity = Self::calculate_capacity(width, height);
        if data_len > capacity {
            return Err(SteganoError::ExtractError(
                "Corrupted data: claimed length exceeds image capacity".to_string()
            ));
        }
        
        // Extract the actual data
        let mut extracted_data = vec![0u8; data_len];
        
        for byte_idx in 0..data_len {
            let mut byte = 0u8;
            for bit_idx in 0..8 {
                let x = bit_count % width;
                let y = bit_count / width;
                let color_idx = (bit_count % 3) as usize;
                
                let pixel = img.get_pixel(x, y);
                let bit = pixel[color_idx] & 1;
                
                byte |= bit << bit_idx;
                bit_count += 1;
            }
            extracted_data[byte_idx] = byte;
        }

        Ok(extracted_data)
    }

    fn can_embed(&self, carrier_path: &str, data_size: usize) -> Result<bool> {
        let img = image::open(carrier_path)?;
        let (width, height) = img.dimensions();
        let capacity = Self::calculate_capacity(width, height);
        Ok(data_size <= capacity)
    }
}
