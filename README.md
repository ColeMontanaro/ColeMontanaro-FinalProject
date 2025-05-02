# Final Project Write‑Up

## A. Project Overview

**Goal:**  
Analyze the Amazon Product Co‑Purchasing Network to identify key products and communities by computing connected components, degree centrality, clustering coefficients, and subset‑based closeness and betweenness centralities. This helps Amazon improve recommendations, bundling, and cross‑category promotions.

**Dataset:**  
- **Source:** SNAP “Amazon product co‑purchase” graph  
- **Files:**  
  - `com‑amazon.ungraph.txt`: 334,863 nodes, 925,872 undirected edges  
- **Link:** https://snap.stanford.edu/data/com-Amazon.html  

---

## B. Data Processing

1. **Loading**  
   - Used Rust’s `BufReader` to stream `com‑amazon.ungraph.txt`.  
   - Parsed each whitespace‑separated pair `(u v)`, skipping lines beginning with `#`.

2. **Cleaning & Transformations**  
   - Ignored comment lines (`# …`).  
   - Parsed only valid two‑integer lines.  
   - Built a bidirectional adjacency list: `HashMap<u32, HashSet<u32>>`.

---

## C. Code Structure

### Modules & Rationale

1. **`graph.rs`**  
   - **Purpose:** Load edge list into adjacency list (`Graph`).  
   - **Rationale:** Central source of graph data for all analyses.

2. **`connected_components.rs`**  
   - **Purpose:** DFS‑based detection of connected components.  
   - **Rationale:** Identify isolated subgraphs (none found in this dataset).

3. **`centrality.rs`**  
   - **Purpose:** Compute degree centrality (neighbor count).  
   - **Rationale:** Fast identification of “hub” products.

4. **`clustering.rs`**  
   - **Purpose:** Compute local and average clustering coefficients.  
   - **Rationale:** Measure tightness of co‑purchase communities.

5. **`centrality_closeness.rs`**  
   - **Purpose:** Compute **closeness** centrality for a **subset** of source nodes.  
   - **Rationale:** Limit expensive BFS runs to top‑degree products.

6. **`centrality_betweenness.rs`**  
   - **Purpose:** Compute **betweenness** centrality for a **subset** of sources using Brandes’ algorithm.  
   - **Rationale:** Identify bridge products without full‑graph cost.

7. **`main.rs`**  
   - **Purpose:** Orchestrate loading and run all analyses.  
   - **Rationale:** Single executable entry point.

8. **`tests.rs`**  
   - **Purpose:** Unit tests for each delivered function.  
   - **Rationale:** Ensure correctness before submission.

### Key Functions & Types

- **`load_graph(path: &str) -> Graph`**  
  - **Inputs:** file path  
  - **Outputs:** `HashMap<u32, HashSet<u32>>`  
  - **Logic:** Stream lines, skip comments, parse edges bidirectionally.

- **`connected_components(graph: &Graph) -> Vec<Vec<u32>>`**  
  - **Inputs:** adjacency list  
  - **Outputs:** list of node‑lists for each component  
  - **Logic:** DFS from each unvisited node, collect its component.

- **`degree_centrality(graph: &Graph) -> Vec<(u32, usize)>`**  
  - **Inputs:** adjacency list  
  - **Outputs:** sorted `(node, degree)` descending  
  - **Logic:** Count neighbors for each node, sort.

- **`clustering_coefficient(graph: &Graph) -> Vec<(u32, f64)>`**  
  - **Inputs:** adjacency list  
  - **Outputs:** local clustering per node  
  - **Logic:** For each node, count existing edges among neighbors vs. possible.

- **`average_clustering(graph: &Graph) -> f64`**  
  - **Inputs:** adjacency list  
  - **Outputs:** global average of local coefficients  
  - **Logic:** Sum local values / node count.

- **`closeness_centrality_subset(graph: &Graph, sources: &[u32]) -> Vec<(u32, f64)>`**  
  - **Inputs:** adjacency list, chosen sources  
  - **Outputs:** sorted `(node, closeness)` for sources  
  - **Logic:** For each source, BFS to measure reachable distances, compute `(reachable‑1)/sum`.

- **`betweenness_centrality_subset(graph: &Graph, sources: &[u32]) -> Vec<(u32, f64)>`**  
  - **Inputs:** adjacency list, chosen sources  
  - **Outputs:** sorted `(node, betweenness)` for all nodes  
  - **Logic:** For each source, run `bfs_paths` then `accumulate` per Brandes, sum contributions.

### Main Workflow

1. **Load** graph.  
2. **Find** connected components.  
3. **Compute** degree centrality; select top 50 as `sources`.  
4. **Compute** clustering coefficients on full graph.  
5. **Compute** closeness & betweenness only for `sources`.  
6. **Print** top 5 of each metric.

---

## D. Tests

**Cargo test output:**
```text
running 4 tests
test tests::tests::test_closeness_line ... ok
test tests::tests::test_betweenness_line ... ok
test tests::tests::test_clustering_triangle ... ok
test tests::tests::test_connected_components ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

test_connected_components: verifies DFS groups {1,2,3} and {4,5} correctly.

test_clustering_triangle: checks that a 3‑node clique yields coefficient 1.0 for all.

test_closeness_line: on line 1–2–3, ensures node 2 has highest closeness.

test_betweenness_line: on line 1–2–3, ensures node 2 has highest betweenness.

## E. Results

cargo run --release output:

Loaded graph with 334863 nodes
Found 1 connected components
Largest component size: 334863

Top 5 nodes by degree:
Node 548091: degree 549
Node 458358: degree 324
Node 222074: degree 257
Node 199628: degree 230
Node 515301: degree 228

Average clustering coefficient: 0.3967
Node 548519 has highest local clustering: 1.0000

Closeness centrality (top 50):
Node 537519: 0.125640
Node 199628: 0.124667
Node 98756:  0.124555
Node 222074: 0.124489
Node 35512:  0.124262

Betweenness centrality (top 50):
Node 222074: 718060.425464
Node 284825: 550627.296016
Node 89000:  522266.455572
Node 502784: 514092.913860
Node 154855: 394169.592840

Interpretation:

Single giant component → fully connected co‑purchase network.

Degree hubs (e.g. 548091) are top “also‑bought” items.

Clustering ≈ 0.4 → moderately cohesive product communities; perfect‑clique node (548519) ideal for bundles.

Closeness ~0.125 → these top 50 products are ~8 steps from any other, guiding deep catalog exploration.

High betweenness (e.g. 222074) → bridge items linking clusters, critical for cross‑category promotions.

## F. Usage Instructions

1. Clone & Enter Repo

git clone https://github.com/YourUsername/ColeMontanaro-FinalProject

cd ColeMontanaro-FinalProject

2. Build & Run (release)

cargo run --release

Expected runtime: ~15 s on a modern 8‑core machine.

## G. Citations

N/A
