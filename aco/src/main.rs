
// use std::collections::HashMap;
// use rand::Rng;
// use random_choice::random_choice;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;

mod city;
use city::City;

mod graph;
use graph::Graph;

mod ant2;
mod aco;
use aco::ACO;

use std::time::{Duration, Instant};





fn main() {
    let now = Instant::now();
    
    static CITIES: Lazy<Vec<City>> = Lazy::new(|| city::cities_from_coordinates("coordinates.txt"));

    let short_path: Vec<&City> = city::get_shortest_path("shortest_path.txt", &CITIES);
    
    static pheromone_graph:Lazy<Arc<Mutex<Graph>>> = Lazy::new(|| graph::create_pheromone_graph(&CITIES, 0.0005));
    static distance_graph:Lazy<Graph> = Lazy::new(|| graph::create_distance_graph(&CITIES));

    let mut aco = ACO::new(&CITIES, &pheromone_graph, &distance_graph, 20, 100);
    aco.optimize_concurrent();

    println!("\nelapsed time -> {} secs", now.elapsed().as_secs());
    
    }

// TODO -> Move two_opt to global update instead of ant
