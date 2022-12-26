use std::fs;


fn str_to_int_tuple(str_coords: &str) -> (i32, i32) {
    // Takes a string coordinates like " 4, 50" and 
    // converts it into an int tuple (4,50)
    let vec1: Vec<&str> = str_coords.split(", ").collect();
    let vec2: Vec<i32> = vec1.iter()
                        .map(|x| x.trim()
                        .parse::<i32>()
                        .unwrap()) // Catches errors
                        .collect(); // Takes all the values in an Iterator's stream and sticks them into a Vec 
    let t2: (i32, i32) = (vec2[0], vec2[1]);
    return t2;
}

fn get_coordinates(file_path:&str) -> Vec<(i32, i32)> {
    let coords_str:String = fs::read_to_string(file_path).expect("Cannot read file");
    let lines: Vec<&str> = coords_str.split("\r\n").collect();
    let mut vec1: Vec<(i32,i32)> = Vec::new();
    for line in lines {
        let t2:(i32, i32) = str_to_int_tuple(line);
        vec1.push(t2);
    }
    return vec1;
}

#[derive(Debug)]
struct City {
    name: i32,
    x: i32,
    y: i32,
}


impl City {
    fn print_city(&self) {
        println!("Name:{}, x:{}, y:{}", self.name, self.x, self.y);
    }
}


fn create_city_vec(coords:Vec<(i32, i32)>) -> Vec<City> {
    // Takes a vec of coordinates and returns a vec of Cities
    // City name starts at 1, & x,y coordinates
    let mut city_vec = Vec::with_capacity(coords.len());
    for (i, (c1,c2)) in (1..=coords.len()).zip(coords) {
        city_vec.push(City{name:(i as i32), x:c1, y:c2});
    } 
    return city_vec;
}


// fn get_shortest_path(city_nums:&str) -> Vec<City> {
    
// }


fn main() {
    
    let coord_vec: Vec<(i32, i32)> = get_coordinates("coordinates.txt");
    println!("coord_vec = {:?}\n", coord_vec);
    
    let city_vec:Vec<City> = create_city_vec(coord_vec);
    println!("city_vec={:?}\n", city_vec);

    let short_path_nums :String = fs::read_to_string("shortest_path.txt").expect("Cannot read file");    
    let lines: Vec<&str> = short_path_nums.split(" ").collect();
    let vec2: Vec<i32> = lines.iter()
                        .map(|x| x.trim()
                        .parse::<i32>()
                        .unwrap())
                        .collect();
    let mut short_path: Vec<City> = Vec::with_capacity(vec2.len());
    for city_num in vec2{
        'inner_loop: for city in city_vec.iter().clone(){
            if city_num == city.name {
                short_path.push(City{name:city.name, x:city.x, y:city.y});
                break 'inner_loop
            }
            
        }
    }
    println!("short_path:{:?}", short_path);
}
