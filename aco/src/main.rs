use std::fs;
use std::collections::HashMap;
use rand::Rng;
use std::rc::Rc;


#[derive(Debug, Copy, Clone)]
struct City {
    name: i32,
    x: i32,
    y: i32,
}

impl PartialEq for City {
    // Used to compare 2 cities
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
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

// =======================================================================================================
// Ant 
// =======================================================================================================

#[derive(Debug, Clone)]
struct Ant<'a> {
    cities_list: &'a Vec<City>,
    current_node: &'a City,
    visited_nodes: Vec<&'a City>,
    phermone_graph: &'a HashMap<i32, HashMap<i32, f32>>,
    distance_graph: &'a HashMap<i32, HashMap<i32, f32>>,
    beta: f32,
    q0: f32,
    rho: f32,
    tau: f32,
}


impl<'a> Ant<'a>{
    fn get_unvisited_cities(&self) -> Option<Vec<City>> {
        if self.visited_nodes.len() > 0 {
            let mut unvisisted_nodes:Vec<City> = Vec::new();
            for city in self.cities_list{
                if !self.visited_nodes.contains(&city){
                    unvisisted_nodes.push(*city);
                }
            } 
            Some(unvisisted_nodes)
        } else {
            None
        }
    }


    fn score_node(&self, node_name:i32) -> f32 {
        // Scores a node based on the current node and node passed 
        // (self.phermone_graph[&self.current_node.name][&node_name]) * 
        // f32::powf(1.0/self.distance_graph[&self.current_node.name][&node_name], self.beta as f32)
        let phermone = self.phermone_graph.get(&self.current_node.name).unwrap().get(&node_name).unwrap();
        let distance = self.distance_graph.get(&self.current_node.name).unwrap().get(&node_name).unwrap();
        phermone * f32::powf(1.0/distance, self.beta)
    }


    fn choose_node(&self) -> Vec<City> {
        // q -> random value between 0,1
        let q:f32 = rand::thread_rng().gen();
        let univisted = self.get_unvisited_cities().unwrap();
        univisted
    }
}


fn get_city(city_name: i32, cities: &Vec<City>) -> Option<&City> {
    // Takes a city name and returns the city object
    Some(cities.iter().find(|city| city.name == city_name)?)    
}

fn choose_random_start_city(cities_list:&Vec<City>) -> &City {
    let rand_index = rand::thread_rng().gen_range(0..=cities_list.len()-1);
    &cities_list[rand_index]
}

fn spawn_ant<'a>(
    cities_list: &'a Vec<City>,
    phermone_graph:&'a HashMap<i32, HashMap<i32, f32>>,
    distance_graph:&'a HashMap<i32, HashMap<i32, f32>>) -> Ant<'a> {
    // Create an ant object
    let start_node = choose_random_start_city(&cities_list);
    Ant{cities_list:cities_list, 
        current_node:start_node, 
        visited_nodes:Vec::new(), 
        phermone_graph:phermone_graph,
        distance_graph:distance_graph,
        beta:2.0, 
        q0:0.9, 
        rho:0.1, 
        tau:0.005}
}

// =======================================================================================================
// Graph 
// =======================================================================================================

fn get_fully_connected_cities(cities_list:&Vec<City>) -> Vec<(&City, &City)> {
    // Takes a vec of cities and returns a vec of tuples with every combination
    // of city as tuples 
    let mut city_combinations:Vec<(&City, &City)> = Vec::new(); 
    for city1 in cities_list {
        for city2 in cities_list {
            if city1.name != city2.name {
                city_combinations.push((&city1, &city2))
            }
        }
    }
    city_combinations
}

fn add_nodes_phermone(from_city:&City, to_city:&City, initial_phermone:f32, graph:&mut HashMap<i32, HashMap<i32, f32>>) {
    // Creates 2 nodes in a graph and adds a phermone edge between them 
    graph.entry(from_city.name).or_insert(HashMap::new())
        .entry(to_city.name).or_insert(initial_phermone);
}

fn create_phermone_graph(cities_list:&Vec<City>, initial_phermone:f32) -> HashMap<i32, HashMap<i32, f32>> {
    // Create a HashMap with a city name and its neighbours with initial phermone amount 
    let mut graph: HashMap<i32, HashMap<i32, f32>> = HashMap::new();
    let city_tuples = get_fully_connected_cities(cities_list);
    for cities in city_tuples {
        add_nodes_phermone(cities.0, cities.1, initial_phermone, &mut graph)
    }
    graph
}

