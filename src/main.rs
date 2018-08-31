extern crate serde_json;

use std::env;
use std::fs;
use std::io::{self, Read};
use serde_json::Value;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let file_contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let item_price_list: Value = serde_json::from_str(&file_contents)
        .expect("JSON was not well-formatted");

    let mut register = Register{
        state: State::Ready,
        price_list: item_price_list,
        receipts: Vec::new(),
    };

    register.start_new_receipt();

    loop{
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer);
        let buffer = buffer.trim().to_string();

        match register.state {
            State::Ready => {
                if buffer == "" {
                    continue;
                } else if let Ok(num) = buffer.parse::<usize>(){
                    register.print_receipt(num);
                } else {
                    register.add_to_receipt(&buffer);
                }
            }
            State::Ringing => {
                if buffer == "" {
                    let receipt_id = register.receipts.len()-1;
                    register.print_receipt(receipt_id);
                    register.start_new_receipt();
                }
                else {
                    register.add_to_receipt(&buffer);
                }
            }
        }
    }
}

struct Register{
    state: State,
    price_list: Value,
    receipts: Vec<Receipt>,
}

#[derive(Debug)]
struct Receipt{
    total: u64,
    items: Vec<Item>,
}

#[derive(Debug)]
struct Item{
    name: String,
    price: u64,
}

enum State{
    Ready,
    Ringing,
}

impl Register{
    fn start_new_receipt(&mut self){
        let mut new_receipt = Receipt{
            total: 0,
            items: Vec::new(),
        };
        self.receipts.push(new_receipt);
    }

    fn add_to_receipt(&mut self, item: &String){
        self.state = State::Ringing;
        let item_price = &self.price_list[item];
        if let Value::Number(number) = item_price{
            let receipt_id = self.receipts.len()-1;
            let price = number.as_u64().unwrap();
            self.receipts[receipt_id].total += price;
            self.receipts[receipt_id].items.push(Item{
                name: item.clone(),
                price: price,
            });
            println!("Item Price: {}", price);
        } else {
            println!("Item rejected. \n");
        }
    }

    fn print_receipt(&mut self, receipt_id:usize){
        self.state = State::Ready;
        if receipt_id < self.receipts.len(){ 
            println!("Receipt_ID: {},\n{:?}",
                    receipt_id, 
                    self.receipts[receipt_id]
            );
        }
        else {
            println!("Receipt not available.");
        }
    }
}
