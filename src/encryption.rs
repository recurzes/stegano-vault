use aes_gcm::{
    aead::Aead,
    aes::{cipher::typenum, Aes256Dec},
    Aes256Gcm, Key, KeyInit, Nonce,
};
use hound;
use image::{GenericImage, GenericImageView};
use rand::{thread_rng, RngCore};

pub struct CrytpoManager {
    aes_key: [u8; 32],
}

pub struct AudioSteganography;

impl AudioSteganography {
    pub fn embed_in_audio(audio_path: &str, data: &[u8], output_path: &str) {
        let mut reader = hound::WavReader::open(audio_path).expect("Failed to open audio filed");
        let spec = reader.spec();
        let samples: Vec<i16> = reader.samples::<i16>().map(|s| s.unwrap()).collect();

        let mut modified_samples = samples.clone();
        for (i, &byte) in data.iter().enumerate() {
            if i >= modified_samples.len() {
                break;
            }
            modified_samples[i] = (modified_samples[i] & !1) | (byte as i16 & 1);
        }

        let mut writer =
            hound::WavWriter::create(output_path, spec).expect("Failed to create output");
        for sample in modified_samples {
            writer.write_sample(sample).expect("Failed to write sample");
        }
    }

    pub fn extract_from_audio(audio_path: &str) -> Result<Vec<u8>, String> {
        let mut reader = hound::WavReader::open(audio_path).expect("Failed to open audio file");
        let samples: Vec<i16> = reader.samples::<i16>().map(|s| s.unwrap()).collect();

        if samples.len() < 4 {
            return Err("Audio file too short".to_string());
        }
        let mut len_bytes = [0u8; 4];
        for i in 0..4 {
            len_bytes[i] = (samples[i] & 1) as u8;
        }
        let data_len = u32::from_le_bytes(len_bytes) as usize;
        if data_len + 4 > samples.len() {
            return Err("Corrupted data: length exceeds audio file capacity".to_string());
        }
        let mut extracted_data = Vec::with_capacity(data_len);
        for i in 0..data_len {
            extracted_data.push((samples[i + 4] & 1) as u8);
        }

        Ok(extracted_data)
    }
}

impl CrytpoManager {
    // Create a new instance with a random key
    pub fn new() -> Self {
        let mut key = [0u8; 32];
        thread_rng().fill_bytes(&mut key);
        Self { aes_key: key }
    }
    
    // Create with an existing key
    pub fn with_key(key: [u8; 32]) -> Self {
        Self { aes_key: key }
    }

    // Get a copy of the current key
    pub fn get_key(&self) -> [u8; 32] {
        self.aes_key
    }

    pub fn encrypt_aes(&self, plaintext: &[u8]) -> Vec<u8> {
        let key = Key::<Aes256Gcm>::from_slice(&self.aes_key);
        let cipher = Aes256Gcm::new(key);
        
        // Generate a random nonce
        let mut nonce_bytes = [0u8; 12];
        thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        // Prepend nonce to ciphertext for later decryption
        let mut result = nonce_bytes.to_vec();
        let encrypted = cipher.encrypt(nonce, plaintext.as_ref())
            .expect("Encryption Failed");
        result.extend_from_slice(&encrypted);
        
        // Include a length prefix for easier extraction
        let len = result.len() as u32;
        let len_bytes = len.to_le_bytes();
        let mut final_result = len_bytes.to_vec();
        final_result.extend_from_slice(&result);
        
        final_result
    }

    pub fn decrypt_aes(&self, ciphertext: &[u8]) -> Vec<u8> {
        // Need at least 4 bytes for length + 12 bytes for nonce + 1 byte for data
        if ciphertext.len() < 17 {
            panic!("Invalid ciphertext: too short");
        }
        
        // Extract the length prefix
        let len_bytes: [u8; 4] = ciphertext[0..4].try_into().unwrap();
        let expected_len = u32::from_le_bytes(len_bytes) as usize;
        
        // Validate length
        if expected_len != ciphertext.len() - 4 {
            panic!("Corrupted data: length mismatch");
        }
        
        // Extract the actual data
        let actual_data = &ciphertext[4..];
        
        // Extract the nonce from the first 12 bytes
        let nonce = Nonce::from_slice(&actual_data[0..12]);
        
        let key = Key::<Aes256Gcm>::from_slice(&self.aes_key);
        let cipher = Aes256Gcm::new(key);
        
        // Decrypt using the extracted nonce and the rest of the ciphertext
        cipher.decrypt(nonce, &actual_data[12..])
            .expect("Decryption Failed")
    }
    
