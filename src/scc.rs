use std::collections::{HashMap, HashSet};

fn dfs(graph: &HashMap<u32, HashSet<u32>>, node: u32, visited: &mut HashSet<u32>, stack: &mut HashSet<u32>) {
    visited.insert(node);
    if let Some(neighbors) = graph.get(&node) {
        for &neighbor in neighbors {
            if !visited.contains(&neighbor) {
                dfs(graph, neighbor, visited, stack);
            }
        }
    }
    stack.insert(node);  // Use insert instead of push for HashSet
}

fn transpose_graph(graph: &HashMap<u32, HashSet<u32>>) -> HashMap<u32, HashSet<u32>> {
    let mut transposed = HashMap::new();
    for (node, neighbors) in graph {
        for &neighbor in neighbors {
            transposed.entry(neighbor).or_insert_with(HashSet::new).insert(*node);
        }
    }
    transposed
}

// src/scc.rs
pub fn kosaraju_scc(graph: &HashMap<u32, HashSet<u32>>) -> Vec<HashSet<u32>> {
    let mut visited = HashSet::new();
    let mut stack = HashSet::new();  // Change to HashSet
    // Step 1: Fill vertices in stack according to their finishing times
    for &node in graph.keys() {
        if !visited.contains(&node) {
            dfs(graph, node, &mut visited, &mut stack);
        }
    }

    // Step 2: Transpose the graph
    let transposed_graph = transpose_graph(graph);
    visited.clear();

    // Step 3: Perform DFS on the transposed graph using nodes in stack order
    let mut sccs = Vec::new();
    while let Some(node) = stack.iter().next().cloned() {
        if !visited.contains(&node) {
            let mut scc = HashSet::new();
            dfs(&transposed_graph, node, &mut visited, &mut scc);
            sccs.push(scc);
        }
    }
    sccs
}