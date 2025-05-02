# ColeMontanaro-FinalProject

## A. Project Overview
### Goal: What question are you answering?

Identify important relationships and key points in the Amazon Product Co-Purchasing Network.

Focus on finding connected components and computing degree centrality.

Relevance to Amazon:

Product Recommendations: Understanding co-purchase relationships helps improve recommendation systems by suggesting products frequently bought together.

Targeted Marketing: Identifying influential products aids in targeting advertisements and promotions more effectively.

Inventory Management: Central products may help predict demand, improving stock management and reducing overstock/understock issues.

### Dataset: Source, size

Dataset: Amazon Product Co-Purchasing Network.

Files:

com-amazon.ungraph.txt: 334,863 nodes and 925,872 edges.

com-amazon.all.dedup.cmty.txt: Precomputed community assignments (not directly used).

Source: https://snap.stanford.edu/data/com-Amazon.html

## B. Data Processing

### How I loaded it into Rust:

Used a HashMap<u32, HashSet<u32>> to represent the graph adjacency list, and read the edge list from com-amazon.ungraph.txt, adding edges bidirectionally.

### Any cleaning or transformations applied:

Skipped comment lines (lines starting with #). Ignored malformed lines. Added edges in both directions to account for the undirected nature of the graph.

## C. Code Structure
### Modules:
1. graph.rs: Loads the graph from the edge list file.
2. connected_components.rs: Contains logic to find connected components using DFS.
3. centrality.rs: Computes degree centrality for all nodes.
4. tests.rs: Contains unit tests for the modules.

Purpose of each and rationale for organization:

Modular approach to separate responsibilities: graph loading, connected component analysis, centrality computation, and testing.

### Key Functions & Types:

graph.rs

Graph: Type alias for the graph structure (HashMap<u32, HashSet<u32>>).

load_graph: Loads the graph from the edge list.

Inputs: File path (&str).

Outputs: Graph (HashMap<u32, HashSet<u32>>).

Logic: Reads the file and populates the adjacency list.

connected_components.rs

connected_components: Identifies connected components using DFS.

Inputs: Reference to the graph (&HashMap<u32, HashSet<u32>>).

Outputs: List of connected components (Vec<Vec<u32>>).

Logic: Uses DFS to find unvisited nodes and group them into components.

centrality.rs

degree_centrality: Computes degree centrality for all nodes.

Inputs: Reference to the graph (&HashMap<u32, HashSet<u32>>).

Outputs: List of nodes and their degree (Vec<(u32, usize)>).

Logic: Counts the neighbors for each node and sorts nodes by degree.

### Main Workflow

Graph Loading: load_graph loads the graph.

Connected Components: connected_components groups nodes into connected components.

Degree Centrality: degree_centrality calculates the degree for each node and returns the top 5 nodes.

## D. Tests

### Cargo test output:

running 4 tests

test tests::tests::test_all_connected_nodes ... ok

test tests::tests::test_connected_components ... ok

test tests::tests::test_single_node ... ok

test tests::tests::test_two_disconnected_nodes ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

### Test Cases
1. test_connected_components: Verifies DFS correctly identifies disconnected components.
2. test_single_node: Ensures the function handles a single-node graph correctly.
3. test_two_disconnected_nodes: Verifies the function identifies two isolated nodes as separate components.
4. test_all_connected_nodes: Verifies the function identifies all nodes as connected in a single component.

## E. Results

### Program Outputs:

Loaded graph with 334863 nodes

Found 1 connected components

Largest connected component size: 334863

Top 5 nodes by degree:

Node 548091: degree 549

Node 458358: degree 324

Node 222074: degree 257

Node 199628: degree 230

Node 515301: degree 228

### Interpretation in Project Context:

One large connected component suggests all 334,863 nodes form a single connected network of co-purchased products.
The top 5 nodes by degree represent highly influential products in the network, useful for improving recommendations, marketing, and inventory management.

## F. Usage Instructions:
### How to Build and Run

Clone the repository:
git clone https://github.com/ColeMontanaro/ColeMontanaro-FinalProject

Change to the directory:
cd ColeMontanaro-FinalProject/

Run the program:
cargo run

### Command-line Arguments: No user input required during runtime.

### Expected Runtime:
cargo test runtime: ~0.00s
cargo run runtime: ~4.00s

## G. Citations
N/A



