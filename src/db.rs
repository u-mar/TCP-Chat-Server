use rusqlite::{params,Connection, Result};

#[derive(Debug, Clone)]
pub struct Message {
    pub username: String,
    pub text: String,
    pub timestamp: String,
}

pub struct Database {
    conn: Connection
}

impl Database {
    pub fn new(path:&str) -> Result<Self>{
        let conn = Connection::open(path)?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY,
                username TEXT UNIQUE NOT NULL,
                online BOOLEAN NOT NULL DEFAULT 0
            )",
            (),
        )?;
    
        conn.execute("CREATE TABLE IF NOT EXISTS messages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            text TEXT NOT NULL,
            timestamp TEXT NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(id)
        )",() )?;
    
        
        Ok(Self { conn })

    }

    pub fn add_user(&self, username: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO users (username, online) VALUES (?1, 0)",
            params![username],
        )?;
        Ok(())
    }

    pub fn make_online(&self,username:&str,online:bool) -> Result<()> {
        self.conn.execute("UPDATE users SET online=?1 WHERE username=?2",params![online,username],)?;
        Ok(())
    }
    
    pub fn save_message(&self,username: &str,message:&str,timestamp:&str) -> Result<()> {
        self.conn.execute("INSERT INTO messages (user_id,text,timestamp) VALUES(
            (select id from users where username=?1),
            ?2,
            ?3)",params![username,message,timestamp],)?;

        Ok(())
    

    }

    pub fn get_last_messages(&self, n: usize) -> Result<Vec<Message>> {
        let mut stmt = self.conn.prepare(
            "SELECT u.username, m.text, m.timestamp
             FROM messages m
             JOIN users u ON m.user_id = u.id
             ORDER BY m.id DESC
             LIMIT ?1",
        )?;

        let rows = stmt.query_map(params![n], |row| {
            Ok(Message {
                username: row.get(0)?,
                text: row.get(1)?,
                timestamp: row.get(2)?,
            })
        })?;

        let mut messages = Vec::new();
        for row in rows {
            messages.push(row?);
        }

        Ok(messages)
    }
}



