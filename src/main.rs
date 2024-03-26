// Importing necessary modules from Rust Libraries
use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;

fn handle_client(mut stream: TcpStream) {
    // This is a buffer to read data from the client
    let mut buffer = [0; 1024];
    // This line reads data from the stream and stores it in the buffer
    stream
        .read(&mut buffer)
        .expect("Failed to read from client!");
    // This line converts the data in the buffer into a UTF-8 encoded string
    let request = String::from_utf8_lossy(&buffer[..]);

    // Extract the required path from the HTTP request
    let request_line = request.lines().next().unwrap_or("");
    let requested_path = request_line.split_whitespace().nth(1).unwrap_or("/");

    // Set the root directory for your server files
    let root_dir = "public";

    // Construct the path to the required file
    let path = if requested_path == "/" {
        format!("{}/index.html", root_dir)
    } else {
        format!("{}{}", root_dir, requested_path)
    };

    // Check if the file exists and is not a directory.
    if Path::new(&path).is_file() {
        // Read the file contents
        let mut file = File::open(&path).expect("Failed to open file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)
            .expect("Failed to read file");

        // Determine the content type based on the file extension
        let content_type = match Path::new(&path).extension().and_then(|ext| ext.to_str()) {
            Some("html") => "text/html",
            Some("png") => "image/png",
            Some("jpeg") => "image/jpeg",
            _ => "text/plain",
        };

        // Construct the HTTP response with the appropriate content type.
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: {}; charset=UTF-8\r\n\r\n",
            content_type
        );

        // Send the response back to the client
        stream
            .write(response.as_bytes())
            .expect("Failed to write response header");
        stream
            .write(&contents)
            .expect("Failed to write response body");
    } else {
        // If the file does not exist, return a 404 Not Found response.
        let response = "HTTP/1.1 404 Not Found\r\n\r\n";
        stream.write(response.as_bytes()).expect("Failed to write 404 response");
    }
}

fn main() {
    // Bind the server to the specific address and port
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");
    println!("Server listening on 127.0.0.1:8080");

    // Listen for incoming connections
    for stream in listener.incoming() {
        match stream {
            // If a connection is successfully established, hendle the client in a new thread
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            // If there is an error with the connection, log the error
            Err(e) => {
                eprintln!("Failed to establish connection: {}", e);
                // stderr - standard error stream
            }
        }
    }
}
