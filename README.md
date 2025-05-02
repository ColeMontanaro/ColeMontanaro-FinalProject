# Final Project Write‑Up

## A. Project Overview
**Goal**  
Analyze the Amazon Product Co‑Purchasing Network to uncover key products (hubs, bridges) and community structure using graph‑theoretic metrics.

**Dataset**  
- **Source:** SNAP “Amazon product co‑purchase” graph  
- **Size:** 334 ,863 nodes · 925 ,872 undirected edges  
- **Link:** <https://snap.stanford.edu/data/com-Amazon.html>

---

## B. Data Processing
- **Loading** – Streamed `com‑amazon.ungraph.txt` with `BufReader`.  
- **Cleaning** – Skipped `#` comment lines; accepted only two‑integer rows.  
- **Transformation** – Inserted each undirected edge symmetrically into an adjacency list (`HashMap<u32, HashSet<u32>>`).

---

## C. Code Structure

### Modules & Rationale
| Module | Purpose | Rationale |
| ------ | ------- | --------- |
| `graph.rs` | Load edge list into `Graph`. | Single authoritative loader. |
| `connected_components.rs` | DFS component detection. | Find isolated sub‑graphs. |
| `centrality.rs` | Degree centrality. | Fast hub detection. |
| `clustering.rs` | Local & average clustering. | Community tightness. |
| `centrality_closeness.rs` | Closeness (subset). | Cut BFS cost. |
| `centrality_betweenness.rs` | Betweenness (subset, Brandes). | Cheap bridge detection. |
| `main.rs` | CLI entry; orchestrates analysis steps. | Keep interface simple. |
| `tests.rs` | Unit tests. | Regression safety. |

### Key Functions & Types
| Item | Purpose | In / Out | Core Logic |
| ---- | ------- | -------- | ---------- |
| `type Graph` | Compact undirected graph. | — | Symmetric edge inserts for O(1) look‑ups. |
| `load_graph` | Build `Graph`. | `&str` → `Graph` | Stream lines → parse → insert `u↔v`. |
| `connected_components` | List components. | `&Graph` → `Vec<Vec<u32>>` | DFS w/ visited set. |
| `degree_centrality` | Rank by degree. | `&Graph` → `Vec<(u32,usize)>` | Count neighbors, sort. |
| `clustering_coefficient` | Local clustering. | `&Graph` → `Vec<(u32,f64)>` | Count neighbor edges / `k·(k‑1)/2`. |
| `average_clustering` | Mean clustering. | `&Graph` → `f64` | Average of above. |
| `closeness_centrality_subset` | Closeness. | `&Graph`, `[u32]` → `Vec<(u32,f64)>` | BFS → `(reachable‑1)/Σdist`. |
| `betweenness_centrality_subset` | Betweenness. | `&Graph`, `[u32]` → `Vec<(u32,f64)>` | Brandes forward/backward for each source. |

### Main Workflow
1. **Load graph** with `load_graph` → creates in‑memory `Graph`.  
2. **Detect components** (`connected_components`) → verifies the market is a single, unified network.  
3. **Score hubs** via `degree_centrality` → choose **top 50 nodes** as the “source” set for more expensive metrics.  
4. **Measure community tightness** using `clustering_coefficient` and `average_clustering` across the whole graph.  
5. **Calculate distance influence** with `closeness_centrality_subset` for those 50 sources (reduces BFS workload).  
6. **Identify bridging items** via `betweenness_centrality_subset`, again limited to the source set.  
7. **Print summaries** that feed directly into the interpretation section.

---

## D. Tests

~~~bash
cargo test
~~~
Output:
~~~text
running 4 tests
test tests::test_closeness_line ... ok
test tests::test_betweenness_line ... ok
test tests::test_clustering_triangle ... ok
test tests::test_connected_components ... ok
result: ok. 4 passed; 0 failed; finished in 0.00s
~~~

**Individual checks**  
1. **connected_components** – Detects two groups `{1,2,3}` & `{4,5}` → DFS correct.  
2. **clustering_triangle** – 3‑node clique returns coefficient 1.0 → clustering math correct.  
3. **closeness_line** – Line graph 1‑2‑3 gives node 2 max closeness → BFS distances right.  
4. **betweenness_line** – Same line graph gives node 2 max betweenness → Brandes accumulation right.

---

## E. Results
Build and run the program:

~~~bash
cargo run --release
~~~
Output:
~~~text
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
Node 98756: 0.124555
Node 222074: 0.124489
Node 35512: 0.124262

Betweenness centrality (top 50):
Node 222074: 718060.425464
Node 284825: 550627.296016
Node 89000: 522266.455572
Node 502784: 514092.913860
Node 154855: 394169.592840
~~~

**Interpretation (Amazon takeaways)**  
- **Unified product space** means any item can influence the entire catalog—great for recommendation coverage.  
- **High‑degree hubs** (e.g., 548091) are prime “also‑bought” anchors; prioritizing them boosts cross‑selling.  
- **Average clustering ≈ 0.4** reveals moderate natural bundles; leverage clusters for themed promotions.  
- **Perfect‑clique SKU 548519** is ideal for multi‑item bundles or starter kits.  
- **Bridge products** (betweenness leaders like 222074) link separate categories—surface these to drive discovery across departments.

---

## F. Usage Instructions
1. **Clone the repository**
   ~~~bash
   git clone https://github.com/YourUsername/ColeMontanaro-FinalProject
   cd ColeMontanaro-FinalProject
   ~~~
2. **Build the project** (optimized release build)
   ~~~bash
   cargo build --release
   ~~~
3. **Run the analysis**
   ~~~bash
   cargo run --release
   ~~~
   - *Command‑line arguments:* none.  
   - *User interaction:* none—program prints results to stdout.  
   - *Expected runtime:* ~20 seconds (depends on computer)

---

## G. Citations
N/A