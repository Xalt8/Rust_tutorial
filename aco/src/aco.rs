use crate::ant2::{Ant, get_tour_name_tuples};
use crate::city::City;
use crate::graph::{Graph, calculate_distance};
use std::sync::{Arc, Mutex};


fn two_opt_swap<T: Clone>(tour: Vec<T>, i: usize, k: usize) -> Vec<T> {
    // Takes two index values i, k
    // Takes the start node to i+1 in a tour and adds it to new tour
    // Takes all nodes from i+1 to k and adds them in reverse order to the new tour
    // Takes nodes k+1 to the end of the end of the tour and adds them to new tour 
    let new_tour: Vec<T> = tour[0..=i].iter()
        .chain(tour[i+1..=k].iter().rev())
        .chain(tour[k+1..tour.len()].iter())
        .cloned().collect();
    new_tour
}


pub struct ACO<'a> {
    pub best_path: Vec<i32>,
    pub best_path_distance: f32,
    alpha:f32,
    iterations:i32,
    // tau:f32,
    num_ants:i32,
    // ants_list:Vec<Ant<'a>>,
    cities_list: &'a Vec<City>,
    pheromone_graph:&'a Arc<Mutex<Graph>>,
    distance_graph:&'a Graph,
}

impl <'a> ACO <'a>{

    pub fn new(cities_list:&'a Vec<City>, 
               pheromone_graph:&'a Arc<Mutex<Graph>>, 
               distance_graph:&'a Graph, 
               num_ants:i32,
               iterations:i32) -> Self {
        Self{best_path:Vec::new(),
            best_path_distance:std::f32::INFINITY,
            pheromone_graph: pheromone_graph,
            distance_graph: distance_graph,
            num_ants: num_ants,
            cities_list: cities_list,
            iterations:iterations,
            alpha:0.1,
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


    fn global_update_pheromone(&mut self) {
        // Takes the best tour and applies the global update pheromone 
        let mut pher_graph = self.pheromone_graph.lock().unwrap();
        let tour_city_tuples:Vec<(i32, i32)> = get_tour_name_tuples(&self.best_path);
        for (from_city,to_city) in tour_city_tuples {
            let old_pheromone:f32 = *pher_graph.get(&from_city).unwrap().get(&to_city).unwrap();
            let new_pheromone:f32 = (1.0 - self.alpha) * old_pheromone + self.alpha * f32::powf(self.best_path_distance, -1.0);
            if let Some(from_city_name) = pher_graph.get_mut(&from_city){
                from_city_name.insert(to_city, new_pheromone);
            }
        }
    }

    fn two_opt(&mut self, tour:&Vec<i32>) -> Vec<i32> {
        let mut new_tour:Vec<i32> = Vec::with_capacity(tour.len());   
        loop {
            let mut improved:bool = false;
            for i in 0..(tour.len()-1){
                for j in i+1..tour.len(){
                    new_tour = two_opt_swap(tour.to_vec(), i, j);
                    let new_tour_dist = self.get_tour_length(&new_tour);
                    if new_tour_dist < self.best_path_distance {
                        self.best_path_distance = new_tour_dist;
                        self.best_path = new_tour.clone();
                        improved = true;
                        break;
                    }
                }
            if improved {
                break;
            }
        }
        if !improved{
            break;
        }
        }
        new_tour
    }

    
    pub fn optimize(&mut self, short_path:Vec<City>) {

        let short_path_names:Vec<i32> = short_path.iter().map(|city| city.name).collect();
        let short_path_dist:f32 = self.get_tour_length(&short_path_names);

        for i in 0..self.iterations {

            println!("Iteration {:?}, best distance -> {:?}", i, self.best_path_distance);
            
            if short_path_dist == self.best_path_distance {
                println!("\nShort path found at {} iteration", i);
                break;
            }

            let ants:Vec<Ant> = (0..self.num_ants).map(|_|Ant::new(self.cities_list, self.pheromone_graph, self.distance_graph)).collect();
            let tours:Vec<Vec<i32>> = ants.iter().map(|ant| ant.make_tour()).collect();  
            for (ant, tour) in ants.iter().zip(tours.iter()) {
                ant.local_pheromone_update(tour);
            }
            let tour_dists:Vec<f32> = tours.iter().map(|tour| self.get_tour_length(tour)).collect();
            for (tour, dist) in tours.iter().zip(tour_dists) {
                if dist < self.best_path_distance {
                    self.best_path_distance = dist;
                    self.best_path = tour.clone();    
                } 
            }
            self.global_update_pheromone();
        }
    }

}
