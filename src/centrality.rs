// Module: centrality
// Purpose: Provides centrality measures to quantify node importance in a graph.

use std::collections::{HashMap, HashSet};

/// Computes degree centrality for all nodes in the graph.
/// Degree centrality measures how many direct neighbors (edges) each node has.

/// # Inputs:
/// * `graph` - A reference to a graph represented as an adjacency list (HashMap where keys are nodes and values are sets of neighbors).

/// # Output:
/// * A sorted vector of tuples (node ID, degree), sorted in descending order by degree.

/// # Logic:
/// For each node, count its number of neighbors using `len()` on the HashSet of neighbors, then sort the results in descending order.
pub fn degree_centrality(graph: &HashMap<u32, HashSet<u32>>) -> Vec<(u32, usize)> {
    // Map each node to the count of its neighbors
    let mut degrees: Vec<_> = graph.iter().map(|(&node, neighbors)| (node, neighbors.len())).collect();
    // Sort nodes by descending degree
    degrees.sort_by(|a, b| b.1.cmp(&a.1));
    degrees
}