mod todolist;

use std::env;
use todolist::{TodoItem, TodoList};

fn main() {
    let args = env::args().collect::<Vec<String>>();

    println!("{:?}", args);
    //println!("Hello, world!");
}
