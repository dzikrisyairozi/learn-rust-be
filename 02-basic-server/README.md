# 2. Basic Server (No Framework)

## 2.1 Introduction

In this chapter, we’ll build a **basic server** without relying on a framework like Actix Web. This hands-on approach helps you understand how networking works under the hood. We’ll look at `std::net` for a simpler synchronous example, or optionally `tokio::net` if you want to explore asynchronous I/O.

## 2.2 Plain TCP Server (std::net)

### 2.2.1 Listening for Connections
We can use the `TcpListener` from `std::net` to listen on a port and accept incoming connections:

```rust
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Server listening on port 7878...");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New connection: {}", stream.peer_addr()?);
                handle_connection(&mut stream)?;
            },
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            },
        }
    }
    Ok(())
}

fn handle_connection(stream: &mut TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 512];
    stream.read(&mut buffer)?;
    println!("Received: {}", String::from_utf8_lossy(&buffer[..]));

    let response = "Hello from Rust server!";
    stream.write(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}
```

- **TcpListener**: Binds to a socket (`127.0.0.1:7878`).
- **incoming()**: Returns an iterator of `TcpStream`s—each one is a client connection.
- **handle_connection**: Reads data from the client, prints it, and sends back a greeting.

### 2.2.2 How It Works
1. The server listens for incoming TCP connections.
2. Each incoming connection is handled in a loop (in this simple example, sequentially).
3. We read the raw bytes sent by the client, then write a response.

## 2.3 Handling Basic HTTP-Like Requests

To make this a bit more “HTTP-like,” we can parse the incoming data as if it were an HTTP request:

```rust
fn handle_connection(stream: &mut TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("Raw request:\n{}", request);

    // Very naive "parse"
    let lines: Vec<&str> = request.lines().collect();
    if !lines.is_empty() {
        let first_line = lines[0];
        if first_line.contains("GET / ") {
            let response = "HTTP/1.1 200 OK\r\n\r\nHello World!";
            stream.write_all(response.as_bytes())?;
        } else {
            let response = "HTTP/1.1 404 NOT FOUND\r\n\r\nNot Found!";
            stream.write_all(response.as_bytes())?;
        }
    }

    stream.flush()?;
    Ok(())
}
```

- We read data into a buffer, convert it to a string, and split it by lines.
- A **very naive** check: if the request starts with `GET / `, we respond with a “Hello World!” page. Otherwise, we send a 404.
- Real HTTP parsing requires handling headers, methods, etc., but this example shows the basics.

## 2.4 Optional: Async with tokio::net

If you’d like to see non-blocking I/O, you can use **Tokio**. Install it in `Cargo.toml`:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

Then write an async main function:

```rust
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878").await?;
    println!("Async server listening on port 7878```");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("New connection: {}", addr);

        tokio::spawn(async move {
            let mut buffer = [0; 512];
            if let Ok(bytes_read) = socket.read(&mut buffer).await {
                println!("Received: {}", String::from_utf8_lossy(&buffer[..bytes_read]));
                let response = b"Hello async world!";
                if let Err(e) = socket.write_all(response).await {
                    eprintln!("Failed to write to socket: {}", e);
                }
            }
        });
    }
}
```

- **tokio::main**: Sets up an async runtime.
- **listener.accept()**: Waits for new TCP connections without blocking other tasks.
- **tokio::spawn**: Runs the connection handler in a separate asynchronous task.

## 2.5 Understanding the Underlying Mechanics

By manually handling connections:
- You see how **TCP** underpins HTTP.
- You learn to read raw data and parse it yourself.
- You appreciate why higher-level frameworks are so helpful, as they handle HTTP parsing, routing, and more.

---

- This approach is **educational**, showing you the low-level details.
- For real-world apps, you’d probably move to a framework for faster development and maintenance.
- Understanding these fundamentals helps you debug network issues and write efficient servers.

## 2.6 Conclusion

You’ve just written a bare-bones server in Rust, which gives you a clearer view of how data flows in and out over TCP. Next, we’ll see how a *backend framework* like Actix Web handles many of these details for you, making it faster to build robust APIs.

<p align="center">
  <sub>
    © 2025 Learn Rust Backend by <a href="https://github.com/dzikrisyairozi">Dzikri Syairozi</a>. 
    Licensed under the <a href="../LICENSE">MIT License</a>.
  </sub>
</p>
