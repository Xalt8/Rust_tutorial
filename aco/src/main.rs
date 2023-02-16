
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

mod ant;

mod ant_colony;
use ant_colony::AntColony;

mod ant2;

mod aco;
use aco::ACO;

use std::time::{Duration, Instant};



fn main() {
    let now = Instant::now();


    let cities: Vec<City> = city::cities_from_coordinates("coordinates.txt");
    println!("cities -> {:?}\n", cities);

    let short_path: Vec<City> = city::get_shortest_path("shortest_path.txt", &cities);
    let short_path_clone = short_path.clone();
    // let short_path_clone2 = short_path.clone();
    // println!("short_path -> {:?}\n", short_path);
    let short_distance:f32 = ant_colony::get_tour_length(short_path);
    println!("short_distance -> {:?}\n", short_distance);

    let pheromone_graph:Arc<Mutex<Graph>> = graph::create_pheromone_graph(&cities, 0.0005);
    let distance_graph:Graph = graph::create_distance_graph(&cities);
    
    let mut aco = ACO::new(&cities, &pheromone_graph, &distance_graph, 10, 100);
    aco.optimize(short_path_clone);

    println!("\nelapsed time -> {} secs", now.elapsed().as_secs());
    // let mut ant1 = ant::Ant::new(&cities, &pheromone_graph, &distance_graph);
    // let tour = ant1.goes_on_tour();
    // println!("\ntour_len() -> {:?}", tour.len());
    // let tour_dist:f32 = ant_colony::get_tour_length(tour);
    // println!("ant1 tour -> {:?}", tour_dist);

    // let ant2 = ant2::Ant::new(&cities, &pheromone_graph, &distance_graph);
    // let tour2:Vec<i32> = ant2.make_tour();
    // let tour2_clone = tour2.clone();
    // let tour2_clone2 = tour2.clone();
    // println!("\ntour_len -> {:?}", tour2.len());
    // let tour2_dist = ant_colony::get_tour_length(tour2);
    // println!("tour2_dist -> {}", tour2_dist);
    // let tour2_cities:Vec<i32> = tour2_clone.iter().map(|city| city.name).collect();
    // println!("tour2_cities -> {:?}", tour2_cities);
    // println!("tour2_city_tuples -> {:?}", ant_colony::get_tour_city_tuples(&tour2_clone2));

    // let short_path_cities:Vec<i32> = short_path_clone.into_iter().map(|city| city.name).collect();
    // println!("\nshort_path_cities -> {:?}", short_path_cities);
    // let short_path_tuples = ant_colony::get_tour_city_tuples(&short_path_clone2);
    // let short_path_tuples_names:Vec<(i32, i32)> = short_path_tuples.iter().map(|(city1, city2)| (city1.name, city2.name)).collect(); 
    // println!("\nshort_path_tuples_names -> {:?}", short_path_tuples_names);
    // let short_path_distances:Vec<f32> = short_path_tuples.iter().map(|(city1, city2)| graph::calculate_distance(city1, city2)).collect();
    // println!("\nshort_path_distances -> {:?}", short_path_distances);
    
    // let first_2_cities:(&City, &City) = short_path_tuples[0];
    // println!("\nfirst_2_cities -> {:?}", first_2_cities);
    // let first_dist:f32 = graph::calculate_distance(first_2_cities.0, first_2_cities.1);
    // println!("first_dist -> {:?}", first_dist);

    }

