# Chanllenges

>1. Research Neo4j's graph algorithm and data-science tools for connected data analysis. Summarize the most relevant for analyzing social networks.
>2. Find tweets or other social network dataset and develop questions you could analyze with Neo4j.
>3. Implement a simple graph database in Neo4j and write a Cypher query to analyze node relationships.
>4. Build a small Neo4j graph to model how troll accounts might coordinate influence campaigns on social media.
>5. Prototype a Neo4j pipeline to gather tweets from the Twitter API and ingest them into s graph for analysis.

## I. Graph Algorithm and data science tools 
### 1. Core GDS categories? 
- **Centrality** -> Who is important/influential?
- **Community Detection** -> What groups exist?
- **Similarity & Link Prediction** -> Who is similar / likely to connect? 
- **Path Finding** -> How imformation flows
- **Node Embedding** -> ML-ready representations 

### 2. Centrality (influence & power analysis)
**Key Algorithms**
- **Degree Centrality** -> number of connections (popularity)
- **PageRange** -> influence based on important neighbors 
- **Betweeness Centrality** -> connectors/bridges between commnunities 
- **Closeness Centrality** -> how fast someone reaches others 
- **Eigenvector Centrality** -> influence via influential neighbors

**Social network usecases**
- Identify **influencers (KOLs)** -> PageRank / Eigenvector
- Detect **gatekeepers / brokers** -> Betweenness
- Find **viral spreaders** -> Closeness

### 3. Community Detection (group discovery)

**Most important algorithms**

- **Louvain** -> scalable, widely used modularity optimization 
- **Leiden** -> improved Louvain (more stable communities)
- **Label Propagation (LPA)** -> very fast, good for large graphs 
- **Weakly Connected Comonents (WCC)** -> find isolated subgraphs 
- **Triangle Count / Clustering Coefficient** -> local group density

**Social network use cases**

- Detect **friend circles/communities**
- Identify **interest froups (e.g., Reddit subreddits)**
- Find **fraud rings or coordinated behavior**
- Segment users for **targeted recommendations**

- Typically pipeline:
1. WCC -> remove small disconnected parts 
2. Louvain/leiden -> detect communities 

### 4. Link Prediction (future relationships)

**Key algorithms**

- **Common Neighbors** -> shared friends 
- **Adamic-Adar** -> weighted shared neighbors
- **Prefential Attachment** -> rich get richer
- **Resource Allocation** -> based on community membership 

**Social network use cases**

- Friend recommendations ("People you may know")
- Suggest followers / connections 
- Detect **missing links** in incomplete data 

### 5. Similarity algorithms (user matching)

- **Node Similarity (Jaccard, Cosine)**
- **K-Nearest Neighbors (KNN)**

**Use cases**
- Recommends:
    - Friends
    - Contents
    - Groups
- Identify **lookalike users**

### 6. Node Embeddings (advanced ML layer)
These convert graph structue -> vectors for ML 

**Examples**
- **FastRP**
- **Node2Vec**
- **GraphSAGE**

`Why importants for social networks`
- Feed into:
    - Recommendation systems 
    - Frauddetections models 
    - User classification

### 7. Path Finding (information flow)

**Algorithms**

- Shortest path (Dijkstra, A*)
- All paths / k-shortest paths 

**Use cases**

- Trace **rumor spread**
- Analyze **degrees of separation**
- Identify **critical commnunication paths**


### 8. References:
- [graph-data-science-introduction](https://neo4j.com/docs/graph-data-science/current/introduction/)