pub mod dir;
pub mod file;
pub mod progress;

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