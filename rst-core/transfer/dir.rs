use std::{fs, net::TcpStream, path::Path};
use flate2::{read::{GzDecoder}, write::GzEncoder, Compression};
use tar::{Archive, Builder};

use crate::{error, io::stream::{ProgressReader, ProgressWriter}, print_verbose, transfer::{progress::*, *}};

/// Send a directory over a TCP stream
/// 
/// # Arguments
///
/// * `stream` - The TCP stream to send the directory over
/// * `dir_path` - The path to the directory to send
/// * `options` - The options for the send operation
///
pub fn send_dir(mut stream: TcpStream, dir_path: &str, options: SendOptions) {
    let path = Path::new(dir_path);
    if !path.is_dir() {
        error::Error::DirNotFound(dir_path.to_string()).exit();
    }

    print_verbose!("Sending directory '{}'", dir_path);

    // Prepare the progress bar
    let total_bytes = calc_dir_size(path);
    
    if total_bytes == 0 {
        error::Error::DirEmpty(dir_path.to_string()).exit();
    }

    let progress = SendProgressBar::new(total_bytes, 30);

    let progress_writer = ProgressWriter::new(stream, progress);

    if options.gzip {
        // Wrap socket in gzip encoder
        let encoder = GzEncoder::new(progress_writer, Compression::default());
        let mut tar = Builder::new(encoder);

        if let Err(_) = tar.append_dir_all(".", path) {
            error::Error::ReadFailed.exit();
        }

        // Finish the operation
        let encoder = tar.into_inner().unwrap();
        let finished_writer = encoder.finish().unwrap();
        let _stream = finished_writer.finish();
    } else {
        let mut tar = Builder::new(progress_writer);

        if let Err(_) = tar.append_dir_all(".", path) {
            error::Error::ReadFailed.exit();
        }

        // Finish the operation
        let finished_writer = tar.into_inner().unwrap();
        let _stream = finished_writer.finish();
    }
}

/// Receive a directory over a TCP stream
/// 
/// # Arguments
///
/// * `stream` - The TCP stream to receive the directory from
/// * `dir_path` - The path to the directory to receive
/// * `options` - The options for the receive operation
///
pub fn recv_dir(mut stream: TcpStream, dir_path: &str, options: RecvOptions) {
    let path = Path::new(dir_path);

    if let Err(_) = fs::create_dir_all(path) {
        error::Error::WriteFailed.exit();
    }

    print_verbose!("Receiving directory into: {}", dir_path);

    // Prepare the progress bar
    let counter = RecvByteCounter::new();
    let progress_reader = ProgressReader::new(stream, counter);

    if options.decompress {
        let decoder = GzDecoder::new(progress_reader);
        let mut archive = Archive::new(decoder);

        if let Err(_) = archive.unpack(path) {
            error::Error::WriteFailed.exit();
        }
    } else {
        let mut archive = Archive::new(progress_reader);

        if let Err(_) = archive.unpack(path) {
            error::Error::WriteFailed.exit();
        }
    }
}

fn calc_dir_size(dir: &Path) -> u64 {
    match dir.read_dir() {
        Ok(res) => {            
            res.map(|entry| {
                let entry = entry.unwrap();
                let metadata = entry.metadata().unwrap();
                if metadata.is_dir() {
                    calc_dir_size(entry.path().as_path())
                } else {
                    metadata.len()
                }
            }).sum()
        },
        Err(err) => {
            println!("Error: {}", err.to_string());
            0
        },
    }
}
