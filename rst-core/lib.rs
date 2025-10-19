pub mod progress;

use std::{io::{Read, Write}, net::TcpStream};

use crate::progress::{Progress, RecvByteCounter, SendProgressBar};

pub fn send_file(mut stream: TcpStream, file_path: &str) {
    let mut file = std::fs::File::open(file_path).expect("Failed to open file");
    let total_bytes = file.metadata().unwrap().len();

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
}