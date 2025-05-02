// Module: connected_components
// Purpose: Finds all connected components in an undirected graph using DFS.

use std::collections::{HashMap, HashSet};

/// Finds all connected components in an undirected graph.

/// Input:
/// `graph` - A reference to a graph represented as an adjacency list (HashMap where keys are nodes and values are sets of neighbors).

/// Output:
/// A vector of connected components, where each component is a vector of node IDs.

/// Logic:
/// Iteratively performs Depth-First Search (DFS) starting from each unvisited node. Nodes are grouped into components based on connectivity.
pub fn connected_components(graph: &HashMap<u32, HashSet<u32>>) -> Vec<Vec<u32>> {
    let mut visited = HashSet::new();  // Set to track visited nodes
    let mut components: Vec<Vec<u32>> = Vec::new();  // Vector to store the components

    for &node in graph.keys() {  // Iterate through each node
        if !visited.contains(&node) {  // If the node is unvisited, start DFS

            let mut component = Vec::new();  // To store nodes in the current component
            let mut stack = vec![node];  // DFS stack to explore neighbors

            while let Some(current) = stack.pop() {  // Process nodes in the stack
                if visited.insert(current) {  // Mark node as visited
                    component.push(current);  // Add it to the current component
                    // Add unvisited neighbors to the stack for further exploration
                    stack.extend(graph.get(&current).unwrap_or(&HashSet::new()).iter().cloned());
                }
            }
            components.push(component);  // Store the completed component
        }
    }
    components
}