use rusqlite::{Connection, Result};

#[derive(Debug)]
struct ToDo{
    id: u64,
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug)]
struct Status {
    is_completed: bool,
}

fn main() {
    println!("Hello, world!");
}
