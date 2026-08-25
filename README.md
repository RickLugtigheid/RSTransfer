# RSTransfer
RSTransfer (`rst`) is a lightweight command-line tool for **raw TCP socket (file) transfer**, enabling simple and fast sending or receiving of a single file between two machines — without any higher-level protocols.

## Features
- Send or receive a **single file** over raw TCP
- Optional **Gzip compression** to save bandwidth
- Minimal dependencies; no TLS, HTTP, or other protocols
- Designed for portability and raw speed

## Installation

### Option 1: Linux (Debian-based)
You can install the latest `.deb` release directly:
```bash
wget https://github.com/RickLugtigheid/RSTransfer/releases/latest/download/rst_amd64.deb
sudo apt-get install ./rst_amd64.deb
rm ./rst_amd64.deb
```

### Option 2: Manual Installation (Using Cargo)
If you have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed:
```bash
git clone https://github.com/RickLugtigheid/RSTransfer.git
cd RSTransfer/
cargo install --path rst-cli/
```
After installation, the binary will typically be available in:
- Linux/macOS: `~/.cargo/bin/rst`
- Windows: `%USERPROFILE%\.cargo\bin\rst`
> [!TIP]
> If you're on **Windows**, make sure to add the Cargo bin directory to your system `PATH` so you can run `rst` from any terminal.
>
> If you're on **Linux**, make sure to move the generated binary to the `/usr/bin` directory so you can run `rst` from any terminal.

## Usage

### Sending
```bash
rst send --file <path> --host <ip> [--port <port>] [--gzip] [--force-close]
```
- `--file`: File to send ( use `--dir` to send a directory )
- `--dir`: Directory to send ( use `--file` to send a single file )
- `--host`: Destination IP or hostname
- `--port`: Port to connect to (default: 7777)
- `--gzip`: Compress the file using gzip before sending

### Receiving
```bash
rst recv --file <path> [--port <port>] [--decompress] [--force-close]
```
- `--file`: Destination file to save received data
- `--dir`: Destination directory to save received data
- `--port`: Port to listen on (default: `7777`)
- `--decompress`: Decompress using gzip (must match sender's `--gzip`)

## Examples

### Send a file to a remote host
```bash
rst send --file ./video.mp4 --host 192.168.0.42 --gzip
```

### Receive a file and save it to disk
```bash
rst recv --file ./video.mp4 --port 7777 --decompress
```

## Building from source
*Make sure you have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed before running the following command:*
```bash
cargo build --release
```
This will create a `target/release/rst` executable in the project root.

## License
MIT License

## Contributing
Pull requests are welcome. See issues for TODOs like:
- [ ] Multi-file support (via tar streaming)
- [ ] Encryption support (via TLS)
- [ ] Pipe-to-stdout or stdin integration

