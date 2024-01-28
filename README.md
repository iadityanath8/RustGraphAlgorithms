# Graph Algorithms Implementation

This repository contains implementations of various graph algorithms in Rust. These algorithms are designed to work with a graph data structure represented as an adjacency list.<br>
Majority of the graph algorithms have been implemented using rust design pattern involving traits, struct and compile time dispatch.

## Implemented Algorithms

- **Depth-First Search (DFS):** Recursive implementation of DFS for traversing graphs.
- **Cycle Detection:** Detect cycles in a directed graph using DFS as well as BFS using Kahn Algorithm.
- **Tarjans Algorithm:** Detect a backedge of a graph in O(N) time to find articulation point and Strongly connected components.
- **Topological Sorting:** Implemented Topological sorting algorithm using dfs using stack variant and using Kahn Algorithms.
- **Dijkstra's Algorithm:** Detects shortest path from a sorce node to the other nodes (Right now it does not uses Binary Heap for further optimization).
## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/) installed on your machine.

### Clone the Repository

```bash
git clone https://github.com/iadityanath8/GraphAlgorithms.git
cd GraphAlgorithms
