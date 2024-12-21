use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    // Create a TCP listener bound to localhost on port 7878
    // The '?' operator will return the error if binding fails
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Basic TCP server listening on port 7878...");

    // Loop forever, handling each incoming connection
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                // For each successful connection, print the client's address
                println!("New connection from: {}", stream.peer_addr()?);
                
                // Handle this connection
                if let Err(e) = handle_connection(&mut stream) {
                    eprintln!("Error handling connection: {}", e);
                }
            }
            Err(e) => {
                // If accepting the connection failed, print the error
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }
    Ok(())
}

fn handle_connection(stream: &mut TcpStream) -> std::io::Result<()> {
    // Create a buffer to store incoming data
    let mut buffer = [0; 1024];
    
    // Read data from the stream into our buffer
    let bytes_read = stream.read(&mut buffer)?;
    
    // Convert the received bytes to a string, replacing invalid UTF-8 sequences
    let message = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("Received message: {}", message);

    // Prepare and send a response
    let response = format!("Server received {} bytes\n", bytes_read);
    stream.write_all(response.as_bytes())?;
    
    // Ensure the response is sent immediately
    stream.flush()?;
    Ok(())
}