
#![allow(unused)] // Use this to allow imported crates(modules) that are not used 

use std::io;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use rand::Rng;
use std::cmp::Ordering;
use std::ops::Add; // Traits -> specify the functionality for different data types
use std::collections::HashMap;
use std::thread; 
use std::time::Duration;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};

mod restaurant;
use crate::restaurant::order_food;


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


fn vector_example(){
    // Vectors are like arrays -> they can grow if mutable
    // They can only store values of the same type
    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1,2,3,4];
    println!("vec2 before push = {:?}", vec2);
    vec2.push(5);
    println!("vec2 after push = {:?} and first val = {}", vec2, vec2[0]);
    
    // We need to dereference i by using the " * " before we can use it to multiply
    // We also need to "&mut" before we can change values in vec2
    for i in &mut vec2{
        *i *= 2;
    } 
    println!("vec2 after multiplication = {:?}", vec2);
}

fn sum_list_example(list: &[i32]) -> i32{
    let mut sum: i32 = 0;
    for &val in list.iter(){
        sum += &val
    }
    return sum;
}

fn sum_list_example2(list: &[i32]) -> i32{
    // let mut sum: i32 = 0;
    let sum = list.iter().sum();
    return sum;
}

fn generics_example<T:Add <Output = T >>(x:T, y:T) -> T {
    // Specify the data types to be used at a later time
    // Used when we have functions that have multiple different types of data
    // Defined using <T>
    // Cannot used x + y with generics
    // Used the trait Add imported above -> all types that allow Addition can be used
    return x + y; 
}

fn ownership_example(){
    // Stack: stores values in the Last in First Out (LIFO)
    //      Data stored in stack must have defined size
    // Heap: When putting data on the heap -> you request for 
    // a certain amount of space. The OS finds the space and 
    // returns a pointer to that space.
    // RULES:
    // 1) Each value have a variable that's called it owner
    // 2) There is only one owner at a time
    // 3) When the owner goes out of scope the value dissapears  

    let str1 = String::from("World");
    let str2 = str1; // cannot use str1 -> ownership moved to str2
    
    let str3 = String::from("World");
    let str4 = str3.clone(); // creates a copy can use str3
    println!("Hello {}", str3);

}

fn hash_map_example(){
    let mut heroes = HashMap::new();
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("The Flash", "Barry Allen");

    for (k,v) in heroes.iter(){
        println!("{} = {}", k,v);
    }

    if heroes.contains_key(&"Batman"){
        let the_batman = heroes.get(&"Batman");
        match the_batman {
            Some(x) => println!("Batman is the Dark Knight"),
            None => println!("Batman not found"),
        };
    }
}


fn struct_example(){
    #[derive(Debug)] // Use to print the struct
    struct Customer{
        name: String,
        address: String,
        balance: f64,
    }

    let mut bob = Customer{
        name:String::from("Bob Smith"),
        address:String::from("255 Main St"),
        balance: 234.50,
    };
    println!("{:?}", bob);

    // Change a struct value:
    bob.address = String::from("505 Main St");
    println!("{:?}", bob);
}

fn trait_example() {
    
    const PI:f32 = 3.141;
    
    // Traits are similar to interfaces 
    // have methods that must be implemented
    trait Shape{
        fn new(length: f32, width: f32) -> Self; // constructor 
        fn area(&self) -> f32; // will receive a shape and return f32
    }

    struct Rectangle {length: f32, width: f32};
    struct Circle {length: f32, width: f32};

    // Implement the Shape trait for rectangle:
    impl Shape for Rectangle{
        // constructor
        fn new(length:f32, width:f32) -> Rectangle {
            return Rectangle{length, width};
        }
        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    }

    // Implement the Shape trait for circle:
    impl Shape for Circle{
        // constructor
        fn new(length:f32, width:f32) -> Circle {
            return Circle{length, width};
        }
        fn area(&self) -> f32 {
            return (self.length/2.0).powf(2.0) * PI;
        }
    }

    let rec: Rectangle = Shape::new(10.0, 15.0);
    let circ: Circle = Shape::new(10.0, 15.0);

    println!("Rect area = {}", rec.area());
    println!("Circ area = {}", circ.area());
}

