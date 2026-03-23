use chrono::prelude::*;
use pretty_sqlite::print_table;
use rusqlite::Connection;
use std::env;

struct Birthday {
    name: String,
    day: i32,
    month: i32,
    year: i32,
}

struct App {
    conn: Connection,
}

impl App {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let conn = Connection::open("birth.db")?;
        Ok(App { conn })
    }

    fn add_birthday(&mut self, birthday: &Birthday) -> Result<(), Box<dyn std::error::Error>> {
        self.conn.execute(
            "INSERT INTO birthdays (name, month, day, year) VALUES (?1, ?2, ?3, ?4)",
            (
                &birthday.name,
                &birthday.month,
                &birthday.day,
                &birthday.year,
            ),
        )?;
        Ok(())
    }

    fn list(&self) -> Result<(), Box<dyn std::error::Error>> {
        print_table(&self.conn, "birthdays")?;
        Ok(())
    }

    fn delete_birthday(&self, id: i32) -> Result<(), Box<dyn std::error::Error>> {
        let rows_affected = self.conn.execute("DELETE FROM birthdays WHERE id = ?1", [id])?;
        if rows_affected == 0 {
            println!("No birthday found with id {}", id);
        } else {
            println!("Deleted birthday with id {}", id);
        }
        Ok(())
    }

    fn next_birthday(&self) -> Result<(), Box<dyn std::error::Error>> {
        let now = Local::now();
        let today = get_doy(now.month(), now.day());

        let mut stmt = self.conn.prepare(
            "SELECT name, month, day, year FROM birthdays",
        )?;

        let mut closest_name: Option<String> = None;
        let mut min_days: u32 = 366;

        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, u32>(1)?,
                row.get::<_, u32>(2)?,
                row.get::<_, i32>(3)?,
            ))
        })?;

        for row in rows {
            let (name, month, day, _year) = row?;
            let bday = get_doy(month, day);
            let days_until = (bday + 365 - today) % 365;

            if days_until < min_days {
                min_days = days_until;
                closest_name = Some(name);
            }
        }

        match closest_name {
            Some(name) if min_days == 0 => println!("{}'s birthday is today!", name),
            Some(name) => println!("{}'s birthday is next, in {} days", name, min_days),
            None => println!("No birthdays stored yet"),
        }

        Ok(())
    }
}

fn get_doy(month: u32, day: u32) -> u32 {
    let cum_days: [u32; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
    cum_days[(month - 1) as usize] + day
}

pub fn match_functions() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();
        return;
    }

    match args[1].as_str() {
        "new" => {
            if args.len() < 6 {
                eprintln!("Usage: rembirth new <name> <month> <day> <year>");
                return;
            }
            if let Err(e) = new(&args) {
                eprintln!("Error: {}", e);
            }
        }
        "get" => {
            if let Err(e) = list() {
                eprintln!("Error: {}", e);
            }
        }
        "next" => {
            if let Err(e) = next_birthday() {
                eprintln!("Error: {}", e);
            }
        }
        "delete" => {
            if args.len() < 3 {
                eprintln!("Usage: rembirth delete <id>");
                return;
            }
            if let Err(e) = delete(&args) {
                eprintln!("Error: {}", e);
            }
        }
        "--help" | "-h" => help(),
        other => eprintln!("Unknown command '{}'. Use --help for usage.", other),
    }
}

pub fn help() {
    println!(
        "Usage: rembirth <command>

Commands:
  new <name> <month> <day> <year>   Add a new birthday
  get                               List all birthdays
  next                              Show the next upcoming birthday
  delete <id>                       Delete a birthday by its id
  --help, -h                        Show this help message"
    );
}

fn new(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let birthday = Birthday {
        name: args[2].clone(),
        month: args[3].parse()?,
        day: args[4].parse()?,
        year: args[5].parse()?,
    };

    let mut app = App::new()?;
    app.add_birthday(&birthday)?;
    println!("Added birthday for {}", birthday.name);
    Ok(())
}

fn list() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new()?;
    app.list()?;
    Ok(())
}

fn delete(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let id: i32 = args[2].parse()?;
    let app = App::new()?;
    app.delete_birthday(id)?;
    Ok(())
}

fn next_birthday() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new()?;
    app.next_birthday()?;
    Ok(())
}
