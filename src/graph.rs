// Module: graph
// Purpose: Loads an undirected edge list from a file into an adjacency list.

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Type alias: Graph represented as `node -> set of neighbors`.
pub type Graph = HashMap<u32, HashSet<u32>>;

/// Reads an edge‑list file and builds a bidirectional graph.
///
/// Inputs:
/// - `path`: path to edge list file (lines "u v", comments start with '#')
///
/// Output:
/// - `Graph`: adjacency list
///
/// Logic:
/// - Parse each valid line and insert edges both ways.
pub fn load_graph(path: &str) -> Graph {
    let file = File::open(path).expect("Failed to open graph file");
    let reader = BufReader::new(file);
    let mut graph: Graph = HashMap::new();

    for line in reader.lines().flatten() {
        if line.starts_with('#') {
            continue;
        }
        let parts: Vec<u32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();
        if parts.len() == 2 {
            let (u, v) = (parts[0], parts[1]);
            graph.entry(u).or_default().insert(v);
            graph.entry(v).or_default().insert(u);
        }
    }

    graph
}