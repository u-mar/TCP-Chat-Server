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

        conn.execute("
        CREATE TABLE IF NOT EXISTS rooms (
            id INTEGER PRIMARY KEY,
            room_name TEXT NOT NULL,
            created_time DATETIME DEFAULT CURRENT_TIMESTAMP
        )", ())?;

        conn.execute("
        CREATE TABLE IF NOT EXISTS room_members (
            id INTEGER PRIMARY KEY,
            user_id INTEGER NOT NULL,
            room_id INTEGER NOT NULL,
            joined_time DATETIME DEFAULT CURRENT_TIMESTAMP,
            active BOOLEAN NOT NULL DEFAULT 1,
            FOREIGN KEY (user_id) REFERENCES users(id),
            FOREIGN KEY (room_id) REFERENCES rooms(id)
        )
        ", ())?;
    
        conn.execute("CREATE TABLE IF NOT EXISTS messages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            room_id INTEGER NOT NULL,
            text TEXT NOT NULL,
            timestamp TEXT NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(id),
            FOREIGN KEY(room_id) REFERENCES rooms(id)
        )",() )?;

        conn.execute(
            "INSERT OR IGNORE INTO rooms (room_name) VALUES ('general')",
            (),
        )?;
    
        
        Ok(Self { conn })

    }

    pub fn add_user(&self, username: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO users (username, online) VALUES (?1, 0)",
            params![username],
        )?;
        Ok(())
    }

    pub fn create_room(&self,room_name:&str) -> Result<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO rooms(room_name) VALUES(?1)"
            , params![room_name],
        )?;
        Ok(())
    }

    pub fn join_room(&self,user_id:&i64,room_id:&i64) -> Result<()> {
        self.conn.execute(
            "INSERT INTO room_members(user_id,room_id) VALUES(?1,?2)
            ", params![user_id,room_id]
        )?;
        Ok(())
    }

    pub fn leave_room(&self,user_id:&i64,room_id:&i64) -> Result<()> {
        self.conn.execute(
            "UPDATE room_members SET active=0 WHERE user_id=?1 AND room_id=?2"
            ,params![user_id,room_id]
        )?;
 
        Ok(())

    }



    pub fn make_online(&self,username:&str,online:bool) -> Result<()> {
        self.conn.execute("UPDATE users SET online=?1 WHERE username=?2",params![online,username],)?;
        Ok(())
    }
    
    pub fn save_message(&self,username: &str,message:&str,timestamp:&str,room_id:&i64) -> Result<()> {
        self.conn.execute("INSERT INTO messages (user_id,text,timestamp,room_id) VALUES(
            (select id from users where username=?1),
            ?2,
            ?3,?4)",params![username,message,timestamp,room_id],)?;

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

    pub fn get_user_id(&self, username: &str) -> Result<i64> {
        self.conn.query_row(
            "SELECT id FROM users WHERE username = ?1",
            params![username],
            |row| row.get(0),
        )
    }
    
    pub fn get_room_id(&self, room_name: &str) -> Result<i64> {
        self.conn.query_row(
            "SELECT id FROM rooms WHERE room_name = ?1",
            params![room_name],
            |row| row.get(0),
        )
    }
    

    pub fn get_users(&self) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT username FROM users")?;
        let rows = stmt.query_map([], |row| {
            let username: String = row.get(0)?;
            Ok(username)
        })?;
    
        let mut users = Vec::new();
        for row in rows {
            users.push(row?);
        }
    
        Ok(users)
    }

    pub fn get_rooms(&self) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT room_name FROM rooms")?;
        let rows = stmt.query_map([], |row| {
            let room_name: String = row.get(0)?;
            Ok(room_name)
        })?;
    
        let mut rooms = Vec::new();
        for row in rows {
            rooms.push(row?);
        }
    
        Ok(rooms)

    }
    
}



