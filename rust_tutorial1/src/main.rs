
#![allow(unused)] // Use this to allow imported crates(modules) that are not used 

use std::io;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use rand::Rng;


fn greeting(){
    /// Ask the user their name from CL
    /// Returns a greeting 
    println!("What is your name?");
    let mut name = String::new();
    let greeting: &str = "Nice to meet you";
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to readline");
    println!("Hello {}! {}", name.trim_end(), greeting);
}

fn constants_name_shadowing(){
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age: &str = "47";
    // "Name shadowing" -> You can have 2 variables of different 
    // types with the same name
    let mut age: u32 = age.trim().parse()
        .expect("age wasn't assigned a number");
    age = age + 1;
    println!("age:{}, mil: {}", age, ONE_MIL);
    
}

fn main() {
    constants_name_shadowing()
}
