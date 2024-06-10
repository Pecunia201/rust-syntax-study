#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    println!("What is your name?");
    let mut name = String::new(); // mut = mutable, immutable by default
    let greeting = "Nice to meet you";
    io::stdin().read_line(&mut name) // error  handling
        .expect("Didn't recieve input");
    
    println!("Hello {}! {}", name.trim(), greeting);
}
