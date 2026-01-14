use chrono::prelude::*;


fn next_birthday() {
    let now_utc: DateTime<Utc> = Utc::now();
    let now_local: DateTime<Local> = Local::now();

    println!("UTC time: {}", now_utc);
    println!("Local time: {}", now_local);
}
 
