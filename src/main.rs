// src/main.rs
// Module: main
// Purpose: Loads the graph and runs connected components, degree, clustering, and subset‑based centralities.

mod graph;
mod connected_components;
mod centrality;
mod clustering;
mod centrality_closeness;
mod centrality_betweenness;
mod tests;

use crate::graph::load_graph;
use crate::connected_components::connected_components;
use crate::centrality::degree_centrality;
use crate::clustering::{average_clustering, clustering_coefficient};
use crate::centrality_closeness::closeness_centrality_subset;
use crate::centrality_betweenness::betweenness_centrality_subset;

fn main() {
    let path = "data/com-amazon.ungraph.txt";
    let graph = load_graph(path);
    println!("Loaded graph with {} nodes", graph.len());

    // Connected components
    let comps = connected_components(&graph);
    println!("Found {} connected components", comps.len());
    let largest = comps.iter().max_by_key(|c| c.len()).unwrap();
    println!("Largest component size: {}", largest.len());

    // Degree centrality → pick top 50
    let deg = degree_centrality(&graph);
    println!("\nTop 5 nodes by degree:");
    for (n, d) in deg.iter().take(5) {
        println!("Node {}: degree {}", n, d);
    }
    let top_n = 50;
    let sources: Vec<u32> = deg.iter().take(top_n).map(|(n, _)| *n).collect();

    // Clustering metrics (full graph)
    let avg_c = average_clustering(&graph);
    println!("\nAverage clustering coefficient: {:.4}", avg_c);
    let local = clustering_coefficient(&graph);
    let (mx_node, mx_val) = local
        .iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap();
    println!("Node {} has highest local clustering: {:.4}", mx_node, mx_val);

    // Closeness for top 50
    let close = closeness_centrality_subset(&graph, &sources);
    println!("\nCloseness centrality (top {}):", top_n);
    for (n, c) in close.iter().take(5) {
        println!("Node {}: {:.6}", n, c);
    }

    // Betweenness for top 50
    let between = betweenness_centrality_subset(&graph, &sources);
    println!("\nBetweenness centrality (top {}):", top_n);
    for (n, b) in between.iter().take(5) {
        println!("Node {}: {:.6}", n, b);
    }
}