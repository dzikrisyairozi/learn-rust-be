use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    // Create a TCP listener for our HTTP server
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("HTTP server listening on http://127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                handle_http_connection(&mut stream)?;
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
    Ok(())
}

fn handle_http_connection(stream: &mut TcpStream) -> std::io::Result<()> {
    // Buffer to store the HTTP request
    let mut buffer = [0; 4096];
    let bytes_read = stream.read(&mut buffer)?;
    
    // Convert the request to a string
    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    
    // Split the request into lines for parsing
    let lines: Vec<&str> = request.lines().collect();
    
    // Parse the request line (e.g., "GET / HTTP/1.1")
    if let Some(request_line) = lines.first() {
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        
        // Handle different routes
        match (parts.get(0), parts.get(1)) {
            (Some(&"GET"), Some(&"/")) => {
                send_response(stream, "200 OK", "Welcome to Rust HTTP Server!")
            }
            (Some(&"GET"), Some(&"/about")) => {
                send_response(stream, "200 OK", "About Page")
            }
            _ => send_response(stream, "404 NOT FOUND", "Page not found")
        }
    } else {
        send_response(stream, "400 BAD REQUEST", "Invalid request")
    }
}

fn send_response(stream: &mut TcpStream, status: &str, content: &str) -> std::io::Result<()> {
    // Construct HTTP response with headers
    let response = format!(
        "HTTP/1.1 {}\r\n\
         Content-Type: text/plain\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\
         \r\n\
         {}",
        status,
        content.len(),
        content
    );
    
    // Send the response
    stream.write_all(response.as_bytes())?;
    stream.flush()
}