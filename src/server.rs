use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, BufWriter};

const SHARED_SECRET: &str = "secret_token";

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    match stream.read(&mut buf) {
        Ok(size) => {
            let message = String::from_utf8_lossy(&buf[..size]);
            if message.trim() == SHARED_SECRET {
                println!("Client authenticated successfully");
                stream.write_all(b"Authenticated\n").unwrap();
                let mut writer = BufWriter::new(stream.try_clone().expect("Failed to clone stream"));
                loop {
                    writer.write_all(b"hello Anubhav\n").unwrap();
                    writer.flush().unwrap();
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
            } else {
                println!("Unauthorized client attempted to connect");
            }
        },
        Err(_) => {
            println!("Error occurred, terminating connection with client");
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8088").expect("Failed to bind");
    println!("Server listening on port 8088");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                let cloned_stream = stream.try_clone().expect("Failed to clone stream");
                std::thread::spawn(move || {
                    handle_client(cloned_stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
