use chrono::prelude::*;
use pretty_sqlite::print_table;
use rusqlite::Connection;
use std::{env, usize};

struct Birthday {
    name: String,
    day: i32,
    month: i32,
    year: i32,
}

impl Birthday {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn day(&self) -> i32 {
        self.day
    }
    pub fn month(&self) -> i32 {
        self.month
    }
    pub fn year(&self) -> i32 {
        self.year
    }
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
                &birthday.name(),
                &birthday.month(),
                &birthday.day(),
                &birthday.year(),
            ),
        )?;
        Ok(())
    }

    fn list(&self) -> Result<(), Box<dyn std::error::Error>> {
        print_table(&self.conn, "Birthdays")?;
        Ok(())
    }

    fn calculations(&self) -> Result<(), Box<dyn std::error::Error>> {
        let local_datetime: DateTime<Local> = Local::now();
        let curr_month: u32 = local_datetime.month();
        let curr_date: u32 = local_datetime.day();

        let conn = Connection::open("birth.db")?;
        let max_id: i64 = conn.query_row("SELECT MAX(id) FROM Birthdays", [], |row| row.get(0))?;

        let mut min_days_until: u32 = 366;
        let mut closest_id: Option<i64> = None;

        let day_rn: u32 = get_doy(curr_month, curr_date);

        for i in 1..=max_id {
            let month: u32 =
                match conn.query_row("select month from birthdays where id = ?1", [i], |row| {
                    row.get(0)
                }) {
                    Ok(mon) => mon,
                    Err(_) => continue,
                };

            let day: u32 =
                match conn.query_row("select day from birthdays where id = ?1", [i], |row| {
                    row.get(0)
                }) {
                    Ok(da) => da,
                    Err(_) => continue,
                };

            let _year: u32 =
                match conn.query_row("select year from birthdays where id = ?1", [i], |row| {
                    row.get(0)
                }) {
                    Ok(ye) => ye,
                    Err(_) => continue,
                };

            let db_day: u32 = get_doy(month, day);

            let days_until = (db_day + 365 - day_rn) % 365;

            if days_until < min_days_until {
                min_days_until = days_until;
                closest_id = Some(i);
            }

            println!(
                "Closest birthday ID: {:?}, in {} days",
                closest_id, min_days_until
            );
        }

        fn get_doy(mon: u32, day: u32) -> u32 {
            let cum_days: Vec<u32> = vec![0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
            let month_offset = cum_days.get((mon - 1) as usize).unwrap(); 
            month_offset + day
        }
        Ok(())
    }
}

pub fn match_functions() {
    let args: Vec<String> = env::args().collect();
    let query = args[1].as_str();

    if args.len() >= 2 {
        match query {
            "new" => {
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
                if let Err(e) = calculations() {
                    eprintln!("Error: {}", e);
                }
            }
            "--help" => help(),
            _ => eprintln!("Not a function, use the \"--help\" function next time"),
        }
    }
}

pub fn help() {
    println!(
        "
  These are common birthday commands used in various situations:

  new       add a new birthday 
  get       get a list of everything in the birthday database in table fashion 
  next      get the next birthday in terms of days according to the current local date

  "
    );
}

pub fn new(argum: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let birthing = Birthday {
        name: argum[2].clone(),
        month: argum[3].parse::<i32>()?,
        day: argum[4].parse::<i32>()?,
        year: argum[5].parse::<i32>()?,
    };

    let mut app = App::new()?;
    app.add_birthday(&birthing)?;
    println!("Added birthday for {}", birthing.name());
    Ok(())
}

pub fn list() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new()?;
    app.list()?;
    Ok(())
}

pub fn calculations() -> Result<(), Box<dyn std::error::Error>> {
    //box dyn is dynamic error handle
    let app = App::new()?;
    app.calculations()?;
    Ok(())
}
