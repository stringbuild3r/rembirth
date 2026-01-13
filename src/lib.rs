use std::env;


struct Birthday {
    id: i32,
    name: String,
    day: i32,
    month: i32,
    year: i32,
}

pub fn match_functions() {

    let args: Vec<String> = env::args().collect();
    let query  = args[1].as_str();
   
 if args.len() >= 2 {
    match query {
        "new"=> new(),
        "get" => list(),
        _ => println!("Unknown command"),
    }

    }
}
pub fn new() {
    println!()
}


pub fn list() {
    unimplemented!()
}



fn adding_to_db() -> Birthday {
    unimplemented!()
}
