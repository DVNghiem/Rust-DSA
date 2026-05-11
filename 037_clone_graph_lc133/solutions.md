# Clone Graph - Solution Analysis

## Problem Overview

Given a node in a connected undirected graph, create a deep copy (clone) of the entire graph. Each node has a value and list of neighbors. The clone must have identical structure but different memory addresses.

## Solution 1: BFS with HashMap

### Code Implementation

```rust
pub fn clone_graph_bfs(node: Option<Box<Node>>) -> Option<Box<Node>> {
    if node.is_none() { return None; }

    let mut map: HashMap<i32, Box<Node>> = HashMap::new();
    let mut queue = VecDeque::new();

    // Clone start node
    let start = Box::new(Node::new(node.as_ref().unwrap().val));
    map.insert(start.val, start);
    queue.push_back(node.unwrap());

    while let Some(mut orig_node) = queue.pop_front() {
        let cloned = map.get_mut(&orig_node.val).unwrap();

        for neighbor_opt in orig_node.neighbors.iter() {
            let neighbor = neighbor_opt.as_ref().ok_or(()).unwrap();

            // Clone neighbor if not seen
            if !map.contains_key(&neighbor.val) {
                let cloned_neighbor = Box::new(Node::new(neighbor.val));
                map.insert(neighbor.val, cloned_neighbor);
                queue.push_back(neighbor.as_ref().clone());
            }

            // Add neighbor to cloned node's neighbors
            let cloned_neighbor = map.get(&neighbor.val).unwrap().clone();
            cloned.neighbors.push(Some(cloned_neighbor));
        }
    }

    map.remove(&node.as_ref().unwrap().val)
}
```

### Line-by-Line Analysis

1. **`if node.is_none() { return None; }`**: Handle empty graph edge case.

2. **`let mut map: HashMap<i32, Box<Node>> = HashMap::new();`**: HashMap stores original node val → cloned node.

3. **`let start = Box::new(Node::new(node.as_ref().unwrap().val)); map.insert(start.val, start);`**: Clone the starting node and add to map.

4. **`queue.push_back(node.unwrap());`**: Add original start node to queue for processing.

5. **`while let Some(mut orig_node) = queue.pop_front() { ... }`**: BFS main loop.

6. **`let cloned = map.get_mut(&orig_node.val).unwrap();`**: Get the cloned version of current node.

7. **Neighborhood cloning**: For each neighbor in original node:
   - If neighbor not yet cloned, clone it and add to queue
   - Add the cloned neighbor to cloned node's neighbors list

8. **`map.remove(&node.as_ref().unwrap().val)`**: Return the cloned start node.

### BFS Visualization

```
Original graph:      HashMap after processing:
    1                      1 -> 1' (cloned)
    |                      0 -> 0' (cloned)
    0 -- 2                  2 -> 2' (cloned)

Start: clone 0, queue=[0]
Process 0: neighbors [1,2]
  - clone 1, queue=[1]
  - clone 2, queue=[2] (order might vary)
Process 1: neighbors [0,2]
  - 0 already cloned, just add neighbor
  - 2 already cloned, just add neighbor
Process 2: neighbors [0,1]
  - both already cloned, just add neighbors

Result: 0' with neighbors [1', 2'], etc.
```

## Solution 2: DFS with HashMap

```rust
pub fn clone_graph_dfs(node: Option<Box<Node>>) -> Option<Box<Node>> {
    if node.is_none() { return None; }

    fn dfs(node: &Box<Node>, map: &mut HashMap<i32, Box<Node>>) -> Box<Node> {
        if let Some(cloned) = map.get(&node.val) {
            return cloned.clone();
        }

        let mut cloned = Box::new(Node::new(node.val));
        map.insert(node.val, cloned.clone());

        for neighbor_opt in &node.neighbors {
            let neighbor = neighbor_opt.as_ref().unwrap();
            let cloned_neighbor = dfs(neighbor, map);
            cloned.neighbors.push(Some(cloned_neighbor));
        }

        cloned
    }

    let mut map = HashMap::new();
    dfs(node.as_ref().unwrap(), &mut map)
}
```

### DFS vs BFS

```
BFS: Explore level by level using queue
     - Good for finding shortest paths
     - Iterative, no recursion stack overflow

DFS: Go as deep as possible before backtracking
     - Natural for graph/tree cloning
     - Recursive, simpler code
     - May hit stack overflow on very deep graphs
```

## Solution 3: Index-Based Cloning

This approach uses a two-pass method:
1. First pass: collect all nodes and assign indices
2. Second pass: clone with proper neighbor mapping

```rust
pub fn clone_graph_v3(node: Option<Box<Node>>) -> Option<Box<Node>> {
    // First: get all nodes, build index mapping
    // Second: create cloned nodes, set up neighbors using index
    // Return clone of start node
}
```

## Complexity Comparison

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| BFS | O(V+E) | O(V) | Queue-based |
| DFS | O(V+E) | O(V) | Recursive |
| Index-Based | O(V+E) | O(V) | Two-pass |

## Key Insights

1. **HashMap is essential**: Without it, we'd create duplicate clones or lose track of what's been cloned.

2. **Value as key**: Since all node values are unique (per LeetCode definition), we can use val as HashMap key.

3. **Clone on first encounter**: When we first see a node, we clone it and add to processing queue. Subsequent encounters just use the existing clone.

4. **Clone neighbors before adding**: Ensure neighbor is in map before adding to current node's neighbor list.

## Test Case Analysis

### Test: `test_two_nodes_connected`

```
Adjacency: [[1], [0]]  (0 connected to 1)

Initial: map = {}, queue = []

Clone start node (0): map = {0: 0'}, queue = [0]

Process 0:
  neighbors: [1]
  1 not in map → clone 1, map = {0: 0', 1: 1'}, queue = [1]
  Add neighbor: 0'.neighbors = [1']

Process 1:
  neighbors: [0]
  0 already in map
  Add neighbor: 1'.neighbors = [0']

Result: map[0] (0') is the cloned graph ✓
```

## Edge Cases

1. **Self-loops**: Node pointing to itself - handled because neighbor cloning checks map before adding.

2. **Disconnected graph**: Problem states "connected", but our algorithm would handle it by traversing all reachable nodes.

3. **Single node**: Just clone that node with no neighbors.

## Follow-up Answers

**Q: Why use val as key instead of pointer?**
A: Box<Node> doesn't implement Eq/Hash easily. Since problem guarantees unique val for each node, using val is simpler.

**Q: Can we avoid HashMap?**
A: Use recursion stack for DFS and clone before exploring neighbors. But HashMap is cleaner and avoids duplicate work.

**Q: What if node values aren't unique?**
A: Would need to use pointer comparison or assign our own unique IDs to original nodes.

**Q: Space complexity O(V)?**
A: Yes, HashMap stores all V nodes. Could be O(V+E) if we also store edges, but we build clone's edges during cloning anyway.

**Q: Time O(V+E)?**
A: Each node cloned once O(V), each edge processed once O(E). Total O(V+E).