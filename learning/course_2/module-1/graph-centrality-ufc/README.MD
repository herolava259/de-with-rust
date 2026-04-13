## `Reflection Questions`

### 1. What is the 'closeness centrality' in the context of this program, and how is it calculated?

- Closeness centrality is a way of detecting nodes that are able to spread information very efficiently through a graph 

- The closeness centrality os a node measures its average farness (inverse distance) to all other nodes. Node with a high closeness score have the shortest distances to all other noders.

- For each node u, the **Closeness Centrality** algorithm calculates the sum of its distances to all other nodes, based on calculating the shortest paths between all pairs of nodes. The resulting sum is then inverted to determine the closeness centrality score for that node. 

> *raw closeness centrality* : 
> `raw-closeness-centrality(u) = 1 / sum(distance-from-u-to-all-other-nodes)`

> *normalized closeness centrality*: 
> `normalized-closeness-centrality(u) = (number-of-nodes -1) / sum(distance-from-u-to-all-other-nodes) `

> *dealing with unconnected graphs from Wassweman and Faust*
> `Wasserman-Faust normalized closeness centrality(u) = (n-1)^2 / ( (number-of-nodes - 1) * sum(dist-from-u-to-all-other-nodes))

### 2. How does the add_edge function work, and why do you need to pass in an array of NodeIndex?

### 3. Why do we calculate the  fegree of a node by counting its outgoing edges?

## `Challenge Questions:`

### 1. How would you modify the program to also calculate and display the betweeness centrality of each fighter?

### a. What is betweeness-centrality? 
- Betweeness centrality is a way of detecting the amount of influence a node has over the flow of information in a graph. It is often used to find nodes that serve as a bridge from one part of a graph to another.
- The algorithm calculates shortest paths between all pairs of nodes in a graph. Each node receives a score, based on the number of shortest patsh that pass through the node. Nodes that more frequently lie on shortest paths between other nodes will have higher betweeness centrality scores.
$$
c_B(v) = \sum_{s,t \in V} \frac{\sigma(s, t | v)}{\sigma(s, t)}
$$

>- where $V$ is the set of nodes, $\sigma(s, t)$ is the number of shortest $(s, t)$-paths, and $\sigma(s, t|v)$ is the number of those paths passing through some node $v$ other than $s$ and $t$. if $s=t,\,\sigma(s,t)=1$, and if $v \in {s, t},\, \sigma(s,t|v)=0$. The denominator $\sigma(s,t)$ is a normalization factor that can be turned off to get the raw path counts.


### b. How?
- Step 1: Use `pet-graph lib in rust` to calculate shortest path of all pairs of node in the graph. Specially one use `dijkstra or bellman_forn` algorithm to calculate them. 
- Step 2: Use the formula above to calculate betweeness centrality each node in the graph

### c. References

- wiki: [from-network-x](https://networkx.org/documentation/stable/reference/algorithms/generated/networkx.algorithms.centrality.betweenness_centrality.html)

- neo4j-doc: [betweeness-centrality-usage](https://neo4j.com/docs/graph-data-science/current/algorithms/betweenness-centrality/)

- ultipa graph: [graph-analytics & algs](https://www.youtube.com/watch?v=6XCcyxP0emk)

- pet-graph: [alogrithm-in-graph](https://docs.rs/petgraph/latest/petgraph/algo/index.html)

### 2. How could you adapt this code to represent s different kind of network, such as a social network or transportation network?

### 3. Can you add functionality to add new fighters and fight to the network?