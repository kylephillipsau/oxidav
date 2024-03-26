// Importing necessary modules from Rust Libraries
use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream){
    // This is a buffer to read data from the client
    let mut buffer = [0; 1024];
    // This line reads data from the stream and stores it in the buffer
    stream.read(&mut buffer).expect("Failed to read from client!");
    // This line converts the data in the buffer into a UTF-8 encoded string
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request: {}", request);

    // Open the HTML file
    let mut file = File::open("index.html").expect("Failed to read from client!");
    let mut contents = String::new();

    // Read the contents of the HTML file
    file.read_to_string(&mut contents).expect("Failed to open HTML file");

    // Construct the HTTP response with a status line, headers, and the HTML content.
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n{}",
        contents
    );

    // Send the response back to the client
    stream.write(response.as_bytes()).expect("Failed to write response!");
}

fn main() {
    // Bind the server to the specific address and port
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");
    println!("Server listening on 127.0.0.1:8080");

    // Listen for incoming connections
    for stream in listener.incoming(){
        match stream{
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
