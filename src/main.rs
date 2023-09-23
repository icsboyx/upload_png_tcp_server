use chrono::format::StrftimeItems;
use chrono::prelude::*;
use log::{debug, error, info, trace};
use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

mod startup_config;
use startup_config::*;

/// The main function of the program that loads the configuration, initializes the logger, binds to a socket, and listens for incoming TCP connections.
fn main() {
    // Load the configuration
    let config = load_config();

    // Initialize the logger with the log level from the configuration
    env_logger::Builder::new()
        .parse_filters(&config.log_level)
        .init();

    // Log the successful loading of the configuration and the log level
    info!(
        "Config loaded successfully. Logging level: {}",
        config.log_level
    );

    // Log the loaded configuration in trace level
    trace!("Config file Loaded: {:#?}", &config);

    // Bind to the socket specified in the configuration
    let listener = TcpListener::bind(&config.bind_ip_port).expect("Failed to bind to socket");

    // Log the successful binding to the socket
    info!("Server started on address:port {}", &config.bind_ip_port);

    // Log the local address of the listener in debug level
    debug!(
        "Listening for TCP data on {:?}",
        listener.local_addr().unwrap()
    );

    // Listen for incoming TCP connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Spawn a new thread to handle each connection
                info!("######## Connection Started ########");
                thread::spawn(move || {
                    handle_client(stream, load_config());
                });
            }
            Err(e) => {
                // Log the error if there is any
                error!("Error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream, config: Config) {
    let mut buffer = [0u8; 1024]; // Adjust the buffer size as needed
    let time = Local::now();
    let format_str = StrftimeItems::new("%Y_%m_%d_%H_%M_%S");
    let formatted_time = time.format_with_items(format_str).to_string();
    let filename = formatted_time.to_string() + "_received_image.png";
    let link = format!(
        "{}://{}{}{}",
        config.protocol, config.server_fqdn, config.url, filename
    );
    let mut file =
        File::create(config.data_dir + "/" + &filename.clone()).expect("Failed to create file");
    let mut file_buffer: Vec<u8> = vec![];
    loop {
        // Read data from the stream into a buffer
        let bytes_read = stream.read(&mut buffer).unwrap();

        // Check if any bytes were read from the stream
        if bytes_read > 0 {
            // Create a slice of the buffer that contains only the bytes that were read
            let payload = &buffer[..bytes_read];

            // Extend the file buffer with the payload slice
            file_buffer.extend(payload);

            debug!("Bytes read: {}", &buffer.len());
            debug!("payload is now : {}", &payload.len());
            debug!("file_buffer is now  {}", &file_buffer.len());
            debug!(
                "Control bytes read: {:x?}",
                &file_buffer[file_buffer.len() - 12..]
            );

            // Get the last 8 bytes of the file buffer as a slice
            let control_bytes = &file_buffer[file_buffer.len() - 12..file_buffer.len() - 4];

            // Log the value of the control bytes slice for debugging purposes
            debug!("Test Byres are {:x?}", &control_bytes);

            // Check if the control bytes match a specific byte pattern
            if control_bytes == [0x0, 0x0, 0x0, 0x0, 0x49, 0x45, 0x4e, 0x44] {
                // If the control bytes match, write the entire buffer to a file
                file.write_all(&file_buffer).unwrap();
                debug!("Found control bytes");

                // Send a link to the client using the TcpStream object
                stream.write_all(link.as_bytes()).unwrap();
                info!("Link successfully sent to client.");
                info!("######## Connection Terminated ########");
                break;
            }
            if bytes_read == 0 {
                info!("######## Connection Terminated ########");
                break;
            }
        }
    }
}
