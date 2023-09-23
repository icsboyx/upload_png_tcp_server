
# Rust TCP Server for uploading PNG Image

This is a Rust project that implements a TCP server that listens for incoming connections and handles them in a specific way. The server is designed to receive image data from clients and save it to disk (PNG format), while also sending a link to the client that can be used to access the saved image.

## Getting Started

To get started with this project, you will need to have Rust and Cargo installed on your system. You can download Rust and Cargo from the official Rust website: https://www.rust-lang.org/tools/install

Once you have Rust and Cargo installed, you can clone this repository to your local machine and run the server using the following command:

git clone https://github.com/<username>/<repository>.git

```
cargo run
```

This will start the server and listen for incoming connections on the IP address and port specified in the configuration file.

## Configuration

The server is configured using a configuration file located at `config.toml`. This file contains the following settings:

- `bind_ip_port`: The IP address and port that the server should bind to.
- `data_dir`: The directory where the received images should be saved.
- `log_level`: The logging level for the server.
- `protocol`: The protocol used by the server (e.g. http or https).
- `server_fqdn`: The fully qualified domain name of the server.
- `url`: The URL path where the received images should be accessible.

You can modify these settings to suit your needs.

## Dependencies

This project uses the following Rust crates:

- `chrono`: A crate for working with dates and times.
- `log`: A crate for logging messages.
- `std::fs`: A crate for working with the file system.
- `std::io`: A crate for working with input and output.
- `std::net`: A crate for working with network connections.
- `std::thread`: A crate for working with threads.
- `toml`: A crate for working with TOML configuration files.

These dependencies are managed using the Cargo package manager.

## Usage

The server listens for incoming TCP connections and handles them in the `handle_client()` function. This function reads data from the stream into a buffer, appends the buffer to a file buffer, and checks for a specific byte pattern at the end of the file buffer. If the byte pattern is found, the entire file buffer is written to a file, a link to the file is sent to the client, and the connection is terminated.

To use the server, you can connect to it using a TCP client and send image data to it. The server will automatically save the image data to disk and send a link to the client that can be used to access the saved image.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.