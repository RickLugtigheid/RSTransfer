use std::{fs, io::{Read, Result, Write}};

use crate::{error, transfer::progress::{Progress, RecvByteCounter, SendProgressBar}};

/// Wrapper for a IO (stream) writer that also tracks progress
pub struct ProgressWriter<W: Write, P: Progress> {
    writer: W,
    progress: P,
}


impl<W: Write, P: Progress> ProgressWriter<W, P> {
    pub fn new(writer: W, progress: P) -> Self {
        Self { writer, progress }
    }

    pub fn finish(mut self) -> W {
        self.progress.finish();
        self.writer
    }
}

impl<W: Write, P: Progress> Write for ProgressWriter<W, P> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let bytes = self.writer.write(buf)?;
        self.progress.update(bytes as u64);
        Ok(bytes)
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}

pub struct ProgressReader<R: Read, P: Progress> {
    reader: R,
    progress: P,
}

impl<R: Read, P: Progress> ProgressReader<R, P> {
    pub fn new(reader: R, progress: P) -> Self {
        Self { reader, progress }
    }
}

impl<R: Read, P: Progress> Read for ProgressReader<R, P> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytes = self.reader.read(buf)?;
        if bytes > 0 {
            self.progress.update(bytes as u64);
        }
        Ok(bytes)
    }
}

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
