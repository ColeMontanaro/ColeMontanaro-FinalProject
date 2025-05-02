# Cole Montanaro Final Project Writeup

## A. Project Overview
**Goal**  
Analyze the Amazon Product Co‑Purchasing Network to find key products and communities (connected components, degree/closeness/betweenness centralities, clustering).

**Dataset**  
- **Source:** SNAP “Amazon product co‑purchase” graph  
- **Size:** 334 ,863 nodes · 925 ,872 undirected edges  
- **Link:** <https://snap.stanford.edu/data/com-Amazon.html>

---

## B. Data Processing
- **Loading** – Streamed `com‑amazon.ungraph.txt` with `BufReader`; parsed `(u v)` pairs, skipping `#` comments.  
- **Cleaning** – Accepted only two‑integer lines.  
- **Transformation** – Built bidirectional adjacency list → `HashMap<u32, HashSet<u32>>`.

---

## C. Code Structure
### Modules (Purpose → Rationale)
| Module | Purpose | Rationale |
| ------ | ------- | --------- |
| `graph.rs` | Load edge list into `Graph`. | Single authoritative loader. |
| `connected_components.rs` | DFS component detection. | Identify isolated subgraphs. |
| `centrality.rs` | Degree centrality. | Fast hub detection. |
| `clustering.rs` | Local & average clustering. | Community tightness. |
| `centrality_closeness.rs` | Closeness (subset). | Cut BFS cost. |
| `centrality_betweenness.rs` | Betweenness (subset, Brandes). | Cheap bridge detection. |
| `main.rs` | CLI entry; orchestrates analysis. | Keep interface simple. |
| `tests.rs` | Unit tests. | Regression safety. |

### Key Functions & Types
| Item | Purpose | In / Out | Core Logic |
| ---- | ------- | -------- | ---------- |
| `type Graph = HashMap<u32, HashSet<u32>>` | Store undirected graph. | — | Symmetric edge inserts for O(1) look‑ups. |
| `load_graph` | Build `Graph` from file. | `&str` → `Graph` | Stream lines → parse → insert `u↔v`. |
| `connected_components` | List components. | `&Graph` → `Vec<Vec<u32>>` | DFS w/ visited set. |
| `degree_centrality` | Rank by degree. | `&Graph` → `Vec<(u32,usize)>` | Count neighbors, sort. |
| `clustering_coefficient` | Local clustering. | `&Graph` → `Vec<(u32,f64)>` | Count neighbor edges / `k·(k‑1)/2`. |
| `average_clustering` | Mean clustering. | `&Graph` → `f64` | Average of above. |
| `closeness_centrality_subset` | Closeness (sources). | `&Graph`, `[u32]` → `Vec<(u32,f64)>` | BFS → `(reachable‑1)/Σdist`. |
| `betweenness_centrality_subset` | Betweenness (sources). | `&Graph`, `[u32]` → `Vec<(u32,f64)>` | Brandes forward/backward on each source. |

### Main Workflow
1. `load_graph`  
2. `connected_components`  
3. `degree_centrality` → pick top 50 sources  
4. `clustering_coefficient` & `average_clustering`  
5. `closeness_centrality_subset` & `betweenness_centrality_subset`  
6. Print summaries.

---

## D. Tests
~~~text
running 4 tests
test tests::test_closeness_line ... ok
test tests::test_betweenness_line ... ok
test tests::test_clustering_triangle ... ok
test tests::test_connected_components ... ok
result: ok. 4 passed; 0 failed; finished in 0.00s
~~~
- **connected_components** – Two groups `{1,2,3}`/`{4,5}` detected.  
- **clustering_triangle** – 3‑node clique → coefficient 1.0.  
- **closeness_line / betweenness_line** – On line graph 1‑2‑3, node 2 highest closeness & betweenness.  
*Why?* Proves each metric is computed correctly on minimal cases.

---

## E. Results
~~~text
cargo run --release

Loaded graph with 334863 nodes
Found 1 connected component
Top‑degree nodes: 548091(549) 458358(324) 222074(257) 199628(230) 515301(228)
Average clustering coefficient: 0.3967 (node 548519 = 1.0)
Top closeness: 537519 0.1256 … (50 total)
Top betweenness: 222074 7.18e5 … (50 total)
~~~
**Interpretation**  
- Single giant component → fully connected ecosystem.  
- Degree hubs = “also‑bought” anchors.  
- Clustering ≈ 0.4 → moderate product communities; node 548519 is a tight bundle.  
- High betweenness (222074) → cross‑category bridge items.

---

## F. Usage Instructions
~~~bash
# Clone repo
git clone https://github.com/YourUsername/ColeMontanaro-FinalProject
cd ColeMontanaro-FinalProject

# Build & run (release build)
cargo run --release   # ≈ 15 s on an 8‑core machine
~~~
_No CLI flags; program prints metrics to stdout._

---

## G. Citations
N/A