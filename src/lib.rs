use rusqlite::Connection;
use std::env;
use pretty_sqlite::print_table;

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
        let conn = Connection::open("birth.db")?; //claude generated
            let max_id: i64 = conn.query_row(
                "SELECT MAX(id) FROM Birthdays",
                [],
                |row| row.get(0)
            )?;

        for l in 


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

//cargo run -- new "aryan" 01 30 2007
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

pub fn calculations() -> Result<(), Box<dyn std::error::Error>> { //box dyn is dynamic error handle
    let app = App::new()?; 
    app.calculations()?;
    Ok(())
}


