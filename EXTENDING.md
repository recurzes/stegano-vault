# Extending SteganoVault

This guide provides practical examples for extending SteganoVault with new features.

## Example 1: Adding PDF Steganography

### Step 1: Create the Implementation

Create `src/steganography/pdf.rs`:

```rust
use crate::error::{Result, SteganoError};
use crate::steganography::traits::Steganography;

pub struct PdfSteganography;

impl PdfSteganography {
    pub fn new() -> Self {
        PdfSteganography
    }

    fn calculate_capacity(pdf_size: usize) -> usize {
        // Example: use metadata or whitespace
        pdf_size / 100  // Simplified
    }
}

impl Steganography for PdfSteganography {
    fn embed(&self, carrier_path: &str, data: &[u8], output_path: &str) -> Result<()> {
        // Read PDF
        let pdf_content = std::fs::read(carrier_path)?;
        
        // Embed data in PDF metadata or whitespace
        // (actual implementation would go here)
        
        // Write modified PDF
        std::fs::write(output_path, pdf_content)?;
        
        Ok(())
    }
    
    fn extract(&self, carrier_path: &str) -> Result<Vec<u8>> {
        // Read PDF
        let pdf_content = std::fs::read(carrier_path)?;
        
        // Extract hidden data
        // (actual implementation would go here)
        let extracted_data = vec![];
        
        Ok(extracted_data)
    }
    
    fn can_embed(&self, carrier_path: &str, data_size: usize) -> Result<bool> {
        let metadata = std::fs::metadata(carrier_path)?;
        let capacity = Self::calculate_capacity(metadata.len() as usize);
        Ok(data_size <= capacity)
    }
}
```

### Step 2: Export the Module

Update `src/steganography/mod.rs`:

```rust
pub mod traits;
pub mod image;
pub mod audio;
pub mod pdf;  // Add this line

pub use traits::Steganography;
pub use image::ImageSteganography;
pub use audio::AudioSteganography;
pub use pdf::PdfSteganography;  // Add this line
```

### Step 3: Add CLI Support

Update `src/cli/mod.rs`:

```rust
// Add to imports
use crate::steganography::PdfSteganography;

// Add arguments in Command::new()
.arg(
    Arg::new("encrypt-pdf")
        .long("encrypt-pdf")
        .value_name("FILE")
        .help("Encrypt and embed data into a PDF file")
        .action(ArgAction::Set),
)
.arg(
    Arg::new("decrypt-pdf")
        .long("decrypt-pdf")
        .value_name("FILE")
        .help("Extract and decrypt data from a PDF file")
        .action(ArgAction::Set),
)

// Add handlers in run() method
if let Some(file) = matches.get_one::<String>("encrypt-pdf") {
    Self::encrypt_pdf(&crypto_manager, file, output)?;
} else if let Some(file) = matches.get_one::<String>("decrypt-pdf") {
    Self::decrypt_pdf(&crypto_manager, file)?;
}

// Add implementation methods
fn encrypt_pdf(
    crypto_manager: &CryptoManager,
    pdf_path: &str,
    output: Option<&String>,
) -> Result<()> {
    let data = Self::get_user_input()?;
    println!("Encrypting data...");
    let encrypted_data = crypto_manager.encrypt(&data)?;
    
    let output_path = output.map(|s| s.as_str()).unwrap_or("output.pdf");
    println!("Embedding encrypted data in PDF...");
    
    let steg = PdfSteganography::new();
    steg.embed(pdf_path, &encrypted_data, output_path)?;
    println!("Data encrypted and hidden inside {}", output_path);
    
    Ok(())
}

fn decrypt_pdf(crypto_manager: &CryptoManager, pdf_path: &str) -> Result<()> {
    println!("Extracting data from PDF...");
    let steg = PdfSteganography::new();
    let extracted_data = steg.extract(pdf_path)?;
    
    println!("Decrypting extracted data...");
    let decrypted_data = crypto_manager.decrypt(&extracted_data)?;
    
    match String::from_utf8(decrypted_data.clone()) {
        Ok(text) => println!("Decrypted data: {}", text),
        Err(_) => {
            println!("Decrypted data (binary, {} bytes):", decrypted_data.len());
            println!("{:02X?}", &decrypted_data[..decrypted_data.len().min(50)]);
        }
    }
    
    Ok(())
}
```

### Step 4: Update Dependencies (if needed)

Add to `Cargo.toml` if using a PDF library:

```toml
[dependencies]
# ... existing dependencies ...
pdf = "0.8"  # or whatever PDF library you choose
```

## Example 2: Adding Post-Quantum Encryption

### Step 1: Create Implementation

Create `src/crypto/post_quantum.rs`:

```rust
use crate::error::{Result, SteganoError};

pub struct PostQuantumManager {
    // Post-quantum key material
    key: Vec<u8>,
}

impl PostQuantumManager {
    pub fn new() -> Self {
        // Generate post-quantum key
        Self {
            key: vec![0; 64],  // Placeholder
        }
    }

    pub fn with_key(key: Vec<u8>) -> Result<Self> {
        if key.len() != 64 {
            return Err(SteganoError::InvalidKey(
                "Post-quantum key must be 64 bytes".to_string()
            ));
        }
        Ok(Self { key })
    }

    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>> {
        // Implement post-quantum encryption
        // (e.g., using Kyber, Dilithium, etc.)
        Ok(plaintext.to_vec())
    }

    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        // Implement post-quantum decryption
        Ok(ciphertext.to_vec())
    }

    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
}
```

### Step 2: Export Module

Update `src/crypto/mod.rs`:

