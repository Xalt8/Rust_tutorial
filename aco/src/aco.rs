use crate::ant2::{Ant};
use crate::city::City;
use crate::graph::{Graph, get_tour_tuples, get_tour_length, get_tour_length_generic};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct ACO<'a> {
    pub best_path: Vec<City>,
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

    
    fn global_update_pheromone(&mut self) {
        // Takes the best tour and applies the global update pheromone 
        let mut pher_graph = self.pheromone_graph.lock().unwrap();
        let tour_city_tuples:Vec<(City, City)> = get_tour_tuples(&self.best_path);
        for (from_city,to_city) in tour_city_tuples {
            let old_pheromone:f32 = *pher_graph.get(&from_city).unwrap().get(&to_city).unwrap();
            let new_pheromone:f32 = (1.0 - self.alpha) * old_pheromone + self.alpha * f32::powf(self.best_path_distance, -1.0);
            if let Some(from_city_name) = pher_graph.get_mut(&from_city){
                from_city_name.insert(to_city, new_pheromone);
            }
        }
    }

    
    pub fn optimize(&mut self, short_path:Vec<&City>) {

        
        let short_path_dist:f32 = get_tour_length_generic(short_path);

        for i in 0..self.iterations {
            
            println!("Iteration {:?}, best distance found -> {:.2}, shortest_path_distance -> {:.2}", i, self.best_path_distance, short_path_dist);
            
            if short_path_dist == self.best_path_distance {
                println!("\nShort path found at {} iteration", i);
                break;
            }
            
            // Concurrency
            let handle = thread::spawn(move||{
                for i in 0..self.num_ants {
                     
                }
            });
            
            
            
                let ants:Vec<Ant> = (0..self.num_ants).map(|_|Ant::new(self.cities_list, self.pheromone_graph, self.distance_graph)).collect();
                let tours:Vec<Vec<City>> = ants.iter().map(|ant| ant.make_tour()).collect();  
                for (ant, tour) in ants.iter().zip(tours.iter()) {
                    let new_tour = ant.two_opt(tour);
                    ant.local_pheromone_update(&new_tour);
                }

                let tour_dists:Vec<f32> = tours.iter().map(|tour| get_tour_length_generic(tour.to_vec())).collect();
                for (tour, dist) in tours.iter().zip(tour_dists) {
                    if dist < self.best_path_distance {
                        self.best_path_distance = dist;
                        self.best_path = tour.clone();    
                    } 
                }
            
            
            handle.join().unwrap();
            self.global_update_pheromone();
        }
    }

}
