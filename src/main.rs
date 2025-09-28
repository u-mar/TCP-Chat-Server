use std::io::{Read,Write};
use std::net::{TcpListener,TcpStream};
use std::sync::{Arc, Mutex};
use chat_server::ChatServer;
use message_handler::MessageHandler;
use chrono::Local;
use colored::Colorize;

pub mod chat_server;
pub mod message_handler;
fn client_handler(mut stream:TcpStream,server:Arc<dyn MessageHandler>,clients:Arc<Mutex<Vec<(TcpStream,String)>>>) {
    let mut buffer = [0;1024];
    stream.write_all(b"Welcome! Please enter your username: ").unwrap();
    stream.flush().unwrap();

    let buf_read = stream.read(&mut buffer).unwrap();
    let username = String::from_utf8_lossy(&buffer[..buf_read]).trim_end_matches(|c: char| c == '\n' || c == '\r').to_string();

    {
        let mut clients_lock = clients.lock().unwrap();
        for (client, client_name) in clients_lock.iter_mut() {
            let now = Local::now().format("%H:%M").to_string();
            let joined = format!("[{}]: *** {} joined the chat ***\n",now.red(),username.green());
            client.write_all(joined.as_bytes()).unwrap();
            if client.peer_addr().unwrap() == stream.peer_addr().unwrap() {
                *client_name = username.clone();
            }
        }
    }

    loop {
        match stream.read(&mut buffer) {
            Ok(bytes) => {
                if bytes == 0 {
                    let mut clients_lock2 = clients.lock().unwrap();
                    let mut left = "".to_string();
                    for (client, client_name) in clients_lock2.iter_mut() {
                        if client.peer_addr().unwrap() == stream.peer_addr().unwrap() {
                            left = client_name.to_string();
                        }

                    }
                    for (client, _) in clients_lock2.iter_mut() {
                        let now = Local::now().format("%H:%M").to_string();
                        let left_format = format!("[{}]: {} left the chat\n",now.red(),left.red());
                        client.write_all(left_format.as_bytes()).unwrap();

                    }
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
                let clients_for_thread:Arc<Mutex<Vec<(TcpStream,String)>>> = Arc::clone(&clients);
                let mut clients: std::sync::MutexGuard<'_, Vec<(TcpStream, String)>> = clients.lock().unwrap();
                clients.push((stream.try_clone().unwrap(),String::new()));
                std::thread::spawn(move || {client_handler(stream,server,clients_for_thread)});
            }
            Err(e) => {
                eprintln!("Failed to establish a connection {}",e);
            }
        }
    }
}

