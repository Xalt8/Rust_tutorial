
from math import sqrt
import numpy as np
import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation
from celluloid import Camera
import ant


def calculate_distance(coords1:tuple, coords2:tuple) -> float:
    ''' Calculates the Euclidean distance between 2 points '''
    return sqrt((coords1[0] - coords2[0])**2 + (coords1[1] - coords2[1])**2)



def k_means_clustering(k:int, cities:list) -> list:
    ''' Takes a list of cities (coordinate tuples) and divides the 
        cities into "k" number of sub-lists '''
    # Choose initial centroids
    centroids = [cities[0], cities[1]]

    while True:
        # Calculate the distance for every city and centroid
        distances = [[calculate_distance(city, centroid) for city in cities] for centroid in centroids]
        # Get the index value of the closest distance
        closest_centroid_index = np.argmin(distances, axis=0)
        # Create k number of clusters and use the index to append to cluster 
        clusters = [[] for _ in range(k)]
        for city, centroid_index in zip(cities, closest_centroid_index):
            clusters[centroid_index].append(city)
        
        # Calculate the new centroids 
        new_centroids = []
        for i, cluster in enumerate(clusters):
            if len(cluster) > 0:
                new_centroids.append(tuple(np.array(cluster).mean(axis=0)))
            else:
                new_centroids.append(centroids[i])

        # Check if centroids have changed
        if new_centroids == centroids:
            break
        else:
            centroids = new_centroids
    
    return clusters


def get_connected_cities_indicies(city_index:int, cities:list[ant.City]) -> np.ndarray:
    assert city_index < len(cities), "The city_index is out of bounds"
    city_indicies = np.arange(len(cities), dtype=np.int32)
    return city_indicies[city_indicies != city_index]


def plot_graph(cities:list[ant.City], pher_graph:np.ndarray):
    fig = plt.figure(figsize=(8, 5))
    camera = Camera(fig)

    ax = plt.axes(xlim=(0, max(city.x for city in cities) + 10), ylim=(0, max(city.y for city in cities) + 10))
    ax.set_title("Pheromone graph")

    for i in range(10):
        ax.text(x=10, y=42, s="Iteration {}".format(i))

        for i, city in enumerate(cities):
            ax.text(x=city.x, y=city.y, s=i + 1, bbox=dict(boxstyle='circle', facecolor='pink', edgecolor='blue'))
            
        random_pher_graph = np.random.rand(*pher_graph.shape)
        for from_city_idx, _ in enumerate(cities):
            for to_city_idx in get_connected_cities_indicies(city_index=from_city_idx, cities=cities):
        
                ax.plot(
                    [cities[from_city_idx].x, cities[to_city_idx].x],
                    [cities[from_city_idx].y, cities[to_city_idx].y],
                    linewidth=random_pher_graph[from_city_idx][to_city_idx],
                    color='red',
                    alpha=0.4)
        camera.snap()
    
    animation = camera.animate(interval=500, blit=True, repeat=False)
    plt.show()
    # animation.save('graph_animation.mp4')


def plot_csv(file_name:str, cities_list:list[ant.City]) -> plt.Axes:
    ''' Takes a file name and list of cities and plots the pheromone graph from file'''
    loaded_blob = np.loadtxt(file_name, delimiter=',')
    reshaped_blob = loaded_blob.reshape((-1, len(cities_list), len(cities_list)))
    
    fig = plt.figure(figsize=(8, 5))
    camera = Camera(fig)
    ax = plt.axes(xlim=(0, max(city.x for city in cities_list) + 5), ylim=(0, max(city.y for city in cities_list) + 5))
    ax.set_title("Pheromone graph")

    for j, graph in enumerate(reshaped_blob):
        ax.text(x=10, y=38, s="Iteration {}".format(j))
        for from_city_idx, from_city in enumerate(cities_list):
            for to_city_idx in get_connected_cities_indicies(city_index=from_city_idx, cities=cities):
                ax.plot(
                    [cities_list[from_city_idx].x, cities_list[to_city_idx].x],
                    [cities_list[from_city_idx].y, cities_list[to_city_idx].y],
                    linewidth=graph[from_city_idx][to_city_idx], color='red', alpha=0.4)
                ax.text(x=from_city.x, y=from_city.y, s=from_city_idx + 1, bbox=dict(boxstyle='circle', facecolor='pink', edgecolor='blue'))
        camera.snap()

    shortest_tour = np.arange(len(cities))
    for from_city_idx2, to_city_idx2 in zip(shortest_tour, shortest_tour[1:] + shortest_tour[:1]):
        ax.plot([cities_list[from_city_idx2].x, cities_list[to_city_idx2].x],
                [cities_list[from_city_idx2].y, cities_list[to_city_idx2].y],
                linestyle='dashed', linewidth=2, color='royalblue')
        for from_city_idx, from_city in enumerate(cities_list):
            for to_city_idx in get_connected_cities_indicies(city_index=from_city_idx, cities=cities):
                ax.plot(
                    [cities_list[from_city_idx].x, cities_list[to_city_idx].x],
                    [cities_list[from_city_idx].y, cities_list[to_city_idx].y],
                    linewidth=reshaped_blob[-1][from_city_idx][to_city_idx], color='red', alpha=0.4)
        
    camera.snap()
    animation = camera.animate(interval=200, blit=True, repeat=False)
    plt.show()
    




if __name__ == "__main__":
    # cities = [(1,1), (5,6), (2,1), (4,3), (5,4)]
    # print(f"\ncities -> {cities}\n")
    # clusters = k_means_clustering(k=2, cities=cities)
    # print(f"clusters -> {clusters}")
    
    city1 = ant.City(name='1', x=5, y=10)
    city2 = ant.City(name='2', x=5, y=25)
    city3 = ant.City(name='3', x=15, y=30)
    city4 = ant.City(name='4', x=10, y=35)

    cities = [city1, city2, city3, city4]
    pher_graph = np.zeros(shape=(len(cities), len(cities)))
    
    # for _ in range(10):
    #     with open('pher_graph.csv','a') as file:
    #         np.savetxt(file, pher_graph, delimiter=",")
    #         pher_graph = np.random.rand(*pher_graph.shape) 

    plot_csv(file_name='pher_graph.csv', cities_list=cities)
    # plot_graph(cities=cities, pher_graph=pher_graph)




