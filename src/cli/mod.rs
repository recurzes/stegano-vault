use clap::{Arg, ArgAction, Command};
use std::io::{self, Write};
use crate::crypto::{CryptoManager, KeyManager};
use crate::steganography::{Steganography, ImageSteganography, AudioSteganography, PdfSteganography};
use crate::error::{Result, SteganoError};

pub struct Cli;

impl Cli {
    pub fn run() -> Result<()> {
        let matches = Command::new("SteganoVault")
            .version("1.0")
            .author("Recurzion")
            .about("Secure steganography tool with encryption")
            .arg(
                Arg::new("encrypt-image")
                    .long("encrypt-image")
                    .value_name("FILE")
                    .help("Encrypt and embed data into an image")
                    .action(ArgAction::Set),
            )
            .arg(
                Arg::new("decrypt-image")
                    .long("decrypt-image")
                    .value_name("FILE")
                    .help("Extract and decrypt data from an image")
                    .action(ArgAction::Set),
            )
            .arg(
                Arg::new("encrypt-audio")
                    .long("encrypt-audio")
                    .value_name("FILE")
                    .help("Encrypt and embed data into an audio file")
                    .action(ArgAction::Set),
            )
            .arg(
                Arg::new("decrypt-audio")
                    .long("decrypt-audio")
                    .value_name("FILE")
                    .help("Extract and decrypt data from an audio file")
                    .action(ArgAction::Set),
            )
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
            .arg(
                Arg::new("keyfile")
                    .long("keyfile")
                    .value_name("FILE")
                    .help("Path to keyfile for encryption/decryption")
                    .action(ArgAction::Set),
            )
            .arg(
                Arg::new("output")
                    .long("output")
                    .short('o')
                    .value_name("FILE")
                    .help("Path to output file (default: output.png, output.wav, or output.pdf)")
                    .action(ArgAction::Set),
            )
            .get_matches();

        // Handle keyfile option
        let keyfile = matches.get_one::<String>("keyfile")
            .ok_or_else(|| SteganoError::InvalidKey(
                "No keyfile specified. Use --keyfile option to specify a key file.".to_string()
            ))?;

        let crypto_manager = KeyManager::load_or_create(keyfile)?;
        
        // Check if this is a new key
        if !std::path::Path::new(keyfile).exists() {
            println!("Created and saved new key to {}", keyfile);
        }

        // Get output file path if specified
        let output = matches.get_one::<String>("output");

        // Handle operations
        if let Some(file) = matches.get_one::<String>("encrypt-image") {
            Self::encrypt_image(&crypto_manager, file, output)?;
        } else if let Some(file) = matches.get_one::<String>("decrypt-image") {
            Self::decrypt_image(&crypto_manager, file)?;
        } else if let Some(file) = matches.get_one::<String>("encrypt-audio") {
            Self::encrypt_audio(&crypto_manager, file, output)?;
        } else if let Some(file) = matches.get_one::<String>("decrypt-audio") {
            Self::decrypt_audio(&crypto_manager, file)?;
        } else if let Some(file) = matches.get_one::<String>("encrypt-pdf") {
            Self::encrypt_pdf(&crypto_manager, file, output)?;
        } else if let Some(file) = matches.get_one::<String>("decrypt-pdf") {
            Self::decrypt_pdf(&crypto_manager, file)?;
        } else {
            return Err(SteganoError::InvalidData(
                "No operation specified. Use --help for usage information.".to_string()
            ));
        }

        Ok(())
    }

    fn get_user_input() -> Result<Vec<u8>> {
        print!("Enter the sensitive data to encrypt: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        Ok(input.trim().as_bytes().to_vec())
    }

    fn encrypt_image(
        crypto_manager: &CryptoManager,
        image_path: &str,
        output: Option<&String>,
    ) -> Result<()> {
        let data = Self::get_user_input()?;
        println!("Encrypting data...");
        let encrypted_data = crypto_manager.encrypt(&data)?;
        
        let output_path = output.map(|s| s.as_str()).unwrap_or("output.png");
        println!("Embedding encrypted data in image...");
        
        let steg = ImageSteganography::new();
        steg.embed(image_path, &encrypted_data, output_path)?;
        println!("Data encrypted and hidden inside {}", output_path);
        
        Ok(())
    }

    fn decrypt_image(crypto_manager: &CryptoManager, image_path: &str) -> Result<()> {
        println!("Extracting data from image...");
        let steg = ImageSteganography::new();
        let extracted_data = steg.extract(image_path)?;
        
        println!("Decrypting extracted data...");
        let decrypted_data = crypto_manager.decrypt(&extracted_data)?;
        
        match String::from_utf8(decrypted_data.clone()) {
            Ok(text) => println!("Decrypted Data: {}", text),
            Err(_) => {
                println!("Decrypted data (binary, {} bytes):", decrypted_data.len());
                println!("{:02X?}", &decrypted_data[..decrypted_data.len().min(50)]);
            }
        }
        
        Ok(())
    }

    fn encrypt_audio(
        crypto_manager: &CryptoManager,
        audio_path: &str,
        output: Option<&String>,
    ) -> Result<()> {
        let data = Self::get_user_input()?;
        println!("Encrypting data...");
        let encrypted_data = crypto_manager.encrypt(&data)?;
        
        let output_path = output.map(|s| s.as_str()).unwrap_or("output.wav");
        println!("Embedding encrypted data in audio...");
        
        let steg = AudioSteganography::new();
        steg.embed(audio_path, &encrypted_data, output_path)?;
        println!("Data encrypted and hidden inside {}", output_path);
        
        Ok(())
    }

    fn decrypt_audio(crypto_manager: &CryptoManager, audio_path: &str) -> Result<()> {
        println!("Extracting data from audio...");
        let steg = AudioSteganography::new();
        let extracted_data = steg.extract(audio_path)?;
        
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
}
