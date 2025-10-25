pub trait MessageHandler {
    fn handle_message(&self,username:&str,message:&str,current_room:&i64);

    fn send_message(&self,username:&str,message:&str,current_room:&i64);
}