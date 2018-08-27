extern crate serde_json;

use std::env;
use std::fs;
use serde_json::Value;

/*
struct Item{
    name: String,
    price: u16,
}
*/


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let json: Value = serde_json::from_str(&contents)
        .expect("JSON was not well-formatted");

     println!("{}", json);
}

