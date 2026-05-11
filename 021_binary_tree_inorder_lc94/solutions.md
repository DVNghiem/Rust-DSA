# Solutions: Binary Tree Inorder Traversal (LeetCode #94)

This document provides detailed, line-by-line analysis of each exercise implementation.

---

## Exercise 1: Inorder Traversal - Recursive

### The Solution

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

### Line-by-Line Analysis

```rust
let mut result = Vec::new();
```
**Purpose:** Initialize result vector to store traversal order.

```rust
inorder_recursive(&root, &mut result);
```
**Purpose:** Call recursive helper with reference to root.

```rust
fn inorder_recursive(node: &Option<Box<TreeNode>>, result: &mut Vec<i32>) {
```
**Purpose:** Helper function that takes optional node reference and result vector.

```rust
if let Some(n) = node {
    inorder_recursive(&n.left, result);
    result.push(n.val);
    inorder_recursive(&n.right, result);
}
```
**Purpose:** If node exists:
1. Recursively traverse left subtree (Left)
2. Visit current node (Root)
3. Recursively traverse right subtree (Right)

### Visual Walkthrough

```
    1
   / \
  2   3
 / \   \
4   5   6

Inorder: Left -> Root -> Right

1. Traverse left of 1 (which is subtree rooted at 2)
2. Traverse left of 2 (which is 4) - no children, push 4
3. Visit 2, push 2
4. Traverse right of 2 (which is 5) - no children, push 5
5. Visit 1, push 1
6. Traverse left of 3 (none) - skip
7. Visit 3, push 3 (wait, there's 6)
8. Traverse right of 3 (which is 6) - no children, push 6

Result: [4, 2, 5, 1, 3, 6]
```

### Complexity Analysis

| Metric | Value | Explanation |
|--------|-------|-------------|
| **Time** | O(n) | Visit each node exactly once |
| **Space** | O(h) | Recursion depth equals tree height |

---

## Exercise 2: Inorder Traversal - Iterative

### The Solution

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

### Key Insight

Instead of recursive calls, we use an explicit stack to simulate the call stack:
- Go as far left as possible, pushing nodes onto stack
- Pop and visit node
- Move to right subtree

---

## Summary Table

| Exercise | Time | Space | Key Technique |
|----------|------|-------|---------------|
| 1: Recursive | O(n) | O(h) | Implicit recursion |
| 2: Iterative | O(n) | O(h) | Explicit stack |
| 3: Preorder | O(n) | O(h) | Root, left, right |
| 4: Postorder | O(n) | O(h) | Left, right, root |
| 5: Level Order | O(n) | O(w) | BFS with queue |
| 6: Zigzag | O(n) | O(w) | Alternating order |
| 7: Morris | O(n) | O(1) | Threaded tree |

## Key Takeaways

1. **Inorder**: Left, Root, Right
2. **Preorder**: Root, Left, Right
3. **Postorder**: Left, Right, Root
4. **Level order**: BFS using queue
5. **Morris**: No extra space using threading