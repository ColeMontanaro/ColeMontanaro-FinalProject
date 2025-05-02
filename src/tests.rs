// src/tests.rs
// Module: tests
// Purpose: Unit tests for connected components, clustering, and subset‑based centralities.

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use crate::connected_components::connected_components;
    use crate::clustering::clustering_coefficient;
    use crate::centrality_closeness::closeness_centrality_subset;
    use crate::centrality_betweenness::betweenness_centrality_subset;

    /// Tests connected_components on a small undirected graph.
    #[test]
    fn test_connected_components() {
        let mut graph: HashMap<u32, HashSet<u32>> = HashMap::new();
        graph.entry(1).or_default().insert(2);
        graph.entry(2).or_default().extend([1, 3]);
        graph.entry(3).or_default().insert(2);
        graph.entry(4).or_default().insert(5);
        graph.entry(5).or_default().insert(4);

        let comps = connected_components(&graph);
        let sets: Vec<HashSet<u32>> =
            comps.into_iter().map(|v| v.into_iter().collect()).collect();
        assert!(sets.contains(&HashSet::from([1, 2, 3])));
        assert!(sets.contains(&HashSet::from([4, 5])));
    }

    /// Tests clustering_coefficient on a triangle graph.
    #[test]
    fn test_clustering_triangle() {
        let mut g: HashMap<u32, HashSet<u32>> = HashMap::new();
        for &(u, v) in &[(1, 2), (2, 3), (3, 1)] {
            g.entry(u).or_default().insert(v);
            g.entry(v).or_default().insert(u);
        }
        let coeffs = clustering_coefficient(&g);
        for &(_, c) in &coeffs {
            assert!((c - 1.0).abs() < 1e-6);
        }
    }

    /// Tests closeness_centrality_subset on a 3-node line: 1–2–3.
    #[test]
    fn test_closeness_line() {
        let mut g: HashMap<u32, HashSet<u32>> = HashMap::new();
        g.entry(1).or_default().insert(2);
        g.entry(2).or_default().extend([1, 3]);
        g.entry(3).or_default().insert(2);
        let sources = vec![1, 2, 3];
        let close = closeness_centrality_subset(&g, &sources);
        assert_eq!(close[0].0, 2);
    }

    /// Tests betweenness_centrality_subset on a 3-node line: 1–2–3.
    #[test]
    fn test_betweenness_line() {
        let mut g: HashMap<u32, HashSet<u32>> = HashMap::new();
        g.entry(1).or_default().insert(2);
        g.entry(2).or_default().extend([1, 3]);
        g.entry(3).or_default().insert(2);
        let sources = vec![1, 2, 3];
        let bc = betweenness_centrality_subset(&g, &sources);
        assert_eq!(bc[0].0, 2);
        assert!(bc[0].1 > 0.0);
    }
}