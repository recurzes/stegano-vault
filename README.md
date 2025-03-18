# SteganoVault

A secure steganography tool for hiding encrypted sensitive data within image and audio files.

## 🔒 Description

SteganoVault is a command-line utility that combines advanced encryption (AES-256-GCM) with steganography techniques to securely hide sensitive information within ordinary-looking image and audio files. The tool allows users to encrypt data before embedding it, making the hidden information virtually undetectable without the correct encryption key.

## ✨ Features

- **Dual-layer security**: Combines AES-256-GCM encryption with steganography
- **Image steganography**: Hide encrypted data within PNG images
- **Audio steganography**: Embed secret information in WAV audio files
- **Key management**: Create and manage encryption keys
- **Command-line interface**: Easy to use in scripts or manual operation

## 🔧 Installation

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

## 📖 Usage

### Generating a key

```bash
# Create a new encryption key
./stegano-vault --keyfile my.key
```

### Hiding data in an image

```bash
# Encrypt and embed data in an image
./stegano-vault --encrypt-image input.png --keyfile my.key --output secret.png
```

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

## 🔐 Security

SteganoVault uses the following security measures:

- **AES-256-GCM** for encryption, providing both confidentiality and integrity
- **Random nonces** for each encryption operation
- **LSB (Least Significant Bit) steganography** to hide data with minimal perceptible changes
- **Separate key files** for secure key storage

⚠️ **Important**: Keep your key file secure. If lost, encrypted data cannot be recovered.

## 🧠 Technical Details

### Encryption

- Uses the AES-256-GCM (Galois/Counter Mode) algorithm
- 256-bit encryption keys (32 bytes)
- Unique random nonce for each encryption operation

### Image Steganography

The tool hides data by modifying the least significant bits of pixel values in the image, making changes imperceptible to the human eye.

### Audio Steganography

For audio files, data is embedded by modifying the least significant bits of audio samples, resulting in inaudible changes to the sound.

## 📄 License

[MIT License](LICENSE)

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request
