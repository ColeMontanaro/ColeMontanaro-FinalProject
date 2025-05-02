#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use crate::connected_components::connected_components;

    /// Tests connected components on a small undirected graph.
    ///
    /// Graph structure:
    /// 1 - 2 - 3 (Component 1)
    /// 4 - 5     (Component 2)
    ///
    /// Expected Output:
    /// Two connected components: {1, 2, 3} and {4, 5}
    #[test]
    fn test_connected_components() {
        let mut graph: HashMap<u32, HashSet<u32>> = HashMap::new();
        
        // Build the graph:
        // Component 1: 1 - 2 - 3
        graph.entry(1).or_default().insert(2);
        graph.entry(2).or_default().extend([1, 3]);
        graph.entry(3).or_default().insert(2);
        
        // Component 2: 4 - 5
        graph.entry(4).or_default().insert(5);
        graph.entry(5).or_default().insert(4);

        // Call the connected_components function to detect components
        let components = connected_components(&graph);

        // Convert components to sets for easier comparison
        let component_sets: Vec<HashSet<u32>> = components.into_iter()
            .map(|v| v.into_iter().collect()) // Convert Vec<u32> to HashSet<u32>
            .collect();

        // Expected components as HashSets
        let expected_sets: Vec<HashSet<u32>> = vec![
            HashSet::from([1, 2, 3]),
            HashSet::from([4, 5]),
        ];

        // Assert that all expected components are found
        assert_eq!(component_sets.len(), expected_sets.len());
        for expected_set in expected_sets {
            assert!(component_sets.contains(&expected_set));
        }
    }

    /// Additional edge case: Test with a graph containing a single node.
    ///
    /// Graph structure:
    /// 1 (Component 1)
    ///
    /// Expected Output:
    /// One connected component: {1}
    #[test]
    fn test_single_node() {
        let mut graph: HashMap<u32, HashSet<u32>> = HashMap::new();
        
        // Single node, no edges
        graph.insert(1, HashSet::new());

        // Call the connected_components function
        let components = connected_components(&graph);

        // Convert components to sets for easier comparison
        let component_sets: Vec<HashSet<u32>> = components.into_iter()
            .map(|v| v.into_iter().collect()) // Convert Vec<u32> to HashSet<u32>
            .collect();

        // Expected components as HashSets
        let expected_sets: Vec<HashSet<u32>> = vec![
            HashSet::from([1]),
        ];

        // Assert that all expected components are found
        assert_eq!(component_sets.len(), expected_sets.len());
        for expected_set in expected_sets {
            assert!(component_sets.contains(&expected_set));
        }
    }

    /// Additional edge case: Test with a graph containing two disconnected nodes.
    ///
    /// Graph structure:
    /// 1 (Component 1)
    /// 2 (Component 2)
    ///
    /// Expected Output:
    /// Two connected components: {1}, {2}
    #[test]
    fn test_two_disconnected_nodes() {
        let mut graph: HashMap<u32, HashSet<u32>> = HashMap::new();
        
        // Two disconnected nodes
        graph.insert(1, HashSet::new());
        graph.insert(2, HashSet::new());

        // Call the connected_components function
        let components = connected_components(&graph);

        // Convert components to sets for easier comparison
        let component_sets: Vec<HashSet<u32>> = components.into_iter()
            .map(|v| v.into_iter().collect()) // Convert Vec<u32> to HashSet<u32>
            .collect();

        // Expected components as HashSets
        let expected_sets: Vec<HashSet<u32>> = vec![
            HashSet::from([1]),
            HashSet::from([2]),
        ];

        // Assert that all expected components are found
        assert_eq!(component_sets.len(), expected_sets.len());
        for expected_set in expected_sets {
            assert!(component_sets.contains(&expected_set));
        }
    }

    /// Additional edge case: Test with a graph where all nodes are connected in a single component.
    ///
    /// Graph structure:
    /// 1 - 2 - 3 - 4 (Single Component)
    ///
    /// Expected Output:
    /// One connected component: {1, 2, 3, 4}
    #[test]
    fn test_all_connected_nodes() {
        let mut graph: HashMap<u32, HashSet<u32>> = HashMap::new();
        
        // Build a single connected component: 1 - 2 - 3 - 4
        graph.entry(1).or_default().insert(2);
        graph.entry(2).or_default().extend([1, 3]);
        graph.entry(3).or_default().extend([2, 4]);
        graph.entry(4).or_default().insert(3);

        // Call the connected_components function
        let components = connected_components(&graph);

        // Convert components to sets for easier comparison
        let component_sets: Vec<HashSet<u32>> = components.into_iter()
            .map(|v| v.into_iter().collect()) // Convert Vec<u32> to HashSet<u32>
            .collect();

        // Expected components as HashSets
        let expected_sets: Vec<HashSet<u32>> = vec![
            HashSet::from([1, 2, 3, 4]),
        ];

        // Assert that all expected components are found
        assert_eq!(component_sets.len(), expected_sets.len());
        for expected_set in expected_sets {
            assert!(component_sets.contains(&expected_set));
        }
    }
}