use std::fs;
use rand::Rng;
use random_choice::random_choice;
use std::borrow::Borrow;

use crate::city::City;

pub type GraphMatrix = Vec<Vec<f32>>;


pub fn calculate_distance_generic<T>(city1:T, city2:T) -> f32 
where 
    T: Borrow<City>,
{ 
    let city1:&City = city1.borrow();
    let city2:&City = city2.borrow();
    (((city1.x - city2.x) as f32).powf(2.0) + ((city1.y - city2.y) as f32).powf(2.0)).sqrt()
}

pub fn cities_from_coordinates3(file_path:&str) -> Vec<City> {
    /* Takes a file name and returns a vec of City objects */ 
    let mut cities:Vec<City> = Vec::new();
    
    let file = fs::read_to_string(file_path).expect("Failed to read file");
    for (i, line) in file.lines().enumerate(){
        let coords:Vec<&str> = line.split(",").map(|s| s.trim()).collect();
        if let (Ok(x), Ok(y)) = (coords[0].parse::<i32>(), coords[1].parse::<i32>()) {
            let city = City{name:(i+1) as i32, x:x, y:y};
            cities.push(city);
        };
    }
    cities
}


pub fn get_short_path_indicies(file_path:&str) -> Vec<usize> {
    /*  Takes a file path a returns the index values of the shortest path
        The file contains the name of cities starting from 1 
        The indicies start from zero 
    */
    let file = fs::read_to_string(file_path).expect("Failed to read file");
    let shortest_path_idx :Vec<usize> = file.split_whitespace()
                                    .map(|s| 
                                    s.trim()
                                    .parse::<usize>().unwrap() -1)
                                    .collect();
    shortest_path_idx
}


fn get_connected_cities_indicies(city_index:usize, cities_list:&Vec<City>) -> Vec<usize> {
    // Given a city_index returns a vec of all connected cities
    assert!(city_index < cities_list.len(), "city_index provided is out of bounds");
    let cities_idx:Vec<usize> = (0..cities_list.len()).collect();
    let connected_idx:Vec<usize> = cities_idx.iter().filter(|c| *c!= &city_index).cloned().collect::<Vec<usize>>();
    connected_idx 
}


pub fn get_pheromone_graph(cities_list:&Vec<City>, initial_pheromone_value:f32) -> GraphMatrix {
    // Takes a vec of City and initial pheromone value and creates a pheromone matrix
    let mut pher_graph:Vec<Vec<f32>> = vec![vec![0.0; cities_list.len()]; cities_list.len()];
    for from_city_idx in 0..cities_list.len(){
        for to_city_idx in get_connected_cities_indicies(from_city_idx, cities_list){
            pher_graph[from_city_idx][to_city_idx] = initial_pheromone_value;
        }
    }
    pher_graph
}


pub fn get_distance_graph(cities_list:&Vec<City>) -> GraphMatrix {
    // Creates a distance matrix betweeen all connected cities
    let mut dist_graph:Vec<Vec<f32>> = vec![vec![0.0; cities_list.len()]; cities_list.len()];
    for from_city_idx in 0..cities_list.len(){
        for to_city_idx in get_connected_cities_indicies(from_city_idx, cities_list){
            dist_graph[from_city_idx][to_city_idx] = 
            calculate_distance_generic(&cities_list[from_city_idx], &cities_list[to_city_idx]);
        }
    }
    dist_graph
}


fn score_city(from_city_idx:usize, to_city_idx:usize, pher_graph:&GraphMatrix, dist_graph:&GraphMatrix, beta:f32) -> f32 {
    // Scores the to_city_idx based on from_city_idx
    let beta:f32 = if beta!=0.0 {beta} else{0.20};
    let score:f32 = pher_graph[from_city_idx][to_city_idx] * f32::powf(1.0/dist_graph[from_city_idx][to_city_idx], beta);
    score
}


pub fn argmax<T: PartialOrd + Copy>(array:Vec<T>) -> usize {
    // Returns the index of the maximum value in a vec
    let mut max_index = 0;
    for (i, val) in array.iter().enumerate(){
        if val > &array[max_index]{
            max_index = i
        }
    }
    max_index
}


