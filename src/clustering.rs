// Module: clustering
// Purpose: Computes local and average clustering coefficients for graph nodes.

use std::collections::{HashMap, HashSet};

/// Computes local clustering coefficient for each node:
///   C(v) = (# edges among neighbors) / (possible edges among neighbors)
///
/// Inputs:
/// - `graph`: adjacency list node → neighbor set
///
/// Output:
/// - `Vec<(node, coeff)>` sorted by node ID
pub fn clustering_coefficient(graph: &HashMap<u32, HashSet<u32>>) -> Vec<(u32, f64)> {
    let mut coeffs: Vec<(u32, f64)> = Vec::new();

    for (&node, neighbors) in graph {
        let k = neighbors.len();
        if k < 2 {
            coeffs.push((node, 0.0));
            continue;
        }
        // Convert to Vec for half‑matrix iteration
        let neigh_vec: Vec<u32> = neighbors.iter().cloned().collect();
        let mut links = 0;
        for i in 0..neigh_vec.len() {
            for j in (i + 1)..neigh_vec.len() {
                if graph[&neigh_vec[i]].contains(&neigh_vec[j]) {
                    links += 1;
                }
            }
        }
        let possible = (k * (k - 1) / 2) as f64;
        coeffs.push((node, links as f64 / possible));
    }

    coeffs.sort_by_key(|&(n, _)| n);
    coeffs
}

/// Computes average clustering coefficient over all nodes.
///
/// Inputs:
/// - `graph`: adjacency list
///
/// Output:
/// - single `f64` average
pub fn average_clustering(graph: &HashMap<u32, HashSet<u32>>) -> f64 {
    let coeffs = clustering_coefficient(graph);
    let sum: f64 = coeffs.iter().map(|&(_, c)| c).sum();
    sum / coeffs.len() as f64
}