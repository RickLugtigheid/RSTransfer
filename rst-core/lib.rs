pub mod transfer;
pub mod error;

// Internal modules
mod io;

use std::io::Read;
use sha2::{Sha256, Digest};

#[macro_export]
macro_rules! print_verbose {
    // No arguments: just print a newline (if that's needed)
    () => {
        std::io::Write::write_all(&mut std::io::stdout(), b"\n").unwrap();
    };

    // With arguments: format the message and print it with the "verbose" label in blue
    ($($arg:tt)*) => {{
        // Print the "verbose" prefix in blue, followed by the formatted message
        print!("\x1b[34mverbose\x1b[0m: ");
        println!($($arg)*); // We can use println! here instead of print!
    }};
}

/// Calculate the SHA256 hash of a file
/// 
/// # Arguments
///
/// * `file_path` - The path to the file to calculate the hash for
///
/// # Returns
///
/// * `Some(String)` - The SHA256 hash of the file
/// * `None` - If the file could not be opened
/// 
fn sha256_file(file_path: &str) -> Option<String> {
    // Attempt to open the file
    let file = match std::fs::File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open file '{}': {}", file_path, e);
            return None;
        }
    };

    let mut reader = std::io::BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192]; // 8 KB buffer

    // Read the file in chunks and update the hasher
    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break, // EOF
            Ok(n) => hasher.update(&buffer[..n]),
            Err(e) => {
                eprintln!("Error reading file '{}': {}", file_path, e);
                
                return None;
            }
        }
    }

    Some(format!("{:x}", hasher.finalize()))
}
