use rusqlite::{Connection, ToSql};
use serde::{Deserialize, Serialize};

use super::table::Table;

#[derive(Serialize, Deserialize)]
pub struct Users {
    id: Option<i64>,
    username: String,
    telegram_id: i64,
    date: i64,
}

impl Table for Users {
    fn init_table(conn: &Connection) -> Result<(), ()> {
        match conn.execute("create table if not exists users (id INTEGER PRIMARY KEY, username TEXT, telegram_id INTEGER, date INTEGER)", []) {
            Ok(_) => {
                Ok(())
            },
            Err(_) => {
                println!("failed to create Users table");
                Err(())
            }
        }
    }

    fn get_params(&self) -> Vec<Box<dyn ToSql>> {
        vec![
            Box::new(self.username.clone()),
            Box::new(self.telegram_id),
            Box::new(self.date),
        ]
    }

    fn insert(&self, conn: &Connection) -> Result<(), rusqlite::Error> {
        conn.execute(
            "INSERT INTO users(id, username, telegram_id, date) VALUES (NULL, ?, ?, ?)",
            rusqlite::params_from_iter(self.get_params()),
        )?;
        Ok(())
    }

    fn update(&self, conn: &Connection) -> Result<(), rusqlite::Error> {
        let mut params = self.get_params();
        params.push(Box::new(self.id.unwrap_or(-1)));
        conn.execute(
            "UPDATE users SET username=?, telegram_id=?, date=? WHERE id=?",
            rusqlite::params_from_iter(params),
        )?;
        Ok(())
    }

    fn delete(&self, conn: &Connection) -> Result<(), rusqlite::Error> {
        conn.execute(
            "DELETE FROM users WHERE id=?",
            rusqlite::params![self.id.unwrap_or(-1)],
        )?;
        Ok(())
    }
}

impl Users {
    pub fn is_user_new(username: String, id: i64) -> bool {
        let connection: Connection = Connection::open("./database.db").unwrap();
        let mut stmt = match connection.prepare("SELECT * FROM users WHERE telegram_id = ?") {
            Ok(stmt) => stmt,
            Err(_) => {
                println!("Unable to prepare statement");
                return true;
            }
        };

        let users = match stmt.query_map([id], |row| {
            Ok(Users {
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
                .collect::<Vec<Users>>(),
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
                    let _ = Users::update_user_date(user.telegram_id);
                    return false;
                }
            }
        }

        let _ = Users::create_user(username, id);
        true
    }

    pub fn create_user(username: String, id: i64) -> Result<(), ()> {
        let connection: Connection = match Connection::open("./database.db") {
            Ok(connection) => connection,
            Err(_) => return Err(()),
        };

        let user = Users {
            id: None,
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
        let connection: Connection = match Connection::open("./database.db") {
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
}
