# Solutions: Invert Binary Tree (LeetCode #226)

This document provides detailed, line-by-line analysis of each exercise implementation.

---

## Exercise 1: Invert Tree - Recursive

### The Solution

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

### Line-by-Line Analysis

```rust
if let Some(node) = root {
```
**Purpose:** Base case - if node is None, return None.

```rust
let left = invert_tree(node.left);
let right = invert_tree(node.right);
```
**Purpose:** Recursively invert left and right subtrees. Store results.

```rust
Some(Box::new(TreeNode {
    val: node.val,
    left: right,
    right: left,
}))
```
**Purpose:** Create new node with same value but swapped children.

### Complexity Analysis

| Metric | Value | Explanation |
|--------|-------|-------------|
| **Time** | O(n) | Visit each node exactly once |
| **Space** | O(h) | Recursion depth equals tree height |

---

## Exercise 4: Symmetric Check

### The Solution

```rust
pub fn is_symmetric(root: Option<Box<TreeNode>>) -> bool {
    fn mirror(t1: &Option<Box<TreeNode>>, t2: &Option<Box<TreeNode>>) -> bool {
        match (t1, t2) {
            (None, None) => true,
            (Some(n1), Some(n2)) => {
                n1.val == n2.val &&
                mirror(&n1.left, &n2.right) &&
                mirror(&n1.right, &n2.left)
            }
            _ => false,
        }
    }
    mirror(&root, &root)
}
```

### Key Insight

A tree is symmetric if left subtree is mirror of right subtree:
- Left child's left matches right child's right
- Left child's right matches right child's left

---

## Summary Table

| Exercise | Time | Space | Key Technique |
|----------|------|-------|---------------|
| 1: Invert Recursive | O(n) | O(h) | Swap children |
| 2: Invert Iterative | O(n) | O(w) | BFS swap |
| 3: In Place | O(n) | O(h) | Modify in place |
| 4: Symmetric | O(n) | O(h) | Mirror check |
| 5: Same Tree | O(n) | O(h) | Compare nodes |

## Key Takeaways

1. **Swap left and right** recursively
2. **Base case** is empty tree
3. **Symmetric** = left mirrors right
4. **Same tree** = both sides identical
5. **Time O(n)** for all operations