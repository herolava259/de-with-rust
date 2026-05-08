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
  - Local Clustering Coefficient 
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

## **Leiden**

- is an alg for detecting communities in large networks. The alg separaes nodes into disjoint communities so as to maximize a modularity score for each community. Modularity quantifies the quality of an assignment of nodes to communities, that is how densely connected nodes in a community are, compared to how connected they would be in a random network.

- The Leiden alg is a hierarchical clustering algorithm, that  recursively merges communities into single nodes by greedily optimizing the modularity and the process repeats in the condensed graph. It modifies Louvain alg to address some of its shortcomings, namely the case where some of the communities found by Louvain alg are not well-connected. This is achieved by periodically randomly breaking down communities into smaller well-connected ones.

## **Local Clustering Coefficient**

- computes the local clustering coefficient for each node in the graph.
- The local clustering coefficient $C_n$ of a node $n$ describes the likelihood that the neighbours of $n$ are also connected.
- To compute $C_n$ we use the number of triangles as a node is a part of $T_n$ and the degree of the node $d_n$.   
- To compute the local clustering coefficient is as follows:

$$
C_n = \frac{2T_n}{d_n(d_n-1)}
$$

- As we can see the triangle count is required to compute the local clustering coefficient. To do this the **Triangle Count** alg is utilised.

## **Louvain**

- The Louvain method is an algorithm to detect communities in large networks. It maximizes a modularity score for each community, where the modularity quantifies the quality of an assignment of nodes to communities. This mean evaluating how much more densely connected the nodes within a community are, compared to how connected they would be in a ramdom network. 

- The Louvain alg is a hierarchical clustering algorithm, that recursively merges communities into a single node and executes the modularity clustering on the condensed graphs. 

## **Modularity Metric**

- Modularity is a metric that allows you to evaluate the quality of a community detection. Relationships of nodes in a community $C$ connect to nodes either within $C$ or outside $C$. Graphs with high modularity have dense connections between the nodes within communities but sparse connections between nodes in different communities.

## **Modularity Optimization**

- tries to detect communities in the graph based on their modularity. *Modularity* is a measure of the structure of a graph, measuring the density of connections within a module or community. Graphs with a high modularity score will have many connections within a community but only few pointing outwards to other communities. The alg will explore for every node if its modularity score might increase if changes its community to one of its neighboring nodes.

## Strongly Connected Components

- finds maximal sets of connected nodes in a directected graph. 
- A set is considered a strongly connected component if there is a directed path between each pair of nodes within the set. It is often used early in a graph analysis process to help us get an idea of how our graphs is structured. 

### Use cases:

- In the analysis of powerful transnational corporations, SCC can be used to find the set of firms in which every member owns directly and/or indirectly owns shares in every other member.

- SCC can be used to compute the connectivity of different network configurations when measuring routing performance in multihop wireless networks.

- Strongly Connected Components algs can be used as a first step graph-algs that work only strongly connected graph. In social networks, a group of people are generally strongly connected. Many people in these groups generally like some common pages, or play common games. The SCC algorithm can be used to find such groups and suggest the commonly liked pages or games to the people in the group who have not yet liked those pages or games.

## Triangle Count

- counts the number of triangles for each node in the graph. A triangle is a set of three nodes where each node has a relationship to the other two. In graph theory terminology, this is sometimes referred to as a 3-clique. The Triangle Count algorithm in the GDS library only finds triangles in undirected graphs.

- Triangle counting has gained popularity in social network analysis, where it is used to detect communities and measure the cohesiveness of those communities. It can also be used to determine the stability of a graph, and is often used as part of the computation of network indicies such as clustering coefficients. The Triangle Count algorithm is also used to compute the `Local Clustering Coefficient`.

- For some use-cases it is useful to only count triangles that contain nodes with specific node labels. This is also something that can save computation time, as the algorithm will not have to consider nodes that do not have the specified labels.

- The algorithm supports node label filtering by allowing you to specify a list of up to three node labels in the configuration. If three labels are specified, the algorithm will only count triangles that contain nodes with all three labels, and each label must be represented by at least one node in the triangle. if fewer than three labels are specified, the alg will count triangles where each specified label is represented by at least one node in the triangle, but nodes with no specified label can also be part of the triangle. If a label is repeated in the list, it will represented by as many nodes as times it appear in the list.