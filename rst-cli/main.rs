use std::{net::{IpAddr, SocketAddr, TcpListener, TcpStream}, path::Path, str::FromStr};

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
            // Before starting, check if the file exists
            //
            if !Path::new(&file).exists() {
                Error::FileNotFound(file).exit();
                return Ok(());
            }
            
            // Send file over raw TCP
            let stream = create_stream(&host, port);
            send_file(stream, &file, gzip);
        }
        Commands::Recv {
            file,
            port,
            decompress,
        } => {

            // Before starting, check if the file already exists
            // if so ask the user if they want to overwrite it.
            //
            if Path::new(&file).exists() {
                println!("File '{}' already exists. Overwrite? (y/n)", file);
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                if input.trim() != "y" {
                    println!("Aborting transfer.");
                    return Ok(());
                }
            }

            // Listen for incoming connection
            let listener = TcpListener::bind(format!("0.0.0.0:{}", port));
            if listener.is_err() {
                Error::UnableToListen(port).exit();
            }
            let listener = listener.unwrap();
            
            // Accept (one) connection.
            // Loop unil we get a successful connection (or Ctrl-C)
            // 
            let stream = loop {
                match listener.incoming().next() {
                    Some(Ok(s)) => break s,
                    Some(Err(e)) => {
                        println!("Error accepting connection: {}", e);
                    }
                    None => { }
                }
            };

            // Receive file over raw TCP
            recv_file(stream, &file, decompress);
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