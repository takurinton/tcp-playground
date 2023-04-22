use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener};
use std::thread;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:65535").unwrap();
    println!("Server listening on port 65535");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("new connection addr: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    let mut data = [0 as u8; 50];
                    // Should I use a reference here?
                    let mut stream = stream;
                    while match stream.read(&mut data) {
                        Ok(size) => {
                            stream.write(&data[0..size]).unwrap();
                            true
                        }
                        Err(_) => {
                            stream.shutdown(Shutdown::Both).unwrap();
                            return;
                        }
                    } {}
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    drop(listener);
}
