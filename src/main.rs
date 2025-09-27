use std::io::{Read,Write};
use std::net::{TcpListener,TcpStream};
use std::sync::{Arc, Mutex};

use chat_server::ChatServer;
use message_handler::MessageHandler;


pub mod chat_server;
pub mod message_handler;
fn client_handler(mut stream:TcpStream,server:Arc<dyn MessageHandler>) {
    let mut buffer = [0;1024];
    stream.write_all(b"Welcome! Please enter your username: ").unwrap();
    stream.flush().unwrap();

    let buf_read = stream.read(&mut buffer).unwrap();
    let username = String::from_utf8_lossy(&buffer[..buf_read]).trim_end_matches(|c: char| c == '\n' || c == '\r').to_string();

    println!("{} joined the chat\n", username);

    loop {
        match stream.read(&mut buffer) {
            Ok(bytes) => {
                if bytes == 0 {
                    break;
                }
                let message = String::from_utf8_lossy(&buffer[..bytes]);
                server.handle_message(&username, &message);
            }
            Err(_) => {
                break;
            }
            
        }
    }
}

fn main() {
    let messages:Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let clients:Arc<Mutex<Vec<(TcpStream,String)>>> = Arc::new(Mutex::new(Vec::new()));
    let server = Arc::new(ChatServer{
        messages:Arc::clone(&messages),
        clients:Arc::clone(&clients),
    });
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");
    println!("Chat server started on 127.0.0.1:8080");
    for stream in listener.incoming(){
        match stream {
            Ok(stream) => {
                let server = Arc::clone(&server);
                let mut clients = clients.lock().unwrap();
                clients.push((stream.try_clone().unwrap(),String::new()));
                std::thread::spawn(move || {client_handler(stream,server)});
            }
            Err(e) => {
                eprintln!("Failed to establish a connection {}",e);
            }
        }
    }
}

