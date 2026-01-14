use std::env;
use rusqlite::Connection;
//use crate::calculations::next_birthday;
mod calculations;

#[allow(unused_variables)]
#[allow(dead_code)]
struct Birthday {
    name: String,
    day: i32,
    month: i32,
    year: i32,
}

#[allow(dead_code)]
impl Birthday {
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn day(&self) -> &i32 {
        &self.day
    }
    pub fn month(&self) -> &i32 {
        &self.month
    }
    pub fn year(&self) -> &i32 {
        &self.year
    }

}
pub fn match_functions() {
    let args: Vec<String> = env::args().collect();
    let query  = args[1].as_str();
   
 if args.len() >= 2 {
    match query {
        "new"=> new(&args),
        "get" => list(),
 //      "next" => next_birthday(),
        "--help" => help(),
        _ => eprintln!("Not a function, use the \"--help\" function next time"),
        }
    }
}


pub fn help() {
    println!("
  These are common birthday commands used in various situations:

  new       add a new birthday 
  get       get a list of everything in the birthday database in table fashion 
  next      get the next birthday in terms of days according to the current local date

  ");
}



//cargo run -- new "aryan" 01 30 2007
pub fn new(argum: &[String]) {
    let birthing = Birthday {
        name: argum[2].clone(),
        month: argum[3].parse::<i32>().unwrap(),
        day: argum[4].parse::<i32>().unwrap(),
        year: argum[5].parse::<i32>().unwrap(),
    };

    adding_to_db(&birthing);

}

//TODO: implement this everything is said and done
pub fn list() {


    unimplemented!()
}


fn adding_to_db(f_birth: &Birthday) {
    let conn = Connection::open("birth.db").expect("failed to open database");

    conn.execute(
        "INSERT INTO birthdays (name, month, day, year) VALUES (?1, ?2, ?3, ?4)",
     (f_birth.name(), f_birth.month(), f_birth.day(), f_birth.year()),
    ).expect("Failed to insert birthday");

    println!("Added birthday for {}", f_birth.name());
}
