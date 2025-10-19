pub mod progress;
pub mod error;

use std::{io::{Read, Write}, net::TcpStream};
use sha2::{Sha256, Digest};

use crate::progress::{Progress, RecvByteCounter, SendProgressBar};

pub fn send_file(mut stream: TcpStream, file_path: &str) {
    // Open the file
    let file = std::fs::File::open(file_path);
    if file.is_err() {
        error::Error::FileNotFound(file_path.to_string()).exit();
    }
    let mut file = file.unwrap();

    // Get the total file size
    let total_bytes = file.metadata().unwrap().len();

    // Initialize the progress bar and buffer
    let mut progress = SendProgressBar::new(total_bytes, 30);
    let mut buffer = [0u8; 4096];
    
    loop {
        // Read a chunk of data from the file
        let bytes_read = file.read(&mut buffer).expect("Read failed");
        if bytes_read == 0 {
            break;
        }

        // Write the chunk to the socket
        stream.write_all(&buffer[..bytes_read]).expect("Write failed");
        progress.update(bytes_read as u64);
    }

    progress.finish();

    let sha256 = sha256_file(file_path);
    if sha256.is_none() {
        return;
    }
    println!("SHA256 of send file: \n{}", sha256.unwrap());
    
    println!("--------------------------------------------------\n");
    println!("The file was successfully received. To verify integrity, compare the checksum with the sender's provided value.");
}

pub fn recv_file(mut stream: TcpStream, file_path: &str) {
    let mut file = std::fs::File::create(file_path).expect("Failed to create file");

    let mut progress = RecvByteCounter::new();
    let mut buffer = [0u8; 4096];

    loop {
        // Read a chunk of data from the socket
        let bytes_read = stream.read(&mut buffer).expect("Read failed");
        if bytes_read == 0 {
            break;
        }

        // Write the chunk to the file
        file.write_all(&buffer[..bytes_read]).expect("Write failed");
        progress.update(bytes_read as u64);
    }

    progress.finish();

    let sha256 = sha256_file(file_path);
    if sha256.is_none() {
        return;
    }
    println!("SHA256 of received file: \n{}", sha256.unwrap());
    
    println!("--------------------------------------------------\n");
    println!("File transfer complete.");
    println!("You can verify the integrity of the transferred file by comparing its checksum with the one generated here.");
}

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