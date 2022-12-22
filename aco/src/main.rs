use std::fs;
// use std::io::Error;

fn get_coordinates(file_path:&str) {
    let coords = fs::read_to_string(file_path).expect("Cannot read file");
    
    println!("{}", coords);
}

fn main() {
    get_coordinates("coordinates.txt");
}
