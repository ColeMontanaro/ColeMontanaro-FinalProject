// Module: connected_components
// Purpose: Finds all connected components in an undirected graph using DFS.

use std::collections::{HashMap, HashSet};

/// Finds all connected components in an undirected graph.
///
/// # Input:
/// * `graph` - A reference to a graph represented as an adjacency list.
///
/// # Output:
/// * A vector of connected components, where each component is a vector of node IDs.
///
/// # Logic:
/// Iteratively performs DFS starting from each unvisited node.
pub fn connected_components(graph: &HashMap<u32, HashSet<u32>>) -> Vec<Vec<u32>> {
    let mut visited = HashSet::new();
    let mut components: Vec<Vec<u32>> = Vec::new();

    for &node in graph.keys() {
        if !visited.contains(&node) {
            let mut component = Vec::new();
            let mut stack = vec![node];

            while let Some(current) = stack.pop() {
                if visited.insert(current) {
                    component.push(current);
                    stack.extend(graph.get(&current).unwrap_or(&HashSet::new()).iter().cloned());
                }
            }

            components.push(component);
        }
    }

    components
}