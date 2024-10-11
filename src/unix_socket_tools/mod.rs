use std::fs;
use std::path::Path;
use tokio::io::AsyncReadExt;
use tokio::net::{UnixListener, UnixStream};
use tokio::task;

pub async fn handle_client(mut stream: UnixStream, buffer_sender: async_channel::Sender<String>) {
    // Buffer to store incoming data
    let mut buffer = [0; 1024];

    // Read data from the stream
    match stream.read(&mut buffer).await {
        Ok(size) => {
            let message = String::from_utf8_lossy(&buffer[..size]).to_string();
            // Write to log file

            // Send to async buffer sender
            buffer_sender
                .send_blocking(message)
                .expect("Buffer channel closed")
        }
        Err(e) => {
            // Print error message if reading fails
            eprintln!("Failed to read from stream: {}", e);
        }
    }
}

pub async fn start_socket_server(buffer_sender: async_channel::Sender<String>, socket_path: &str) {
    // Remove the socket file if it already exists
    if Path::new(socket_path).exists() {
        fs::remove_file(socket_path).expect("Could not remove existing socket file");
    }

    // Bind the Unix listener to the socket path
    let listener = UnixListener::bind(socket_path).expect("Could not bind");

    println!("Server listening on {}", socket_path);

    // Loop to accept incoming connections
    loop {
        // Accept an incoming connection
        match listener.accept().await {
            Ok((stream, _)) => {
                // Handle the connection in a separate task
                task::spawn(handle_client(stream, buffer_sender.clone()));
            }
            Err(e) => {
                // Print error message if a connection fails
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}
