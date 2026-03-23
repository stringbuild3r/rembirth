use rembirth::match_functions;
use rusqlite::{Connection, Result};
use std::path::Path;

fn create_database() -> Result<()> {
    let db_path = "birth.db";

    if Path::new(db_path).exists() {
        return Ok(());
    }

    let conn = Connection::open(db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS birthdays (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            month INTEGER NOT NULL,
            day INTEGER NOT NULL,
            year INTEGER NOT NULL
        )",
        (),
    )?;
    Ok(())
}

fn main() {
    if let Err(e) = create_database() {
        eprintln!("Error creating database: {}", e);
    }

    match_functions();
}
