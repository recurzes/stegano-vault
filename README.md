# SteganoVault

A secure steganography tool for hiding encrypted sensitive data within image and audio files.

## Description

SteganoVault is a command-line utility that combines advanced encryption (AES-256-GCM) with steganography techniques to securely hide sensitive information within ordinary-looking image and audio files. The tool encrypts data before embedding it, making the hidden information virtually undetectable without the correct encryption key.

## Features

- Dual-layer security: Combines AES-256-GCM encryption with steganography
- Image steganography: Hide encrypted data within PNG images
- Audio steganography: Embed secret information in WAV audio files
- PDF steganography: Conceal data within PDF documents
- Key management: Create and manage encryption keys automatically
- Modular architecture: Easy to extend with new steganography methods
- Command-line interface: Simple integration into scripts and workflows

## Installation

### Prerequisites

- Rust and Cargo (1.55.0 or later)

### Building from source

```bash
# Clone the repository
git clone https://github.com/YOUR_USERNAME/stegano-vault.git
cd stegano-vault

# Build the project
cargo build --release

# The binary will be available at
# target/release/stegano-vault
```

## Usage

### Generating a key

The tool automatically creates a new encryption key the first time you use it:

```bash
# Just specify a keyfile that doesn't exist - it will be created automatically
./stegano-vault --keyfile my.key --encrypt-image input.png --output secret.png

# You'll see: "Created and saved new key to my.key"
```

The key file (`my.key`) contains a randomly generated 256-bit (32-byte) encryption key. Keep this file secure - you'll need it to decrypt your data later.

### Hiding data in an image

```bash
# Encrypt and embed data in an image
./stegano-vault --encrypt-image input.png --keyfile my.key --output secret.png
```

You will be prompted to enter the sensitive data to encrypt.

### Extracting data from an image

```bash
# Extract and decrypt data from an image
./stegano-vault --decrypt-image secret.png --keyfile my.key
```

### Hiding data in an audio file

```bash
# Encrypt and embed data in an audio file
./stegano-vault --encrypt-audio input.wav --keyfile my.key --output secret.wav
```

### Extracting data from an audio file

```bash
# Extract and decrypt data from an audio file
./stegano-vault --decrypt-audio secret.wav --keyfile my.key
```

### Hiding data in a PDF file

```bash
# Encrypt and embed data in a PDF file
./stegano-vault --encrypt-pdf input.pdf --keyfile my.key --output secret.pdf
```

### Extracting data from a PDF file

```bash
# Extract and decrypt data from a PDF file
./stegano-vault --decrypt-pdf secret.pdf --keyfile my.key
```

## Security

SteganoVault uses the following security measures:

- **AES-256-GCM** for encryption, providing both confidentiality and integrity
- **Random nonces** for each encryption operation
- **LSB (Least Significant Bit) steganography** to hide data with minimal perceptible changes
- **Separate key files** for secure key storage

**Important**: Keep your key file secure. If lost, encrypted data cannot be recovered.

## Technical Details

### Encryption

- Uses the AES-256-GCM (Galois/Counter Mode) algorithm
- 256-bit encryption keys (32 bytes)
- Unique random nonce for each encryption operation
- Authenticated encryption provides both confidentiality and integrity

### Image Steganography

The tool hides data by modifying the least significant bits of pixel values in the image. Data is distributed across the RGB color channels, making changes imperceptible to the human eye. The implementation includes:

- Bit-by-bit embedding across color channels
- Length prefix for reliable extraction
- Capacity checking to ensure data fits within the image

### Audio Steganography

For audio files, data is embedded by modifying the least significant bits of audio samples, resulting in inaudible changes to the sound. The implementation includes:

- LSB modification of 16-bit audio samples
- Length prefix for reliable extraction
- Support for standard WAV file formats

### PDF Steganography

For PDF files, data is appended after the PDF EOF marker. Most PDF readers ignore data after the EOF marker, making this an effective steganography technique. The implementation includes:

- Trailing bytes approach (data after %%EOF marker)
- Length prefix for reliable extraction
- Marker validation for data integrity
- Compatible with most PDF viewers

## Project Structure

The project follows a modular architecture for easy extension:

```
src/
├── main.rs              # Entry point
├── lib.rs               # Library exports
├── cli/                 # Command-line interface
│   └── mod.rs
├── crypto/              # Cryptography modules
│   ├── mod.rs
│   ├── encryption.rs    # AES-256-GCM encryption
│   └── key_manager.rs   # Key file management
├── steganography/       # Steganography modules
│   ├── mod.rs
│   ├── traits.rs        # Steganography trait
│   ├── image.rs         # Image steganography
│   ├── audio.rs         # Audio steganography
│   └── pdf.rs           # PDF steganography
└── error/               # Error handling
    └── mod.rs
```

## Extending the Project

### Adding a new steganography method

To add support for a new file type or steganography technique:

1. Create a new file in `src/steganography/` (e.g., `video.rs`)
2. Implement the `Steganography` trait:

```rust
use crate::error::Result;
use crate::steganography::traits::Steganography;

pub struct VideoSteganography;

impl VideoSteganography {
    pub fn new() -> Self {
        VideoSteganography
    }
}

impl Steganography for VideoSteganography {
    fn embed(&self, carrier_path: &str, data: &[u8], output_path: &str) -> Result<()> {
        // Implementation here
        Ok(())
    }
    
    fn extract(&self, carrier_path: &str) -> Result<Vec<u8>> {
        // Implementation here
        Ok(vec![])
    }
    
    fn can_embed(&self, carrier_path: &str, data_size: usize) -> Result<bool> {
        // Implementation here
        Ok(true)
    }
}
```

3. Export the new module in `src/steganography/mod.rs`
4. Add CLI commands in `src/cli/mod.rs`

## License

MIT License

## Contributing

Contributions are welcome. Please follow these steps:

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/new-feature`)
3. Commit your changes (`git commit -m 'Add new feature'`)
4. Push to the branch (`git push origin feature/new-feature`)
5. Open a Pull Request
