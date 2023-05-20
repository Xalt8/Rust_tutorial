import numpy as np
from ant import City, calculate_distance
import time
import cProfile

def score_city(from_city_idx:int, 
               to_city_idx:int, 
               distance_graph:np.ndarray, 
               pheromone_graph:np.ndarray, 
               beta:float = 0.2) -> float:
    return pheromone_graph[from_city_idx][to_city_idx] * (1/distance_graph[from_city_idx][to_city_idx])**beta    


def make_tour(cities_list:list[City], dist_graph:np.ndarray, pher_graph:np.ndarray, q0=0.90) -> np.ndarray:
    tour = np.full(shape=len(cities_list), fill_value=999999, dtype=np.int32)
    cities_idx = np.arange(len(cities_list))
    start_city_indx = np.random.choice(cities_idx)
    tour[0] = start_city_indx
    for i in np.arange(start=1, stop=tour.size):
        # unvisited_idx = np.setdiff1d(cities_idx, tour)
        unvisited_idx = np.array(list((set(cities_idx) - set(tour))))
        scores = score_city(from_city_idx = tour[i-1], to_city_idx = unvisited_idx, 
                            distance_graph = dist_graph, pheromone_graph = pher_graph)
        assert unvisited_idx.size == scores.size, "unvisited_idx and scores are not the same size"

        if np.random.random() < q0:
            tour[i] = unvisited_idx[np.argmax(scores)]
        else:
            prob_dist = scores/np.sum(scores)
            chosen_city_index = int(np.random.choice(a=unvisited_idx, size=1, p=prob_dist))
            tour[i] = chosen_city_index
    # assert np.setdiff1d(tour, cities_idx).size == 0, "tour and city indices are different"
    return tour


def local_pheromone_update(tour:np.ndarray, 
                           pher_graph:np.ndarray, 
                           cities_list:list[City], 
                           rho:float=0.1, 
                           tau:float=0.0005) -> np.ndarray :
    ''' Returns the pheromone graph after appying the local pheromone update rule to the pheromone graph'''
    for from_city_idx in tour:
        for to_city_idx in get_connected_cities_indicies(city_index=from_city_idx, cities=cities_list):
            pher_graph[from_city_idx][to_city_idx] = (1 - rho) * pher_graph[from_city_idx][to_city_idx] + (rho * tau)
    return pher_graph


def global_pheromone_update(best_tour:np.ndarray, 
                            best_path_distance:float,
                            pher_graph:np.ndarray, 
                            cities_list:list[City], 
                            alpha:float=0.1) -> np.ndarray:
    """ Returns the pheromone graph after applying the global pheromone update to it """
    for from_city_idx in best_tour:
        for to_city_idx in get_connected_cities_indicies(city_index=from_city_idx, cities_list=cities_list):
            pher_graph[from_city_idx][to_city_idx] = \
                (1-alpha) * pher_graph[from_city_idx][to_city_idx] + alpha * (best_path_distance ** -1)
    return pher_graph


def get_tour_distance(tour:np.ndarray, cities_list:list[City]) -> float:
    ''' Takes a tour of index values and returns the distance travelled'''
    return np.sum([calculate_distance(cities_list[from_city_idx], cities_list[to_city_idx]) 
                   for from_city_idx, to_city_idx in zip(tour, np.roll(a=tour, shift=-1))])


def optimize(cities_list:list[City],
             shortest_path:np.ndarray, 
             pher_graph:np.ndarray, 
             dist_graph:np.ndarray,
             iterations:int,
             num_ants:int) -> np.ndarray:

    best_tour = np.full(shape=len(cities_list), fill_value=np.nan)
    best_tour_distance:float = np.Infinity

    for i in range(iterations):
        print(f"Iteration -> {i}, best_tour_distance -> {best_tour_distance:.2f}")

        if round(best_tour_distance,2) == round(get_tour_distance(tour=shortest_path, cities_list=cities_list),2):
            print(f"\nFound shortest path at {i} iteration")
            break

        tours = np.array([make_tour(cities_list=cities_list, dist_graph=dist_graph, pher_graph=pher_graph)
                        for _ in range(num_ants)])
        for tour in tours:
            pher_graph = local_pheromone_update(tour= tour, pher_graph= pher_graph, cities_list= cities_list)
            tour_distance = get_tour_distance(tour=tour, cities_list=cities_list)
            if tour_distance < best_tour_distance:
                best_tour = tour
                best_tour_distance = tour_distance
    
    return best_tour


def get_connected_cities_indicies(city_index:int, cities:list[City]) -> np.ndarray:
    """ Returns the index values of the neighbouring and connected cities to a given city_index """
    assert city_index < len(cities), "The city_index is out of bounds"
    city_indicies = np.arange(len(cities), dtype=np.int32)
    return city_indicies[city_indicies != city_index]


def get_pheromone_graph(cities_list:list, initial_pheromone:float) -> np.ndarray:
    ''' Returns a matrix (from_city_idx, to_city_idx) and initial pheromone as values'''
    n = len(cities_list)
    pheromone_graph = np.zeros(shape=(n,n))
    for from_city_idx, _ in enumerate(cities_list):
        for to_city_idx in get_connected_cities_indicies(city_index=from_city_idx, cities=cities_list):
            pheromone_graph[from_city_idx][to_city_idx]=initial_pheromone
    return pheromone_graph


def get_distance_graph(cities_list:list) -> np.ndarray:
    ''' Returns a matrix (from_city_idx, to_city_idx) and distance as values'''
    n = len(cities_list)
    distance_graph = np.zeros(shape=(n,n))
    for from_city_idx, from_city in enumerate(cities_list):
        for to_city_idx in get_connected_cities_indicies(city_index=from_city_idx, cities=cities_list):
            distance_graph[from_city_idx][to_city_idx]=calculate_distance(city1=from_city, city2=cities_list[to_city_idx])
    return distance_graph


def main():
    with open("coordinates.txt", 'r') as f:
        lines = [line.strip().split(", ") for line in f]
    CITIES = [City(str(name), int(x), int(y)) for name, (x, y) in enumerate(iterable=lines, start=1)]

    with open("shortest_path.txt", 'r', encoding='utf8') as f:
        shortest_lines = f.readline()
    # SHORT_PATH_CITIES = [City(name=city_name, x=city.x, y=city.y) for city_name in shortest_lines.split(" ")
    #                         for city in CITIES 
    #                         if city.name == city_name]
    short_tour = np.array([int(line)-1 for line in shortest_lines.split(" ")], dtype=np.int32)

    pheromone_graph = get_pheromone_graph(cities_list=CITIES, initial_pheromone=0.0005)
    distance_graph = get_distance_graph(cities_list=CITIES)

    best_tour = optimize(cities_list=CITIES, 
                         shortest_path=short_tour, 
                         pher_graph=pheromone_graph, 
                         dist_graph=distance_graph, 
                         iterations=200, 
                         num_ants=10)


if __name__ == "__main__":

    start_time = time.perf_counter()

    main()

    end_time = time.perf_counter()
    print(f'\nTime taken: {end_time-start_time:.2f} seconds\n')
    
    # cProfile.run('main()', sort='cumtime')
    