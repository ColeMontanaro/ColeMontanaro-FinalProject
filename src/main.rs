// Module: main
// Purpose: Coordinates program flow—loading graph, finding components, computing degree centrality.

mod graph;
mod connected_components;
mod centrality;
mod tests;

use crate::graph::load_graph;
use crate::connected_components::connected_components;
use crate::centrality::degree_centrality;

fn main() {
    let path = "data/com-amazon.ungraph.txt";  // Path to the graph data file

    let graph = load_graph(path);  // Load the graph
    println!("Loaded graph with {} nodes", graph.len());

    // Find connected components (not SCCs, since the graph is undirected)
    let components = connected_components(&graph);
    println!("Found {} connected components", components.len());

    // Largest component size
    if let Some(largest) = components.iter().max_by_key(|c| c.len()) {
        println!("Largest connected component size: {}", largest.len());
    }

    // Degree centrality
    let centrality = degree_centrality(&graph);
    println!("Top 5 nodes by degree:");
    for (node, degree) in centrality.iter().take(5) {  // Display top 5 nodes by degree
        println!("Node {}: degree {}", node, degree);
    }
}