use std::net::TcpStream;
use std::io::{self, Read, Write};
use std::str;
use std::thread;

fn main() -> io::Result<()> {
    // Connect to the server
    let mut stream = TcpStream::connect("127.0.0.1:8002")?;
    println!("Connected to the server!");

    // Clone the stream for the reading thread
    let mut read_stream = stream.try_clone()?;

    // Spawn a thread to continuously read from the server
    let reader_thread = thread::spawn(move || {
        let mut buffer = [0; 1024];
        loop {
            match read_stream.read(&mut buffer) {
                Ok(0) => {
                    println!("Server closed the connection");
                    break;
                }
                Ok(bytes_read) => {
                    let response = str::from_utf8(&buffer[..bytes_read]).unwrap();
                    println!("Received: {}", response);
                }
                Err(e) => {
                    eprintln!("Error reading from server: {}", e);
                    break;
                }
            }
        }
    });

    // Main thread for sending messages
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let message = input.trim();
        if message == "quit" {
            println!("Closing the connection...");
            break;
        }

        stream.write(message.as_bytes())?;
        println!("Sent message: {}", message);
    }

    // Wait for the reader thread to finish
    reader_thread.join().unwrap();

    Ok(())
}