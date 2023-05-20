from dataclasses import dataclass, field
import numpy as np
import matplotlib.pyplot as plt
import matplotlib.lines as mlines
import time
import cProfile


cityName = str
Graph = dict[cityName, dict[cityName, float]]


@dataclass(frozen=True)
class City:
    name: str
    x: int
    y: int

    def __hash__(self):
        return hash((self.name, self.x, self.y))


@dataclass
class Ant:
    cities_list: field(default_factory=list, init=False)
    pheromone_graph: field(default_factory=Graph, init=False)
    distance_graph: field(default_factory=Graph, init=False)
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
            # unvisited_cities:list[City] = [city for city in self.cities_list if city not in visited_cities]
            unvisited_cities = list(set(self.cities_list) - set(visited_cities))
            scores = np.array([self.score_city(from_city = visited_cities[-1], to_city = city) 
                                           for city in unvisited_cities], dtype=np.float64)
            assert len(unvisited_cities) == len(scores), "scores & unvisited_cities are not the same length"

            q = np.random.random()
            if q < self.q0:
                visited_cities.append(unvisited_cities[np.argmax(scores)])
            else:
                prob_dist = scores/np.sum(scores)
                chosen_city_index = int(np.random.choice(a=len(unvisited_cities), size=1, p=prob_dist))
                visited_cities.append(unvisited_cities[chosen_city_index])
        return visited_cities
    
    
    def local_pheromone_update(self, tour:list[City]) -> None :
        tour_tuples:list[(City, City)] = get_tour_tuples(tour)
        for from_city, to_city in tour_tuples:
            self.pheromone_graph[from_city.name][to_city.name] = \
            (1 - self.rho) * self.pheromone_graph[from_city.name][to_city.name] + (self.rho * self.tau)


@dataclass
class ACO:
    cities_list: list[City]
    pheromone_graph: Graph
    distance_graph: Graph
    iterations: int
    num_ants: int
    best_path:list[City] = field(default_factory=list)
    best_path_distance:float = np.Infinity
    alpha: float = 0.1


    def global_update_pheromone(self) -> None:
        """ Applies global pheromone update rule to pheromone graph"""
        tour_tuples:list[(City, City)] = get_tour_tuples(self.best_path)
        for from_city, to_city in tour_tuples:
            self.pheromone_graph[from_city.name][to_city.name] = \
            (1-self.alpha) * self.pheromone_graph[from_city.name][to_city.name] + self.alpha * (self.best_path_distance ** -1)


    def optimize(self, shorest_path:list[City]):
        
        for i in range(self.iterations):
            print(f"Iteration -> {i}, best distance found -> {round(self.best_path_distance,2)}")

            shortest_path_distance:float = get_tour_length(shorest_path)
            if round(shortest_path_distance,2) == round(self.best_path_distance,2):
                print(f"\nShortest path found at {i} iteration")
                break

            ants = [Ant(cities_list=self.cities_list, pheromone_graph=self.pheromone_graph, distance_graph=self.distance_graph) 
                    for _ in range(self.num_ants)]
            tours:list[list[City]] = [ant.make_tour() for ant in ants]
            for ant, tour in zip(ants, tours):
                ant.local_pheromone_update(tour)
            
            tour_distances:list[float] = [get_tour_length(tour) for tour in tours]
            for tour, tour_distance in zip(tours, tour_distances):
                if tour_distance < self.best_path_distance:
                    self.best_path_distance = tour_distance
                    self.best_path = tour
            
            self.global_update_pheromone()


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


def get_tour_tuples(tour:list[City]) -> list[(City, City)]:
    """ Takes a tour and returns a list of city tuples (from_city, to_city)
        including the last and first cities """
    
    return [(from_city, to_city) for (from_city, to_city) in zip(tour, tour[1:] + tour[:1])]


def get_tour_length(tour:list[City]) -> float:
    """ Takes a list of Cities and returns the distance travelled for that tour"""
    tour_tuples:list[(City, City)] = get_tour_tuples(tour)
    tour_length:float = sum([calculate_distance(from_city, to_city) for from_city, to_city in tour_tuples])
    return tour_length


def plot_tour(tour:list[City], short_path:list[City]) -> plt.Axes:
    
    _, ax = plt.subplots(1,1, figsize=(8,5))
    
    # Plot the cities
    for city in tour:
        ax.text(city.x, city.y, s=city.name, bbox=dict(boxstyle="circle", facecolor='lightblue', edgecolor='blue'))
    
    # Plot the shortest path
    short_path_line_format = {'color':'red', 'linestyle':'-', 'alpha':0.2, 'linewidth':6, 'label':"Shortest path"}
    for from_city, to_city in get_tour_tuples(short_path):
        ax.plot([from_city.x, to_city.x], [from_city.y, to_city.y], **short_path_line_format)
    
    # Plot the tour -> best path found
    best_path_line_format = {'color':'blue', 'linestyle':'--', 'label':'Best path found'}
    for from_city, to_city in get_tour_tuples(tour):
        ax.plot([from_city.x, to_city.x], [from_city.y, to_city.y], **best_path_line_format)
    
    short_path_legend_handle = mlines.Line2D([], [], **short_path_line_format)
    tour_path_legend_handle = mlines.Line2D([], [], **best_path_line_format)

    ax.text(1, 10, f"Shortest path distance: {get_tour_length(short_path):.2f}\nBest path distance: {get_tour_length(tour):.2f}", 
            verticalalignment='top', bbox={'boxstyle':'round', 'facecolor':'wheat', 'alpha':0.4})

    plt.xlim(0, max([city.x for city in tour]) +10)
    plt.ylim(0, max([city.y for city in tour]) +10)
    plt.legend(handles=[short_path_legend_handle, tour_path_legend_handle])
    plt.title("ACO")
    plt.show()


if __name__ == '__main__':

    start_time = time.perf_counter()

    with open("coordinates.txt", 'r') as f:
        lines = [line.strip().split(", ") for line in f]
    CITIES = [City(str(name), int(x), int(y)) for name, (x, y) in enumerate(iterable=lines, start=1)]
    print(f"\nintial_cities_distance -> {get_tour_length(CITIES)}\n")

    with open("shortest_path.txt", 'r', encoding='utf8') as f:
        shortest_lines = f.readline()
        
    SHORT_PATH_CITIES = [City(name=city_name, x=city.x, y=city.y) for city_name in shortest_lines.split(" ")
                           for city in CITIES 
                           if city.name == city_name]
    print(f"shortest_path_distance -> {get_tour_length(SHORT_PATH_CITIES)}\n")
    
    pheromone_graph = get_pheromone_graph(cities_list=CITIES, initial_pheromone=0.0005)
    distance_graph = get_distance_graph(CITIES)
    
    aco = ACO(cities_list=CITIES, pheromone_graph=pheromone_graph, distance_graph=distance_graph, iterations=200, num_ants=10)
    aco.optimize(SHORT_PATH_CITIES)
    
    end_time = time.perf_counter()
    print(f'\nTime taken: {end_time-start_time:.2f} seconds\n')
    plot_tour(tour=aco.best_path, short_path=SHORT_PATH_CITIES)

    # cProfile.run('aco.optimize(SHORT_PATH_CITIES)', sort='cumtime')
    
    
    