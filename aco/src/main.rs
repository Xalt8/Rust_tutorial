
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
use graph::calculate_distance;
// type Graph = HashMap<i32, HashMap<i32, f32>>;

mod ant;
use ant::Ant;

mod ant_colony;
use ant_colony::AntColony;



fn main() {
    
    let cities: Vec<City> = city::cities_from_coordinates("coordinates.txt");
    println!("cities -> {:?}\n", cities);

    let short_path: Vec<City> = city::get_shortest_path("shortest_path.txt", &cities);
    println!("short_path -> {:?}\n", short_path);
        
    let pheromone_graph:Arc<Mutex<Graph>> = graph::create_pheromone_graph(&cities, 0.0005);
    let pheromone_graph_clone:Arc<Mutex<Graph>> = pheromone_graph.clone();

    let distance_graph:Graph = graph::create_distance_graph(&cities);
    println!("\ndistance_graph_entry-> {:?}", distance_graph[&1]);
    
    // let mut ant3:Ant = Ant::new(&cities, &pheromone_graph, &distance_graph);
    // // println!("\nant3.current_node -> {:?}, visited_nodes -> {:?}", ant3.current_node, ant3.visited_nodes);

    // let tour:Vec<City> = ant3.goes_on_tour();
    // println!("\ntour -> {:?}", tour); 

    let aco = AntColony::new(&cities, &pheromone_graph_clone, &distance_graph, 10);
    println!("\naco_best_distance -> {:?}", aco.best_path_distance);


    }

