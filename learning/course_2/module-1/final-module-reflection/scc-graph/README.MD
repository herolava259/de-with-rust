# Strongly Connected Component

## I. Terminology:

- $G = (V, E)$ : be a directed graoh with vertices $V$ and edges $E \subseteq V \times V $

- $ n = |V|$ : the number of vertices
- $m = |E|$ : the number of edges 

## II. Strongly Connected Component

- $ C \subseteq V$ is a `strongly connected component` if the following conditions hold:
  - for all $u, v \in C $ if $ u \neq v$ there exists a path from u to v and a path from v to u 
  - `AND`: $C$ is maximal, in the sense that no vertex can be added without violating the above condition. 

### III. Condensation Graph
- is defined : $ G^{SCC} = (V^{SCC}, E^{SCC})$ as follows:
  - the vertices of $ G^{SCC} $ are the *strongly connected component* of G; i.e $ V^{SCC} = SCC(G)$
  - `AND` for all vertices $ C_i, C_j $ of the condensation graph, there is an edge from $C_i$ to $C_j$ $\iff C_i \neq C_j $ and there exist $ a \in C_i $ and $ b \in C_j$ such that there is an edge from $a$ to $b$ in G.

- Property: No existence `circle` in condensation graph or the graph has a property that `acyclic`

## IV. Kosaraju's algorithm

### Time Complexity : $O(n+m)$

### Terms:
- $t_{out}[v]$ time stamp of vertice **v** when one use dfs to explore the graph. It is a number when one finish explore v or leave v and subsequence of out-neighbor node of v

- $t_{in}[v]$ the first time when one use dfs to explore the vertice **v**

- $t_{out}[C]$ called exit-time of a strongly-connected-components $C$ as the maximum of the values $t_{out}[v]$ for all $v \in C$

- $t_{in}[C]$ be able to inference definition from $t_{out}[C]$ but it is the minimum (meaning the earliest) of them 

### Theorem 1: Let $C$ and $C'$ be two different strongly connected, and let there be an edge from $C$ to $C'$ in the condensation graph. Then $t_{out}[C] > t_{out}[C']$

<details>
<summary>
Proof
</summary>
There are two different cases, depending on which component will first be reached by depth first search:

- `Case 1`: the component $C$ was reached first (i.e, $t_{in}[C] < t_{in}[C']$). In this case, some vertices $v \in C$ at some moment all other vertices of the components $C$ and $C'$ are not visited yet. Since there is an edge from $C$ to $C'$ in the condensation graph, not only are all other vertices in $C$ reachable from $v$ in $G$, but all vertices $C'$ are reachable as well. This means that this `dfs` execution, which is running from vertex $v$, will also visit all other vertices of the components $C$ and $C'$ in the future, so these vertices will be descendants of $v$ in the depth first search tree. This implies that for each vertex $ u \in (C \cup C') \ \{v\}$, we have that $t_{out}[v] > t_{out}[u]$. Therefore $t_{out}[C] > t_{out}[C']$.

- `Case 2`: the component $C'$ was reached first ($t_{in}[C] > t_{in}[C']$). In this case, depth first search visit some vertex $v \in C'$ at some moment at which all other vertices of the components $C$ and $C'$ are not visited yet. Since there is an edge from $C$ to $C'$ in the condensation graph, $C$ is not reachable from $C'$, by the acyclicity property. Hence, the `dfs` execution that is running from vertex v will not reach any vertices of $C$, but it will visit all vertices of $C'$. The vertices of $C$ will be visited by some `dfs` execution later during this step of the algorithm, so indeed we have $t_{out}[C] > t_{out}[C']$
</details>

If we sort all vertices $v \in V$ in decreasing order of their exit time $t_{out}[v]$, then the first vertex $u$ will belong to the "root" strongly connected component, which has no incoming edges in the condensation graph.

Now we want to run some type of search from this vertex $u$ will belong to the "root" strongly connected component, which has no incoming edges in the condensation graph. Now we want to run some type of search from this vertex $u$ so that it will visit all vertices in its strongly connected component, but not other vertices. By repeatly doing so, we can gradually find all strongly connected components: we remove all vertices belonging to the first found component, then we find the next remaining vertex with largest value of $t_{out}$ and run this search from it, and so on. In the end, we will have found all strongly connected components.

### Theorem 2: Let $G^T denote the transpose graph of $G$, obtained by reversing the edge direction in $G$. Then, $SCC(G) = SCC(G^T)$. Furthermore, the condensation graph of $G^T$ is the transpose of the condensation graph of $G$

<details>
<summary></summary>
</details>


## Tarjan 