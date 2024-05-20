# Rust HTTP Server

This project implements a basic HTTP server in Rust. It can serve static files (e.g., HTML, PNG, JPEG) from a specified directory and handles basic HTTP requests. The server also supports basic authentication for certain HTTP methods. The server is designed to listen on `127.0.0.1:8080`.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Setup and Installation](#setup-and-installation)
- [Usage](#usage)
- [Project Structure](#project-structure)
- [Code Explanation](#code-explanation)
- [License](#license)

## Prerequisites

To run this project, you need to have Rust installed on your machine. If you don't have Rust installed, you can install it from [rust-lang.org](https://www.rust-lang.org/).

## Setup and Installation

1. **Clone the repository:**
   ```sh
   git clone https://github.com/kylephillips/rust-http-server.git
   cd rust-http-server
   ```

2. **Create a `.env` file:**
   Create a `.env` file in the root directory and add your secret token:
   ```sh
   SECRET_TOKEN=your_secret_token
   ```

3. **Build the project:**
   ```sh
   cargo build
   ```

## Usage

1. **Run the server:**
   ```sh
   cargo run
   ```

2. The server will start listening on `127.0.0.1:8080`. Open your web browser and navigate to `http://127.0.0.1:8080` to see the server in action.

## Project Structure

- **`src/main.rs`**: The main entry point of the application, containing the server logic.
- **`public/`**: The directory where your static files (HTML, images, etc.) are stored. You can create this directory and add an `index.html` file for testing.
- **`.env`**: A file to store environment variables, including the secret token used for authentication.

## Code Explanation

### `handle_client` function

This function handles the client's request and sends the appropriate response. It reads the request, determines the requested file, and serves the file if it exists. If the file does not exist, it sends a `404 Not Found` response.

The function supports the following HTTP methods:
- **GET**: Serves static files from the `public/` directory.
- **POST**: Saves the request body to a file.
- **PUT**: Appends the request body to a file.
- **DELETE**: Deletes the specified file.

### Authentication

The server uses a simple token-based authentication mechanism for `POST`, `PUT`, and `DELETE` methods. The token is read from the `.env` file. Requests to these methods must include an `Authorization` header with the token in the following format:
```
Authorization: Bearer <your_secret_token>
```

### `is_authorized` function

This function checks if the request is authorized by comparing the `Authorization` header with the secret token stored in the `.env` file.

### `main` function

The main function sets up the TCP listener on `127.0.0.1:8080` and handles incoming connections. Each connection is handled in a new thread to allow multiple clients to connect simultaneously.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.