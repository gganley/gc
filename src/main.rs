/// Gregory Ganley
extern crate gc;

use nom::types::CompleteStr;
use std::io;
use std::io::prelude::*;

use gc::binopexpr;

fn main() {
    for line in io::stdin().lock().lines() {
        let input = line.unwrap();
        if input == "\n" {
            println!("{:?}", "No user input");
            return;
        }
        let tranformed: &'static str = Box::leak(input.into_boxed_str());

        let result = binopexpr(CompleteStr::from(tranformed));
        match result {
            Ok((_, x)) => println!("> {:?}", x),
            Err(err) => panic!(err),
        }
    }
}
