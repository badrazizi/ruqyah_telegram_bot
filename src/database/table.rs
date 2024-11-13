use rusqlite::{Connection, ToSql};

pub trait Table {
    fn init_table(conn: &Connection) -> Result<(), ()>;
    fn get_params(&self) -> Vec<Box<dyn ToSql>>;
    fn insert(&self, conn: &Connection) -> Result<(), rusqlite::Error>;
    fn update(&self, conn: &Connection) -> Result<(), rusqlite::Error>;
    fn delete(&self, conn: &Connection) -> Result<(), rusqlite::Error>;
}