```rust
pub mod encryption;
pub mod key_manager;
pub mod post_quantum;  // Add this

pub use encryption::CryptoManager;
pub use key_manager::KeyManager;
pub use post_quantum::PostQuantumManager;  // Add this
```

### Step 3: Add CLI Support

Add a `--crypto-type` flag to select encryption method:

```rust
.arg(
    Arg::new("crypto-type")
        .long("crypto-type")
        .value_name("TYPE")
        .help("Encryption type: aes256 (default) or post-quantum")
        .action(ArgAction::Set),
)
```

## Example 3: Adding Batch Processing

### Step 1: Create Batch Module

Create `src/batch/mod.rs`:

```rust
use crate::crypto::CryptoManager;
use crate::steganography::{Steganography, ImageSteganography};
use crate::error::Result;
use std::path::Path;

pub struct BatchProcessor {
    crypto: CryptoManager,
}

impl BatchProcessor {
    pub fn new(crypto: CryptoManager) -> Self {
        Self { crypto }
    }

    pub fn encrypt_directory(
        &self,
        input_dir: &str,
        output_dir: &str,
        data: &[u8],
    ) -> Result<Vec<String>> {
        let encrypted = self.crypto.encrypt(data)?;
        let mut processed = Vec::new();

        for entry in std::fs::read_dir(input_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("png") {
                let filename = path.file_name().unwrap();
                let output_path = Path::new(output_dir).join(filename);

                let steg = ImageSteganography::new();
                steg.embed(
                    path.to_str().unwrap(),
                    &encrypted,
                    output_path.to_str().unwrap(),
                )?;

                processed.push(output_path.to_string_lossy().to_string());
            }
        }

        Ok(processed)
    }
}
```

### Step 2: Export and Use

Update `src/lib.rs`:

```rust
pub mod batch;
pub use batch::BatchProcessor;
```

## Example 4: Adding Compression

### Step 1: Create Compression Module

Create `src/crypto/compression.rs`:

```rust
use crate::error::{Result, SteganoError};

pub struct CompressionManager;

impl CompressionManager {
    pub fn compress(data: &[u8]) -> Result<Vec<u8>> {
        use flate2::write::GzEncoder;
        use flate2::Compression;
        use std::io::Write;

        let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
        encoder.write_all(data)
            .map_err(|e| SteganoError::InvalidData(format!("Compression failed: {}", e)))?;
        encoder.finish()
            .map_err(|e| SteganoError::InvalidData(format!("Compression finalization failed: {}", e)))
    }

    pub fn decompress(data: &[u8]) -> Result<Vec<u8>> {
        use flate2::read::GzDecoder;
        use std::io::Read;

        let mut decoder = GzDecoder::new(data);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)
            .map_err(|e| SteganoError::InvalidData(format!("Decompression failed: {}", e)))?;
        Ok(decompressed)
    }
}
```

### Step 2: Integrate with Crypto

Update `CryptoManager` to optionally compress before encryption:

```rust
pub fn encrypt_with_compression(&self, plaintext: &[u8]) -> Result<Vec<u8>> {
    let compressed = CompressionManager::compress(plaintext)?;
    self.encrypt(&compressed)
}

pub fn decrypt_with_decompression(&self, ciphertext: &[u8]) -> Result<Vec<u8>> {
    let decrypted = self.decrypt(ciphertext)?;
    CompressionManager::decompress(&decrypted)
}
```

## Example 5: Adding Custom Error Types

For specific error handling:

```rust
// In src/error/mod.rs
#[derive(Debug)]
pub enum SteganoError {
    // ... existing variants ...
    
    // New variants
    CapacityExceeded { required: usize, available: usize },
    UnsupportedFormat(String),
    CorruptedData { expected_checksum: u32, actual_checksum: u32 },
}

impl fmt::Display for SteganoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // ... existing matches ...
            SteganoError::CapacityExceeded { required, available } => {
                write!(f, "Capacity exceeded: need {} bytes, have {} bytes available",
                       required, available)
            }
            SteganoError::UnsupportedFormat(format) => {
                write!(f, "Unsupported format: {}", format)
            }
            SteganoError::CorruptedData { expected_checksum, actual_checksum } => {
                write!(f, "Data corrupted: checksum mismatch (expected {:#X}, got {:#X})",
                       expected_checksum, actual_checksum)
            }
        }
    }
}
```

## Testing Extensions

Create tests for new features:

```rust
// tests/pdf_steg_tests.rs
#[cfg(test)]
mod tests {
    use stegano_vault::*;

    #[test]
    fn test_pdf_steganography() -> Result<()> {
        let steg = PdfSteganography::new();
        let data = b"Secret message";
        
        steg.embed("test.pdf", data, "output.pdf")?;
        let extracted = steg.extract("output.pdf")?;
        
        assert_eq!(data, extracted.as_slice());
        Ok(())
    }
}
```

## Best Practices

1. **Always implement the trait completely** - Don't leave placeholder implementations in production code

2. **Add proper error handling** - Return `Result` types and handle all error cases

3. **Write tests** - Test your extensions thoroughly

4. **Update documentation** - Add your new features to README.md

5. **Follow the pattern** - Look at existing implementations (image.rs, audio.rs) as templates

6. **Check capacity** - Always validate that the carrier can hold the data before embedding

7. **Add integration tests** - Test the full encrypt → embed → extract → decrypt cycle

## Need Help?

- Look at existing implementations in `src/steganography/`
- Check the `Steganography` trait in `src/steganography/traits.rs`
- Review error handling in `src/error/mod.rs`
- See CLI patterns in `src/cli/mod.rs`
