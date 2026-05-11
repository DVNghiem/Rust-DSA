# Solutions: Maximum Depth of Binary Tree (LeetCode #104)

This document provides detailed, line-by-line analysis of each exercise implementation.

---

## Exercise 1: Maximum Depth - Recursive

### The Solution

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

### Line-by-Line Analysis

```rust
if let Some(node) = root {
```
**Purpose:** Base case - empty tree has depth 0.

```rust
let left_depth = max_depth(node.left);
let right_depth = max_depth(node.right);
```
**Purpose:** Recursively calculate depth of left and right subtrees.

```rust
1 + left_depth.max(right_depth)
```
**Purpose:** Current depth is 1 (for current node) plus the max of left and right depths.

### Complexity Analysis

| Metric | Value | Explanation |
|--------|-------|-------------|
| **Time** | O(n) | Visit each node exactly once |
| **Space** | O(h) | Recursion depth equals tree height |

---

## Exercise 5: Diameter of Binary Tree

### The Solution

```rust
pub fn diameter_of_binary_tree(root: Option<Box<TreeNode>>) -> i32 {
    let mut diameter = 0;

    fn height_and_diameter(node: &Option<Box<TreeNode>>, dia: &mut i32) -> i32 {
        if let Some(n) = node {
            let left_h = height_and_diameter(&n.left, dia);
            let right_h = height_and_diameter(&n.right, dia);

            *dia = (*dia).max(left_h + right_h);

            1 + left_h.max(right_h)
        } else {
            0
        }
    }

    height_and_diameter(&root, &mut diameter);
    diameter
}
```

### Key Insight

The diameter is the longest path between any two nodes, which can pass through any node. For each node, the diameter passing through it is `left_height + right_height`. We track the maximum across all nodes.

---

## Summary Table

| Exercise | Time | Space | Key Technique |
|----------|------|-------|---------------|
| 1: Max Depth | O(n) | O(h) | Recursion |
| 2: Iterative | O(n) | O(w) | BFS |
| 3: Min Depth | O(n) | O(h) | Recursion |
| 4: Balanced | O(n) | O(h) | Height check |
| 5: Diameter | O(n) | O(h) | Track max path |
| 6: Max Width | O(n) | O(w) | Index positions |

## Key Takeaways

1. **Depth** = nodes on longest root-to-leaf path
2. **Formula**: 1 + max(left_depth, right_depth)
3. **Diameter** = max(left_height + right_height) across all nodes
4. **Balanced** = |left_depth - right_depth| <= 1 for all nodes
5. **Height** used to compute many tree properties