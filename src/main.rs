use rusqlite::{Connection, Result};
use std::path::Path;

fn create_database() -> Result<()> {
    let db_path = "birth.db";

    if Path::new(db_path).exists() {
        println!("Database already exists, skipping creation.");
            return Ok(());
    }

    println!("Creating new database...");
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

    println!("Database created successfully!");
    Ok(())
}

fn main() {
    if let Err(e) = create_database() {
        eprintln!("Error creating database: {}", e);
    }




}