    // Image steganography methods - bit-by-bit embedding for better reliability
    pub fn embed_in_image(image_path: &str, data: &[u8], output_path: &str) {
        let mut img = image::open(image_path).expect("Failed to open image");
        let (width, height) = img.dimensions();
        
        // First, embed the length of data (4 bytes = 32 bits)
        let len_bytes = (data.len() as u32).to_le_bytes();
        
        // Make sure the image is large enough
        let total_bits_needed = 32 + (data.len() * 8);
        let total_pixels = width * height;
        if total_bits_needed > (total_pixels * 3).try_into().unwrap() {  // 3 color channels (RGB) per pixel
            panic!("Image too small to embed data");
        }
        
        // Embed length (32 bits)
        let mut bit_count = 0;
        
        for byte_idx in 0..4 {
            for bit_idx in 0..8 {
                let x = bit_count % width;
                let y = bit_count / width;
                let color_idx = bit_count % 3;  // Distribute across RGB channels
                
                let bit = (len_bytes[byte_idx] >> bit_idx) & 1;
                
                let mut pixel = img.get_pixel(x, y);
                pixel[color_idx.try_into().unwrap()] = (pixel[color_idx.try_into().unwrap()] & 0xFE) | bit;
                img.put_pixel(x, y, pixel);
                
                bit_count += 1;
            }
        }
        
        // Embed the actual data
        for byte_idx in 0..data.len() {
            for bit_idx in 0..8 {
                let x = bit_count % width;
                let y = bit_count / width;
                let color_idx = bit_count % 3;  // Distribute across RGB channels
                
                let bit = (data[byte_idx] >> bit_idx) & 1;
                
                let mut pixel = img.get_pixel(x, y);
                pixel[color_idx.try_into().unwrap()] = (pixel[color_idx.try_into().unwrap()] & 0xFE) | bit;
                img.put_pixel(x, y, pixel);
                
                bit_count += 1;
            }
        }

        img.save(output_path).expect("Failed to save image");
    }

    pub fn extract_from_image(image_path: &str) -> Result<Vec<u8>, String> {
        let img = image::open(image_path).expect("Failed to open image");
        let (width, height) = img.dimensions();
        
        // First extract the data length (4 bytes = 32 bits)
        let mut len_bytes = [0u8; 4];
        let mut bit_count = 0;
        
        // Extract Length using LSB
        for byte_idx in 0..4 {
            let mut byte = 0u8;
            for bit_idx in 0..8 {
                let x = bit_count % width;
                let y = bit_count / width;
                let color_idx = bit_count % 3;
                
                let pixel = img.get_pixel(x, y);
                let bit = pixel[color_idx.try_into().unwrap()] & 1;
                
                // Store bits in correct order
                byte |= bit << bit_idx;
                bit_count += 1;
            }
            len_bytes[byte_idx] = byte;
        }
        
        let data_len = u32::from_le_bytes(len_bytes) as usize;

        if data_len < 16 {
            return Err("Invalid data lengthL: too small for encrypted content".to_string());
        }
        
        // Validate data length
        let total_bits_needed = 32 + (data_len * 8);
        let total_pixels = width * height;
        if total_bits_needed > (total_pixels * 3).try_into().unwrap() {
            return Err("Corrupted data: claimed length exceeds image capacity".to_string());
        }
        
        // Now extract the actual data
        let mut extracted_data = vec![0u8; data_len];
        
        for byte_idx in 0..data_len {
            let mut byte = 0u8;
            for bit_idx in 0..8 {
                let x = bit_count % width;
                let y = bit_count / width;
                let color_idx = bit_count % 3;
                
                let pixel = img.get_pixel(x, y);
                let bit = pixel[color_idx.try_into().unwrap()] & 1;
                
                byte |= bit << bit_idx;
                bit_count += 1;
            }
            extracted_data[byte_idx] = byte;
        }

        if extracted_data.len() < 16 {
            return Err("Extracted data is too short to be valid".to_string());
        }

        Ok(extracted_data)
    }
}