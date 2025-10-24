pub mod progress;
pub mod error;

use flate2::{read::{GzDecoder}, write::GzEncoder, Compression};
use std::{fs, io::{Read, Write}, net::TcpStream};
use sha2::{Sha256, Digest};

use crate::progress::{Progress, RecvByteCounter, SendProgressBar};

pub fn send_file(mut stream: TcpStream, file_path: &str, use_gzip: bool) {
    // Open the file
    let file = std::fs::File::open(file_path);
    if file.is_err() {
        error::Error::FileNotFound(file_path.to_string()).exit();
    }
    let file = file.unwrap();

    // Get the total file size
    let total_bytes = file.metadata().unwrap().len();

    // Initialize the progress bar and buffer
    let progress = SendProgressBar::new(total_bytes, 30);
    
    //
    if use_gzip {
        // Create a GzEncoder with GZIP compression level (default is 6)
        let mut encoder = GzEncoder::new(stream, Compression::default());

        // Write the file to the encoder
        write_file_to_stream(file, &mut encoder, progress);

        // Flush the encoder and close the stream
        encoder.finish().expect("Failed to flush encoder");
    } else {
        // Write the file to the stream
        write_file_to_stream(file, &mut stream, progress);
    }

    let sha256 = sha256_file(file_path);
    if sha256.is_none() {
        return;
    }
    println!("SHA256 of send file: \n{}", sha256.unwrap());
    
    println!("--------------------------------------------------\n");
    println!("The file was successfully received. To verify integrity, compare the checksum with the sender's provided value.");
}

fn write_file_to_stream(mut file: fs::File, writer: &mut dyn Write, mut progress: SendProgressBar) {
    let mut buffer = [0u8; 4096];
    
    loop {
        // Read a chunk of data from the file
        //
        let bytes_read = file.read(&mut buffer);
        if bytes_read.is_err() {
            // TODO: We might want to retry here (for max of x times)
            // to avoid temporary network issues stopping the transfer.
            error::Error::ReadFailed.exit();
        }
        let bytes_read = bytes_read.unwrap();
        if bytes_read == 0 {
            break;
        }

        // Write the chunk to the socket
        //
        if writer.write_all(&buffer[..bytes_read]).is_err() {
            error::Error::WriteFailed.exit();
        }
        progress.update(bytes_read as u64);
    }

    progress.finish();
}

pub fn recv_file(mut stream: TcpStream, file_path: &str, decompress: bool) {
    let file = std::fs::File::create(file_path).expect("Failed to create file");

    let progress = RecvByteCounter::new();
    
    if decompress {
        // Create a GzDecoder with GZIP compression level (default is 6)
        let mut decoder = GzDecoder::new(stream);

        // Write the decoder to the file
        write_stream_to_file(&mut decoder, file, progress);
    } else {
        // Write the stream to the file
        write_stream_to_file(&mut stream, file, progress);
    }

    let sha256 = sha256_file(file_path);
    if sha256.is_none() {
        return;
    }
    println!("SHA256 of received file: \n{}", sha256.unwrap());
    
    println!("--------------------------------------------------\n");
    println!("File transfer complete.");
    println!("You can verify the integrity of the transferred file by comparing its checksum with the one generated here.");
}

fn write_stream_to_file(reader: &mut dyn Read, mut file: fs::File, mut progress: RecvByteCounter) {
    let mut buffer = [0u8; 4096];

    loop {
        // Read a chunk of data from the socket
        //
        let bytes_read = reader.read(&mut buffer);
        if bytes_read.is_err() {
            // TODO: We might want to retry here (for max of x times)
            // to avoid temporary network issues stopping the transfer.
            error::Error::ReadFailed.exit();
        }
        let bytes_read = bytes_read.unwrap();
        if bytes_read == 0 {
            break;
        }

        // Write the chunk to the file
        //
        if file.write_all(&buffer[..bytes_read]).is_err() {
            error::Error::WriteFailed.exit();
        }
        progress.update(bytes_read as u64);
    }

    progress.finish();
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