use std::{net::{IpAddr, SocketAddr, TcpListener, TcpStream}, path::Path, str::FromStr};

use clap::{Parser, Subcommand, error::Result};
use rst_core::{error::Error, print_verbose, transfer::{dir::{recv_dir, send_dir}, file::*, *}};

#[derive(Parser)]
#[command(name = "rst", version, author, about = "Raw Socket Transfer Tool")]
#[clap(disable_help_flag = true)]
struct Cli {
    #[arg(short, long, global = true, action = clap::ArgAction::Count)]
    /// Set verbosity level (e.g., -v, -vv, -vvv)
    verbose: u8,

    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands  {
    /// Send a file over raw TCP
    #[command(alias = "s")]
    Send {
        /// File to send
        #[arg(short, long, default_value = "")]
        file: String,

        /// Directory to send
        #[arg(short, long, default_value = "")]
        dir: String,

        /// Destination host
        #[arg(value_name = "HOST")]
        host: Option<String>,
        
        #[arg(short = 'h', long = "host", value_name = "HOST")]
        host_option: Option<String>,

        /// Port to connect to [default: 7777]
        #[arg(short, long, default_value = "7777")]
        port: u16,

        /// Compress file using gzip
        #[arg(long)]
        gzip: bool,
    },
    /// Receive a file over raw TCP
    #[command(alias = "r")]
    Recv {
        /// Destination file
        #[arg(short, long, default_value = "")]
        file: String,

        /// Destination directory
        #[arg(short, long, default_value = "")]
        dir: String,

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
            dir,
            host,
            host_option,
            port,
            gzip,
        } => {
            let host = host.or(host_option).expect("host is required");
            
            // Is file or dir specified?
            //
            if file.is_empty() && dir.is_empty() {
                Error::NoFileOrDir.exit();
            }

            // Handle file
            //
            if !file.is_empty() {

                // Before starting, check if the file exists
                //
                if !Path::new(&file).exists() {
                    Error::FileNotFound(file).exit();
                    return Ok(());
                }
                
                // Send file over raw TCP
                let stream = create_stream(&host, port);
                let options = SendOptions {
                    gzip,
                    verbose: cli.verbose,
                };
                send_file(stream, &file, options);
            } else {
                // Before starting, check if the directory exists
                //
                if !Path::new(&dir).exists() {
                    Error::DirNotFound(dir).exit();
                    return Ok(());
                }

                // Send dir over raw TCP
                let stream = create_stream(&host, port);
                let options = SendOptions {
                    gzip,
                    verbose: cli.verbose,
                };
                send_dir(stream, &dir, options);
            }
        }
        Commands::Recv {
            file,
            dir,
            port,
            decompress,
        } => {

            // Is file or dir specified?
            //
            if file.is_empty() && dir.is_empty() {
                Error::NoFileOrDir.exit();
            }

            if !file.is_empty() {
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
            if cli.verbose >= 1 {
                print_verbose!("Waiting for incoming connection...");
            }
            let stream = loop {
                match listener.incoming().next() {
                    Some(Ok(s)) => break s,
                    Some(Err(e)) => {
                        println!("Error accepting connection: {}", e);
                    }
                    None => { }
                }
            };

            // Receive file or dir over raw TCP
            let options = RecvOptions {
                decompress,
                verbose: cli.verbose,
            };

            if !file.is_empty() {
                recv_file(stream, &file, options);
            } else {
                recv_dir(stream, &dir, options);
            }
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
