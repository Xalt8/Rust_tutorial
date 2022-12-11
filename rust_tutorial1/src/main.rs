
#![allow(unused)] // Use this to allow imported crates(modules) that are not used 

use std::io;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use rand::Rng;
use std::cmp::Ordering;

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
    // "Name shadowing" -> 
    // 2 variables of different types with the same name
    let mut age: u32 = age.trim().parse()
        .expect("age wasn't assigned a number");
    age = age + 1;
    println!("age:{}, mil: {}", age, ONE_MIL);
    
}

fn floats_integers_random_nums(){

    println!("Max u32: {}", u32::MAX); // 4294967295
    println!("Min u32: {}", u32::MIN); // 0
    println!("Min u64: {}", u64::MIN); // 0
    println!("Max u64: {}", u64::MAX); // 18446744073709551615
    println!("Min i32: {}", i32::MIN); // -2147483648
    println!("Max i32: {}", i32::MAX); // 2147483647
    println!("Min i64: {}", i64::MIN); // -9223372036854775808
    println!("Max i64: {}", i64::MAX); // 9223372036854775807
    println!("Min f64: {} Max f64 {}", f64::MIN, f64::MAX);
    
    // start a variable name with underscore for rust compiler 
    // to ignore unused variable name
    let _is_true: bool = true;
    // strings are created using double quotes and characters with single quotes
    let _my_grade: char = 'A';
    // f32 bit will have 6 digits of precision and a f64 will have 14 bits of precision
    
    // Generate random numbers between 1 to 100
    // In range start number included and end number excluded
    let random_num: i32 = rand::thread_rng().gen_range(0..100);
    println!("Random number= {}", random_num);
}


fn conditional_statements(age: i32){
    if (age >= 1) && (age <= 18){
        println!("You are not an adult");
    } else if (age == 21) || (age == 50){
        println!("Special year");
    } else if age >= 65{
        println!("You're too old");
    } else {
        println!("Unknown age type")
    }

    // terinary operators
    let can_vote: bool = if age >= 18 {
        true
    } else {
        false
    };
    println!("Can vote = {}", can_vote);
}

fn match_statements(age:i32){
    match age {
        // Match statements are separated by comma
        // create a range and include the max number use =
        1..=18 => println!("You are not an adult"),
        21 | 50 => println!("Special year"),
        65..=i32::MAX => println!("You're too old"),
        // Underscore will match everything else
        _ => println!("Unknown age type"),
    };

    let voting_age:i32 = 18;
    match age.cmp(&voting_age){
        Ordering::Less => println!("Can't vote"),
        Ordering::Greater => println!("Can vote"),
        Ordering::Equal => println!("You can just about vote"),
    };
}

fn array_loop() {
    /// Content of an array must have the same data type and fixed size
    let arr1: [i32; 10] = [1,2,3,4,5,6,7,8,9,10]; 
    println!("1st element {}", arr1[0]);
    println!("Length of array: {}", arr1.len());
    for num in arr1{
        if num %2 != 0 {
            println!("Odd num {}", num)
        } 
    }
}

fn strings_example(){
    // There are 2 types of strings:
    // String -> vector of bytes that can be changed
    // &str -> points to a string and allows viewing the string
    let mut str1 = String::new();
    // Push a character onto str1
    str1.push('A');
    println!("str1: {}", str1);
    str1.push_str(" BCDEF");
    println!("str1: {}", str1);
    // Split the string on whitespace
    for (i, word) in str1.split_whitespace().enumerate(){
        println!("{} word: {}", i, word);
    }

    let str2: String = str1.replace("A", "Another");
    println!("New string {}", str2);
    
    let str3 = String::from("z y x x w w v u");
    let mut v1: Vec<char> = str3.chars().collect();
    v1.sort();
    v1.dedup(); // remove duplicates
    for ch in v1{
        println!("{}", ch)
    }
    
    // Create a string literal
    let str4: &str = "Random string";
    // Heap allocated string
    let mut str5: String = str4.to_string();
    println!("{}, {}", str5, str4);

    // Convert a string to a byte array
    let byte_array = str5.as_bytes();

    // String slice
    let str6 = &str5[0..=5];
    println!("string slice = {}, string length = {}", str6, str6.len());
    str5.clear();
    println!("deleted str5 = {}", str5);   

    // Combine strings
    let str7 = String::from("Just a");
    let str9 = String::from(" another day");
    let str8 = str7 + &str9;
    println!("Combined string = {}", str8); 

    // Convert string to unicode
    for cha in str8.bytes(){
        println!("Unicode string {}", cha)
    }
}

fn enum_example(){
    enum Day {
        Monday, 
        Tuesday, 
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self{
                Day::Saturday | Day::Sunday => true,
                _ => false,
            }
        }
    }
    let today:Day = Day::Monday;
    println!("Is today a weekend? {}", today.is_weekend());
}


fn main() {
    enum_example()
}