pub fn build_tour(cities_list:&Vec<City>, pher_graph:&GraphMatrix, dist_graph:&GraphMatrix, q0:f32, beta:f32) -> Vec<usize>{
    let cities_idx:Vec<usize> = (0..cities_list.len()).into_iter().collect();
    let mut tour:Vec<usize> = Vec::with_capacity(cities_list.len());
    let start_city_idx = rand::thread_rng().gen_range(0..cities_list.len()-1);
    tour.push(start_city_idx);
    for _ in 1..cities_idx.len() {
        let unvisited: Vec<&usize> = cities_idx.iter().filter(|&city| !tour.contains(city)).collect();
        let scores: Vec<f32> = unvisited.iter().map(|&to_city_idx| 
                                score_city(*tour.last().unwrap(), *to_city_idx, pher_graph, dist_graph, beta)).collect::<Vec<f32>>();
        let q:f32 = rand::thread_rng().gen();
        if q < q0 {
            let max_index:usize = argmax(scores);
            tour.push(*unvisited[max_index]);
        } else{
            let sum_scores:f32 = scores.iter().sum();
            let prob_dist:Vec<f32> = scores.iter().map(|score| score/sum_scores).collect();
            // let indices:Vec<_> = (0..=univisted.len()).collect();
            let indices:Vec<usize> =  (0..unvisited.len()).collect();
            let choice:&usize = random_choice().random_choice_f32(&indices, &prob_dist, 1).first().unwrap();
            tour.push(*unvisited[*choice]);
        }
    }
    assert!(tour.len() == cities_idx.len(), "Tour and cities are not the same length");
    tour
}


pub fn local_pheromone_update3(mut pher_graph:GraphMatrix, tour:Vec<usize>, cities_list:&Vec<City>, rho:f32, tau:f32) -> GraphMatrix {
    // Updates the pheromone graph given a tour
    for from_city_idx in tour{
        for to_city_idx in get_connected_cities_indicies(from_city_idx, cities_list){
            let old_pher_val:f32 = pher_graph[from_city_idx][to_city_idx];
            let new_pher_val = (1.0 - rho) * old_pher_val + (rho * tau); 
            pher_graph[from_city_idx][to_city_idx]=new_pher_val;
        }
    }
    pher_graph
}


fn global_pheromone_update(mut pher_graph:GraphMatrix, 
                               tour:Vec<usize>, 
                               cities_list:&Vec<City>, 
                               alpha:f32, 
                               best_path_distance:f32) -> GraphMatrix {
    for from_city_idx in tour{
        for to_city_idx in get_connected_cities_indicies(from_city_idx, cities_list){
            let old_pheromone:f32 = pher_graph[from_city_idx][to_city_idx];
            let new_pheromone:f32 = (1.0 - alpha) * old_pheromone + alpha * f32::powf(best_path_distance, -1.0);
            pher_graph[from_city_idx][to_city_idx] = new_pheromone;
        }
    }
    pher_graph
}

pub fn get_tour_distance(tour:Vec<usize>, cities_list:&Vec<City>) -> f32 {
    // Takes a vec of city indicies (tour) and returns the total distance travelled
    let mut tour2:Vec<usize> = tour.clone();
    tour2.rotate_left(1);
    let distance:f32 = tour.iter()
                        .zip(tour2)
                        .map(|(from_city_idx, to_city_idx)| 
                        calculate_distance_generic(cities_list[*from_city_idx], cities_list[to_city_idx])).sum();
    distance
}

pub struct ACO3<'a>{
    cities_list:&'a Vec<City>, 
    pher_graph: GraphMatrix, 
    dist_graph:GraphMatrix, 
    iterations:i32, 
    num_ants:i32, 
    q0:f32, 
    beta:f32,
    rho:f32, 
    tau:f32,
    alpha:f32,
    shortest_tour:Vec<usize>
}

impl <'a> ACO3 <'a> {

    pub fn new(cities_list:&'a Vec<City>, pher_graph: GraphMatrix, dist_graph:GraphMatrix, iterations:i32, num_ants:i32, shortest_tour:Vec<usize>) -> ACO3<'a> {
        ACO3{cities_list, pher_graph, dist_graph, iterations, num_ants, shortest_tour, q0:0.90, beta:0.20, rho:0.1, tau:0.0005, alpha:0.1}
    } 

    pub fn optimize(&mut self) -> Vec<usize>{
        let mut best_tour:Vec<usize>= Vec::new();
        let mut best_tour_distance= std::f32::INFINITY;

        for i in 0..self.iterations{

            println!("Iteration -> {}, best_distance_found -> {:.2}", i, best_tour_distance);

            if get_tour_distance(self.shortest_tour.clone(), self.cities_list).round() == best_tour_distance.round() {
                println!("\nShort path found at {} iteration", i);
                break;
            }

            let tours:Vec<Vec<usize>> = (0..self.num_ants)
                                        .map(|_| 
                                        build_tour(self.cities_list, &self.pher_graph, &self.dist_graph, self.q0, self.beta))
                                        .collect();
            for tour in tours{
                self.pher_graph = local_pheromone_update3(self.pher_graph.clone(), tour.clone(), self.cities_list, self.rho, self.tau);
                let tour_distance:f32 = get_tour_distance(tour.clone(), self.cities_list);
                if tour_distance < best_tour_distance{
                    best_tour = tour.clone();
                    best_tour_distance = tour_distance;
                    }
            }
            self.pher_graph = global_pheromone_update(self.pher_graph.clone(), best_tour.clone(), self.cities_list, self.alpha, best_tour_distance);
        }
        best_tour
    }
}