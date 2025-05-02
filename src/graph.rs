// Module: graph
// Purpose: Loads an undirected graph from a file into an adjacency list structure.

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Type alias for graph structure as an adjacency list.
/// Maps each node ID to a set of its neighbors.
pub type Graph = HashMap<u32, HashSet<u32>>;

/// Loads an undirected graph from a text file of node pairs.

/// # Inputs:
/// * `path` - Path to the file containing the edge list.

/// # Output:
/// * A graph as a HashMap from node ID to a set of neighbor node IDs.

/// # Logic:
/// Reads file line-by-line, ignores comment lines (starting with '#'), splits node pairs,
/// and inserts edges bidirectionally to represent an undirected graph.
pub fn load_graph(path: &str) -> Graph {
    let file = File::open(path).expect("File not found");
    let reader = BufReader::new(file);
    let mut graph: Graph = HashMap::new();

    for line in reader.lines().flatten() {  // Read each line of the file
        if line.starts_with('#') {  // Skip comment lines
            continue;
        }

        // Parse integers from line
        let parts: Vec<u32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        if parts.len() == 2 {  // Ensure each line represents an edge with two nodes
            let (u, v) = (parts[0], parts[1]);

            // Insert edge in both directions for undirected graph
            graph.entry(u).or_default().insert(v);
            graph.entry(v).or_default().insert(u);
        }
    }
    graph
}