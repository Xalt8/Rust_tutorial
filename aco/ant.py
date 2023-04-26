from dataclasses import dataclass, field
import numpy as np

cityName = str
Graph = dict[cityName, dict[cityName, float]]

@dataclass
class City:
    name: str
    x: int
    y: int


@dataclass
class Ant:
    cities_list: field(default_factory=list, init=False)
    pheromone_graph: field(default_factory=dict, init=False)
    distance_graph: field(default_factory=dict, init=False)
    beta = 2.0
    q0 = 0.9
    rho = 0.1
    tau = 0.0005


    def score_city(self, from_city:City, to_city:City) -> float:
        return self.pheromone_graph[from_city.name][to_city.name] * \
                    (1/self.distance_graph[from_city.name][to_city.name])**self.beta    


    def make_tour(self) -> list[City]:
        visited_cities = []
        start_city = self.cities_list[np.random.randint(0, len(self.cities_list)-1)]
        visited_cities.append(start_city)
        while len(visited_cities) != len(self.cities_list):
            unvisited_cities:list[City] = [city for city in self.cities_list if city not in visited_cities]
            scores = np.array([self.score_city(from_city = visited_cities[-1], to_city = city) 
                                           for city in unvisited_cities], dtype=np.float64)
            assert(len(unvisited_cities) == len(scores), "scores & unvisited_cities are not the same length")

            q = np.random()
            if q < self.q0:
                visited_cities.append(unvisited_cities[np.argmax(scores)])
            else:
                prob_dist = scores/np.sum(scores)
                chosen_city = int(np.random.choice(a=unvisited_cities, size=1, p=prob_dist))
                visited_cities.append(chosen_city)
        return visited_cities
    
    
    def local_pheromone_update(self, tour:list[City]) -> None :
        tour_tuples:list[(City, City)] = [(from_city, to_city) for from_city, to_city in 
                                          zip(self.cities_list[0:-1], self.cities_list[1:])]
        tour_tuples.append((self.cities_list[-1], self.cities_list[0])) # Close the loop
        for from_city, to_city in tour_tuples:
            self.pheromone_graph[from_city.name][to_city.name] = \
            (1 - self.rho) * self.pheromone_graph[from_city.name][from_city.name] + (self.rho * self.tau)


@dataclass
class ACO:
    best_path: list[City]
    best_path_distance: float
    alpha: float
    iteationas: int
    num_ants: int
    cities_list: list[City]
    pheromone_graph: Graph
    distance_graph: Graph

    def optimize(self) :
        ants = [Ant(cities_list=self.cities_list, pheromone_graph=self.pheromone_graph, distance_graph=self.distance_graph) 
                for _ in range(self.num_ants)]
        tours:list[list[City]] = [ant.make_tour() for ant in ants]
        for ant, tour in zip(ants, tours):
            ant.local_pheromone_update(tour)
        

with open("coordinates.txt", 'r') as f:
    lines = [line.strip().split(", ") for line in f]
    
CITIES = [City(str(name), int(x), int(y)) for name, (x, y) in enumerate(lines, start=1) ]


def get_connected_cities(city_name:str, cities_list:list[City]) -> list[City] :
    ''' Takes a city name and returns its connected neighbours
        assuming all cities are connected '''
    return [city for city in cities_list if city.name!= city_name]
    

def get_pheromone_graph(cities_list:list, initial_pheromone:float) -> Graph:
    ''' Creates a dict with the from_city as a key and to_city and pheromone as values'''
    pheromone_graph = dict()
    for from_city in cities_list:
        pheromone_graph[from_city.name] = dict()
        for to_city in get_connected_cities(from_city.name, cities_list=cities_list):
            pheromone_graph[from_city.name][to_city.name]=initial_pheromone
    return pheromone_graph


def calculate_distance(city1:City, city2:City)-> float:
    ''' Takes 2 cities and returns the distance between them'''
    return round(np.sqrt(np.abs(city1.x - city2.x)**2 + np.abs(city1.y - city2.y)**2),2)


def get_distance_graph(cities_list:list) -> Graph:
    ''' Creates a dict with the from_city as a key and to_city and distance as values'''
    distance_graph = dict()
    for from_city in cities_list:
        distance_graph[from_city.name] = dict()
        for to_city in get_connected_cities(from_city.name, cities_list=cities_list):
            distance_graph[from_city.name][to_city.name]=calculate_distance(city1=from_city, city2=to_city)
    return distance_graph



if __name__ == '__main__':
    print(f"cities -> {CITIES}\n")
    pheromone_graph = get_pheromone_graph(cities_list=CITIES, initial_pheromone=0.0005)
    print(f"\npheromone_graph 1 -> {pheromone_graph['1']}\n")
    distance_graph = get_distance_graph(CITIES)
    print(f"distance_graph 1 -> {distance_graph['1']}\n")

    tour_tuples:list[(City, City)] = [(from_city, to_city) for from_city, to_city in 
                                          zip(CITIES[0:-1], CITIES[1:])]
    tour_tuples.append((CITIES[-1], CITIES[0])) # Close the loop

    print(tour_tuples)

    tour_tuples2 = [(from_city, to_city) for from_city, to_city in zip(CITIES, CITIES[1:] + CITIES[:1])]

    print(tour_tuples == tour_tuples2)

    print([city.name for city in CITIES[:1]])