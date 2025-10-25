use std::net::TcpStream;
use std::io::Write;
use std::sync::{Arc, Mutex};
use chrono::Local;
use colored::Colorize;
use crate::db::Database;
use crate::message_handler::MessageHandler;


pub struct ChatServer {
    pub messages:Arc<Mutex<Vec<String>>>,
    pub clients:Arc<Mutex<Vec<(TcpStream,String,i64)>>>,
}

impl MessageHandler for ChatServer {
    fn handle_message(&self,username:&str,message:&str,current_room:&i64){
        let mut messages = self.messages.lock().unwrap();

        messages.push(format!("{} {}",username,message));

        self.send_message(username, message,current_room);

        println!("Received message from {}:\n {}",username,message);
    }

    fn send_message(&self,username:&str,message:&str,current_room:&i64){
        let mut clients = self.clients.lock().unwrap();
        let user:Vec<String> = message.split_ascii_whitespace().filter(|x| x.contains("@")).map(|x| x.trim_start_matches('@').to_string()).collect();
        let now = Local::now().format("%H:%M").to_string();
        let formated_message = format!("[{}] {}: {}",now.red(),username.green(),message);

        for (client,client_name,room) in clients.iter_mut() {
            println!("Am {} and in room {} and provided should be {}",client_name,room,current_room);
            if room == current_room {
                if user.contains(client_name) {
                    client.write_all(formated_message.as_bytes()).unwrap();
                    client.write_all(b"\n").unwrap();
                    client.flush().unwrap();
                }
                else if user.contains(&"all".to_string()){
                    client.write_all(formated_message.as_bytes()).unwrap();
                    client.write_all(b"\n").unwrap();
                    client.flush().unwrap();
    
                }

            }
            
        }
    }
    
}



