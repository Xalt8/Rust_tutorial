use crate::ant2::{Ant, argmax};
use crate::city::City;
use crate::graph::{Graph, get_tour_tuples, get_tour_length_generic};
use std::sync::{Arc, Mutex};
use std::thread;
use rayon::prelude::*;


#[derive(Debug, Clone)]
pub struct ACO  {
    pub best_path: Vec<City>,
    pub best_path_distance: f32,
    alpha: f32,
    iterations: i32,
    num_ants: i32,
    cities_list: &'static Vec<City>,
    pheromone_graph: &'static Arc<Mutex<Graph>>,
    distance_graph: &'static Graph,
}

impl ACO {
    pub fn new(cities_list: &'static Vec<City>, 
               pheromone_graph: &'static Arc<Mutex<Graph>>, 
               distance_graph: &'static Graph, 
               num_ants: i32,
               iterations: i32) -> Self {
        Self {
            best_path: Vec::new(),
            best_path_distance: std::f32::INFINITY,
            pheromone_graph: pheromone_graph,
            distance_graph: distance_graph,
            num_ants: num_ants,
            cities_list: cities_list,
            iterations: iterations,
            alpha: 0.1,
        }
    }


    pub fn optimize_concurrent(&mut self) {
        let aco_mutex: Arc<Mutex<ACO>> = Arc::new(Mutex::new(self.clone()));

        println!("\noptimize_concurrent()\n");

        for i in 0..self.iterations{
            println!("Iteration {}, best_dist-> {}" , i, aco_mutex.lock().unwrap().best_path_distance);
            let mut handles = vec![];
            for _ in 0..self.num_ants{
                let aco_mutex:Arc<Mutex<ACO>> = Arc::clone(&aco_mutex);
                let handle = thread::spawn({
                    move ||{
                    let mut aco_mutex = aco_mutex.lock().unwrap();
                    let ant = Ant::new(&aco_mutex.cities_list, &aco_mutex.pheromone_graph, &aco_mutex.distance_graph);
                    let tour:Vec<City> = ant.make_tour();
                    let tour2:Vec<City> = ant.two_opt(&tour);
                    ant.local_pheromone_update(&tour);
                    let tour_dist:f32 = get_tour_length_generic(tour2.to_vec());
                    if tour_dist < aco_mutex.best_path_distance {
                        aco_mutex.best_path_distance = tour_dist;
                        aco_mutex.best_path = tour2;
                        }
                    } 
                });
                handles.push(handle);
            }
            for handle in handles{
                handle.join().unwrap();
            }
            let mut aco_mutex = aco_mutex.lock().unwrap();
            aco_mutex.global_update_pheromone();
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

        println!("\noptimize()\n");

        for i in 0..self.iterations {
            
            println!("Iteration {:?}, best distance found -> {:.2}, shortest_path_distance -> {:.2}", i, self.best_path_distance, short_path_dist);
            
            if short_path_dist == self.best_path_distance {
                println!("\nShort path found at {} iteration", i);
                break;
            }
            
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
            self.global_update_pheromone();
        }
    }



    pub fn optimize_concurrent_rayon(&mut self, short_path:Vec<&City>) {
        let short_path_dist:f32 = get_tour_length_generic(short_path);
        
        println!("\noptimize_concurrent_rayon()\n");
        
        for i in 0..self.iterations {
            
            println!("Iteration {:?}, best dist found -> {:.2}, shortest_path_distance -> {:.2}", i, self.best_path_distance, short_path_dist);
            
            if short_path_dist == self.best_path_distance {
                println!("\nShort path found at {} iteration", i);
                break;
            }
            let ants:Vec<Ant> = (0..self.num_ants).map(|_|Ant::new(self.cities_list, self.pheromone_graph, self.distance_graph)).collect();
            let ant_tours:Vec<(usize, Vec<City>)> = ants.par_iter().enumerate().map(|(index, ant)|{
                let tour = ant.make_tour();
                let new_tour = ant.two_opt(&tour);
                ant.local_pheromone_update(&new_tour);
                (index, new_tour)
            }).collect();
            let tour_dists:Vec<f32> = ant_tours.par_iter()
                                     .map(|(_, tour)| 
                                     get_tour_length_generic(tour.to_vec()))
                                     .collect();
            let max_index:usize = argmax(tour_dists.clone());
            if self.best_path_distance > tour_dists[max_index] {
                // let best_tour = ant_tours[max_index].1.clone();
                self.best_path_distance = tour_dists[max_index];
                self.best_path = ant_tours[max_index].1.clone();
            }
            self.global_update_pheromone();
        }
            
    }    
}







   
