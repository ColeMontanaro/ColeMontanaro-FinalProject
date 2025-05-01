use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub type Graph = HashMap<u32, HashSet<u32>>;

/// Loads an undirected graph from the Amazon dataset.
pub fn load_graph(path: &str) -> Graph {
    let file = File::open(path).expect("File not found");
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
