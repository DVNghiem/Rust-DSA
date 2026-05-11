# Invert Binary Tree (LeetCode #226)

## Problem Statement

Given the root of a binary tree, invert the tree and return the root.

## Examples

```
Input: root = [4, 2, 7, 1, 3, 6, 9]
Output: [4, 7, 2, 9, 6, 3, 1]

Input: root = [2, 1, 3]
Output: [2, 3, 1]
```

## Visual Walkthrough

```
Original:
       4                    Inverted:
      / \                         4
     2   7     →               / \
    / \   \                   7   2
   1   3   9                     \ / \
                              9   6   3
                                  / \
                                 1   3
```

## Implementation

```rust
pub fn invert_tree(root: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    if let Some(node) = root {
        let left = invert_tree(node.left);
        let right = invert_tree(node.right);
        Some(Box::new(TreeNode {
            val: node.val,
            left: right,
            right: left,
        }))
    } else {
        None
    }
}
```

## Complexity Analysis Table

| Approach | Time | Space |
|----------|------|-------|
| Recursive | O(n) | O(h) |
| Iterative | O(n) | O(w) |

## Key Takeaways

1. **Swap left and right** children
2. **Recursive** is simplest approach
3. **Base case** is empty tree
4. **Time O(n)** - visit each node once

## Real-World Applications

1. **Mirror reflection**: Flip binary structures
2. **Perspective transformation**: Change viewing angle
3. **Image processing**: Flip operations
4. **XML/HTML parsing**: Invert nested structures
5. **Network routing**: Reverse paths