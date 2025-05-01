#[cfg(test)]
mod tests {
    use std::collections::{HashSet, HashMap};
    use crate::scc;

    #[test]
    fn test_scc() {
        let mut graph: HashMap<u32, HashSet<u32>> = HashMap::new();
        graph.entry(1).or_insert(HashSet::new()).insert(2);
        graph.entry(2).or_insert(HashSet::new()).insert(3);
        graph.entry(3).or_insert(HashSet::new()).insert(1);
        graph.entry(4).or_insert(HashSet::new()).insert(5);
        graph.entry(5).or_insert(HashSet::new()).insert(4);

        let sccs = scc::kosaraju_scc(&graph);

        // Convert output into sets for easier comparison if needed
        let scc_sets: Vec<HashSet<u32>> = sccs.into_iter()
            .map(|v| v.into_iter().collect())
            .collect();

        let expected: Vec<HashSet<u32>> = vec![
            HashSet::from([1, 2, 3]),
            HashSet::from([4, 5]),
        ];

        assert_eq!(scc_sets.len(), expected.len());
        for s in expected {
            assert!(scc_sets.contains(&s));
        }
    }
}
