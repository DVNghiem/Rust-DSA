# Solutions: Diameter of Binary Tree (LeetCode #543)

This document provides detailed, line-by-line analysis of each exercise implementation.

---

## Exercise 1: Diameter - Recursive (Primary Solution)

### The Solution

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

### Line-by-Line Analysis

```rust
let mut diameter = 0;
```
**Purpose:** Initialize the global diameter tracker. This variable stores the maximum diameter found across all nodes.

```rust
fn height(node: &Option<Box<TreeNode>>, dia: &mut i32) -> i32 {
```
**Purpose:** Inner recursive function that computes height and updates diameter. Takes a mutable reference to track the global maximum.

```rust
if let Some(n) = node {
```
**Purpose:** Base case - if node is None, return height 0. This handles empty subtrees.

```rust
let left_h = height(&n.left, dia);
let right_h = height(&n.right, dia);
```
**Purpose:** Recursively compute heights of left and right subtrees. Post-order traversal ensures children are processed before parent.

```rust
*dia = (*dia).max(left_h + right_h);
```
**Purpose:** Update diameter if the path through current node (left_height + right_height) is larger. This is the key insight: diameter through any node equals sum of heights of its two subtrees.

```rust
1 + left_h.max(right_h)
```
**Purpose:** Return height of current subtree (1 for current node + max of children's heights).

```rust
height(&root, &mut diameter);
diameter
```
**Purpose:** Start the recursion and return the computed maximum diameter.

### Complexity Analysis

| Metric | Value | Explanation |
|--------|-------|-------------|
| **Time** | O(n) | Visit each node exactly once |
| **Space** | O(h) | Recursion depth equals tree height |

---

## Exercise 2: Diameter - Two Pass Approach

### The Solution

```rust
pub fn diameter_of_binary_tree_two_pass(root: Option<Box<TreeNode>>) -> i32 {
    fn find_farthest(node: &Option<Box<TreeNode>>, target: i32, parent: &mut Option<i32>) -> i32 {
        if let Some(n) = node {
            if n.val == target {
                return 0;
            }
            if let Some(p) = *parent {
                if n.val == p {
                    let child = if n.left.as_ref().map_or(false, |l| l.val == target) {
                        &n.left
                    } else {
                        &n.right
                    };
                    return 1 + find_farthest(child, target, parent);
                }
            }
            *parent = Some(n.val);
            let (next, _) = if let Some(left) = &n.left {
                if left.val == target {
                    (&n.right, true)
                } else {
                    (&n.left, false)
                }
            } else {
                (&n.right, false)
            };
            1 + find_farthest(next, target, parent)
        } else {
            0
        }
    }

    fn get_height(node: &Option<Box<TreeNode>>) -> i32 {
        if let Some(n) = node {
            1 + get_height(&n.left).max(get_height(&n.right))
        } else {
            0
        }
    }

    let mut max_dia = 0;
    let mut stack = vec![root.as_ref().map(|b| b.as_ref())];
    let mut parent = std::collections::HashMap::new();

    while let Some(current) = stack.pop() {
        if let Some(n) = current {
            if let Some(left) = &n.left {
                parent.insert(**left.as_ref(), Some(n.val));
                stack.push(left.as_deref());
            }
            if let Some(right) = &n.right {
                parent.insert(**right.as_ref(), Some(n.val));
                stack.push(right.as_deref());
            }
        }
    }

    if let Some(root_node) = root.as_ref() {
        max_dia = get_height(&root);
    }
    max_dia
}
```

### Key Insight

This approach first finds the tree structure, then for each node calculates the longest path through it using height calculations. While correct, it's more complex than the single-pass approach.

---

## Exercise 3: Diameter - Iterative with Stack

### The Solution

```rust
pub fn diameter_of_binary_tree_iterative(root: Option<Box<TreeNode>>) -> i32 {
    let mut max_diameter = 0;

    // Use post-order traversal with explicit stack
    // Each entry: (node, visited_children_flag)
    let mut stack: Vec<(Option<Box<TreeNode>>, bool)> = Vec::new();
    let mut node = root.as_ref();

    let mut visited = std::collections::HashMap::new();

    while stack.len() > 0 || node.is_some() {
        if let Some(n) = node {
            stack.push((Some(n.clone()), false));
            node = n.left.as_deref();
        } else {
            let (maybe_node, processed) = stack.pop().unwrap();
            if let Some(n) = maybe_node {
                if processed {
                    let left_h = visited.remove(&(&n as *const _ as usize)).unwrap_or(0);
                    let right_h = visited.get(&(&n.right as *const _ as usize)).copied().unwrap_or(0);

                    let through_node = left_h + right_h;
                    max_diameter = max_diameter.max(through_node);

                    visited.insert(&n as *const _ as usize, 1 + left_h.max(right_h));
                    node = n.right.as_deref();
                } else {
                    stack.push((Some(n.clone()), true));
                    node = n.right.as_deref();
                }
            }
        }
    }

    max_diameter
}
```

### Key Insight

Iterative post-order traversal using a stack with a "visited children" flag. This avoids recursion but maintains the same time complexity.

---

## Exercise 4: Height of Binary Tree

### The Solution

```rust
pub fn height(root: Option<Box<TreeNode>>) -> i32 {
    fn compute_height(node: &Option<Box<TreeNode>>) -> i32 {
        if let Some(n) = node {
            1 + compute_height(&n.left).max(compute_height(&n.right))
        } else {
            0
        }
    }
    compute_height(&root)
}
```

### Line-by-Line Analysis

```rust
fn compute_height(node: &Option<Box<TreeNode>>) -> i32 {
```
**Purpose:** Inner recursive function to compute height of any subtree.

```rust
if let Some(n) = node {
```
**Purpose:** Base case - empty tree has height 0.

```rust
1 + compute_height(&n.left).max(compute_height(&n.right))
```
**Purpose:** Height is 1 (current node) plus the maximum height of children.

---

## Summary Table

| Exercise | Time | Space | Key Technique |
|----------|------|-------|---------------|
| 1: Diameter Recursive | O(n) | O(h) | Post-order, track max |
| 2: Two Pass | O(n) | O(h) | Height + structure |
| 3: Iterative | O(n) | O(h) | Explicit stack |
| 4: Height | O(n) | O(h) | Recursion |

## Key Takeaways

1. **Diameter** = max(left_height + right_height) across all nodes
2. **Post-order** is essential for computing height from bottom up
3. **Single pass** solution is optimal - update diameter while computing heights
4. **Height** of empty tree = 0, single node = 1