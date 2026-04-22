# Refection Questions 

## 1. How does the Kosaraju's algorithm works to detect strongly connected components in a directed graph?

## 2. How are Twitter users and their interactions represented in the graph?

## 3. What is the significance of the strongly connected components in the context of social network analysis?


# Challenge Questions 

## 1. Expand the dataset and analyze the change in the structure and size of detected communities.

## 2. Modify the program to detect and print the largest community.

## 3. Can you think of other real-life  applications for community detection algorithms? 

# Community Detection with Neo4j

## Introduction

- Community detection algorithm are used to evaluate how groups of nodes are clustered or partitioned, as well as their tendency to strengthen or break apart. The Neo4j GDS library includes the following community detection algorithms, grouped by quality tier:
  - Clique counting
  - Conductance metric
  - HDBSCAN 
  - K-Core Decomposition
  - K-1 Coloring
  - K-Means Clustering
  - Label Propagation 
  - Leiden 
  - Local Clustering COefficient 
  - Louvain 
  - Modularity metric
  - Modularity Optimization
  - Strongly Connected Components
  - Triangle Count 
  - Weakly Connected Components
  - Approximate Maximum k-cut
  - Speak-Listener Label Propagation 

## Clique Counting

> Terms:
> \* A clique is a subset of nodes where all nodes are connected to each other, sometime called a complete subgraph. 

- The clique algorithm counts the number of cliques of various sizes in the graph. 

- Counting the number of cliques in the graph, can give an understanding for the topology and clustering of the graph in its entirety as well as for individual nodes. The algorithm counts only cliques of size 3 or larger, since cliques of sizes 1 and 2 are trivial. 
- [reference](https://neo4j.com/docs/graph-data-science/current/algorithms/clique-counting/)

## Condutance metric

> `Terms`:
> Condutance is a metric allows you to evaluate the quality of a community detection. Relationships of nodes a community **C** connecte to nodes either **C** or outside **C**. The conductance is the ratio between relationships that point outside **C** and the total number of relationships of **C**. The lower the conductance, the more "well-knit" a community is. 

- The alg runs in time linear to the number of relationships in the graph.

- [reference](https://neo4j.com/docs/graph-data-science/current/algorithms/conductance/)

## *HDBSCAN*

> `Terms`:
> which stands for *Hierarchical Density-Based Spatial Clustering of Applications with Noise*
> is a clustering alg used to identify clusters of similar data points within a dataset. It builds upon the **DBSCAN** alg but adds hierarchical structure, making it more robust to varying densities within the data. 

- Unlike DBSCAN, HDBSCAN does not require tuning a specific density parameter; instead, it runs over a range of parameters, creating a hierachy of clusters. This hierachical approach allows **HDBSCAN** to find clusters of varying densities and to be more adaptable to real-world data. 

- HDBSCAN is known for its ease of use, noise tolerance, and ability to handle data with varying densities, making it a versatile tool for clustering tasks, especially when dealing with complex, high-dimensional datasets.

- [reference](https://neo4j.com/docs/graph-data-science/current/algorithms/hdbscan/)

- [paper](https://arxiv.org/pdf/1602.03730)


## **K-Core Decomposition**

> The alg constitutes a process of separates the nodes in a graph into groups based on the degree sequence and topology of the graph.

- The term `i-core` refers to a maximal subgraph of the original graph such that each node in this subgraph has degree at least `i`. The maximility ensures that it is not possible to find another subgraph with more nodes where this degree property holds.

- i-core subgraph belong to j-core subgraph if `j < i`

- Std alg for K-core decomposition iteratively remove the node of lwest degree until the graph becomes empty. When a node is removed from the graph, all of its relationships are removed, and the degree of its neighbors is reduced by one. With this approach, the different core groups are discorvered one-by-one.


## **K-1 Coloring**

> The K-1 Coloring alg assigns a color to every node in the graph, trying to optimize  for two objectives:
  > 1. To make sure that every neighbor of a given node has a different color than the node itself.
  > 2. To use as few colors as possible.

## **Label Propagation**

- The Label Propagation algorithm (LPA) is a fast algorithm for finding communities in a graph. It detects these communities using network structure alone as its guide, and doesn't require a pre-defined objective function or prior information about the communities.

- LPA works by propagating labels throughout the network and forming communities based on this process of label propagation.

- The intuition behind the algorithm is that a single label can quickly become dominant in a densely connected group of nodes, but will have trouble crossing a apresely connected region. Labels will get trapped inside a densely connected group of nodes, and those nodes that end up with the same label when the algorithms finish can be considered part of the same community.

- The algorithm works as follows:

  - Every node is initialized with a unique community label (an identifier).
  - These labels propagate through the network.
  - At every iteration of propagation, each node updates its label to the one that maximum numbers of its neighbours belongs to. Ties are broken arbitrarily but deterministically.
  - LPA reaches convergence when each node has the majority label of its neighbours.
  - LPA stops if either convergence, or the user-defined maximum number of iterations is achieved.
- As label propagate, densely connected groups of nodes quickly reach a consensus on a unique label. At the end of the propagation only a few labels will remain-most will have disappeared. Nodes that have the same community label at convergence are said to belong to the same community.

- One interesting feature of LPA is that nodes can be assigned preliminary labels to narrow down the range of solution genreated. This means that it can be used as semi-supervised way of finding communities where we hand-pick some initial communities.