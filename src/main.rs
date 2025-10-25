use std::fmt::format;
use std::io::{Read,Write};
use std::net::{TcpListener,TcpStream};
use std::sync::{Arc, Mutex};
use std::vec;
use chat_server::ChatServer;
use message_handler::MessageHandler;
use chrono::Local;
use colored::Colorize;

pub mod chat_server;
pub mod db;
pub mod message_handler;

fn client_handler(
    mut stream: TcpStream,
    server: Arc<dyn MessageHandler>,
    clients: Arc<Mutex<Vec<(TcpStream, String)>>>,
) {
    let mut buffer = [0; 1024];
    stream.write_all(b"Welcome! Please enter your username: ").unwrap();
    stream.flush().unwrap();

    let buf_read = stream.read(&mut buffer).unwrap();
    let username = String::from_utf8_lossy(&buffer[..buf_read])
        .trim_end_matches(|c: char| c == '\n' || c == '\r')
        .to_string();

    let db = db::Database::new("test.db").unwrap();
    let mut current_room_id = db.get_room_id("general").unwrap();

    {
        let mut clients_lock = clients.lock().unwrap();

        db.add_user(&username).unwrap();
        db.make_online(&username, true).unwrap();


        clients_lock.retain(|(_, client_name)| client_name != &username);


        for (client, client_name) in clients_lock.iter_mut() {
            if client.peer_addr().unwrap() == stream.peer_addr().unwrap() {
                *client_name = username.clone();
            }
        }


        let now = Local::now().format("%H:%M").to_string();
        let joined = format!("[{}]: *** {} joined the chat ***\n", now.red(), username.green());
        for (client, _) in clients_lock.iter_mut() {
            let _ = client.write_all(joined.as_bytes());
        }

        if let Ok(history) = db.get_last_messages(20) {
            for msg in history {
                let line = format!("[{}] {}: {}\n", msg.timestamp, msg.username, msg.text);
                stream.write_all(line.as_bytes()).unwrap();
            }
        }


    }


    loop {
        match stream.read(&mut buffer) {
            Ok(bytes) => {
                if bytes == 0 {
                    let mut clients_lock2 = clients.lock().unwrap();
                

                    let mut left_user = None;
                    let mut left_msg = String::new();
                
                    for (client, client_name) in clients_lock2.iter() {
                        if client.peer_addr().unwrap() == stream.peer_addr().unwrap() {
                            db.make_online(&client_name, false).unwrap();
                            let now = Local::now().format("%H:%M").to_string();
                            left_msg = format!("[{}]: {} left the chat\n", now.red(), client_name.red());
                            left_user = Some(client_name.clone());
                            break;
                        }
                    }
                

                    clients_lock2.retain(|(client, client_name)| {
                        !(client.peer_addr().unwrap() == stream.peer_addr().unwrap())
                    });
                

                    if !left_msg.is_empty() {
                        for (c, _) in clients_lock2.iter_mut() {
                            let _ = c.write_all(left_msg.as_bytes());
                        }
                    }
                
                    break;
                }
                

                let message = String::from_utf8_lossy(&buffer[..bytes]);
                let parts:Vec<&str> = message.trim().split_whitespace().collect();
                let timestamp = Local::now().format("%H:%M").to_string();
                if message.starts_with("/"){
                    if message.trim() == "/users" {
                        let users = db.get_users().unwrap();
                        for user in users {
                            println!("{:?}",user);
                        }
                    }
                    
                    else if message.trim().starts_with("/rooms") {
                        let rooms = db.get_rooms().unwrap();
                        for room in rooms {
                            println!("{:?}",room);
                        }
                    }
                    // create rooms
                    else if message.trim().starts_with("/create") {
                        if parts.len() < 2 {
                            let _ = stream.write_all(b"Usage: /create <room_name>\n");
                        }
                        let room_name = parts[1];
                        match db.create_room(room_name) {
                            Ok(_) => {
                                let reply = format!("Room {} Created Successfully!\n",room_name);
                                let _ = stream.write_all(reply.as_bytes());
                            } 
                            Err(_) => {
                                let reply = format!("Error Creating {} Room\n",room_name);
                                let _ = stream.write_all(reply.as_bytes());
                            }
                        }
                    }

                    else if message.trim().starts_with("/join") {
                        if parts.len() < 2 {
                            let reply = format!("Usage: /join <room_name>\n");
                            let _ = stream.write_all(reply.as_bytes());
                        }
                        let room_name = parts[1];
                        let room_id = db.get_room_id(&room_name).unwrap();
                        let user_id = db.get_user_id(&username).unwrap();

                        match db.join_room(&user_id, &room_id) {
                            Ok(_) => {
                                let reply = format!("Room {} Joined Successfully!\n",room_name.green());
                                current_room_id = db.get_room_id(room_name).unwrap();
                                let _ = stream.write_all(reply.as_bytes());
                            } 
                            Err(_) => {
                                let reply = format!("Error Joining {} Room\n",room_name);
                                let _ = stream.write_all(reply.as_bytes());
                            }
                        }

                    }

                    // leave room
                    else if message.trim().starts_with("/leave") {
                        if parts.len() < 2 {
                            let reply = format!("Usage: /leave <room_name>\n");
                            let _ = stream.write_all(reply.as_bytes());
                        }
                        let room_name = parts[1];
                        let room_id = db.get_room_id(&room_name).unwrap();
                        let user_id = db.get_user_id(&username).unwrap();

                        match db.leave_room(&user_id, &room_id) {
                            Ok(_) => {
                                let reply = format!("Room {} Left Successfully!\n",room_name.red());
                                let _ = stream.write_all(reply.as_bytes());
                            } 
                            Err(_) => {
                                let reply = format!("Error Leaving {} Room\n",room_name.red());
                                let _ = stream.write_all(reply.as_bytes());
                            }
                        }

                    }

                }
                else{
                    db.save_message(&username, &message, &timestamp,&current_room_id).unwrap();
                    server.handle_message(&username, &message);
                }
            }
            Err(_) => break,
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

// add command / users,rooms,join,create,leave // do this tommorow