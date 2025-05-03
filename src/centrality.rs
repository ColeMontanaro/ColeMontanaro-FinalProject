// Module: centrality
// Purpose: Provides degree centrality measure for graph nodes.

use std::collections::{HashMap, HashSet};

/// Computes degree centrality for each node:
///   degree = number of direct neighbors

/// Inputs:
/// - `graph`: adjacency list mapping node → neighbor set

/// Output:
/// - `Vec<(node, degree)>` sorted descending by degree
pub fn degree_centrality(graph: &HashMap<u32, HashSet<u32>>) -> Vec<(u32, usize)> {
    // Map each node to its neighbor count
    let mut degrees: Vec<(u32, usize)> = graph
        .iter()
        .map(|(&node, neigh)| (node, neigh.len()))
        .collect();

    // Sort descending
    degrees.sort_by(|a, b| b.1.cmp(&a.1));
    degrees
}