# Maximum Depth of Binary Tree (LeetCode #104)

## Problem Statement

Given the root of a binary tree, return its maximum depth.

A binary tree's maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.

## Examples

```
Input: root = [3, 9, 20, null, null, 15, 7]
Output: 3

Input: root = [1, null, 2]
Output: 2
```

## Visual Walkthrough

```
Binary Tree:
       3
      / \
     9   20
        /  \
       15   7

Depth = 3 (3 → 20 → 7 is longest path)
```

## Implementation

```rust
pub fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
    if let Some(node) = root {
        let left_depth = max_depth(node.left);
        let right_depth = max_depth(node.right);
        1 + left_depth.max(right_depth)
    } else {
        0
    }
}
```

## Complexity Analysis Table

| Approach | Time | Space |
|----------|------|-------|
| Recursive | O(n) | O(h) |
| Iterative | O(n) | O(w) |

## Key Takeaways

1. **Depth** = number of nodes from root to deepest leaf
2. **Base case** is empty tree (depth 0)
3. **Formula**: 1 + max(left_depth, right_depth)
4. **Time O(n)** - visit each node once

## Real-World Applications

1. **Tree height**: Used in AVL tree balance
2. **File system**: Directory nesting depth
3. **XML/JSON parsing**: Nesting level
4. **Organization charts**: Hierarchy depth
5. **Decision trees**: Depth for pruning