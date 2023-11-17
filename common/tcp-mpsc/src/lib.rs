/*
Last updated: 11-17-2023

Description: This crate defines the data structures used in the codebase

Author: James Dean
*/
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{self, Read, Write};
use std::str;

pub const LOCALHOST: &str = "127.0.0.1:";

// This enum type is used cor the client to specify the
// thread you would like to interact with.
pub enum TcpService {
    Transaction,
    Block,
}

// This is the implementation of the enum type TcpService.
// The value method is used to specify the port for which thread
// is being used.
impl TcpService {
    fn value(&self) -> &str {
        match self {
            TcpService::Transaction  => "30000",
            TcpService::Block        => "40000",
        }
    }
}

// This method initializes a tcp server for the specified service
// Param: TcpService
pub fn start_tcp_server(service: TcpService) {
    let connection_string = LOCALHOST.to_string() + service.value();
    let listener = TcpListener::bind(connection_string).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
}

// This method allows the client to send a request to a service
// Param: data &str
// Param: TcpService
// Returns: Result<String>
// TODO: Convert interface to Vec<u8>?
pub fn send_request(data: &str, service: TcpService) -> io::Result<String> {
    let connection_string = LOCALHOST.to_string() + service.value();
    let mut stream = TcpStream::connect(connection_string)?;

    // Send data
    stream.write_all(data.as_bytes())?;

    // Read response
    let mut buffer = [0; 1024];
    let len = stream.read(&mut buffer)?;
    let response = str::from_utf8(&buffer[..len]).unwrap_or("");

    Ok(response.to_string())
}

// This method is an example of a handler function for the client.
pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    // Read data from the stream
    match stream.read(&mut buffer) {
        Ok(_) => {
            // Process the data...
            // Send a response...
            let response = "Processed data";
            stream.write_all(response.as_bytes()).unwrap();
        },
        Err(e) => println!("Failed to receive data: {}", e),
    }
}
