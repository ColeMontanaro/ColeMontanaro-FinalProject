// src/centrality_closeness.rs
// Module: centrality_closeness
// Purpose: Computes closeness centrality for a specified subset of source nodes.

use std::collections::{HashMap, HashSet, VecDeque};

/// Computes closeness centrality only for a given subset of source nodes:
///   C(v) = (reachable - 1) / (sum of distances to reachable nodes)

/// Inputs:
/// - `graph`: adjacency list of the graph.
/// - `sources`: slice of node IDs to compute centrality for.

/// Output:
/// - `Vec<(node, score)>`: only for `sources`, sorted descending by score.

/// Logic:
/// - For each `start` in `sources`, run a BFS reusing one queue & dist map to measure distances.
pub fn closeness_centrality_subset(
    graph: &HashMap<u32, HashSet<u32>>,
    sources: &[u32],
) -> Vec<(u32, f64)> {
    let mut results = Vec::new();
    let mut queue = VecDeque::new();
    let mut dist = HashMap::new();

    for &start in sources {
        queue.clear();
        dist.clear();
        dist.insert(start, 0);
        queue.push_back(start);

        while let Some(u) = queue.pop_front() {
            let d_u = dist[&u];
            for &v in graph.get(&u).unwrap_or(&HashSet::new()) {
                if !dist.contains_key(&v) {
                    dist.insert(v, d_u + 1);
                    queue.push_back(v);
                }
            }
        }

        let r = dist.len();
        let score = if r > 1 {
            let total: usize = dist.values().sum();
            (r - 1) as f64 / total as f64
        } else {
            0.0
        };
        results.push((start, score));
    }

    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    results
}