// src/server.rs
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::servicehelper::ServiceHelper;

const SHARED_SECRET: &str = "secret_token";

async fn handle_client(mut stream: TcpStream, service_helper: Arc<Mutex<ServiceHelper>>) {
    let mut buf = vec![0; 1024];
    match stream.read(&mut buf).await {
        Ok(size) => {
            let message = String::from_utf8_lossy(&buf[..size]);
            if message.trim() == SHARED_SECRET {
                println!("Client authenticated successfully");
                if let Err(e) = stream.write_all(b"Authenticated\n").await {
                    eprintln!("Failed to send authenticated response: {}", e);
                    return;
                }

                let menu_items = {
                    let service_helper = service_helper.lock().await;
                    service_helper.get_menu_item_data().await
                };

                if let Err(e) = stream.write_all(menu_items.as_bytes()).await {
                    eprintln!("Failed to send menu items: {}", e);
                }

                stream.shutdown().await.expect("Failed to shutdown the connection");
            } else {
                println!("Unauthorized client attempted to connect");
            }
        }
        Err(e) => {
            println!("Error occurred, terminating connection with client: {}", e);
        }
    }
}

pub async fn run_server(service_helper: Arc<Mutex<ServiceHelper>>) {
    let listener = TcpListener::bind("127.0.0.1:8088").await.expect("Failed to bind");
    println!("Server listening on port 8088");

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                println!("New connection: {}", addr);
                let service_helper = service_helper.clone();
                tokio::spawn(handle_client(stream, service_helper));
            }
            Err(e) => {
                println!("Error accepting connection: {}", e);
            }
        }
    }
}
