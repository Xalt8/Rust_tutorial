use std::sync::{Arc, Mutex, MutexGuard};
use rand::Rng;
use random_choice::random_choice;
use crate::city::City;
use crate::graph::{Graph, calculate_distance};


fn argmax<T: PartialOrd + Copy>(array:Vec<T>) -> usize {
    // Returns the index of the maximum value in a vec
    let mut max_index = 0;
    for (i, val) in array.iter().enumerate(){
        if val > &array[max_index]{
            max_index = i
        }
    }
    max_index
}

pub fn get_tour_name_tuples(tour:&Vec<i32>) -> Vec<(i32, i32)> {
    // Takes a tour of city names and returns a vec of tuples   
    // Also connects the last and first cities
    let mut tour2 = tour.clone();
    tour2.rotate_left(1);
    let tour_city_tuples:Vec<(i32, i32)> = tour.iter()
                                               .zip(tour2.iter())
                                               .map(|(a, b)| (*a,*b)).collect();
    tour_city_tuples
}


fn two_opt_swap<T: Clone>(tour: Vec<T>, i: usize, j: usize) -> Vec<T> {
    // Takes two index values i, k
    // Takes the start node to i+1 in a tour and adds it to new tour
    // Takes all nodes from i+1 to k and adds them in reverse order to the new tour
    // Takes nodes k+1 to the end of the end of the tour and adds them to new tour 
    let new_tour: Vec<T> = tour[0..=i].iter()
        .chain(tour[i+1..=j].iter().rev())
        .chain(tour[j+1..tour.len()].iter())
        .cloned().collect();
    new_tour
}



#[derive(Debug, Clone)]
pub struct Ant<'a> {
    cities_list: &'a Vec<City>,
    pheromone_graph: &'a Arc<Mutex<Graph>>,
    distance_graph: &'a Graph,
    beta: f32,
    q0: f32,
    rho: f32,
    tau: f32,
}


impl<'a> Ant<'a>{
    pub fn new(cities_list:&'a Vec<City>, pheromone_graph:&'a Arc<Mutex<Graph>>,distance_graph: &'a Graph) -> Ant<'a> {
        Ant{cities_list:cities_list,
                           pheromone_graph:pheromone_graph,
                           distance_graph:distance_graph,
                           beta:2.0, 
                           q0:0.9, 
                           rho:0.1, 
                           tau:0.0005}
    }


    fn score_node(&self, from_node_name:&i32, to_node_name:&i32) -> f32 {
        // Scores a node based on the current node and node_name passed
        // Used in make_tour()
        let pher_graph_clone:Arc<Mutex<Graph>> = Arc::clone(&self.pheromone_graph);
        let pher_graph:MutexGuard<Graph> = pher_graph_clone.lock().unwrap(); 
        let phermone:&f32 = pher_graph.get(&from_node_name).unwrap().get(&to_node_name).unwrap();
        let distance:&f32 = self.distance_graph.get(&from_node_name).unwrap().get(&to_node_name).unwrap();
        phermone * f32::powf(1.0/distance, self.beta)
    }


    pub fn make_tour(&self) -> Vec<i32> {
        let mut visited_nodes:Vec<i32> = Vec::with_capacity(self.cities_list.len());
        // First city -> start randomly
        let city_names:Vec<i32> = self.cities_list.iter().map(|city| city.name).collect();
        let rand_index = rand::thread_rng().gen_range(0..city_names.len()-1);
        let start_city_name:i32 = city_names[rand_index];
        visited_nodes.push(start_city_name);
        // Rest of cities
        while visited_nodes.len() != self.cities_list.len() {
            
            let mut univisted:Vec<_> = city_names.iter().filter(|city| !visited_nodes.contains(city)).collect();
            let scores:Vec<f32> = univisted.iter_mut().map(|city| 
                                  self.score_node(visited_nodes.last().unwrap(),
                                  city)).collect();
            assert_eq!(univisted.len(), scores.len(), "unvisited and scores are not equal in length");
            let q:f32 = rand::thread_rng().gen();
            if q < self.q0 {
                let max_index:usize = argmax(scores);
                visited_nodes.push(*univisted[max_index]);
            } else {
                let sum_scores:f32 = scores.iter().sum();
                let prob_dist:Vec<f32> = scores.iter().map(|score| score/sum_scores).collect();
                // let indices:Vec<_> = (0..=univisted.len()).collect();
                let indices:Vec<usize> =  (0..univisted.len()).collect();
                let choice:&usize = random_choice().random_choice_f32(&indices, &prob_dist, 1).first().unwrap();
                visited_nodes.push(*univisted[*choice]);
            }
        }
        visited_nodes
    }


    pub fn local_pheromone_update(&self, tour:&Vec<i32>) {
        let tour_tuples:Vec<(i32, i32)> = get_tour_name_tuples(tour);
        let mut pher_graph:MutexGuard<Graph> = self.pheromone_graph.lock().unwrap();
        for (from_city_name, to_city_name) in tour_tuples {
            let old_pheromone:f32 = *pher_graph.get(&from_city_name).unwrap().get(&to_city_name).unwrap();
            let new_pheromone:f32 = (1.0 - self.rho) * old_pheromone + (self.rho * self.tau);
            let to_city_map = pher_graph.get_mut(&from_city_name)
                                                      .expect("Couldn't find city in local_pheromone_update()");
            to_city_map.insert(to_city_name, new_pheromone);
        }
    }

    fn get_city(&self, city_name: i32) -> &City {
        // Takes a city name and returns the city object
        self.cities_list.iter().find(|city| city.name == city_name).expect("City not found")
    }


    fn get_tour_length(&self, tour:&Vec<i32>) -> f32 {
        // Takes a tour and returns the total dsitance travelled in the tour
        let tour_names_tuples = get_tour_name_tuples(&tour);
        let tour_city_tuples:Vec<(&City, &City)> = tour_names_tuples.iter()
                                                   .map(|(city1, city2)| 
                                                    (self.get_city(*city1), self.get_city(*city2)))
                                                    .collect();
        tour_city_tuples.iter().map(|(city1, city2)| calculate_distance(city1, city2)).sum()
    }


    fn two_opt(&mut self, tour:&Vec<i32>) -> Vec<i32> {
        // Local search heuristic
        let mut best_tour:Vec<i32> = tour.to_vec();
        let mut best_tour_dist:f32 = self.get_tour_length(&best_tour); 
        let mut iterations_since_improvement:usize = 0;
        let mut improved:bool = true;   
        while improved && iterations_since_improvement < 10 {
            improved = false;
            for i in 0..(tour.len()-1){
                for j in i + 1..tour.len(){
                    let new_tour:Vec<i32> = two_opt_swap(tour.to_vec(), i, j);
                    let new_tour_dist = self.get_tour_length(&new_tour);
                    if new_tour_dist < best_tour_dist {
                        best_tour_dist = new_tour_dist;
                        best_tour = new_tour;
                        improved = true;
                        iterations_since_improvement = 0;
                        break;
                    }
                }
            if improved {
                break;
            }
        }
        iterations_since_improvement += 1;
        }
        best_tour.clone()
    }
    
}