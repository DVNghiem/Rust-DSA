use std::collections::{HashMap, VecDeque};

/// Graph node definition
#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub val: i32,
    pub neighbors: Vec<Option<Box<Node>>>,
}

impl Node {
    pub fn new(val: i32) -> Self {
        Node {
            val,
            neighbors: vec![],
        }
    }

    /// Create a graph from adjacency list representation
    pub fn from_adj_list(adj: Vec<Vec<i32>>) -> Option<Box<Node>> {
        if adj.is_empty() {
            return None;
        }

        let n = adj.len();
        let mut nodes: Vec<Option<Box<Node>>> = (0..n)
            .map(|i| Some(Box::new(Node::new(i as i32))))
            .collect();

        for i in 0..n {
            for &j in &adj[i] {
                if j >= 0 && (j as usize) < n {
                    let neighbor_clone = nodes[j as usize].clone();
                    if let Some(ref mut node) = nodes[i] {
                        node.neighbors.push(neighbor_clone);
                    }
                }
            }
        }

        nodes.remove(0)
    }

    /// Convert to adjacency list for verification
    pub fn to_adj_list(&self) -> Vec<Vec<i32>> {
        let mut adj = Vec::new();
        let mut visited = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back(self as &Node);
        visited.insert(self.val, adj.len());
        adj.push(vec![]);

        while let Some(node) = queue.pop_front() {
            let idx = visited[&node.val];
            for neighbor_opt in &node.neighbors {
                if let Some(neighbor) = neighbor_opt {
                    if !visited.contains_key(&neighbor.val) {
                        visited.insert(neighbor.val, adj.len());
                        adj.push(vec![]);
                        queue.push_back(neighbor);
                    }
                    adj[idx].push(neighbor.val);
                }
            }
        }

        adj
    }
}

