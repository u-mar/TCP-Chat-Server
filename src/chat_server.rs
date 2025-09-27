use std::net::TcpStream;
use std::io::Write;
use std::sync::{Arc, Mutex};
use chrono::Local;

use crate::message_handler::MessageHandler;


pub struct ChatServer {
    pub messages:Arc<Mutex<Vec<String>>>,
    pub clients:Arc<Mutex<Vec<(TcpStream,String)>>>,
}

impl MessageHandler for ChatServer {
    fn handle_message(&self,username:&str,message:&str){
        let user:Vec<String> = message.split_ascii_whitespace().filter(|x| x.contains("@")).map(|x| x.to_string()).collect();
        println!("{:?}",user[0]);
        let mut messages = self.messages.lock().unwrap();

        messages.push(format!("{} {}",username,message));

        self.send_message(username, message);
        println!("Received message from {}:\n {}",username,message);
    }

    fn send_message(&self,username:&str,message:&str){
        let mut clients = self.clients.lock().unwrap();
        let user:Vec<String> = message.split_ascii_whitespace().filter(|x| x.contains("@")).map(|x| x.trim_start_matches('@').to_string()).collect();
        let now = Local::now().format("%H:%M").to_string();
        let formated_message = format!("[{}]{}: {}",now,username,message);

        for (client,client_name) in clients.iter_mut() {
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



