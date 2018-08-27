extern crate serde_json;

use std::env;
use std::fs;
use std::io::{self, Read};
use serde_json::Value;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let json: Value = serde_json::from_str(&contents)
        .expect("JSON was not well-formatted");

    println!("{}", json);

    let mut receipt_id: u8;

    loop{
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer);
        if buffer == "\n" {
            println!("Newline detected");
        }
        else {
            println!("item: {}", buffer);
        }
    }
}
