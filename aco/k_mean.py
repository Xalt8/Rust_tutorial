
from math import sqrt
import numpy as np


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



if __name__ == "__main__":
    cities = [(1,1), (2,1), (4,3), (5,4)]
    print(f"\ncities -> {cities}\n")
    clusters = k_means_clustering(k=2, cities=cities)
    print(f"clusters -> {clusters}")