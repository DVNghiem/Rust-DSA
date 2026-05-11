# Clone Graph - LeetCode 133

## Problem Statement

Given a reference of a node in a connected undirected graph.

Return a deep copy (clone) of the graph.

Each node in the graph contains a value (int) and a list (List[Node]) of its neighbors.

## Visual Walkthrough

```
Example:
Input: adjList = [
  [1,2],
  [0,2],
  [0,1]
]

Graph structure:
    1 --- 2
    |     |
    0 ---+

Node 0: val=0, neighbors=[1,2]
Node 1: val=1, neighbors=[0,2]
Node 2: val=2, neighbors=[0,1]

Clone must be identical structure but different memory.
```

### Cloning Process

```
Original:          Clone:
    1                1'
    |                |
    0 -- 2          0'-- 2'

Each node is new, each edge is new.
```

### BFS Cloning Approach

```
1. Use HashMap to track: original_node -> cloned_node
2. BFS from given node
3. For each node:
   - If not cloned yet, create new node
   - Clone all neighbors
   - Add edges between cloned nodes
```

## Approach Comparison

| Approach | Time | Space | Description |
|----------|------|-------|-------------|
| BFS + HashMap | O(V+E) | O(V) | Queue-based traversal |
| DFS + HashMap | O(V+E) | O(V) | Recursive traversal |
| Iterative DFS | O(V+E) | O(V) | Stack-based |

### Why HashMap?

- Need to track which original nodes have been cloned
- Avoid creating duplicate clones
- When we see a neighbor already cloned, just use existing clone

## Implementation Strategy

### BFS Approach

```rust
use std::collections::{HashMap, VecDeque};

// Definition for graph node
pub struct Node {
    pub val: i32,
    pub neighbors: Vec<Option<Box<Node>>>,
}

pub fn clone_graph(node: Option<Box<Node>>) -> Option<Box<Node>> {
    if node.is_none() { return None; }

    let mut map = HashMap::new();
    let mut queue = VecDeque::new();

    // Clone the start node
    let start = Box::new(Node {
        val: node.as_ref().unwrap().val,
        neighbors: vec![],
    });
    map.insert(0, start); // Use pointer or id as key - actually need Box address
}
```

## Edge Cases

1. **Empty graph**: Return None
2. **Single node**: Clone single node
3. **Disconnected components**: Shouldn't happen (connected graph per problem)
4. **Self-loop**: Node points to itself
5. **Multiple edges**: Same neighbor appears twice (unlikely but handle)

## Test Cases

1. Basic graph clone
2. Single node clone
3. Empty input
4. Linear graph
5. Star graph (center with many leaves)
6. Graph with self-loops

## Solution Explanation

### Key Insight

- Use BFS/DFS to traverse original graph
- Maintain HashMap: original_node → cloned_node
- When encountering a neighbor, check if it's been cloned
- If not, clone it and add to queue/map
- Connect cloned nodes appropriately

## Complexity Analysis

- **Time**: O(V + E) - visit each node and edge once
- **Space**: O(V) - HashMap and queue

## Follow-up Questions

1. What if graph is not connected?
2. Can you clone without HashMap (using recursion stack)?
3. How to handle very deep graphs?