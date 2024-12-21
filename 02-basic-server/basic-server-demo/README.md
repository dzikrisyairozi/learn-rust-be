# Basic Server Demo

This project demonstrates two types of basic servers implemented in Rust:
1. A simple TCP server
2. A basic HTTP server

## Prerequisites

- Rust and Cargo installed ([Install Rust](https://www.rust-lang.org/tools/install))
- A terminal or command prompt
- Optional: `curl` or a web browser for testing the HTTP server
- Optional: `netcat` for testing the TCP server

## Project Structure

```
basic-server-demo/
├── src/
│   └── bin/
│       ├── tcp_server.rs    # Simple TCP server implementation
│       └── http_server.rs   # Basic HTTP server implementation
└── Cargo.toml
```

## Running the Servers

### TCP Server

1. Start the TCP server:
```bash
cargo run --bin tcp_server
```
The server will start listening on `127.0.0.1:7878`

2. Test the TCP server using netcat:
```bash
# In a new terminal
echo "Hello, Server!" | nc localhost 7878
```
You should receive a response indicating how many bytes were received.

### HTTP Server

1. Start the HTTP server:
```bash
cargo run --bin http_server
```
The server will start listening on `http://127.0.0.1:8080`

2. Test the HTTP server using curl or your web browser:
```bash
# Using curl
curl http://localhost:8080          # Homepage
curl http://localhost:8080/about    # About page
curl http://localhost:8080/unknown  # 404 error
```

Or simply open these URLs in your web browser:
- http://localhost:8080
- http://localhost:8080/about

## Available Routes (HTTP Server)

- `GET /` - Returns welcome message
- `GET /about` - Returns about page
- Any other path returns a 404 error

## Understanding the Code

### TCP Server
- Located in `src/bin/tcp_server.rs`
- Demonstrates basic TCP socket programming
- Handles raw byte streams
- Echoes back the number of bytes received

### HTTP Server
- Located in `src/bin/http_server.rs`
- Implements a simple HTTP protocol
- Handles different routes
- Returns proper HTTP responses with headers

## Error Handling

Both servers implement basic error handling:
- Connection errors are logged to stderr
- Invalid requests return appropriate error messages
- Uses Rust's `Result` type for robust error propagation

## Notes

- These are educational examples and not production-ready servers
- They demonstrate basic networking concepts in Rust
- The HTTP server implements a minimal subset of the HTTP/1.1 protocol
- Both servers handle one connection at a time (no concurrent connections)

## License

This project is licensed under the MIT License - see the LICENSE file for details.