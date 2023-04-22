use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;
use std::{env, io};

fn main() {
    // send stream from command line arguments
    // make tcp-client ARG=Hello,world!
    let args: Vec<String> = env::args().collect();
    let message = match args.len() {
        1 => "Hello, world!",
        _ => &args[1],
    };

    match TcpStream::connect("localhost:65535") {
        Ok(mut stream) => {
            println!("connect port to 65535");

            let message = message.as_bytes();
            // let message = b"Hello, world!";

            stream.write(message).unwrap();

            let mut data = [0 as u8; 50];
            match stream.read(&mut data) {
                Ok(_) => {
                    // Shave to match if the buffer capacity is less than the message, Not so good...
                    let mut result = Vec::new();
                    for byte in data {
                        if byte != 0 {
                            result.push(byte);
                        }
                    }
                    if &result == message {
                        println!("received: {:?}", result);
                        println!("message: {}", from_utf8(&data).unwrap());
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {
                    println!("Interrupted");
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
