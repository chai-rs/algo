#![allow(dead_code)]

use std::collections::{HashMap, VecDeque};

pub fn shortest_path(graph: Vec<Vec<usize>>, from: usize, to: usize) -> Option<usize> {
    // Handle same node case
    if from == to {
        return Some(0);
    }

    let get_neighbors =
        |from: usize| -> &[usize] { graph.get(from).expect("neighbors out of range") };

    let mut visitor: HashMap<usize, bool> = HashMap::new();
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut level: usize = 0;

    visitor.insert(from, true);
    queue.push_back(from);

    while !queue.is_empty() {
        let mut new_queue = VecDeque::new();
        while !queue.is_empty() {
            let cur = queue.pop_front().expect("queue is empty");
            if cur == to {
                return Some(level);
            }

            let neighbors = get_neighbors(cur);
            for n in neighbors {
                if !visitor.contains_key(n) {
                    new_queue.push_back(*n);
                    visitor.insert(*n, true);
                }
            }
        }

        queue = new_queue;
        level += 1;
    }

    // No path found
    None
}

#[cfg(test)]
mod test {
    use crate::shortest_path;

    #[test]
    fn test_direct_connection() {
        // 0 -> 1
        let graph = vec![vec![1], vec![0]];
        let result = shortest_path(graph, 0, 1);
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_same_node() {
        // 0 -> 1, 1 -> 0
        let graph = vec![vec![1], vec![0]];
        let result = shortest_path(graph, 0, 0);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_two_hops() {
        // 0 -> 1 -> 2
        let graph = vec![vec![1], vec![0, 2], vec![1]];
        let result = shortest_path(graph, 0, 2);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_three_hops() {
        // 0 -> 1 -> 2 -> 3
        let graph = vec![vec![1], vec![0, 2], vec![1, 3], vec![2]];
        let result = shortest_path(graph, 0, 3);
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_multiple_paths_chooses_shortest() {
        // 0 -> 1 -> 3 (2 hops)
        // 0 -> 2 -> 1 -> 3 (3 hops)
        let graph = vec![vec![1, 2], vec![0, 2, 3], vec![0, 1], vec![1]];
        let result = shortest_path(graph, 0, 3);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_complex_graph() {
        // Graph with multiple paths
        //     1 --- 3
        //    / \     \
        //   0   2 --- 4
        let graph = vec![
            vec![1],       // 0 connects to 1
            vec![0, 2, 3], // 1 connects to 0, 2, 3
            vec![1, 4],    // 2 connects to 1, 4
            vec![1, 4],    // 3 connects to 1, 4
            vec![2, 3],    // 4 connects to 2, 3
        ];
        let result = shortest_path(graph, 0, 4);
        assert_eq!(result, Some(3)); // 0 -> 1 -> 2 -> 4
    }

    #[test]
    fn test_disconnected_graph() {
        // 0 -> 1, 2 -> 3 (disconnected components)
        let graph = vec![vec![1], vec![0], vec![3], vec![2]];
        let result = shortest_path(graph, 0, 3);
        // No path exists between disconnected components
        assert_eq!(result, None);
    }

    #[test]
    fn test_star_graph() {
        // Center node 0 connected to all others
        //   1
        //   |
        // 2-0-3
        //   |
        //   4
        let graph = vec![
            vec![1, 2, 3, 4], // 0 connects to all
            vec![0],          // 1 connects to 0
            vec![0],          // 2 connects to 0
            vec![0],          // 3 connects to 0
            vec![0],          // 4 connects to 0
        ];
        let result = shortest_path(graph, 1, 4);
        assert_eq!(result, Some(2)); // 1 -> 0 -> 4
    }

    #[test]
    fn test_cycle_graph() {
        // 0 -> 1 -> 2 -> 3 -> 0 (cycle)
        let graph = vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![2, 0]];
        let result = shortest_path(graph, 0, 2);
        assert_eq!(result, Some(2)); // 0 -> 1 -> 2
    }

    #[test]
    fn test_complete_graph() {
        // Every node connected to every other node
        let graph = vec![vec![1, 2, 3], vec![0, 2, 3], vec![0, 1, 3], vec![0, 1, 2]];
        let result = shortest_path(graph, 0, 3);
        assert_eq!(result, Some(1)); // Direct connection
    }
}
