# ColeMontanaro-FinalProject

Amazon Product Co-Purchasing Network Analysis

Project Overview

Goal: Identify structural communities and influential products in the Amazon Product Co-Purchasing Network by computing strongly connected components (SCCs) and centrality measures.

Dataset:

Name: Amazon Product Co-Purchasing Network

Source: Stanford Network Analysis Project (SNAP)

Files Used:

com-amazon.ungraph.txt (unweighted graph of co-purchases)

com-amazon.all.dedup.cmty.txt (community annotations — not used in this version)

Size: ~334,863 nodes, ~925,872 edges

Link: https://snap.stanford.edu/data/com-Amazon.html

Data Processing

The data is stored in a data/ directory and read using standard file I/O (BufReader).

Only lines with actual edge data (non-comment) are parsed.

Each edge is added to an adjacency list structure (HashMap<u32, HashSet<u32>>), treating the graph as undirected.

Code Structure

Modules:

graph.rs – Loads and represents the graph.

centrality.rs – Computes degree centrality and includes BFS (unused).

scc.rs – Implements Kosaraju’s algorithm for strongly connected components.

community.rs – Placeholder (future use).

tests.rs – Contains unit tests.

Key Functions & Types:

graph::load_graph

Purpose: Load an undirected graph from text file.

Input: File path (&str)

Output: HashMap<u32, HashSet<u32>>

Logic: Parses pairs of product IDs and creates mutual links.

centrality::degree_centrality

Purpose: Compute the number of connections (degree) for each node.

Input: Graph

Output: Sorted vector of (node_id, degree) pairs.

scc::kosaraju_scc

Purpose: Detect SCCs using Kosaraju’s two-pass DFS algorithm.

Input: Graph

Output: Vec<Vec<u32>> — list of components

Logic:

DFS to fill stack by finish time

Transpose the graph

DFS on transposed graph using popped order

Main Workflow

Load the graph.

Compute SCCs and identify the largest component.

Calculate and print top-5 nodes by degree centrality.

Tests

Output

running 1 test
test tests::tests::test_scc ... ok

Explanation

test_scc: Verifies correctness of SCC detection on a small hardcoded graph.

2 known SCCs are compared with expected components using HashSet.

Results

Loaded graph with 334863 nodes
Found 1 strongly connected components
Largest SCC size: 334863
Top 5 nodes by degree:
Node 548091: degree 549
Node 458358: degree 324
Node 222074: degree 257
Node 199628: degree 230
Node 515301: degree 228

Interpretation

The Amazon product network is densely interconnected — a single giant SCC includes all nodes.

The most connected nodes (high degree) may represent essential or popular products.

Usage Instructions

Build and Run

cargo build
cargo run

Testing

cargo test

Runtime

Runtime: ~5–10 seconds (depends on system)

Requirements: Rust toolchain

No command-line arguments required.

Notes

The bfs function is included but currently unused.

Community detection (community.rs) is a placeholder for potential modularity analysis in future versions.

