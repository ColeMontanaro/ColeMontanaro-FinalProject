# ColeMontanaro-FinalProject

Amazon Product Co-Purchasing Network Analysis

Overview

This project analyzes the Amazon Product Co-Purchasing Network using graph algorithms implemented in Rust. It focuses on identifying strongly connected components (SCCs), measuring centrality, and exploring community structure to reveal key insights about the dataset's topology.

Dataset

The analysis is based on the following files located in the data/ directory:

com-amazon.ungraph.txt: An undirected graph where nodes represent products and edges represent frequent co-purchasing.

com-amazon.all.dedup.cmty.txt: Community data identifying groups of related products.

Project Structure

main.rs: Entry point that loads the graph, computes SCCs, and prints top centrality nodes.

graph.rs: Loads and parses the co-purchasing graph.

scc.rs: Implements Kosaraju's algorithm to compute strongly connected components.

centrality.rs: Computes degree centrality and supports BFS for potential enhancements.

community.rs: (Optional) Placeholder for future community detection logic.

tests.rs: Includes a unit test validating the SCC algorithm on a small example graph.

Key Algorithms

Strongly Connected Components (SCC)

Implemented using Kosaraju's algorithm:

Perform a DFS to get finishing times.

Transpose the graph.

DFS in order of finishing times to extract SCCs.

Centrality

Degree centrality is computed as the number of direct neighbors each node has. Nodes with the highest degree are likely influential or popular products.

Running the Project

To compile and test the project:

cargo build
cargo run
cargo test

Output Example

Loaded graph with XXXXX nodes
Found YYYY strongly connected components
Largest SCC size: ZZZZ
Top 5 nodes by degree:
Node A: degree 123
Node B: degree 117
...

Requirements

Ensure you have the following installed:

Rust

The dataset files extracted into the data/ directory

Final Notes

This project is a final deliverable for a data science course using Rust to analyze real-world graphs. The core focus is on graph structure understanding and algorithmic implementation in a systems programming context.

