
// use std::collections::HashMap;
// use rand::Rng;
// use random_choice::random_choice;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;

mod city;
use city::City;

use std::fs;
use std::time::{Instant};

mod ant3;
use crate::ant3::{cities_from_coordinates3, ACO3, get_short_path_indicies, get_pheromone_graph, get_distance_graph};




fn main() {
    let now = Instant::now();
    
    let cities:Vec<City> = cities_from_coordinates3("coordinates.txt");
    let shortest_path_idx:Vec<usize> = get_short_path_indicies("shortest_path.txt");
    let pher_graph = get_pheromone_graph(&cities, 0.0005);
    let dist_graph = get_distance_graph(&cities);
    let best_tour:Vec<usize> = {
        let mut aco = ACO3::new(&cities, pher_graph, dist_graph, 100,10, shortest_path_idx);
        aco.optimize()
    };
    println!("\nbest_tour -> {:?}", best_tour);

    println!("\nelapsed time -> {} secs", now.elapsed().as_secs());
    
    
    }

