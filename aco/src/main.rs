
// use std::collections::HashMap;
// use rand::Rng;
// use random_choice::random_choice;
use std::sync::{Arc, Mutex};

// City coordinates & paths 
mod city;
use city::City;

// Graph -> distance & pheromone
mod graph;
use graph::Graph;
// type Graph = HashMap<i32, HashMap<i32, f32>>;

// mod ant;

// mod ant_colony;
// use ant_colony::AntColony;

mod ant2;

mod aco;
use aco::ACO;

use std::time::{Duration, Instant};



fn main() {
    let now = Instant::now();
    
    let cities: Vec<City> = city::cities_from_coordinates("coordinates.txt");
    println!("cities -> {:?}\n", cities);

    let short_path: Vec<&City> = city::get_shortest_path("shortest_path.txt", &cities);
    
    let pheromone_graph:Arc<Mutex<Graph>> = graph::create_pheromone_graph(&cities, 0.0005);
    let distance_graph:Graph = graph::create_distance_graph(&cities);
    
    let mut aco = ACO::new(&cities, &pheromone_graph, &distance_graph, 20, 100);
    
    aco.optimize(short_path);

    println!("\nelapsed time -> {} secs", now.elapsed().as_secs());
    
    }

// TODO -> Move two_opt to global update instead of ant
