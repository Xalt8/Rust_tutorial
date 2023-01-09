use std::fs;
use std::collections::HashMap;


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

fn parse_coordinates(v: Vec<&str>) -> Vec<(i32, i32)> {
    // From ChatGPT
    v.into_iter().map(|s| {
        let mut coordinates = s.split(',');
        let x = coordinates.next().unwrap().trim().parse::<i32>().unwrap();
        let y = coordinates.next().unwrap().trim().parse::<i32>().unwrap();
        (x, y)
    }).collect()
}


fn cities_from_coordinates(file_path:&str) -> Vec<City> {
    // Takes a file path to coordinates.txt and returns a vec of Cities
    let coords_str:String = fs::read_to_string(file_path).expect("Cannot read file");
    let lines: Vec<_> = coords_str.split("\r\n").collect();
    let int_coords: Vec<(i32, i32)> = parse_coordinates(lines);
    let mut city_vec:Vec<City> = Vec::with_capacity(int_coords.len());
    for (i, (c1,c2)) in (1..=int_coords.len()).zip(int_coords) {
        city_vec.push(City{name:(i as i32), x:c1, y:c2});
    }
    return city_vec;
}


fn get_shortest_path(file_path:&str, city_vec:&Vec<City>) -> Vec<City> {
    // Takes a file with city numbers and returns a vector of Cities
    let short_path_nums :String = fs::read_to_string(file_path).expect("Cannot read file");    
    let lines: Vec<&str> = short_path_nums.split(" ").collect();
    let vec2: Vec<i32> = lines.into_iter().map(|s| s.trim().parse::<i32>().unwrap()).collect();
    let mut short_path: Vec<City> = Vec::with_capacity(vec2.len());
    for city_num in vec2{
        'inner_loop: for city in city_vec.iter().clone(){
            if city_num == city.name {
                short_path.push(City{name:city.name, x:city.x, y:city.y});
                break 'inner_loop
            }
        }
    }
    return short_path;
}   

fn get_city(city_name: i32, cities: &Vec<City>) -> Option<&City> {
    // Takes a city name and returns the city object
    Some(cities.iter().find(|city| city.name == city_name)?)    
}



#[derive(Debug)]
struct Ant<'a> {
    current_node: &'a City,
    visited_nodes: Vec<City>,
    beta: f32,
    q0: f32,
    rho: f32,
    tau: f32,
}

// fn new_ant<'a>(current_node: &'a City, visited_nodes:Vec<City>, beta:f32, q0:f32,rho:f32, tau:f32) -> Ant {
//     Ant{current_node, visited_nodes, beta, q0,rho, tau}
// }

fn spawn_ant<'a>(cities_vec: &Vec<City>) -> Ant {
    let start_node = get_city(1, cities_vec).expect("Couldn't find city"); // Start from City.name == 1
    Ant{current_node:start_node, visited_nodes:Vec::new(), beta:2.0, q0:0.9, rho:0.1, tau:0.1}
}

fn calculate_distance(city1:&City, city2:&City) -> f32 { 
    // Returns the distance between 2 cities
    (((city1.x - city2.x).abs() as f32) + ((city1.y - city2.y).abs() as f32)).powf(2.0)
}

fn add_nodes_phermone(from_city:&City, to_city:&City, initial_phermone:f32, graph:&mut HashMap<i32, HashMap<i32, f32>>) {
    graph.entry(from_city.name).or_insert(HashMap::new())
        .entry(to_city.name).or_insert(initial_phermone);
}


fn main() {
    
    let cities: Vec<City> = cities_from_coordinates("coordinates.txt");
    println!("cities -> {:?}\n", cities);

    let short_path: Vec<City> = get_shortest_path("shortest_path.txt", &cities);
    println!("short_path -> {:?}\n", short_path);
    
    let found_city = get_city(1, &cities).expect("Couldn't find city");
    println!("found_city = {:?}", found_city);

    let ant3 = spawn_ant(&cities);
    println!("ant3 = {:?}", ant3);

    let dist = calculate_distance(&cities[0], &cities[1]);
    println!("dist: {}", dist);
    
    let mut phermones: HashMap<i32, HashMap<i32, f32>> = HashMap::new();
    add_nodes_phermone(&cities[0], &cities[1], 0.1, &mut phermones); 
    add_nodes_phermone(&cities[0], &cities[2], 0.1, &mut phermones); 
    println!("phermones = {:?}", phermones);
    println!("phermones from city0 to city1 = {:?}", phermones[&cities[0].name][&cities[1].name]);
}
    
    

