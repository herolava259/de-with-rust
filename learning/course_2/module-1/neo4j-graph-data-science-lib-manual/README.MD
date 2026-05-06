# Reflection Questions:

## 1. What are centrality algorithms used for in graphs? (To determine the importance of different nodes)?

- [Article-Rank](https://neo4j.com/docs/graph-data-science/current/algorithms/article-rank/)

- [Articulation-Points](https://neo4j.com/docs/graph-data-science/current/algorithms/articulation-points/)

- [Bridges](https://neo4j.com/docs/graph-data-science/current/algorithms/bridges/)

- [CELF](https://neo4j.com/docs/graph-data-science/current/algorithms/celf/)

- [Closeness-Centrality](https://neo4j.com/docs/graph-data-science/current/algorithms/closeness-centrality/)

- [Degree-Centrality](https://neo4j.com/docs/graph-data-science/current/algorithms/degree-centrality/)

- [Eigenvector-Centrality](https://neo4j.com/docs/graph-data-science/current/algorithms/eigenvector-centrality/)

- [Harmonic-Centrality](https://neo4j.com/docs/graph-data-science/current/algorithms/harmonic-centrality/)

- [Page-Rank](https://neo4j.com/docs/graph-data-science/current/algorithms/page-rank/)

- [HITS](https://neo4j.com/docs/graph-data-science/current/algorithms/hits/)


> `Page-Rank` and `Article-Rank` are variants of `eigenvector-centrality` alg

## 2. What are the two tiers of centrality algorithms listed? (Production-quality and Alpha)


## 3. What does the PageRank algorithm use the alpha parameter for? (To control the probability of randomly traversing the graph)?

- Page Rank call alpha parameter as damping factor

## 4. Which algorithm is best for finding nodes influential in information spreading? (Eigenvector Centrality)?

# Challenge Questions:

## 1. How could you determine which centrality algorithm is best for graph analysis?
- **Most connected**
- **Best bridge/articulation-point**
- **Most reachable**
- **Connected to influential nodes?**

## 2. If you needed to find essential nodes in a graph fast, which algorithm would you use and why?

- degree-centrality-algorithm
- beacause:
    - simple
    - easily implementation
    - Time complexcity is linear O(V+E)
    - scale well for hugh graph

- better importance but still fast: **PageRank**

- Why PageRank:
    - Time Complexity: O(kE)
    - good scalabillity
    - better quality

## 3. What are some ways to optimize or parallelize centrality algorithms for large graphs?

- reduce bottlenecks:
    - **traversal cost**
    - **memory access cost**
    - **synchronization cost**
    - **approximation cost**

- Some ways
1. Use graph-friendly data structures first 
    - CSR (Compressed Sparse Row)
    - Adj arrays
    - Edge lists for streaming
2. Parallelize Degree Centrality

- Degree is easiest 

- Each node degree canbe computed independently

- Parallel strategy:
    - Split vertices across threads

3. Parallelize PageRank

4. Parallelize Betweeness Centrality

5. Approximate Betweeness / Closeness

6. Frontier-based parallel BFS

7. GPU acceleration

8. Use graph partitioning

9. use asynchronous iterations for PageRank

10. Use approximation + parallelism together

11. Use graph processing framworks

## 4. How would you handle and interpret nodes with identical centrality scores? 

## 5. If you needed to design a new centrality metric, what considerations and tradeoffs would you need to consider? 

# References 

- [neo4j-graph-ds](https://neo4j.com/docs/graph-data-science/current/algorithms/centrality/)
