//! TCP Echo Server.

use std::thread;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

use ::EchoServer;


fn handle_client(stream: TcpStream) {
    println!("Client connected");

    // Get a separate handle to be able to read and write at the same time
    let mut writer = match stream.try_clone() {
        Err(e) => {
            println!("Could not clone TcpStream handle: {}", e);
            return;
        },
        Ok(w) => w,
    };

    // Iterate through all incoming bytes
    for byte_result in stream.bytes() {
        match byte_result {
            Ok(byte) => {
                match writer.write(&[byte]) {
                    Err(e) => println!("I/O error while writing byte: {}", e),
                    _ => {},
                }
            },
            Err(e) => println!("I/O error while reading byte: {}", e),
        };
    }

    println!("Client disconnected");
}


pub struct TcpEchoServer;

impl EchoServer for TcpEchoServer {

    fn start(&self, host: &str, port: u16) -> Result<(), String> {

        // Initialize TCP listener
        let listener = match TcpListener::bind((host, port)) {
            Ok(value) => {
                println!("Listening on {}:{}...", host, port);
                value
            },
            Err(e) => return Err(format!("Could not bind to {} on port {}: {}", host, port, e)),
        };

        // Main event loop
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(move|| {
                        // connection succeeded
                        handle_client(stream)
                    });
                },
                Err(e) => return Err(format!("Connection failed: {}", e)),
            }
        }

        Ok(())
    }

}
