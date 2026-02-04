use std::{fs, io::{Read, Write}};

use crate::{error, transfer::progress::{Progress, RecvByteCounter, SendProgressBar}};

/// Write a stream to a file
/// 
/// # Arguments
///
/// * `reader` - The stream to read from
/// * `file` - The file to write to
/// * `progress` - The progress bar to update
///
pub fn write_stream_to_file(reader: &mut dyn Read, mut file: fs::File, mut progress: RecvByteCounter) {
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

/// Write a file to a stream
/// 
/// # Arguments
///
/// * `file` - The file to write to the stream
/// * `writer` - The stream to write the file to
/// * `progress` - The progress bar to update
/// 
pub fn write_file_to_stream(mut file: fs::File, writer: &mut dyn Write, mut progress: SendProgressBar) {
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
