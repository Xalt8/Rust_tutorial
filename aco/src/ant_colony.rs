use crate::city::City;
use std::sync::{Arc, Mutex};
use crate::graph::{Graph, calculate_distance};
// use crate::ant::Ant;
use crate::city;
use crate::ant2::Ant;


// pub fn get_tour_city_tuples(tour:Vec<&City>) -> Vec<(&City, &City)> {
//     // Takes a tour of cities and returns the cities in tuples (City1, City2)
//     // and closes the tour loop by including the last city and first first city 
//     let mut tour_city_tuples:Vec<_> = tour[0..tour.len()-1].iter()
//                                                    .zip(tour[1..].iter())
//                                                    .collect();
//     tour_city_tuples.push((tour.last().unwrap(), tour.first().unwrap()));
//     tour_city_tuples
// }

pub fn get_tour_city_tuples<'a>(tour: Vec<&City>) -> Vec<(&'a City, &'a City)> {
    let mut tour_city_tuples: Vec<(&City, &City)> = 
        tour[0..tour.len() - 1].iter()
        .zip(tour[1..].iter())
        .map(|(&a,&b)| (a, b))
        .collect();
    tour_city_tuples.push((&tour.last().unwrap(), &tour.first().unwrap()));
    tour_city_tuples
}


pub fn get_tour_length(tour:Vec<&City>) -> f32 {
    // Takes a tour and returns the distance covered in the tour  
    let tour_city_tuples:Vec<(&City, &City)> = get_tour_city_tuples(tour);
    let tour_length:f32 = tour_city_tuples.iter()
                          .map(|(city1, city2)| 
                          calculate_distance(city1, city2)).sum(); 
    tour_length
}

// pub fn get_tour_length(tour:Vec<i32>) -> f32 {
//     // Takes a vec of city names (tour) and returns the 
//     // distance travelled on that tour
//     let tour_tuples:Vec<i32> = 
// }

pub struct AntColony<'a> {
    pub best_path: Vec<&'a City>,
    pub best_path_distance: f32,
    alpha:f32,
    iterations:i32,
    // tau:f32,
    num_ants:i32,
    ants_list:Vec<Ant<'a>>,
    cities_list: Vec<City>,
    pheromone_graph:&'a Arc<Mutex<Graph>>,
    distance_graph:&'a Graph,
}


impl <'a> AntColony <'a> {
    pub fn new(cities_list: Vec<City>, pheromone_graph:&'a Arc<Mutex<Graph>>, distance_graph:&'a Graph, num_ants:i32, iterations:i32) -> Self {
        let ants:Vec<Ant> = (0..num_ants).map(|_|Ant::new(cities_list, pheromone_graph, distance_graph)).collect();
        Self {best_path:Vec::new(),
             best_path_distance:std::f32::INFINITY,
             alpha:0.1,
             iterations:iterations,
            //  tau:0.005,
             num_ants:num_ants,
             ants_list:ants,
             cities_list:cities_list,
             pheromone_graph:pheromone_graph,
             distance_graph:distance_graph,
        }
    }


    // fn set_global_best(&mut self) {
    //     // Calculates the tour length for all the ants and 
    //     // sets the best distance and best tour
    //     for mut ant in self.ants_list.clone() {
    //         let tour:Vec<City> = ant.goes_on_tour();
    //         let tour_length:f32 = get_tour_length(tour.clone());
    //         if tour_length < self.best_path_distance{
    //             self.best_path_distance = tour_length;
    //             self.best_path = tour.clone();
    //         }
    //     }
    // }


    
    fn global_update_pheromone(&mut self) {
        // Takes the best tour and applies the global update pheromone 
        let mut pher_graph = self.pheromone_graph.lock().unwrap();
        let tour_city_tuples:Vec<(&City, &City)> = get_tour_city_tuples(self.best_path);
        for (from_city,to_city) in tour_city_tuples {
            let old_pheromone:f32 = *pher_graph.get(&from_city.name).unwrap().get(&to_city.name).unwrap();
            let new_pheromone:f32 = (1.0 - self.alpha) * old_pheromone + self.alpha * f32::powf(self.best_path_distance, -1.0);
            if let Some(from_city_name) = pher_graph.get_mut(&from_city.name){
                from_city_name.insert(to_city.name, new_pheromone);
            }
        }
    }


    pub fn optimize(&mut self) {
        let short_path: Vec<&City> = city::get_shortest_path("shortest_path.txt", self.cities_list);
        let shortest_distance:f32 = get_tour_length(short_path);
        
        for i in 0..self.iterations {
            // Ant new() always starts from the firt city!!
            self.ants_list = (0..self.num_ants).map(|_|Ant::new(self.cities_list, self.pheromone_graph, self.distance_graph)).collect();
            // self.set_global_best();
            self.global_update_pheromone();
            println!("Iteration {:?}, best_distance {:?}", i, self.best_path_distance);
            self.ants_list.clear();
            }
        }
    }

