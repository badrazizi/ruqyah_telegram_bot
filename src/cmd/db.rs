use rusqlite::{Connection, ToSql};

pub struct User {
    id: i64,
    username: String,
    telegram_id: i64,
    date: i64,
}

impl User {
    pub fn get_params(&self) -> Vec<Box<dyn ToSql>> {
        vec![
            Box::new(self.username.clone()),
            Box::new(self.telegram_id),
            Box::new(self.date),
        ]
    }

    pub fn insert(&self, conn: &Connection) -> Result<(), rusqlite::Error> {
        conn.execute(
            "INSERT INTO users(id, username, telegram_id, date) VALUES (NULL, ?, ?, ?)",
            rusqlite::params_from_iter(self.get_params()),
        )?;
        Ok(())
    }
}

pub fn is_user_new(username: String, id: i64) -> bool {
    let connection: Connection = Connection::open("./users.db").unwrap();
    let mut stmt = match connection.prepare("SELECT * FROM users WHERE telegram_id = ?") {
        Ok(stmt) => stmt,
        Err(_) => {
            println!("Unable to prepare statement");
            return true;
        }
    };

    let users = match stmt.query_map([id], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            telegram_id: row.get(2)?,
            date: row.get(3)?,
        })
    }) {
        Ok(rows) => rows
            .filter_map(|row| match row {
                Ok(user) => Some(user),
                Err(_) => None,
            })
            .collect::<Vec<User>>(),
        Err(_) => {
            println!("Error while getting users");
            return true;
        }
    };

    if users.len() > 0 {
        if let Some(user) = users.first() {
            let user_date = user.date;
            let week: i64 = 604_800;
            let now = chrono::Local::now().timestamp();
            if now - user_date <= week {
                let _ = update_user_date(user.telegram_id);
                return false;
            }
        }
    }

    let _ = create_user(username, id);
    true
}

pub fn init_tables() -> Result<(), ()> {
    let connection: Connection = match Connection::open("./users.db") {
        Ok(connection) => connection,
        Err(_) => {
            println!("Error while opening database");
            return Err(());
        }
    };
    match connection.execute("create table if not exists users (id INTEGER PRIMARY KEY, username TEXT, telegram_id INTEGER, date INTEGER)", []) {
        Ok(_) => Ok(()),
        Err(_) => {
            println!("failed to create users table");
            return Err(());
        }
    }
}

pub fn create_user(username: String, id: i64) -> Result<(), ()> {
    let connection: Connection = match Connection::open("./users.db") {
        Ok(connection) => connection,
        Err(_) => return Err(()),
    };

    let user = User {
        id: 0,
        username,
        telegram_id: id,
        date: chrono::Local::now().timestamp(),
    };

    match user.insert(&connection) {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}

pub fn update_user_date(id: i64) -> Result<(), ()> {
    let connection: Connection = match Connection::open("./users.db") {
        Ok(connection) => connection,
        Err(_) => return Err(()),
    };

    let mut stmt = match connection.prepare("UPDATE users SET date = ? WHERE telegram_id = ?") {
        Ok(stmt) => stmt,
        Err(_) => return Err(()),
    };

    match stmt.execute([chrono::Local::now().timestamp(), id]) {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}
