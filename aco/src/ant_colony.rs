use crate::city::City;
use std::sync::{Arc, Mutex};
use crate::graph::{Graph, calculate_distance};
use crate::ant::Ant;


pub struct AntColony<'a> {
    pub best_path: Vec<City>,
    pub best_path_distance: f32,
    alpha:f32,
    iterations:i32,
    tau:f32,
    num_ants:i32,
    ants_list:Vec<Ant<'a>>,
}

impl <'a> AntColony <'a> {
    pub fn new(cities_list: &'a Vec<City>, pheromone_graph:&'a Arc<Mutex<Graph>>, distance_graph:&'a Graph, num_ants:i32) -> Self {
        let ants:Vec<Ant> = (0..num_ants).map(|_|Ant::new(cities_list, pheromone_graph, distance_graph)).collect();
        Self {best_path:Vec::new(),
             best_path_distance:std::f32::INFINITY,
             alpha:0.1,
             iterations:20,
             tau:0.005,
             num_ants:num_ants,
             ants_list:ants,
        }
    }


    fn get_tour_length(&self, tour:Vec<City>) -> f32 {
        // Takes a tour and returns the distance covered in the tour  
        let mut zip_city:Vec<(&City, &City)> = tour[0..tour.len()-1].iter()
                                  .zip(tour[1..].iter()).collect();
        zip_city.push((tour.last().unwrap(), tour.first().unwrap()));
        let tour_length:f32 = zip_city.iter().map(|(city1, city2)|  calculate_distance(city1, city2)).sum(); 
        tour_length
    }


    fn set_global_best(&mut self) {
        // Calculates the tour length for all the ants and 
        // sets the best distance and best tour
        for mut ant in self.ants_list.clone() {
            let tour:Vec<City> = ant.goes_on_tour();
            let tour_length:f32 = self.get_tour_length(tour.clone());
            if tour_length < self.best_path_distance{
                self.best_path_distance = tour_length;
                self.best_path = tour.clone();
            }
        }
    }

    
    fn global_update_pheromone() {
        todo!();
    }


    pub fn optimize() {
        todo!();
    }


}
