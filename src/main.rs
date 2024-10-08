use std::collections::HashMap;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Result};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use dotenv::dotenv;

// Function to create and return a HashMap containing content MIME types based on file extensions
fn get_content_type_map() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("html", "text/html");
    map.insert("css", "text/css");
    map.insert("ics", "text/calendar");
    map.insert("js", "application/javascript");
    map.insert("json", "application/json");
    map.insert("png", "image/png");
    map.insert("jpeg", "image/jpeg");
    map.insert("jpg", "image/jpeg");
    map.insert("gif", "image/gif");
    map.insert("svg", "image/svg+xml");
    map.insert("ico", "image/x-icon");
    map.insert("txt", "text/plain");
    map.insert("pdf", "application/pdf");
    map.insert("zip", "application/zip");
    map.insert("xml", "application/xml");
    map.insert("vcard", "text/vcard");
    map.insert("vcf", "text/vcard");
    map.insert("vcard+json", "application/vcard+json");
    map.insert("vcard+xml", "application/vcard+xml");
    map
}

// Function to handle an incoming client connection
fn handle_client(mut stream: TcpStream) -> Result<()> {
    // Buffer to read data from the client
    let mut buffer = [0; 1024];
    // Read data into the buffer
    stream.read(&mut buffer)?;
    // Convert the buffer data into a UTF-8 string
    let request = String::from_utf8_lossy(&buffer[..]);

    // Extract the request line (the first line of the HTTP request)
    let request_line = request.lines().next().unwrap_or("");
    // Split the request line into parts to get the method and path
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let requested_path = parts.next().unwrap_or("/");

    // Extract the authorization header from the request
    let auth_header = request
        .lines()
        .find(|line| line.to_lowercase().starts_with("authorization:"))
        .unwrap_or("");

    // Check if the method requires authentication
    if ["POST", "PUT", "DELETE"].contains(&method) {
        if !is_authorized(auth_header)? {
            let response = "HTTP/1.1 401 Unauthorized\r\n\r\nUnauthorized";
            stream.write(response.as_bytes())?;
            return Ok(());
        }
    }

    // Set the root directory for server files
    let root_dir = "public";
    // Construct the file path based on the requested path
    let path = if requested_path == "/" {
        format!("{}/index.html", root_dir)
    } else {
        format!("{}{}", root_dir, requested_path)
    };

    // Get the content type map
    let content_type_map = get_content_type_map();
    // Get the file extension and look up the content type
    let ext = Path::new(&path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("txt");
    let content_type = content_type_map
        .get(ext)
        .unwrap_or(&"application/octet-stream");

    // Handle the HTTP Methods
    match method {
        "GET" => {
            // Check if the file exists and is a file (not a directory)
            if Path::new(&path).is_file() {
                // Open and read the file
                let mut file = File::open(&path)?;
                let mut contents = Vec::new();
                file.read_to_end(&mut contents)?;

                // Construct the HTTP response with the appropriate content type
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: {}; charset=UTF-8\r\n\r\n",
                    content_type
                );

                // Send the response headers and body to the client
                stream.write(response.as_bytes())?;
                stream.write(&contents)?;
            } else {
                // If the file does not exist, return a 404 Not Found response
                let response = "HTTP/1.1 404 Not Found\r\n\r\n";
                stream.write(response.as_bytes())?;
            }
        }
        "POST" => {
            // Get the content length from the request headers
            let content_length = get_content_length(&request);
            // Read the request body
            let body = read_body(&mut stream, content_length)?;

            // Save the body to a file for demonstration
            let mut file = File::create(&path)?;
            file.write_all(&body)?;

            // Send a response indicating that the data was received and saved
            let response = "HTTP/1.1 201 Created\r\n\r\nPOST request received and data saved";
            stream.write(response.as_bytes())?;
        }
        "PUT" => {
            // Get the content length from the request headers
            let content_length = get_content_length(&request);
            // Read the request body
            let body = read_body(&mut stream, content_length)?;

            // Open the file in append mode and write the data to it
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(&path)?;
            file.write_all(&body)?;

            // Send a response indicating that the data was received and appended
            let response = "HTTP/1.1 200 OK\r\n\r\nPUT request received and data updated";
            stream.write(response.as_bytes())?;
        }
        "DELETE" => {
            // Check if the file exists and is a file
            if Path::new(&path).is_file() {
                // Delete the file
                std::fs::remove_file(&path)?;

                // Send a response indicating that the file was deleted
                let response = "HTTP/1.1 200 OK\r\n\r\nDELETE request received and file deleted";
                stream.write(response.as_bytes())?;
            } else {
                // If the file does not exist, return a 404 Not Found response
                let response = "HTTP/1.1 404 Not Found\r\n\r\n";
                stream.write(response.as_bytes())?;
            }
        }
        _ => {
            let response = "HTTP/1.1 405 Method Not Allowed\r\n\r\n";
            stream.write(response.as_bytes())?;
        }
    }
    Ok(())
}

// Function to check if the request is authorized
fn is_authorized(auth_header: &str) -> Result<bool> {
    // Load environment variables from the .env file
    dotenv().ok();
    // Get the secret token from the environment variables
    let secret_token = env::var("SECRET_TOKEN").map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "SECRET_TOKEN not set"))?;
    // Check if the authorization header matches the secret token
    Ok(auth_header == format!("Authorization: Bearer {}", secret_token))
}

// Function to get the content length from the request headers
fn get_content_length(request: &str) -> usize {
    for line in request.lines() {
        if line.to_lowercase().starts_with("content-length:") {
            if let Some(length) = line.split_whitespace().nth(1) {
                if let Ok(length) = length.parse::<usize>() {
                    return length;
                }
            }
        }
    }
    0
}

// Function to read the request body from the stream
fn read_body(stream: &mut TcpStream, content_length: usize) -> Result<Vec<u8>> {
    let mut body = vec![0; content_length];
    stream.read_exact(&mut body)?;
    Ok(body)
}

fn main() -> Result<()> {
    // Bind the server to the specific address and port
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server listening on http://127.0.0.1:8080");

    // Listen for incoming connections
    for stream in listener.incoming() {
        match stream {
            // If a connection is successfully established, handle the client in a new thread
            Ok(stream) => {
                std::thread::spawn(|| {
                    if let Err(e) = handle_client(stream) {
                        eprintln!("Failed to handle client: {}", e);
                    }
                });
            }
            // If there is an error with the connection, log the error
            Err(e) => {
                eprintln!("Failed to establish connection: {}", e);
            }
        }
    }
    Ok(())
}