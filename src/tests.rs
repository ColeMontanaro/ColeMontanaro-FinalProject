#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashSet, HashMap};

    #[test]
    fn test_scc() {
        // Smaller test graph
        let mut graph: HashMap<u32, HashSet<u32>> = HashMap::new();
        
        // Add some test nodes and edges
        graph.entry(1).or_insert(HashSet::new()).insert(2);
        graph.entry(2).or_insert(HashSet::new()).insert(3);
        graph.entry(3).or_insert(HashSet::new()).insert(1);
        graph.entry(4).or_insert(HashSet::new()).insert(5);
        graph.entry(5).or_insert(HashSet::new()).insert(4);
        
        // Call the SCC function
        let sccs = scc::kosaraju_scc(&graph);
        
        // Expected output: Two strongly connected components
        let expected_sccs: Vec<HashSet<u32>> = vec![
            HashSet::from([1, 2, 3]),
            HashSet::from([4, 5]),
        ];
        
        // Assert the result
        assert_eq!(sccs, expected_sccs);
    }
}