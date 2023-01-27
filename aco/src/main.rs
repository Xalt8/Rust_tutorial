use std::fs;
use std::collections::HashMap;
use rand::Rng;
use random_choice::random_choice;
use std::sync::{Arc, Mutex};

type Graph = HashMap<i32, HashMap<i32, f32>>;


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

#[derive(Debug)]
struct Ant<'a> {
    cities_list: &'a Vec<City>,
    current_node: &'a City,
    visited_nodes: Vec<&'a City>,
    pheromone_graph: Arc<Mutex<Graph>>,
    // pheromone_graph: &'a HashMap<i32, HashMap<i32, f32>>,
    distance_graph: &'a Graph,
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
        // Scores a node based on the current node and node_name passed
        let pher_graph_clone = Arc::clone(&self.pheromone_graph);
        let pher_graph = pher_graph_clone.lock().unwrap(); 
        let phermone = pher_graph.get(&self.current_node.name).unwrap().get(&node_name).unwrap();
        let distance = self.distance_graph.get(&self.current_node.name).unwrap().get(&node_name).unwrap();
        phermone * f32::powf(1.0/distance, self.beta)
    }


    fn choose_node(&self) -> i32 {
        // q -> random value between 0,1
        let q:f32 = rand::thread_rng().gen();
        let univisted = self.get_unvisited_cities().unwrap();
        let scores:Vec<f32> = univisted.iter().map(|city| self.score_node(city.name)).collect(); 
        let mut max_index:usize = 100;
        let mut max_score:f32 = std::f32::NEG_INFINITY; 
        for (i, score) in scores.iter().enumerate(){
            if score > &max_score {
                max_score = *score;
                max_index = i;
            }
        }
        // Use fold instead of for loop
        // let max3 = scores.iter().enumerate().fold((-10000, 0.0), |max, (ind, &val)| if val > max.1 {(ind, val)} else {max});
        
        if q < self.q0 {
            return univisted[max_index].name;    
        } else {
            let sum_scores:f32 = scores.iter().sum();
            let prob_dist:Vec<f32> = scores.iter().map(|score| score/sum_scores).collect();
            let city_names = &univisted.into_iter().map(|city| city.name).collect::<Vec<_>>();
            let choice:Vec<&_> = random_choice().random_choice_f32(&city_names, &prob_dist, 1);
            return **choice.first().unwrap();
        }
    }

    fn get_city(&self, city_name: i32) -> Option<&'a City> {
        // Takes a city name and returns the city object
        self.cities_list.iter().find(|city| city.name == city_name)
    }


    fn visit_city(&mut self, city_name:i32) {
        // Adds the given city_name as the current
        let city = self.get_city(city_name).unwrap();
        self.current_node = &city;
        self.visited_nodes.push(&city);
    }


    fn get_pheromone_value(&self, city_name:i32) -> f32 {
        let pher_graph = self.pheromone_graph.lock().unwrap();
        let pheromone_value:&f32 = pher_graph.get(&self.current_node.name).unwrap().get(&city_name).unwrap();
        *pheromone_value
    }


    fn local_pheromone_update(&mut self, city_name:i32) {
        let mut pher_graph = self.pheromone_graph.lock().unwrap();
        
        let old_pheromone:f32 = self.get_pheromone_value(city_name);
        let new_pheromone:f32 = (1.0 - self.rho) * old_pheromone + (self.rho * self.tau);
        
        if let Some(to_city) = pher_graph.get_mut(&self.current_node.name) {
            to_city.insert(city_name, new_pheromone);
        }
        drop(pher_graph);
    }
} 




fn choose_random_start_city(cities_list:&Vec<City>) -> &City {
    let rand_index = rand::thread_rng().gen_range(0..=cities_list.len()-1);
    let start_city:&City = &cities_list[rand_index];
    start_city
}

fn spawn_ant<'a>(
    cities_list: &'a Vec<City>,
    pheromone_graph: Arc<Mutex<Graph>>,
    distance_graph:&'a Graph) -> Ant<'a> {
    // Create an ant object
    let start_city: &City = choose_random_start_city(&cities_list);
    let mut new_ant = Ant{cities_list:cities_list, 
        current_node:start_city, 
        visited_nodes:Vec::new(), 
        pheromone_graph:pheromone_graph,
        distance_graph:distance_graph,
        beta:2.0, 
        q0:0.9, 
        rho:0.1, 
        tau:0.005};
    new_ant.visited_nodes.push(start_city);
    new_ant
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


fn create_pheromone_graph(cities_list:&Vec<City>, initial_pheromone:f32) -> Arc<Mutex<Graph>> {
    // Create a Acr Mutex HashMap with a city name and its neighbours with initial phermone amount 
    let mut city_graph = Graph::new();
    let city_tuples = get_fully_connected_cities(cities_list);
    for cities in city_tuples {
        city_graph.entry(cities.0.name).or_insert(HashMap::new())
        .entry(cities.1.name).or_insert(initial_pheromone);
    }
    let graph = Arc::new(Mutex::new(city_graph));
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
    
    let dist = calculate_distance(&cities[0], &cities[1]);
    println!("dist: {}", dist);
    
    let pheromone_graph = create_pheromone_graph(&cities, 0.0005);
    // let pheromone_graph = pheromone_graph_arc.lock().unwrap();
    
    let distance_graph = create_distance_graph(&cities);
    println!("\ndistance_graph_entry-> {:?}", distance_graph[&1]);
    
    let random_start = choose_random_start_city(&cities);
    println!("\nrandom_start -> {:?}", random_start);

    let mut ant2 = spawn_ant(&cities, pheromone_graph, &distance_graph);
    if let Some(unvisit) = ant2.get_unvisited_cities(){
        println!("\nunvisited cities before -> {:?}", unvisit);
    }else {
        println!("\nNo visited cities");
    }

    ant2.visited_nodes.push(&cities[10]);
    ant2.visited_nodes.push(&cities[11]);
    println!("\nant2 visited node = {:?}", ant2.visited_nodes);

    let chosen_node = ant2.choose_node();
    println!("\nchosen_node -> {:?}", chosen_node);
    
}