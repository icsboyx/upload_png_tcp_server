
# Rust TCP Server for uploading PNG Image

This is a Rust project that implements a TCP server that listens for incoming connections and handles them in a specific way. The server is designed to receive image data from clients and save it to disk (PNG format), while also sending a link to the client that can be used to access the saved image.



## Configuration

The server is configured using a configuration file located at `config.json`. This file contains the following settings:

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
Using nc;
```bash
cat test.png | nc 127.0.0.1 23456 | xclip -selection clipboard
```
I'm using `xclip` to copy the returned link to the clipboard

### Tags
* v1.0.0: Initial release of the Twitch bot.
* beta: Beta version of the bot with additional features.

### Contribution
Contributions are welcome! If you find a bug or want to add new features, feel free to create a pull request.

## Special Thanks

This project was inspired by the Twitch channel of [Prof. Andrea Pollini](https://www.twitch.tv/profandreapollini) and the supportive Twitch community. Thanks to their encouragement and feedback!



## License

This project is licensed under the MIT License - see the [LICENSE](https://www.mit.edu/~amini/LICENSE.md) for details.