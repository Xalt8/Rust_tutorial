
#![allow(unused)] // Use this to allow imported crates(modules) that are not used 

use std::io;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use rand::Rng;



fn main() {
    println!("What is your name?");
    let mut name = String::new();
    let greeting: &str = "Nice to meet you";
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to readline");
    println!("Hello {}! {}", name.trim_end(), greeting);
    
}
