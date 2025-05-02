// Module: connected_components
// Purpose: Finds all connected components in an undirected graph.

use std::collections::{HashMap, HashSet};

/// Identifies connected components in the graph.
///
/// Input:
/// - `graph`: adjacency list node → neighbor set
///
/// Output:
/// - `Vec<Vec<u32>>`: each inner Vec is one component’s nodes
///
/// Logic:
/// - DFS from each unvisited node, collecting its component.
pub fn connected_components(graph: &HashMap<u32, HashSet<u32>>) -> Vec<Vec<u32>> {
    let mut visited: HashSet<u32> = HashSet::new();
    let mut components: Vec<Vec<u32>> = Vec::new();

    for &node in graph.keys() {
        if !visited.contains(&node) {
            let mut stack = vec![node];
            let mut comp: Vec<u32> = Vec::new();

            while let Some(v) = stack.pop() {
                if visited.insert(v) {
                    comp.push(v);
                    // Push only unvisited neighbors
                    stack.extend(
                        graph
                            .get(&v)
                            .unwrap_or(&HashSet::new())
                            .iter()
                            .filter(|nbr| !visited.contains(nbr))
                            .cloned(),
                    );
                }
            }

            components.push(comp);
        }
    }

    components
}