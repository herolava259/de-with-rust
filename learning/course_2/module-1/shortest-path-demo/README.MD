# Reflection Questions:

## 1. How does Dijkstra's algorithm determine the shortest path between two nodes?
- create the min heap of all node in the graph with ordering metrics that initially one take the weight of edge connect between the start node with each node if exsitence otherwise one put the inifinity for the metric. One call or named the metric that `temporalily distance` from the start node to each node. The min heap maintain for the top of the datastructure that the top of it is the min metric/score of temporarily distances 
- divide multiple step and repeat:
- maintain a data structure (specially is min-heap) of unchecked-nodes with metric for order is temporalily min-distance from start node to this node
- take top node of the min heap with the condition that The node is unchecked. This is minimum distance from start-node to it. Mark it as visited or checked
- update all adjancent nodes with that they is unmark of the newly marked (or finished) for temporaly min distance then put the newly distance of the adj-nodes into the min-heap. 
- repeat all steps above (except to step 1) until to the minheap is empty

## 2. How is the graph created and what does each edge represent?

## 3. What would happen if there was no route between Belem Tower and Lisbon Cathedral?

- Both the cities belong two different connected components.

# Challenge Questions:

## 1. Modify the program to allow the user to specify the start and end nodes.

## 2. Expand the graph to include more landmarks and connections.

## 3. Can you think of other real-life applications for Dijkstra's algorithm?