fn io_error_handling() {
    // create a file
    let path: &str = "lines.txt";
    let output = File::create(path);
    // Result has 2 varients Ok or Error -> 
    // Result<T, U> {
    //  Ok(T) -> returns the type of data 
    //  Error(U) -> returns type of error
    // }
    let mut output = match output {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem creating file: {:?}", error);
        }
    };
    // Write to file
    write!(output, "Some \n random words").expect("Failed to write to file");

    // Open a file and unwrap ignores the result and returns the file if no error
    let input = File::open(path).unwrap();

    // Gets it line-by-line
    let buffered = BufReader::new(input);
    for line in buffered.lines(){
        println!("buffered lines = {}", line.unwrap());
    }

    let output2 = File::create("rand.txt");
    let output2 = match output2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("rand.txt"){
                Ok(fc) => fc,
                Err(error) => panic!("cant't create file {:?}", error),
            },
            _other_error => panic!("Problem opening file: {:?}", error),
        },
    };
}

fn closure_example() {
    // Closure -> function without a name 
    // can be stored in a variable
    // Can be passed into another function
    // let var_name = |parameters| -> return type {BODY}

    let can_vote = |age:i32| {
        age >= 18
    };
    println!("Can vote = {}", can_vote(8));

    let mut samp1 = 5;
    let print_var = || println!("samp1 = {}", samp1);
    print_var();

    samp1 = 10;
    let mut change_var = || samp1 += 1;
    change_var();
    println!("sampl1 = {}", samp1);
    samp1 = 10;
    println!("sampl1 = {}", samp1);
}


fn closure_example2() {
    // Pass a closure to another function
    // Use the 'where' key word to specify the closure's types
    fn use_func<T>(a:i32, b:i32, func:T) -> i32
    where T: Fn(i32, i32) -> i32 {
        func(a, b)
    }
    let sum = |a, b| a + b;
    let prod = |a, b| a * b;

    println!("5 + 4 = {}", use_func(5, 4, sum));
    println!("5 * 4 = {}", use_func(5, 4, prod));

}

fn smart_pointer_example() {
    // Pointer is an address to a location in memory
    // use the & keyword to borrow a value rather than taking it having it cleaned out of memory
    // Strings and vectors are also smart pointers 
    // They own the data and also have functions to manipulate the data
    // Smart pointers have functions beyond referencing a specific location in memory

    // BOX stores data on a heap not stack
    let b_int1 = Box::new(10);
    println!("b_int1 = {}", b_int1);

    // Create a binary tree data structure utilizing BOX
    #[derive(Debug)]
    struct TreeNode<T> {
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T, 
    }
    impl<T> TreeNode<T> {
        pub fn new(key: T) -> Self {
            TreeNode{left:None, right:None, key,}
        }
        pub fn left(mut self, node: TreeNode<T>) -> Self {
            self.left = Some(Box::new(node));
            self
        }
        pub fn right(mut self, node: TreeNode<T>) -> Self {
            self.right = Some(Box::new(node));
            self
        }
    }

    let node1 = TreeNode::new(1)
    .left(TreeNode::new(2))
    .right(TreeNode::new(3));

    println!("node1 = {:?}", node1)
}

fn concurrency_example1() {
    // Create a thread
    // No gurantees when threads will execute -> use 'join' to ensure execution
    let thread1 = thread::spawn(||{
        for i in 1..25 {
            println!("Spawn thread {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..20 {
        println!("Main thread {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    thread1.join().unwrap();
}

fn concurrency_example2() {
    // Arc -> allows multiple workers own the receiver
    // Mutex -> ensures only one worker gets the job at a time 
    pub struct Bank {
        balance: f32,
    }
    fn withdraw(the_bank: &Arc<Mutex<Bank>>, amt:f32) {
        let mut bank_ref = the_bank.lock().unwrap();
        if bank_ref.balance < 5.00 {
            println!("Current balance : {}\n Withdraw a smaller amount", bank_ref.balance);
        } else {
            bank_ref.balance -= amt;
            println!("Customer withdrew {} current balance = {}", amt, bank_ref.balance);
        }
    }

    fn customer(the_bank: Arc<Mutex<Bank>>){
        withdraw(&the_bank, 5.00);
    }
    
    // create an instance of the bank with 
    let bank: Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank {balance:20.00}));
    
    // create a bunch of customer threads
    let handles = (0..10).map(|_|{
        let bank_ref = bank.clone();
        thread::spawn(||{
            customer(bank_ref);
        })
    });

    for handle in handles{
        handle.join().unwrap();
    }

    println!("Final balance = {}", bank.lock().unwrap().balance);
}


fn main() {
    concurrency_example2()
}
