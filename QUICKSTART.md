# SteganoVault Quick Start Guide

## Installation

```bash
# Build the project
cargo build --release

# The binary will be at:
./target/release/stegano-vault
```

## Key Generation

Keys are automatically generated on first use. Just specify a keyfile path that doesn't exist:

```bash
# This will create "my.key" automatically
./target/release/stegano-vault --encrypt-image photo.png --keyfile my.key -o secret.png
```

Output:
```
Created and saved new key to my.key
Enter the sensitive data to encrypt: [your secret text]
Encrypting data...
Embedding encrypted data in image...
Data encrypted and hidden inside secret.png
```

**Important:** Keep `my.key` secure! You'll need it to decrypt your data.

## Basic Usage Examples

### Image Steganography

#### Hide data in an image
```bash
./target/release/stegano-vault --encrypt-image photo.png --keyfile my.key -o secret.png
```

#### Extract data from an image
```bash
./target/release/stegano-vault --decrypt-image secret.png --keyfile my.key
```

### Audio Steganography

#### Hide data in audio
```bash
./target/release/stegano-vault --encrypt-audio song.wav --keyfile my.key -o secret.wav
```

#### Extract data from audio
```bash
./target/release/stegano-vault --decrypt-audio secret.wav --keyfile my.key
```

### PDF Steganography

#### Hide data in PDF
```bash
./target/release/stegano-vault --encrypt-pdf document.pdf --keyfile my.key -o secret.pdf
```

#### Extract data from PDF
```bash
./target/release/stegano-vault --decrypt-pdf secret.pdf --keyfile my.key
```

## Common Workflows

### Hiding a password

```bash
# Encrypt
./target/release/stegano-vault --encrypt-image vacation.png --keyfile pass.key -o vacation_secret.png
# Enter: MySecretPassword123

# Later, retrieve it
./target/release/stegano-vault --decrypt-image vacation_secret.png --keyfile pass.key
# Output: Decrypted Data: MySecretPassword123
```

### Hiding API keys

```bash
# Encrypt
./target/release/stegano-vault --encrypt-pdf invoice.pdf --keyfile api.key -o invoice_secret.pdf
# Enter: sk-1234567890abcdef

# Retrieve
./target/release/stegano-vault --decrypt-pdf invoice_secret.pdf --keyfile api.key
# Output: Decrypted data: sk-1234567890abcdef
```

### Multiple secrets with different keys

```bash
# Secret 1 - Password in image
./target/release/stegano-vault --encrypt-image photo1.png --keyfile pass.key -o secret1.png

# Secret 2 - API key in audio
./target/release/stegano-vault --encrypt-audio music.wav --keyfile api.key -o secret2.wav

# Secret 3 - Credit card in PDF
./target/release/stegano-vault --encrypt-pdf doc.pdf --keyfile cc.key -o secret3.pdf
```

## File Requirements

### Images
- **Format:** PNG recommended (any format supported by the `image` crate)
- **Size:** Must be large enough to hold encrypted data
- **Recommendation:** Use images at least 500x500 pixels for small text

### Audio
- **Format:** WAV files only
- **Size:** Must have enough samples to hold encrypted data
- **Recommendation:** Use at least 1 second of audio for small text

### PDF
- **Format:** Any valid PDF file
- **Size:** Practically unlimited (up to 100MB of hidden data)
- **Recommendation:** PDF is great for larger amounts of data

## Security Tips

1. **Keep your keyfiles secure**
   - Store keyfiles separately from carrier files
   - Use strong file permissions: `chmod 600 my.key`
   - Back them up securely

2. **Choose good carrier files**
   - Use ordinary-looking files that don't stand out
   - Images: vacation photos, screenshots, memes
   - Audio: music, podcasts, sound effects
   - PDF: invoices, documents, receipts

3. **Key management**
   - Use different keys for different secrets
   - Name keys meaningfully: `email_pass.key`, `bank_api.key`
   - Never commit keys to version control

4. **Verify extraction**
   - Always test extraction after encryption
   - Make sure you can decrypt your data before deleting the original

## Troubleshooting

### "Image too small to embed data"
- Use a larger image
- Compress your data before encrypting (not yet implemented)
- Split data across multiple images

### "Could not find PDF %%EOF marker"
- The PDF file might be corrupted
- Some PDFs are encrypted - decrypt them first
- Try a different PDF

### "Decryption failed"
- Wrong key file
- File was modified after encryption
- Using wrong operation (e.g., decrypt-image on an audio file)

### "No keyfile specified"
- You forgot the `--keyfile` option
- Add: `--keyfile your_key.key`

## Advanced Usage

### Custom output location
```bash
./target/release/stegano-vault --encrypt-image input.png --keyfile key.key -o /path/to/output.png
```

### Using relative paths
```bash
./target/release/stegano-vault --encrypt-pdf ../docs/file.pdf --keyfile ~/.keys/secret.key -o ./secure.pdf
```

## Next Steps

- Read `README.md` for detailed technical information
- Check `ARCHITECTURE.md` to understand the code structure
- See `EXTENDING.md` to learn how to add new steganography methods
- Review `MIGRATION_GUIDE.md` if upgrading from v0.1.0

## Quick Reference

| Operation | Command Template |
|-----------|------------------|
| Encrypt Image | `--encrypt-image INPUT.png --keyfile KEY.key -o OUTPUT.png` |
| Decrypt Image | `--decrypt-image INPUT.png --keyfile KEY.key` |
| Encrypt Audio | `--encrypt-audio INPUT.wav --keyfile KEY.key -o OUTPUT.wav` |
| Decrypt Audio | `--decrypt-audio INPUT.wav --keyfile KEY.key` |
| Encrypt PDF | `--encrypt-pdf INPUT.pdf --keyfile KEY.key -o OUTPUT.pdf` |
| Decrypt PDF | `--decrypt-pdf INPUT.pdf --keyfile KEY.key` |
| Help | `--help` |
| Version | `--version` |
