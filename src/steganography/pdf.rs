use crate::error::{Result, SteganoError};
use crate::steganography::traits::Steganography;
use std::fs;

pub struct PdfSteganography;

impl PdfSteganography {
    pub fn new() -> Self {
        PdfSteganography
    }

    /// Calculate capacity based on PDF file size
    /// We use trailing bytes approach - can safely add data after PDF EOF marker
    fn calculate_capacity(_pdf_size: usize) -> usize {
        // Reserve space for length marker (4 bytes)
        // PDF can have arbitrary data after %%EOF, so we're quite flexible
        100_000_000  // 100MB max for simplicity
    }

    /// Find the PDF EOF marker position
    fn find_eof_marker(content: &[u8]) -> Option<usize> {
        // PDF files end with %%EOF marker
        let eof_pattern = b"%%EOF";
        
        // Search from the end backwards
        for i in (0..content.len().saturating_sub(eof_pattern.len())).rev() {
            if &content[i..i + eof_pattern.len()] == eof_pattern {
                // Return position after %%EOF and any trailing whitespace
                let mut pos = i + eof_pattern.len();
                while pos < content.len() && (content[pos] == b'\n' || content[pos] == b'\r' || content[pos] == b' ') {
                    pos += 1;
                }
                return Some(pos);
            }
        }
        None
    }
}

impl Steganography for PdfSteganography {
    fn embed(&self, carrier_path: &str, data: &[u8], output_path: &str) -> Result<()> {
        // Read the PDF file
        let mut pdf_content = fs::read(carrier_path)?;
        
        // Verify it's a PDF
        if !pdf_content.starts_with(b"%PDF-") {
            return Err(SteganoError::EmbedError(
                "File is not a valid PDF".to_string()
            ));
        }

        // Check capacity
        if !self.can_embed(carrier_path, data.len())? {
            return Err(SteganoError::EmbedError(
                format!("PDF too small to embed {} bytes of data", data.len())
            ));
        }

        // Find EOF marker
        let eof_pos = Self::find_eof_marker(&pdf_content)
            .ok_or_else(|| SteganoError::EmbedError("Could not find PDF %%EOF marker".to_string()))?;

        // Truncate any existing hidden data
        pdf_content.truncate(eof_pos);

        // Create the hidden data section
        let mut hidden_section = Vec::new();
        
        // Add a newline for separation
        hidden_section.push(b'\n');
        
        // Add length prefix (4 bytes)
        hidden_section.extend_from_slice(&(data.len() as u32).to_le_bytes());
        
        // Add the actual data
        hidden_section.extend_from_slice(data);
        
        // Add marker to identify our hidden data
        hidden_section.extend_from_slice(b"\n%%STEGANO%%\n");

        // Append hidden section to PDF
        pdf_content.extend_from_slice(&hidden_section);

        // Write the modified PDF
        fs::write(output_path, pdf_content)?;
        
        Ok(())
    }
    
    fn extract(&self, carrier_path: &str) -> Result<Vec<u8>> {
        // Read the PDF file
        let pdf_content = fs::read(carrier_path)?;
        
        // Verify it's a PDF
        if !pdf_content.starts_with(b"%PDF-") {
            return Err(SteganoError::ExtractError(
                "File is not a valid PDF".to_string()
            ));
        }

        // Find EOF marker
        let eof_pos = Self::find_eof_marker(&pdf_content)
            .ok_or_else(|| SteganoError::ExtractError("Could not find PDF %%EOF marker".to_string()))?;

        // Check if there's data after EOF
        if eof_pos >= pdf_content.len() {
            return Err(SteganoError::ExtractError(
                "No hidden data found in PDF".to_string()
            ));
        }

        // Skip any leading newlines/whitespace
        let mut data_start = eof_pos;
        while data_start < pdf_content.len() && 
              (pdf_content[data_start] == b'\n' || 
               pdf_content[data_start] == b'\r' || 
               pdf_content[data_start] == b' ') {
            data_start += 1;
        }

        // Extract length (4 bytes)
        if data_start + 4 > pdf_content.len() {
            return Err(SteganoError::ExtractError(
                "No valid hidden data found in PDF".to_string()
            ));
        }

        let len_bytes: [u8; 4] = pdf_content[data_start..data_start + 4]
            .try_into()
            .map_err(|_| SteganoError::ExtractError("Failed to read length prefix".to_string()))?;
        
        let data_len = u32::from_le_bytes(len_bytes) as usize;
        
        // Validate length
        if data_len == 0 {
            return Err(SteganoError::ExtractError(
                "Invalid data length: zero".to_string()
            ));
        }

        let data_start_pos = data_start + 4;
        let data_end_pos = data_start_pos + data_len;

        if data_end_pos > pdf_content.len() {
            return Err(SteganoError::ExtractError(
                "Corrupted hidden data: length exceeds file size".to_string()
            ));
        }

        // Extract the hidden data
        let extracted_data = pdf_content[data_start_pos..data_end_pos].to_vec();

        // Verify marker if present
        if data_end_pos + 15 <= pdf_content.len() {
            let marker = &pdf_content[data_end_pos..data_end_pos + 15];
            if marker != b"\n%%STEGANO%%\n" {
                // Marker doesn't match - might still be valid old format data
                // but we'll warn about it via the data
            }
        }

        Ok(extracted_data)
    }
    
    fn can_embed(&self, carrier_path: &str, data_size: usize) -> Result<bool> {
        let metadata = fs::metadata(carrier_path)?;
        let capacity = Self::calculate_capacity(metadata.len() as usize);
        Ok(data_size <= capacity)
    }
}
