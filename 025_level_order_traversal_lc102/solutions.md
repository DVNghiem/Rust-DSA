# Solutions: Binary Tree Level Order Traversal (LeetCode #102)

This document provides detailed, line-by-line analysis of each exercise implementation.

---

## Exercise 1: Level Order Traversal - BFS (Primary Solution)

### The Solution

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

### Line-by-Line Analysis

```rust
let mut result = Vec::new();
```
**Purpose:** Initialize the result vector that will hold all levels.

```rust
if root.is_none() {
    return result;
}
```
**Purpose:** Early return for empty tree - avoid creating an empty queue.

```rust
let mut queue = VecDeque::new();
queue.push_back(root.unwrap());
```
**Purpose:** Initialize queue with the root node. VecDeque provides O(1) push/pop at both ends.

```rust
while !queue.is_empty() {
```
**Purpose:** Process all levels until queue is empty (all nodes processed).

```rust
let level_size = queue.len();
```
**Purpose:** Capture the number of nodes at current level BEFORE processing. This is critical - queue length changes as we add children.

```rust
let mut level_vals = Vec::new();
```
**Purpose:** Temporary vector to collect values at current level.

```rust
for _ in 0..level_size {
```
**Purpose:** Process exactly `level_size` nodes - all nodes that were in queue at start of this level.

```rust
let node = queue.pop_front().unwrap();
```
**Purpose:** Remove and get the next node at current level.

```rust
level_vals.push(node.val);
```
**Purpose:** Record current node's value for this level.

```rust
if let Some(left) = node.left {
    queue.push_back(*left);
}
if let Some(right) = node.right {
    queue.push_back(*right);
}
```
**Purpose:** Add children to queue for next level processing. Children will be processed in subsequent iterations.

```rust
result.push(level_vals);
```
**Purpose:** Save current level's values to result.

### Complexity Analysis

| Metric | Value | Explanation |
|--------|-------|-------------|
| **Time** | O(n) | Visit each node exactly once |
| **Space** | O(w) | Queue holds at most one level |

---

## Exercise 2: Level Order Bottom-Up

### The Solution

```rust
pub fn level_order_bottom(root: Option<Box<TreeNode>>) -> Vec<Vec<i32>> {
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
    result.reverse();
    result
}
```

### Key Insight

Same algorithm as level_order, but at the end we reverse the result array. This gives us leaves-to-root order instead of root-to-leaves.

---

## Exercise 3: Zigzag Level Order Traversal

### The Solution

```rust
pub fn zigzag_level_order(root: Option<Box<TreeNode>>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    if root.is_none() {
        return result;
    }
    let mut queue = VecDeque::new();
    queue.push_back(root.unwrap());
    let mut left_to_right = true;

    while !queue.is_empty() {
        let level_size = queue.len();
        let mut level_vals = Vec::new();

        for _ in 0..level_size {
            let node = queue.pop_front().unwrap();
            level_vals.push(node.val);

            if left_to_right {
                if let Some(left) = node.left {
                    queue.push_back(*left);
                }
                if let Some(right) = node.right {
                    queue.push_back(*right);
                }
            } else {
                if let Some(right) = node.right {
                    queue.push_back(*right);
                }
                if let Some(left) = node.left {
                    queue.push_back(*left);
                }
            }
        }
        if !left_to_right {
            level_vals.reverse();
        }
        result.push(level_vals);
        left_to_right = !left_to_right;
    }
    result
}
```

### Key Insight

When `left_to_right` is false, we reverse the level values before adding to result. This creates the zigzag pattern. We also add children in reverse order when going right-to-left.

---

## Exercise 4: Average of Levels

### The Solution

```rust
pub fn average_of_levels(root: Option<Box<TreeNode>>) -> Vec<f64> {
    let mut result = Vec::new();
    if root.is_none() {
        return result;
    }
    let mut queue = VecDeque::new();
    queue.push_back(root.unwrap());

    while !queue.is_empty() {
        let level_size = queue.len();
        let mut sum: i64 = 0;

        for _ in 0..level_size {
            let node = queue.pop_front().unwrap();
            sum += node.val as i64;

            if let Some(left) = node.left {
                queue.push_back(*left);
            }
            if let Some(right) = node.right {
                queue.push_back(*right);
            }
        }
        result.push(sum as f64 / level_size as f64);
    }
    result
}
```

### Key Insight

Same BFS pattern but instead of collecting values, we sum them and divide by level size. Using `i64` for sum prevents overflow for large values.

---

## Exercise 5: Right Side View

### The Solution

```rust
pub fn right_side_view(root: Option<Box<TreeNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    if root.is_none() {
        return result;
    }
    let mut queue = VecDeque::new();
    queue.push_back(root.unwrap());

    while !queue.is_empty() {
        let level_size = queue.len();

        for i in 0..level_size {
            let node = queue.pop_front().unwrap();

            if i == level_size - 1 {
                result.push(node.val);
            }

            if let Some(left) = node.left {
                queue.push_back(*left);
            }
            if let Some(right) = node.right {
                queue.push_back(*right);
            }
        }
    }
    result
}
```

### Line-by-Line Analysis

```rust
if i == level_size - 1 {
    result.push(node.val);
}
```
**Purpose:** Only add the rightmost node at each level (last node processed in the level).

### Complexity Analysis

| Metric | Value | Explanation |
|--------|-------|-------------|
| **Time** | O(n) | Visit each node exactly once |
| **Space** | O(h) | Result only, queue is O(w) |

---

## Summary Table

| Exercise | Time | Space | Key Technique |
|----------|------|-------|---------------|
| 1: Level Order | O(n) | O(w) | BFS with level tracking |
| 2: Bottom-Up | O(n) | O(w) | Same + reverse |
| 3: Zigzag | O(n) | O(w) | Alternating reversal |
| 4: Averages | O(n) | O(w) | Sum and divide |
| 5: Right View | O(n) | O(w) | Last node at each level |

## Key Takeaways

1. **BFS** is natural for level-order traversal
2. **Capture level size** before processing children
3. **Queue** holds nodes waiting to be processed
4. **Result** is Vec<Vec<i32>> - each inner vec is one level
5. **Variations** can be achieved by reversing or filtering