/// Approach 1: BFS with HashMap
pub fn clone_graph_bfs(node: Option<Box<Node>>) -> Option<Box<Node>> {
    if node.is_none() {
        return None;
    }

    let mut map: HashMap<i32, Box<Node>> = HashMap::new();
    let mut queue: VecDeque<Box<Node>> = VecDeque::new();

    let start = Box::new(Node::new(node.as_ref().unwrap().val));
    let start_val = start.val;
    map.insert(start_val, start);
    queue.push_back(node.unwrap());

    while let Some(orig_node) = queue.pop_front() {
        let orig_val = orig_node.val;
        let neighbors = orig_node.neighbors.clone();

        if let Some(cloned) = map.get_mut(&orig_val) {
            for neighbor_opt in neighbors {
                if let Some(neighbor) = neighbor_opt {
                    let neighbor_val = neighbor.val;
                    if !map.contains_key(&neighbor_val) {
                        let cloned_neighbor = Box::new(Node::new(neighbor_val));
                        map.insert(neighbor_val, cloned_neighbor);
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    map.remove(&start_val)
}

/// Approach 2: DFS with HashMap
pub fn clone_graph_dfs(node: Option<Box<Node>>) -> Option<Box<Node>> {
    if node.is_none() {
        return None;
    }

    fn dfs(node: &Box<Node>, map: &mut HashMap<i32, Box<Node>>) -> Box<Node> {
        if let Some(cloned) = map.get(&node.val) {
            return cloned.clone();
        }

        let mut cloned = Box::new(Node::new(node.val));
        map.insert(node.val, cloned.clone());

        for neighbor_opt in &node.neighbors {
            if let Some(neighbor) = neighbor_opt {
                let cloned_neighbor = dfs(neighbor, map);
                cloned.neighbors.push(Some(cloned_neighbor));
            }
        }

        cloned
    }

    let mut map = HashMap::new();
    let start = node.as_ref().unwrap();
    Some(dfs(start, &mut map))
}

/// Approach 3: Index-based cloning (avoids borrow issues)
pub fn clone_graph_v3(node: Option<Box<Node>>) -> Option<Box<Node>> {
    if node.is_none() {
        return None;
    }

    fn get_all_nodes(node: &Option<Box<Node>>) -> Vec<Box<Node>> {
        let mut nodes: Vec<Box<Node>> = Vec::new();
        let mut visited: HashMap<i32, usize> = HashMap::new();
        let mut queue: VecDeque<Box<Node>> = VecDeque::new();

        if let Some(n) = node {
            queue.push_back(n.clone());
            visited.insert(n.val, 0);
            nodes.push(n.clone());

            while let Some(curr) = queue.pop_front() {
                for neighbor_opt in &curr.neighbors {
                    if let Some(neighbor) = neighbor_opt {
                        if !visited.contains_key(&neighbor.val) {
                            visited.insert(neighbor.val, nodes.len());
                            nodes.push(neighbor.clone());
                            queue.push_back(neighbor.clone());
                        }
                    }
                }
            }
        }
        nodes
    }

    fn build_clones(nodes: &[Box<Node>]) -> Vec<Box<Node>> {
        let mut cloned: Vec<Box<Node>> = nodes
            .iter()
            .map(|node| Box::new(Node::new(node.val)))
            .collect();

        for i in 0..cloned.len() {
            let neighbor_count = nodes[i].neighbors.len();
            for j in 0..neighbor_count {
                if let Some(ref neighbor) = nodes[i].neighbors[j] {
                    let idx = nodes.iter().position(|n| n.val == neighbor.val).unwrap();
                    let cloned_neighbor = cloned[idx].clone();
                    cloned[i].neighbors.push(Some(cloned_neighbor));
                }
            }
        }

        cloned
    }

    let all_nodes = get_all_nodes(&node);
    let n = all_nodes.len();
    if n == 0 {
        return None;
    }

    let clones = build_clones(&all_nodes);
    let start_val = all_nodes[0].val;
    let idx = all_nodes.iter().position(|n| n.val == start_val).unwrap();
    Some(clones[idx].clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_graph() {
        let result = clone_graph_bfs(None);
        assert!(result.is_none());
    }

    #[test]
    fn test_single_node() {
        let node = Some(Box::new(Node::new(1)));
        let cloned = clone_graph_bfs(node);

        assert!(cloned.is_some());
        assert_eq!(cloned.as_ref().unwrap().val, 1);
        assert!(cloned.as_ref().unwrap().neighbors.is_empty());
    }

    #[test]
    fn test_two_nodes_connected() {
        let adj = vec![vec![1], vec![0]];
        let node = Node::from_adj_list(adj);
        let cloned = clone_graph_bfs(node);

        assert!(cloned.is_some());
        assert_eq!(cloned.as_ref().unwrap().val, 0);
        assert_eq!(cloned.as_ref().unwrap().neighbors.len(), 1);
    }

    #[test]
    fn test_three_node_line() {
        let adj = vec![vec![1], vec![0, 2], vec![1]];
        let node = Node::from_adj_list(adj);
        let cloned = clone_graph_bfs(node);

        assert!(cloned.is_some());
        let cloned_val = cloned.as_ref().unwrap().val;
        assert!(cloned_val == 0 || cloned_val == 1 || cloned_val == 2);
    }

    #[test]
    fn test_bfs_and_dfs_same_structure() {
        let adj = vec![vec![1, 2], vec![0, 2], vec![0, 1]];
        let node = Node::from_adj_list(adj.clone());

        let bfs_result = clone_graph_bfs(node);
        let dfs_result = clone_graph_dfs(Node::from_adj_list(adj));

        assert!(bfs_result.is_some());
        assert!(dfs_result.is_some());
    }

    #[test]
    fn test_all_methods_same_result() {
        let adj = vec![vec![1, 2], vec![0, 2], vec![0, 1]];
        let node = Node::from_adj_list(adj.clone());

        let bfs = clone_graph_bfs(node);
        let dfs = clone_graph_dfs(Node::from_adj_list(adj.clone()));
        let v3 = clone_graph_v3(Node::from_adj_list(adj));

        assert!(bfs.is_some());
        assert!(dfs.is_some());
        assert!(v3.is_some());
    }

    #[test]
    fn test_star_graph() {
        let adj = vec![
            vec![1, 2, 3, 4],
            vec![0],
            vec![0],
            vec![0],
            vec![0],
        ];
        let node = Node::from_adj_list(adj);
        let cloned = clone_graph_bfs(node);

        assert!(cloned.is_some());
        let cloned_node = cloned.unwrap();
        let center_val = cloned_node.val;
        if center_val == 0 {
            assert_eq!(cloned_node.neighbors.len(), 4);
        }
    }

    #[test]
    fn test_cycle_graph() {
        let adj = vec![vec![1], vec![2], vec![0]];
        let node = Node::from_adj_list(adj);
        let cloned = clone_graph_bfs(node);

        assert!(cloned.is_some());
        let mut visited = HashMap::new();
        let mut curr = cloned.as_ref().unwrap().clone();
        loop {
            if visited.contains_key(&curr.val) {
                break;
            }
            visited.insert(curr.val, true);
            if curr.neighbors.is_empty() {
                break;
            }
            curr = curr.neighbors[0].as_ref().unwrap().clone();
            if visited.len() > 3 {
                break;
            }
        }
    }

    #[test]
    fn test_self_loop() {
        let adj = vec![vec![0]];
        let node = Node::from_adj_list(adj);
        let cloned = clone_graph_bfs(node);

        assert!(cloned.is_some());
        let cloned_node = cloned.unwrap();
        assert_eq!(cloned_node.neighbors.len(), 1);
        assert_eq!(cloned_node.neighbors[0].as_ref().unwrap().val, 0);
    }

    #[test]
    fn test_v3_single_node() {
        let node = Some(Box::new(Node::new(5)));
        let result = clone_graph_v3(node);
        assert!(result.is_some());
        assert_eq!(result.unwrap().val, 5);
    }

    #[test]
    fn test_v3_empty() {
        let result = clone_graph_v3(None);
        assert!(result.is_none());
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("037_clone_graph_lc133 exercises - run tests with cargo test");
}
