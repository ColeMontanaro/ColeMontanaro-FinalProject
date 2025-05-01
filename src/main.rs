mod graph;
mod centrality;
mod scc;
mod community;
mod tests; // Add this

fn main() {
    let graph = graph::load_graph("data/com-amazon.ungraph.txt");
    println!("Loaded graph with {} nodes", graph.len());

    // SCC
    let sccs = scc::kosaraju_scc(&graph);
    println!("Found {} strongly connected components", sccs.len());

    let largest = sccs.iter().max_by_key(|c| c.len()).unwrap();
    println!("Largest SCC size: {}", largest.len());

    // Centrality
    let top_degrees = centrality::degree_centrality(&graph);
    println!("Top 5 nodes by degree:");
    for (node, degree) in top_degrees.iter().take(5) {
        println!("Node {}: degree {}", node, degree);
    }
}