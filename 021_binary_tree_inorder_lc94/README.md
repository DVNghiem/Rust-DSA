# Binary Tree Inorder Traversal (LeetCode #94)

## Problem Statement

Given the root of a binary tree, return the inorder traversal of its nodes' values.

## Examples

```
Input: root = [1, null, 2, 3]
Output: [1, 3, 2]

Input: root = [1, 2, 3, 4, 5, 6, 7]
Output: [4, 2, 5, 1, 6, 3, 7]
```

## Tree Structure

```rust
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}
```

## Inorder Traversal

Visit order: Left -> Root -> Right

```
Binary Tree:
       1
      / \
     2   3
    / \   \
   4   5   6

Inorder: 4 -> 2 -> 5 -> 1 -> 6 -> 3 -> 7
```

## Approaches Overview

### Approach 1: Recursive O(n)

```rust
pub fn inorder_traversal(root: Option<Box<TreeNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    inorder_recursive(&root, &mut result);
    result
}

fn inorder_recursive(node: &Option<Box<TreeNode>>, result: &mut Vec<i32>) {
    if let Some(n) = node {
        inorder_recursive(&n.left, result);
        result.push(n.val);
        inorder_recursive(&n.right, result);
    }
}
```

### Approach 2: Iterative O(n)

```rust
pub fn inorder_iterative(root: Option<Box<TreeNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut stack = Vec::new();
    let mut current = root;

    while current.is_some() || !stack.is_empty() {
        while let Some(node) = current {
            stack.push(node);
            current = node.left;
        }
        let node = stack.pop().unwrap();
        result.push(node.val);
        current = node.right;
    }
    result
}
```

## Complexity Analysis Table

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Recursive | O(n) | O(h) | h = height, stack depth |
| Iterative | O(n) | O(h) | Explicit stack |

## Key Takeaways

1. **Inorder**: Left, Root, Right
2. **Recursive** is simplest implementation
3. **Iterative** uses explicit stack to avoid recursion
4. **Time O(n)** - visit each node once
5. **Space O(h)** - height of tree

## Real-World Applications

1. **Binary Search Tree**: Sorted order traversal
2. **Expression trees**: Infix notation
3. **Syntax trees**: In-order for proper formatting
4. **Game trees**: Evaluation order
5. **File systems**: Directory traversal