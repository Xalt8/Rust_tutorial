use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::city::City;
pub type Graph = HashMap<City, HashMap<City, f32>>;
use std::borrow::Borrow;


pub fn get_fully_connected_cities(cities_list:&Vec<City>) -> Vec<(&City, &City)> {
    // Takes a vec of cities and returns a vec of tuples with every combination
    // of city as tuples 
    let mut city_combinations:Vec<(&City, &City)> = Vec::new(); 
    for city1 in cities_list {
        for city2 in cities_list {
            if city1.name != city2.name {
                city_combinations.push((&city1, &city2))
            }
        }
    }
    city_combinations
}


pub fn create_pheromone_graph(cities_list:&Vec<City>, initial_pheromone:f32) -> Arc<Mutex<Graph>> {
    // Create a Acr Mutex HashMap with a city name and its neighbours with initial phermone amount 
    let mut city_graph = Graph::new();
    let city_tuples = get_fully_connected_cities(cities_list);
    for (from_city, to_city) in city_tuples {
        city_graph.entry(*from_city).or_insert(HashMap::new())
        .entry(*to_city).or_insert(initial_pheromone);
    }
    let graph = Arc::new(Mutex::new(city_graph));
    graph
}


pub fn calculate_distance(city1:&City, city2:&City) -> f32 { 
    // Returns the distance between 2 cities
    (((city1.x - city2.x) as f32).powf(2.0) + 
     ((city1.y - city2.y) as f32).powf(2.0)).sqrt()
}


pub fn create_distance_graph(cities_list:&Vec<City>) -> Graph {
    let mut graph = Graph::new();
    let city_tuples = get_fully_connected_cities(cities_list);
    for (from_city, to_city) in city_tuples {
        // add_nodes_distance(cities.0, cities.1, &mut graph)
        let distance:f32 = calculate_distance(from_city, to_city);
        graph.entry(*from_city).or_insert(HashMap::new())
        .entry(*to_city).or_insert(distance);
    }
    graph
}

pub fn get_tour_tuples(tour:&Vec<City>) -> Vec<(City, City)> {
    // Takes a tour of city names and returns a vec of tuples   
    // and connects the last and first cities in tour
    let mut tour2 = tour.clone();
    tour2.rotate_left(1);
    let tour_city_tuples:Vec<(City, City)> = tour.iter()
                                               .zip(tour2.iter())
                                               .map(|(a, b)| (*a,*b)).collect();
    tour_city_tuples
}

pub fn get_tour_length(tour:&Vec<City>) -> f32 {
    // Takes a tour and returns the total dsitance travelled in the tour
    let tour_tuples = get_tour_tuples(tour);
    tour_tuples.iter().map(|(city1, city2)| calculate_distance(city1, city2)).sum()
}

// ===================================================================================================

// ===================================================================================================

pub fn get_tour_tuples_generic<T>(tour:Vec<T>) -> Vec<(T,T)> 
where 
    T: Borrow<City> + Copy,
{
    let mut tour2 = tour.clone();
    tour2.rotate_left(1);
    let tour_tuples: Vec<_> = tour.iter()
                              .zip(tour2.iter())
                              .map(|(a, b)| (*a, *b))
                              .collect();
    tour_tuples
}


pub fn calculate_distance_generic<T>(city1:T, city2:T) -> f32 
where 
    T:Borrow<City>,
{ 
    let city1 = city1.borrow();
    let city2 = city2.borrow();
    (((city1.x - city2.x) as f32).powf(2.0) + ((city1.y - city2.y) as f32).powf(2.0)).sqrt()
}


pub fn get_tour_length_generic<T>(tour:Vec<T>) -> f32 
where 
    T: Borrow<City> + Copy,
{
    let tour_tuples = get_tour_tuples_generic(tour);
    tour_tuples.iter()
               .map(|(city1, city2)| 
               calculate_distance_generic(city1.borrow(), city2.borrow()))
               .sum()
}



