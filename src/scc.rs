use std::collections::{HashMap, HashSet};

fn dfs(graph: &HashMap<u32, HashSet<u32>>, start: u32, visited: &mut HashSet<u32>, stack: &mut Vec<u32>) {
    let mut temp_stack = vec![start];

    while let Some(node) = temp_stack.pop() {
        if visited.insert(node) {
            temp_stack.extend(graph.get(&node).unwrap_or(&HashSet::new()).iter().cloned());
            stack.push(node);
        }
    }
}

fn transpose(graph: &HashMap<u32, HashSet<u32>>) -> HashMap<u32, HashSet<u32>> {
    let mut transposed_graph: HashMap<u32, HashSet<u32>> = HashMap::new();
    
    for (&node, neighbors) in graph.iter() {
        for &neighbor in neighbors.iter() {
            transposed_graph
                .entry(neighbor)
                .or_insert_with(HashSet::new)
                .insert(node);
        }
    }

    transposed_graph
}

// src/scc.rs
pub fn kosaraju_scc(graph: &HashMap<u32, HashSet<u32>>) -> Vec<Vec<u32>> {
    let mut visited = HashSet::new();
    let mut stack = Vec::new();
    let mut sccs: Vec<Vec<u32>> = Vec::new(); // Changed to Vec<Vec<u32>>

    // First DFS loop to populate the stack with the finish times of nodes
    for &node in graph.keys() {
        if !visited.contains(&node) {
            dfs(graph, node, &mut visited, &mut stack);  // stack is now a Vec<u32>
        }
    }

    // Transpose the graph
    let transposed_graph = transpose(graph);

    // Second DFS loop to find the strongly connected components
    visited.clear();
    while let Some(node) = stack.pop() {
        if !visited.contains(&node) {
            let mut scc: Vec<u32> = Vec::new(); // Changed to Vec<u32>
            dfs(&transposed_graph, node, &mut visited, &mut scc);
            sccs.push(scc);
        }
    }

    sccs
}