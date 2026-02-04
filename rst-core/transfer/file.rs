use std::net::TcpStream;
use flate2::{read::{GzDecoder}, write::GzEncoder, Compression};

use crate::{error, io::stream::{write_file_to_stream, write_stream_to_file}, print_verbose, sha256_file, transfer::progress::{RecvByteCounter, SendProgressBar}};


/// Options for sending a file over a TCP stream
pub struct SendOptions {
    pub gzip: bool,
    pub verbose: u8,
}
/// Options for receiving a file over a TCP stream
pub struct RecvOptions {
    pub decompress: bool,
    pub verbose: u8,
}

/// Send a file over a TCP stream
/// 
/// # Arguments
///
/// * `stream` - The TCP stream to send the file over
/// * `file_path` - The path to the file to send
/// * `options` - The options for the send operation
/// 
pub fn send_file(mut stream: TcpStream, file_path: &str, options: SendOptions) {
    // Open the file
    let file = std::fs::File::open(file_path);
    if file.is_err() {
        error::Error::FileNotFound(file_path.to_string()).exit();
    }
    let file = file.unwrap();

    // Get the total file size
    let total_bytes = file.metadata().unwrap().len();
    
    // Verbosity level 1 - Print basic info
    if options.verbose >= 1 {
        print_verbose!("Starting transfer of file '{}'", file_path);
        print_verbose!("File size: {} bytes", total_bytes);
    }

    // Initialize the progress bar and buffer
    let progress = SendProgressBar::new(total_bytes, 30);

    // Verbosity level 2 - Show compression setting
    if options.verbose >= 2 {
        if options.gzip {
            print_verbose!("Compression enabled (gzip) - sending compressed data.");
        } else {
            print_verbose!("Compression disabled - sending raw data.");
        }
    }

    //
    if options.gzip {
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

/// Receive a file over a TCP stream
/// 
/// # Arguments
///
/// * `stream` - The TCP stream to receive the file from
/// * `file_path` - The path to the file to receive
/// * `options` - The options for the receive operation
/// 
pub fn recv_file(mut stream: TcpStream, file_path: &str, options: RecvOptions) {
    let file = std::fs::File::create(file_path).expect("Failed to create file");

    // Verbosity level 1 - Print basic info
    if options.verbose >= 1 {
        print_verbose!("Starting writing received data to file '{}'", file_path);
    }

    let progress = RecvByteCounter::new();
    
    // Verbosity level 2 - Show decompression setting
    if options.verbose >= 2 {
        if options.decompress {
            print_verbose!("Decompression enabled - receiving decompressed data.");
        } else {
            print_verbose!("No decompression - receiving raw data.");
        }
    }

    if options.decompress {
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
