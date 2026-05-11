# Diameter of Binary Tree (LeetCode #543)

## Problem Statement

Given the root of a binary tree, return the length of the diameter of the tree.

The diameter of a binary tree is the length of the longest path between any two nodes in the tree. This path may or may not pass through the root.

## Examples

```
Input: root = [1,2,3,4,5]
Output: 3
Explanation: 3 is the length of the path [4,2,1,3] or [5,2,1,3].

Input: root = [1,2]
Output: 1
```

## Visual Walkthrough

```
Binary Tree:
        1
       / \
      2   3
     / \
    4   5

Diameter = 3 (path: 4 → 2 → 1 → 3)

For node 2:
  - Left height: 1 (node 4)
  - Right height: 1 (node 5)
  - Diameter through node 2: 1 + 1 = 2

For root node 1:
  - Left height: 2 (node 2 → node 4)
  - Right height: 1 (node 3)
  - Diameter through root: 2 + 1 = 3

Maximum = 3
```

## Key Insights

1. **Diameter Definition**: The longest path between any two nodes
2. **Node Contribution**: For any node, the longest path passing through it = left_height + right_height
3. **Post-order Traversal**: We need to compute heights from bottom up
4. **Global Tracking**: Keep track of maximum diameter seen during traversal

## Implementation Approaches

### Approach 1: Recursive with Global Tracking (Optimal)

```rust
pub fn diameter_of_binary_tree(root: Option<Box<TreeNode>>) -> i32 {
    let mut diameter = 0;
    fn height(node: &Option<Box<TreeNode>>, dia: &mut i32) -> i32 {
        if let Some(n) = node {
            let left_h = height(&n.left, dia);
            let right_h = height(&n.right, dia);
            *dia = (*dia).max(left_h + right_h);
            1 + left_h.max(right_h)
        } else {
            0
        }
    }
    height(&root, &mut diameter);
    diameter
}
```

### Approach 2: Two DFS Passes

1. First DFS to find longest path endpoints
2. Second DFS to compute distance

### Approach 3: BFS with Level Tracking

Use level-order traversal to track distances.

### Approach 4: Iterative with Stack

Simulate post-order traversal using an explicit stack.

## Complexity Analysis Table

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Recursive | O(n) | O(h) | Optimal - single pass |
| Two DFS | O(n) | O(h) | Simpler but two passes |
| BFS | O(n) | O(w) | Level-order tracking |
| Iterative | O(n) | O(h) | Avoids recursion stack |

## Related Problems

- LeetCode #104: Maximum Depth of Binary Tree
- LeetCode #110: Balanced Binary Tree
- LeetCode #124: Binary Tree Maximum Path Sum

## Real-World Applications

1. **Network routing**: Finding longest path in a network topology
2. **Genealogy**: Finding longest ancestral path between two individuals
3. **Organizational charts**: Finding maximum reporting chain length