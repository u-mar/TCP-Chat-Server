use std::net::TcpStream;
use std::io::Write;
use std::sync::{Arc, Mutex};

use crate::message_handler::MessageHandler;


pub struct ChatServer {
    pub messages:Arc<Mutex<Vec<String>>>,
    pub clients:Arc<Mutex<Vec<(TcpStream,String)>>>,
}

impl MessageHandler for ChatServer {
    fn handle_message(&self,username:&str,message:&str){
        let mut messages = self.messages.lock().unwrap();
        messages.push(format!("{} {}",username,message));

        self.send_message(username, message);
        println!("Received message from {}:\n {}",username,message);
    }

    fn send_message(&self,username:&str,message:&str){
        let mut clients = self.clients.lock().unwrap();
        let formated_message = format!("{}: {}",username,message);

        for (client,_) in clients.iter_mut() {
            client.write_all(formated_message.as_bytes()).unwrap();
            client.write_all(b"\n").unwrap();
            client.flush().unwrap();
            
        }
    }
    
}



