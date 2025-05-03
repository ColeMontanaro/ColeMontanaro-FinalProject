// src/centrality_betweenness.rs
// Module: centrality_betweenness
// Purpose: Computes betweenness centrality for a specified subset of source nodes using Brandes’ algorithm.

use std::collections::{HashMap, HashSet, VecDeque};

/// Computes betweenness centrality only for a given subset of source nodes.
 
/// Inputs:
/// - `graph`: adjacency list of the graph.
/// - `sources`: slice of node IDs to use as BFS sources.

/// Output:
/// - `Vec<(node, score)>`: sorted descending by score.

/// Logic:
/// - For each `s` in `sources`, run `bfs_paths` then `accumulate` to update scores.
pub fn betweenness_centrality_subset(
    graph: &HashMap<u32, HashSet<u32>>,
    sources: &[u32],
) -> Vec<(u32, f64)> {
    let mut bc: HashMap<u32, f64> = graph.keys().map(|&v| (v, 0.0)).collect();
    for &s in sources {
        let (stack, pred, sigma) = bfs_paths(graph, s);
        accumulate(&stack, &pred, &sigma, &mut bc, s);
    }
    let mut vec: Vec<(u32, f64)> = bc.into_iter().collect();
    vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    vec
}

/// BFS phase helper: returns the visitation stack, predecessor lists, and path counts σ.
fn bfs_paths(
    graph: &HashMap<u32, HashSet<u32>>,
    source: u32,
) -> (Vec<u32>, HashMap<u32, Vec<u32>>, HashMap<u32, f64>) {
    let mut stack = Vec::new();
    let mut pred: HashMap<u32, Vec<u32>> = graph.keys().map(|&v| (v, Vec::new())).collect();
    let mut sigma: HashMap<u32, f64> = graph.keys().map(|&v| (v, 0.0)).collect();
    let mut dist: HashMap<u32, isize> = graph.keys().map(|&v| (v, -1)).collect();
    let mut queue = VecDeque::new();

    sigma.insert(source, 1.0);
    dist.insert(source, 0);
    queue.push_back(source);

    while let Some(v) = queue.pop_front() {
        stack.push(v);
        let d_v = dist[&v];
        for &w in graph.get(&v).unwrap_or(&HashSet::new()) {
            if dist[&w] < 0 {
                dist.insert(w, d_v + 1);
                queue.push_back(w);
            }
            if dist[&w] == d_v + 1 {
                *sigma.get_mut(&w).unwrap() += sigma[&v];
                pred.get_mut(&w).unwrap().push(v);
            }
        }
    }

    (stack, pred, sigma)
}

/// Accumulation phase helper: back-propagates dependencies to update `bc`.
fn accumulate(
    stack: &Vec<u32>,
    pred: &HashMap<u32, Vec<u32>>,
    sigma: &HashMap<u32, f64>,
    bc: &mut HashMap<u32, f64>,
    source: u32,
) {
    let mut delta: HashMap<u32, f64> = sigma.keys().map(|&v| (v, 0.0)).collect();
    for &w in stack.iter().rev() {
        for &v in &pred[&w] {
            let c = (sigma[&v] / sigma[&w]) * (1.0 + delta[&w]);
            *delta.get_mut(&v).unwrap() += c;
        }
        if w != source {
            *bc.get_mut(&w).unwrap() += delta[&w];
        }
    }
}