fn calculate_distance(city1:&City, city2:&City) -> f32 { 
    // Returns the distance between 2 cities
    (((city1.x - city2.x).abs() as f32) + ((city1.y - city2.y).abs() as f32)).powf(2.0)
}

fn add_nodes_distance(from_city:&City, to_city:&City, graph:&mut HashMap<i32, HashMap<i32, f32>>) {
    let distance:f32 = calculate_distance(from_city, to_city);
    graph.entry(from_city.name).or_insert(HashMap::new())
        .entry(to_city.name).or_insert(distance);
}

fn create_distance_graph(cities_list:&Vec<City>) -> HashMap<i32, HashMap<i32, f32>> {
    let mut graph: HashMap<i32, HashMap<i32, f32>> = HashMap::new();
    let city_tuples = get_fully_connected_cities(cities_list);
    for cities in city_tuples {
        add_nodes_distance(cities.0, cities.1, &mut graph)
    }
    graph
}

fn main() {
    
    let cities: Vec<City> = cities_from_coordinates("coordinates.txt");
    println!("cities -> {:?}\n", cities);

    let short_path: Vec<City> = get_shortest_path("shortest_path.txt", &cities);
    println!("short_path -> {:?}\n", short_path);
    
    let found_city = get_city(1, &cities).expect("Couldn't find city");
    println!("found_city = {:?}", found_city);

    let dist = calculate_distance(&cities[0], &cities[1]);
    println!("dist: {}", dist);
    
    let mut phermones: HashMap<i32, HashMap<i32, f32>> = HashMap::new();
    add_nodes_phermone(&cities[0], &cities[1], 0.1, &mut phermones); 
    add_nodes_phermone(&cities[0], &cities[2], 0.1, &mut phermones); 
    println!("phermones = {:?}", phermones);
    println!("phermones from city0 to city1 = {:?}", phermones[&cities[0].name][&cities[1].name]);

    let phermone_graph = create_phermone_graph(&cities, 0.0005);
    println!("\nphermone_graph entry-> {:?}", phermone_graph[&1]);

    let distance_graph = create_distance_graph(&cities);
    println!("\ndistance_graph_entry-> {:?}", distance_graph[&1]);
    
    let random_start = choose_random_start_city(&cities);
    println!("\nrandom_start -> {:?}", random_start);

    // // Spawn ant test
    // let ant3 = spawn_ant(&cities, &phermone_graph, &distance_graph);
    
    // // Unvisited test
    // if let Some(unvisit) = ant3.get_unvisited_cities(){
    //     println!("\n unvisited cities before -> {:?}", unvisit);
    // }else {
    //     println!("\nNo visited cities");
    // }

    // Add some visited cities
    let mut ant2 = spawn_ant(&cities, &phermone_graph, &distance_graph);
    if let Some(unvisit) = ant2.get_unvisited_cities(){
        println!("\nunvisited cities before -> {:?}", unvisit);
    }else {
        println!("\nNo visited cities");
    }

    ant2.visited_nodes.push(&cities[10]);
    ant2.visited_nodes.push(&cities[11]);
    println!("\nant2 visited node = {:?}", ant2.visited_nodes);

    let unvisit = ant2.get_unvisited_cities().unwrap(); 
    println!("\nunvisit -> {:?}", unvisit);
    
    // let pher2:Vec<&f32> = unvisit.iter().map(|x| ant2.distance_graph.get(&ant2.current_node.name).unwrap().get(&x.name).unwrap()).collect();
    // println!("\npher2 -> {:?}", pher2);

    let test:Vec<i32> = unvisit.iter().map(|x| x.name).collect();
    // let test_dist = ant2.distance_graph.get(&ant2.current_node.name).unwrap().get(&30).unwrap();
    // let test_dist:Vec<_> = ant2.distance_graph.get(&ant2.current_node.name).unwrap().values().clone().collect();
    let test_dist = ant2.score_node(30);
    println!("\ntest ->{:?}, current_node -> {:?}, test_dist -> {:?}", test, ant2.current_node, test_dist);

}
