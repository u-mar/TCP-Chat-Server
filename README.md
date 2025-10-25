# TCP Chat Server  

A simple **multi-client chat server** written in Rust using `std::net`.  
Each user connects via TCP, chooses a username, and can send messages to all participants or certain users in real time.  

---

## ✨ Features  
- [x] Multiple clients can connect simultaneously  
- [x] Username prompt on connect  
- [x] Broadcast messages to all connected users  
- [x] Store chat history in memory (`Vec<String>`)  
- [x] Direct messaging with `@username` 
- [x] Add message formatting (timestamps, system messages)  
- [x] Save chat history to a database and load
- [x] Add **rooms/channels** so users can join different chats 
- [ ] (Planned) WebSocket support for browser-based UI  

---

## 🚀 Getting Started  

### Prerequisites  
- Rust (latest stable). Install via [rustup](https://www.rust-lang.org/tools/install).  

### Clone the repository  
```bash
git clone https://github.com/u-mar/TCP-Chat-Server.git
cd TCP-Chat-Server
```

## 🛠 Tech Stack  
- **Rust** (standard library networking + threading)  
- **Arc + Mutex** for safe shared state  
- **Telnet** (for simple client testing)  

---

## 🔮 Roadmap  
- [ ] Build a **WebSocket version** for a browser-based UI  
- [ ] Implement **user authentication** (login/register)  
- [ ] Add **rooms/channels** so users can join different chats  
- [ ] Support **file sharing** (images, docs, etc.)  
- [ ] Add **encryption (TLS)** for secure communication  
- [ ] Implement a **moderation system** (kick, ban, mute users)  
- [ ] Deploy server as a **Docker container**  
- [ ] Create a **desktop client** (with Rust + egui/iced)  
- [ ] Add **mobile/web notifications** for new messages  

---
                           ❤️ Made with love by **Cumar**