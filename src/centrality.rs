use std::collections::{HashMap, VecDeque, HashSet};

pub fn degree_centrality(graph: &HashMap<u32, HashSet<u32>>) -> Vec<(u32, usize)> {
    let mut degrees: Vec<_> = graph.iter().map(|(&node, neighbors)| (node, neighbors.len())).collect();
    degrees.sort_by(|a, b| b.1.cmp(&a.1));
    degrees
}

pub fn bfs(graph: &HashMap<u32, HashSet<u32>>, start: u32) -> HashMap<u32, usize> {
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();
    distances.insert(start, 0);
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        let dist = distances[&node];
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if !distances.contains_key(&neighbor) {
                    distances.insert(neighbor, dist + 1);
                    queue.push_back(neighbor);
                }
            }
        }
    }

    distances
}
