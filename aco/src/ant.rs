use std::sync::{Arc, Mutex};
use rand::Rng;
use random_choice::random_choice;

use crate::city::City;
use crate::graph::Graph;


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


#[derive(Debug, Clone)]
pub struct Ant<'a> {
    cities_list: &'a Vec<City>,
    current_node: &'a City,
    visited_nodes: Vec<City>,
    pheromone_graph: &'a Arc<Mutex<Graph>>,
    distance_graph: &'a Graph,
    beta: f32,
    q0: f32,
    rho: f32,
    tau: f32,
}


impl<'a> Ant<'a>{

    pub fn new(cities_list: &'a Vec<City>, pheromone_graph: &'a Arc<Mutex<Graph>>, distance_graph:&'a Graph) -> Self {
        // Creates a new Ant object by choosing the first city in the cities list  
        // and adding it to the visited cities list
        let rand_index = rand::thread_rng().gen_range(0..cities_list.len()-1);
        let start_city:&City = &cities_list[rand_index];
        let mut ant = Self{cities_list:cities_list,
                                current_node:start_city,
                                visited_nodes:Vec::new(),  
                                pheromone_graph:pheromone_graph,
                                distance_graph:distance_graph,
                                beta:2.0, 
                                q0:0.9, 
                                rho:0.1, 
                                tau:0.005};
        ant.visit_city(start_city.name);
        ant
    }


    fn get_city(&self, city_name: i32) -> Option<&'a City> {
        // Takes a city name and returns the city object
        // Used in visit_city()
        self.cities_list.iter().find(|city| city.name == city_name)
    }


    fn visit_city(&mut self, city_name:i32) {
        // Adds the given city_name as the current 
        // and adds it to visited nodes list
        let city = self.get_city(city_name).unwrap();
        self.current_node = &city;
        self.visited_nodes.push(*city);
        // println!("visiting city -> {:?}", city);
    }
    

    fn get_unvisited_cities(&self) -> Vec<&City> {
        // Returns a vec of unvisited cities 
        // Used in run_ant_run()
        self.cities_list
        .iter()
        .filter(|city| !self.visited_nodes.contains(city))
        .collect()
    }


    fn score_node(&self, node_name:i32) -> f32 {
        // Scores a node based on the current node and node_name passed
        // Used in choose_node()
        let pher_graph_clone = Arc::clone(&self.pheromone_graph);
        let pher_graph = pher_graph_clone.lock().unwrap(); 
        let phermone = pher_graph.get(&self.current_node.name).unwrap().get(&node_name).unwrap();
        let distance = self.distance_graph.get(&self.current_node.name).unwrap().get(&node_name).unwrap();
        phermone * f32::powf(1.0/distance, self.beta)
    }




    fn choose_node(&self, univisted:Vec<&City>) -> i32 {
        // q -> random value between 0,1
        let q:f32 = rand::thread_rng().gen();
        // let univisted = self.get_unvisited_cities();
        let scores:Vec<f32> = univisted.iter().map(|city| self.score_node(city.name)).collect(); 
        let mut max_index:usize = std::usize::MAX;
        let mut max_score:f32 = std::f32::NEG_INFINITY; 
        for (i, score) in scores.iter().enumerate(){
            if score > &max_score {
                max_score = *score;
                max_index = i;
            }
        }
        
        // let max3 = scores.iter().enumerate().fold((-10000, 0.0), |max, (ind, &val)| if val > max.1 {(ind, val)} else {max});
        
        if q < self.q0 {
            // println!("exploiting, node -> {:?}", univisted[max_index].name);
            return univisted[max_index].name;    
        } else {

            let sum_scores:f32 = scores.iter().sum();
            let prob_dist:Vec<f32> = scores.iter().map(|score| score/sum_scores).collect();
            let city_names = &univisted.into_iter().map(|city| city.name).collect::<Vec<_>>();
            let choice:Vec<&_> = random_choice().random_choice_f32(&city_names, &prob_dist, 1);
            // println!("exploring, node -> {:?}", **choice.first().unwrap());
            return **choice.first().unwrap();
        }
    }

        
    fn local_pheromone_update(&mut self, city_name:i32) {
        // Applies pheromone update to the edge between the current node and the city passed
        // using the previous pheromone and the local pheromone update formula 
        let mut pher_graph = self.pheromone_graph.lock().unwrap();
        let old_pheromone:f32 = *pher_graph.get(&self.current_node.name).unwrap().get(&city_name).unwrap();
        let new_pheromone:f32 = (1.0 - self.rho) * old_pheromone + (self.rho * self.tau);
        if let Some(to_city) = pher_graph.get_mut(&self.current_node.name) {
            to_city.insert(city_name, new_pheromone);
        }
    }

    
    pub fn goes_on_tour(&mut self) -> Vec<City>{
        // Ant traverses the entire graph adding pheromone to every visited node
        while self.visited_nodes.len() != self.cities_list.len() {
            let unvisited:Vec<&City> = self.get_unvisited_cities();
            let chosen_node:i32 = self.choose_node(unvisited);
            self.local_pheromone_update(chosen_node);
            self.visit_city(chosen_node);
            }
        // Close the loop -> connect first and last city
        let start_node:i32 = self.visited_nodes.first().unwrap().name;
        self.local_pheromone_update(start_node);
        return self.visited_nodes.clone();
    }

} 
