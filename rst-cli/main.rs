use std::{net::{IpAddr, SocketAddr, TcpListener, TcpStream}, str::FromStr};

use clap::{error::Result, Parser, Subcommand};
use rst_core::{recv_file, send_file, error::Error};

#[derive(Parser)]
#[command(name = "rst", version, author, about = "Raw Socket Transfer Tool")]
#[clap(disable_help_flag = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands  {
    /// Send a file over raw TCP
    Send {
        /// File to send
        #[arg(short, long)]
        file: String,

        /// Host to send to
        #[arg(short, long)]
        host: String,

        /// Port to connect to [default: 7777]
        #[arg(short, long, default_value = "7777")]
        port: u16,

        /// Compress file using gzip
        #[arg(long)]
        gzip: bool,
    },
    /// Receive a file over raw TCP
    Recv {
        /// Destination file
        #[arg(short, long)]
        file: String,

        /// Port to listen on [default: 7777]
        #[arg(short, long, default_value = "7777")]
        port: u16,

        /// Decompress gzip stream
        #[arg(long)]
        decompress: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Send {
            file,
            host,
            port,
            gzip,
        } => {
            // For now add not implemented warnings
            if gzip {
                eprintln!("Gzip compression is not implemented yet");
            }

            // Send file over raw TCP
            let stream = create_stream(&host, port);
            send_file(stream, &file);
        }
        Commands::Recv {
            file,
            port,
            decompress,
        } => {
            // For now add not implemented warnings
            if decompress {
                eprintln!("Gzip decompression is not implemented yet");
            }

            // Listen for incoming connection
            let listener = TcpListener::bind(format!("0.0.0.0:{}", port));
            if listener.is_err() {
                Error::UnableToListen(port).exit();
            }
            let listener = listener.unwrap();
            
            // Accept connection
            // TODO: Add (better) error handling
            let stream = listener.incoming().next().expect("Failed to accept connection").unwrap();

            // Receive file over raw TCP
            recv_file(stream, &file);
        }
    }

    Ok(())
}

/// Create a TCP stream to the specified host and port
fn create_stream(host: &str, port: u16) -> TcpStream {
    let ip = IpAddr::from_str(host);
    if ip.is_err() {
        Error::InvalidHost(host.to_string()).exit();
    }

    let addr = SocketAddr::new(ip.unwrap(), port);
    let stream = TcpStream::connect(addr);
    if stream.is_err() {
        Error::UnableToConnect(host.to_string(), port).exit();
    }
    stream.unwrap()
}