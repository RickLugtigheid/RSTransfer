use clap::{error::Result, Parser, Subcommand};
use rst_core::progress::{Progress, RecvByteCounter, SendProgressBar};

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

        /// Force close using -N / -q 0
        #[arg(long)]
        force_close: bool,
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

        /// Force close using -N / -q 0
        #[arg(long)]
        force_close: bool,
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
            force_close,
        } => {
            // TODO: Implement
            let total = 10000;
            let mut pb = SendProgressBar::new(total, 30);

            for _ in 0..100 {
                pb.update(100);
                std::thread::sleep(std::time::Duration::from_millis(20));
            }

            pb.finish();
        }
        Commands::Recv {
            file,
            port,
            decompress,
            force_close,
        } => {
            // TODO: Implement
            let mut pb = RecvByteCounter::new();

            for _ in 0..50 {
                pb.update(200);
                std::thread::sleep(std::time::Duration::from_millis(40));
            }

            pb.finish();
        }
    }

    Ok(())
}