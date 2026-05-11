# Binary Tree Level Order Traversal (LeetCode #102)

## Problem Statement

Given the root of a binary tree, return the level order traversal of its nodes' values. (i.e., from left to right, level by level).

## Examples

```
Input: root = [3,9,20,null,null,15,7]
Output: [[3], [9,20], [15,7]]

Input: root = [1]
Output: [[1]]

Input: root = []
Output: []
```

## Visual Walkthrough

```
Binary Tree:
       3
      / \
     9   20
        / \
       15   7

Level 0: [3]
Level 1: [9, 20]
Level 2: [15, 7]

Result: [[3], [9, 20], [15, 7]]
```

## Key Insights

1. **BFS Pattern**: Level-order traversal naturally uses BFS (queue)
2. **Level Separation**: Track nodes at each level separately
3. **Size Snapshot**: Process all nodes at current level before moving to next
4. **Result Structure**: Vec<Vec<i32>> where each inner Vec is one level

## Implementation Approaches

### Approach 1: BFS with Queue (Optimal)

```rust
pub fn level_order(root: Option<Box<TreeNode>>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    if root.is_none() {
        return result;
    }
    let mut queue = VecDeque::new();
    queue.push_back(root.unwrap());
    
    while !queue.is_empty() {
        let level_size = queue.len();
        let mut level_vals = Vec::new();
        
        for _ in 0..level_size {
            let node = queue.pop_front().unwrap();
            level_vals.push(node.val);
            if let Some(left) = node.left {
                queue.push_back(*left);
            }
            if let Some(right) = node.right {
                queue.push_back(*right);
            }
        }
        result.push(level_vals);
    }
    result
}
```

### Approach 2: Recursive with Level Tracking

Pass current level index and extend result vector accordingly.

### Approach 3: DFS Preorder Variant

Use preorder traversal with level parameter.

## Complexity Analysis Table

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| BFS Queue | O(n) | O(w) | Optimal, level-by-level |
| Recursive | O(n) | O(h) | Uses recursion stack |
| DFS | O(n) | O(h) | Preorder with tracking |

## Variations

1. **Zigzag Traversal** (LeetCode #103): Alternate left-to-right and right-to-left
2. **Level Averages** (LeetCode #637): Return average at each level
3. **Right Side View** (LeetCode #199): Return rightmost node at each level

## Related Problems

- LeetCode #107: Binary Tree Level Order Traversal II (bottom-up)
- LeetCode #103: Zigzag Level Order Traversal
- LeetCode #199: Binary Tree Right Side View

## Real-World Applications

1. **Hierarchical data display**: organizational charts
2. **File system traversal**: directory structure
3. **Game AI**: breadth-first search in game trees