use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::city::City;
pub type Graph = HashMap<i32, HashMap<i32, f32>>;



pub fn get_fully_connected_cities(cities_list:&Vec<City>) -> Vec<(&City, &City)> {
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


pub fn create_pheromone_graph(cities_list:&Vec<City>, initial_pheromone:f32) -> Arc<Mutex<Graph>> {
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


pub fn calculate_distance(city1:&City, city2:&City) -> f32 { 
    // Returns the distance between 2 cities
    (((city1.x - city2.x) as f32).powf(2.0) + 
     ((city1.y - city2.y) as f32).powf(2.0)).sqrt()
}


pub fn create_distance_graph(cities_list:&Vec<City>) -> Graph {
    let mut graph = Graph::new();
    let city_tuples = get_fully_connected_cities(cities_list);
    for (from_city, to_city) in city_tuples {
        // add_nodes_distance(cities.0, cities.1, &mut graph)
        let distance:f32 = calculate_distance(from_city, to_city);
        graph.entry(from_city.name).or_insert(HashMap::new())
        .entry(to_city.name).or_insert(distance);
    }
    graph
}