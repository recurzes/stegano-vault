// In main.rs
use clap::{Arg, ArgAction, Command};
use std::fs;
use std::io::{self, Write};
mod encryption;
mod utils;

use encryption::{CrytpoManager, AudioSteganography};

fn main() {
    let matches = Command::new("SteganoVault")
        .version("1.0")
        .author("Recurzion")
        .about("Secure password storage with steganography")
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
                .help("Path to output file (default: output.png or output.wav)")
                .action(ArgAction::Set),
        )
        .get_matches();

    // Handle keyfile option
    let crypto_manager = if let Some(keyfile) = matches.get_one::<String>("keyfile") {
        // Try to load existing key from file
        match fs::read(keyfile) {
            Ok(key_data) => {
                if key_data.len() != 32 {
                    eprintln!("Invalid key size in keyfile, expected 32 bytes");
                    return;
                }
                let mut key = [0u8; 32];
                key.copy_from_slice(&key_data);
                CrytpoManager::with_key(key)
            }
            Err(_) => {
                // Create a new key and save it
                let cm = CrytpoManager::new();
                fs::write(keyfile, &cm.get_key()).expect("Failed to write key to file");
                println!("Created and saved new key to {}", keyfile);
                cm
            }
        }
    } else {
        eprintln!("Warning: No keyfile specified. Encryption/decryption will fail without a keyfile.");
        eprintln!("Use --keyfile option to specify a key file.");
        return;
    };

    // Get output file path if specified
    let output = matches.get_one::<String>("output");

    // Handle encrypt-image operation
    if let Some(file) = matches.get_one::<String>("encrypt-image") {
        let data = utils::Utils::get_sensitive_data();
        println!("Encrypting data...");
        let encrypted_data = crypto_manager.encrypt_aes(&data);
        
        let output_path = output.cloned().unwrap_or("output.png".to_string());
        println!("Embedding encrypted data in image...");
        
        encryption::CrytpoManager::embed_in_image(file, &encrypted_data, &output_path);
        println!("Data encrypted and hidden inside {}", output_path);
    }

    // Handle decrypt-image operation
    if let Some(file) = matches.get_one::<String>("decrypt-image") {
        println!("Extracting data from image...");
        match encryption::CrytpoManager::extract_from_image(file) {
            Ok(extracted_data) => {
                println!("Decrypting extracted data...");
                match std::panic::catch_unwind(|| {
                    crypto_manager.decrypt_aes(&extracted_data)
                }) {
                    Ok(decrypted_data) => {
                        match String::from_utf8(decrypted_data) {
                            Ok(text) => println!("Decrypted Data: {}", text),
                            Err(_) => eprintln!("Warning: Decrypted data is not valid UTF-8 text"),
                        }                        
                    },
                    Err(_) => {
                        eprintln!("Error: Decryption failed. This could be because:");
                        eprintln!("- You're using a different key than the one used for encryption");
                        eprintln!("- The data in the image has been corrupted");
                        eprintln!("- The file doesn't contain valid steganographic data");
                    }
                }
            },
            Err(e) => {
                eprintln!("Error extracting data from image: {}", e);
            }
        }
    }

    // Handle encrypt-audio operation
    if let Some(file) = matches.get_one::<String>("encrypt-audio") {
        let data = utils::Utils::get_sensitive_data();
        println!("Encrypting data...");
        let encrypted_data = crypto_manager.encrypt_aes(&data);
        
        let output_path = output.cloned().unwrap_or("output.wav".to_string());
        println!("Embedding encrypted data in audio...");
        
        encryption::AudioSteganography::embed_in_audio(file, &encrypted_data, &output_path);
        println!("Data encrypted and hidden inside {}", output_path);
    }

    // Handle decrypt-audio operation
    if let Some(file) = matches.get_one::<String>("decrypt-audio") {
        println!("Extracting data from audio...");
        match encryption::AudioSteganography::extract_from_audio(file) {
            Ok(extracted_data) => {
                println!("Decrypting extracted data...");
                match std::panic::catch_unwind(|| {
                    crypto_manager.decrypt_aes(&extracted_data)
                }) {
                    Ok(decrypted_data) => {
                        match String::from_utf8(decrypted_data) {
                            Ok(text) => println!("Decrypted data: {}", text),
                            Err(_) => eprintln!("Warning: Decrypted data is not valid UTF-8 text"),
                        }
                    },
                    Err(_) => {
                        eprintln!("Error: Decryption failed. This could be because:");
                        eprintln!("- You're using a different key than the one used for encryption");
                        eprintln!("- The data in the audio file has been corrupted");
                        eprintln!("- The file doesn't contain valid steganographic data");
                    }
                }
            },
            Err(e) => {
                eprintln!("Error extracting data from audio: {}", e);
            }
        }
    }
}