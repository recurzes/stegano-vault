use stegano_vault::cli::Cli;
use std::process;

fn main() {
    if let Err(e) = Cli::run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
