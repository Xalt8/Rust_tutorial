
from math import sqrt
import numpy as np
import matplotlib.pyplot as plt


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


def plot_tour(tour:list) -> plt.Axes:
    
    fig = plt.figure(figsize=(8,5))
    for i, city in enumerate(tour,1):
        plt.text(x=city[0], y=city[1], s=str(i), color='red', size=10,
                 bbox=dict(boxstyle="circle", facecolor='lightblue', edgecolor='blue'))
    
    for from_city, to_city in zip(tour, tour[1:] + tour[:1]):
        plt.plot([from_city[0], to_city[0]], [from_city[1], to_city[1]], color='green', linestyle='-')

    short_path = [(1,1), (2,1), (4,3), (5,4), (5,6)]
    for from_city, to_city in zip(short_path, short_path[1:] + short_path[:1]):
        plt.plot([from_city[0], to_city[0]], [from_city[1], to_city[1]], color='red', linestyle='-', alpha=0.2, linewidth=6)

    plt.tight_layout()
    plt.show()

if __name__ == "__main__":
    cities = [(1,1), (5,6), (2,1), (4,3), (5,4)]
    print(f"\ncities -> {cities}\n")
    clusters = k_means_clustering(k=2, cities=cities)
    print(f"clusters -> {clusters}")
    plot_tour(cities)