use hound::{WavReader, WavWriter};
use crate::error::{Result, SteganoError};
use crate::steganography::traits::Steganography;

pub struct AudioSteganography;

impl AudioSteganography {
    pub fn new() -> Self {
        AudioSteganography
    }

    fn calculate_capacity(sample_count: usize) -> usize {
        // Each sample can hold 1 bit
        // Subtract 32 bits for length prefix
        if sample_count < 32 {
            0
        } else {
            (sample_count - 32) / 8
        }
    }
}

impl Steganography for AudioSteganography {
    fn embed(&self, carrier_path: &str, data: &[u8], output_path: &str) -> Result<()> {
        let mut reader = WavReader::open(carrier_path)?;
        let spec = reader.spec();
        let samples: Vec<i16> = reader
            .samples::<i16>()
            .collect::<std::result::Result<Vec<_>, _>>()?;

        // Check capacity
        if samples.len() < 32 {
            return Err(SteganoError::EmbedError(
                "Audio file too short to embed data".to_string()
            ));
        }

        let capacity = Self::calculate_capacity(samples.len());
        if data.len() > capacity {
            return Err(SteganoError::EmbedError(
                format!("Audio file too small to embed {} bytes of data", data.len())
            ));
        }

        let mut modified_samples = samples.clone();
        
        // Embed length (4 bytes = 32 bits)
        let len_bytes = (data.len() as u32).to_le_bytes();
        let mut bit_count = 0;
        
        for byte_idx in 0..4 {
            for bit_idx in 0..8 {
                let bit = (len_bytes[byte_idx] >> bit_idx) & 1;
                modified_samples[bit_count] = 
                    (modified_samples[bit_count] & !1) | (bit as i16);
                bit_count += 1;
            }
        }
        
        // Embed the actual data
        for byte_idx in 0..data.len() {
            for bit_idx in 0..8 {
                let bit = (data[byte_idx] >> bit_idx) & 1;
                modified_samples[bit_count] = 
                    (modified_samples[bit_count] & !1) | (bit as i16);
                bit_count += 1;
            }
        }

        let mut writer = WavWriter::create(output_path, spec)?;
        for sample in modified_samples {
            writer.write_sample(sample)?;
        }
        writer.finalize()?;
        
        Ok(())
    }

    fn extract(&self, carrier_path: &str) -> Result<Vec<u8>> {
        let mut reader = WavReader::open(carrier_path)?;
        let samples: Vec<i16> = reader
            .samples::<i16>()
            .collect::<std::result::Result<Vec<_>, _>>()?;

        if samples.len() < 32 {
            return Err(SteganoError::ExtractError(
                "Audio file too short to extract data".to_string()
            ));
        }
        
        // Extract length (4 bytes = 32 bits)
        let mut len_bytes = [0u8; 4];
        let mut bit_count = 0;
        
        for byte_idx in 0..4 {
            let mut byte = 0u8;
            for bit_idx in 0..8 {
                let bit = (samples[bit_count] & 1) as u8;
                byte |= bit << bit_idx;
                bit_count += 1;
            }
            len_bytes[byte_idx] = byte;
        }
        
        let data_len = u32::from_le_bytes(len_bytes) as usize;
        
        // Validate data length
        let capacity = Self::calculate_capacity(samples.len());
        if data_len > capacity {
            return Err(SteganoError::ExtractError(
                "Corrupted data: claimed length exceeds audio file capacity".to_string()
            ));
        }
        
        // Extract the actual data
        let mut extracted_data = vec![0u8; data_len];
        
        for byte_idx in 0..data_len {
            let mut byte = 0u8;
            for bit_idx in 0..8 {
                let bit = (samples[bit_count] & 1) as u8;
                byte |= bit << bit_idx;
                bit_count += 1;
            }
            extracted_data[byte_idx] = byte;
        }

        Ok(extracted_data)
    }

    fn can_embed(&self, carrier_path: &str, data_size: usize) -> Result<bool> {
        let reader = WavReader::open(carrier_path)?;
        let sample_count = reader.len() as usize;
        let capacity = Self::calculate_capacity(sample_count);
        Ok(data_size <= capacity)
    }
}
