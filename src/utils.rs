use std::io::{self, Write};

pub struct Utils;

impl Utils {
    pub fn get_sensitive_data() -> Vec<u8> {
        print!("Enter the sensitive data to encrypt: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        input.trim().as_bytes().to_vec()
    }
}
