
pub enum Error {
    InvalidHost(String),
    UnableToConnect(String, u16),
    UnableToListen(u16),

    FileNotFound(String),
    DirNotFound(String),
    DirEmpty(String),
    NoFileOrDir,

    ReadFailed,
    WriteFailed,
}

impl Error {
    pub fn exit(&self) {
        eprintln!("{}", self);
        std::process::exit(1);
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = "\x1b[1m\x1b[91merror\x1b[0m:";
        match self {
            Error::InvalidHost(host) => write!(f, "{} Invalid host argument '{}'", prefix, host),
            Error::UnableToConnect(host, port) => write!(f, "{} Unable to connect to host '{}' on port '{}'", prefix, host, port),
            Error::UnableToListen(port) => write!(f, "{} Unable to listen on port '{}'", prefix, port),

            Error::FileNotFound(file) => write!(f, "{} File not found: {}", prefix, file),
            Error::DirNotFound(dir) => write!(f, "{} Directory not found: {}", prefix, dir),
            Error::DirEmpty(dir) => write!(f, "{} Directory is empty: {}", prefix, dir),
            Error::NoFileOrDir => write!(f, "{} No file or directory specified", prefix),

            Error::ReadFailed => write!(f, "{} Read failed", prefix),
            Error::WriteFailed => write!(f, "{} Write failed", prefix),
        }
    }
}