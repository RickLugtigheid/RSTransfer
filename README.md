# RSTransfer
RSTransfer (`rst`) is a lightweight command-line tool for **raw TCP socket (file) transfer**, enabling simple and fast sending or receiving of a single file between two machines — without any higher-level protocols.

## Features
- Send or receive a **single file** over raw TCP
- Optional **Gzip compression** to save bandwidth
- Minimal dependencies; no TLS, HTTP, or other protocols
- Designed for portability and raw speed

## Usage

### Sending
```bash
rst send --file <path> --host <ip> [--port <port>] [--gzip] [--force-close]
```
- `--file`: File to send
- `--host`: Destination IP or hostname
- `--port`: Port to connect to (default: 7777)
- `--gzip`: Compress the file using gzip before sending

### Receiving
```bash
rst recv --file <path> [--port <port>] [--decompress] [--force-close]
```
- `--file`: Destination file to save received data
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
