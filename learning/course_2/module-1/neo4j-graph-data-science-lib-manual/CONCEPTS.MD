# Concepts

## I. Centrality

### i. CELF

#### i.a. Start

The influence maximization problem asks for a set of `k` nodes that maximize the expected spread of influence in the network. The set of these initial `k` is called the `seed set`.

Initally we assume that the nodes in the seed set become influenced and the process works as follows. As influenced node influences each of its neighbors with probability `p`. The spread is then the number of nodes that become influenced.

The CELF alg is based on the Greedy. algorithm for the problem. It works iteratively in `k` steps to create the returned seed set `S`, where at each step the node yielding the maximum expected spreed gain is added to `S`. where at each step the node yielding the maximum expected spread gain is added to `S`.

The CELF alg extends on Greedy by introducing a *lazy forwarding* mechanism, which prunes a lot of nodes from being examined, thereby massively reducing the number of conducted simulations.


#### ii. eigenvector centrality

### i.a Start

- Eigenvector Centrality is an algorithm that measures the **transitive** influence of nodes. Relationship originating from high-scoring modes contributes more to the score of a node than connections from low-scoring nodes
- A high eigenvector score means that a node is connected to many nodes who themselves have high scores.

- The alg computes the eigenvector associated with the largest absolute eigenvalue. To compute that eigenvalue, the alg applies the [power iteration](https://en.wikipedia.org/wiki/Power_iteration)