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

    let mut register = Register{receipt_id:0};

    loop{
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer);
        if buffer == "\n" {
            register.print_receipt();
            register.start_new_receipt();
        }
        else {
            println!("item: {}", buffer);
        }
    }
}

struct Register{
    receipt_id: u8,
}

impl Register{
    fn start_new_receipt(& mut self){
        self.receipt_id += 1;
    }

    fn print_receipt(&self){
        println!("Receipt ID: {}", self.receipt_id);
    }
}
