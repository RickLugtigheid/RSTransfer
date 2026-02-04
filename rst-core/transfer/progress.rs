use std::io::Write;

/// Trait for progress reporting
pub trait Progress {
    /// Update the progress bar with the number of bytes sent
    fn update(&mut self, bytes: u64);
    /// Called wen the progress of the operation is complete
    fn finish(&mut self);
}

/// Progress bar for sending file data
pub struct SendProgressBar {
    total_bytes: u64,
    sent_bytes: u64,
    width: usize, // e.g., 40 chars
}

impl SendProgressBar {
    pub fn new(total_bytes: u64, width: usize) -> Self {
        Self {
            total_bytes,
            sent_bytes: 0,
            width,
        }
    }

    /// Handles rendering of the progress bar to the console
    fn render(&self) {
        let ratio = self.sent_bytes as f64 / self.total_bytes as f64;
        let filled = (ratio * self.width as f64).round() as usize;
        let progress = format!(
            "\r[{}{}] {:>3}%",
            "#".repeat(filled),
            "-".repeat(self.width - filled),
            (ratio * 100.0).round() as u64
        );
        print!("{}", progress);
        std::io::stdout().flush().unwrap();
    }
}

impl Progress for SendProgressBar {
    fn update(&mut self, bytes: u64) {
        self.sent_bytes += bytes;
        self.render();
    }
    fn finish(&mut self) {
        self.sent_bytes = self.total_bytes;
        self.render();
        println!(); // move to new line
    }
}

pub struct RecvByteCounter {
    received_bytes: u64,
}

impl RecvByteCounter {
    pub fn new() -> Self {
        Self { received_bytes: 0 }
    }

    /// Handles rendering of the progress to the console
    fn render(&self) {
        print!("\rReceived: {} bytes", self.received_bytes);
        std::io::stdout().flush().unwrap();
    }
}
impl Progress for RecvByteCounter {
    fn update(&mut self, bytes: u64) {
        self.received_bytes += bytes;
        self.render();
    }
    fn finish(&mut self) {
        self.render();
        println!();
    }
}