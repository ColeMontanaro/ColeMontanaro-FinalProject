# Cole Montanaro Final Project Writeup

## A. Project Overview

**Goal:**  
Analyze the Amazon Product Co‑Purchasing Network to identify key products and communities by computing connected components, degree centrality, clustering coefficients, and subset‑based closeness and betweenness centralities. This helps Amazon improve recommendations, bundling, and cross‑category promotions.

**Dataset:**  
- **Source:** SNAP “Amazon product co‑purchase” graph  
- **Files:**  
  - `com‑amazon.ungraph.txt`: 334,863 nodes, 925,872 undirected edges  
- **Link:** <https://snap.stanford.edu/data/com-Amazon.html>

---

## B. Data Processing

1. **Loading**  
   - Used Rust’s `BufReader` to stream `com‑amazon.ungraph.txt`.  
   - Parsed each whitespace‑separated pair `(u v)`, skipping lines that begin with `#`.

2. **Cleaning & Transformations**  
   - Ignored comment lines (`# …`).  
   - Parsed only valid two‑integer lines.  
   - Built a bidirectional adjacency list: `HashMap<u32, HashSet<u32>>`.

---

## C. Code Structure

### Modules & Rationale

| Module | Purpose | Rationale |
| ------ | ------- | --------- |
| `graph.rs` | Load edge list into adjacency list (`Graph`). | Central source for all analyses. |
| `connected_components.rs` | DFS‑based detection of connected components. | Identify isolated subgraphs. |
| `centrality.rs` | Compute degree centrality. | Fast hub detection. |
| `clustering.rs` | Compute local & average clustering coefficients. | Measure community tightness. |
| `centrality_closeness.rs` | Closeness centrality for a subset. | Limit expensive BFS. |
| `centrality_betweenness.rs` | Betweenness centrality (subset, Brandes). | Find bridge products cheaply. |
| `main.rs` | Orchestrate loading and run analyses. | Single entry point. |
| `tests.rs` | Unit tests for each function. | Ensure correctness. |

### Key Functions & Types

| Function | Signature | Description |
| -------- | --------- | ----------- |
| `load_graph` | `fn load_graph(path:&str)->Graph` | Streams file, skips comments, builds adjacency list. |
| `connected_components` | `fn connected_components(graph:&Graph)->Vec<Vec<u32>>` | DFS, returns components. |
| `degree_centrality` | `fn degree_centrality(graph:&Graph)->Vec<(u32,usize)>` | Counts neighbors, sorts descending. |
| `clustering_coefficient` | `fn clustering_coefficient(graph:&Graph)->Vec<(u32,f64)>` | Local clustering per node. |
| `average_clustering` | `fn average_clustering(graph:&Graph)->f64` | Global average clustering. |
| `closeness_centrality_subset` | `fn closeness_centrality_subset(graph:&Graph,src:&[u32])->Vec<(u32,f64)>` | BFS‑based closeness. |
| `betweenness_centrality_subset` | `fn betweenness_centrality_subset(graph:&Graph,src:&[u32])->Vec<(u32,f64)>` | Brandes accumulation. |

### Main Workflow

1. **Load** graph.  
2. **Find** connected components.  
3. **Compute** degree centrality; select top 50 as `sources`.  
4. **Compute** clustering coefficients.  
5. **Compute** closeness & betweenness for `sources`.  
6. **Print** top 5 of each metric.

---

## D. Tests

~~~text
running 4 tests
test tests::tests::test_closeness_line ... ok
test tests::tests::test_betweenness_line ... ok
test tests::tests::test_clustering_triangle ... ok
test tests::tests::test_connected_components ... ok

test result: ok. 4 passed; 0 failed; finished in 0.00s
~~~

- **test_connected_components:** verifies DFS groups `{1,2,3}` and `{4,5}`.  
- **test_clustering_triangle:** 3‑node clique ⇒ coefficient 1.0.  
- **test_closeness_line:** on line 1‑2‑3, node 2 highest closeness.  
- **test_betweenness_line:** on line 1‑2‑3, node 2 highest betweenness.

---

## E. Results

~~~text
cargo run --release output:

Loaded graph with 334863 nodes
Found 1 connected component
Largest component size: 334863

Top 5 nodes by degree:
548091: 549
458358: 324
222074: 257
199628: 230
515301: 228

Average clustering coefficient: 0.3967
Highest local clustering: node 548519 → 1.0000

Closeness centrality (top 50):
537519: 0.125640
199628: 0.124667
98756 : 0.124555
222074: 0.124489
35512 : 0.124262

Betweenness centrality (top 50):
222074: 718060.43
284825: 550627.30
89000 : 522266.46
502784: 514092.91
154855: 394169.59
~~~

**Interpretation**

- **Single giant component** → fully connected co‑purchase network.  
- **Degree hubs** (e.g., 548091) are top “also‑bought” items.  
- **Clustering ≈ 0.4** → moderately cohesive communities; node 548519 forms a perfect clique.  
- **Closeness ≈ 0.125** → top 50 products are ~8 steps from any other.  
- **High betweenness** (e.g., 222074) → bridges linking clusters.

---

## F. Usage Instructions

~~~bash
# Clone & enter
git clone https://github.com/YourUsername/ColeMontanaro-FinalProject
cd ColeMontanaro-FinalProject

# Build & run (release)
cargo run --release   # ≈ 15 s on an 8‑core machine
~~~

---

## G. Citations

N/A