# Kth Smallest Element in BST - LeetCode 230

## Problem Statement

Given the root of a binary search tree (BST) and an integer k, return the k-th smallest value in the tree.

Note that the 1st smallest is the minimum (leftmost) element.

## Visual Walkthrough

```
Example 1:
Input: root = [3,1,4,null,2], k = 1
        3
       / \
      1   4
       \
        2

Inorder traversal: 1 → 2 → 3 → 4
1st smallest = 1 ✓

Example 2:
Input: root = [5,3,6,2,4,null,null,1], k = 3
        5
       / \
      3   6
     / \
    2   4
   /
  1

Inorder traversal: 1 → 2 → 3 → 4 → 5 → 6
3rd smallest = 3 ✓
```

### BST Property and Inorder Traversal

```
Inorder traversal of BST visits nodes in ASCENDING order:

        5
       / \
      3   7          Inorder: 1, 2, 3, 4, 5, 6, 7, 8, 9
     / \ / \
    2  4 6  8
   /       \
  1         9

1 < 2 < 3 < 4 < 5 < 6 < 7 < 8 < 9 ✓
```

### Algorithm Visualization: Counter Method

```
Tree: [5,3,7,2,4,6,8,1]
k = 4

        5
       / \
      3   7
     / \ / \
    2  4 6  8
   /
  1

Step-by-step inorder with counter:

Visit 1: count=1, not k=4 yet
Visit 2: count=2, not k=4 yet
Visit 3: count=3, not k=4 yet
Visit 4: count=4 ← FOUND! Return 4

But wait, k=4 means 4th smallest.
Inorder gives: 1, 2, 3, 4, 5, 6, 7, 8
1st=1, 2nd=2, 3rd=3, 4th=4 ✓
```

## Approach Comparison

| Approach | Time | Space | Description |
|----------|-------|-------|-------------|
| Inorder + Array | O(n) | O(n) | Store all nodes, select kth |
| Inorder with Counter | O(n) | O(h) | Early termination, O(1) extra |
| Morris Traversal | O(n) | O(1) | Threaded tree, no extra space |
| BST Iterator | O(n) | O(h) | For repeated queries |

### Why Inorder Works for BST

Inorder traversal visits:
1. All left descendants (smaller values)
2. Current node
3. All right descendants (larger values)

This natural ascending order property makes inorder perfect for kth smallest queries.

## Implementation Strategy

### Approach 1: Store All in Array

```rust
fn kth_smallest_array(root: Option<&TreeNode>, k: i32) -> i32 {
    let mut values = Vec::new();
    inorder_collect(root, &mut values);
    values[(k - 1) as usize]
}
```

### Approach 2: Early Termination (Optimal)

```rust
fn kth_smallest_counter(root: Option<&TreeNode>, k: i32) -> i32 {
    let mut count = 0;
    let mut result = 0;
    fn inorder(node: Option<&TreeNode>, k: i32, count: &mut i32, result: &mut i32) {
        // ... early termination when count == k
    }
}
```

## Edge Cases

1. **k = 1**: Return minimum (leftmost node)
2. **k = n**: Return maximum (rightmost node)
3. **k > n**: Should not happen per problem constraints
4. **Single node tree**: k must be 1
5. **Skewed tree**: All nodes in single chain
6. **Balanced tree**: Multiple levels

## Test Cases

1. Basic kth smallest
2. k = 1 (minimum)
3. k = n (maximum)
4. Left-skewed tree
5. Right-skewed tree
6. Single node
7. Large k in balanced tree
8. Negative values in BST

## Solution Explanation

The key insight is that BST's inorder traversal visits nodes in ascending order. By tracking a counter during inorder and stopping when we reach k, we achieve O(n) time with O(h) space.

## Complexity Analysis

- **Time**: O(n) worst case - must visit all nodes if k = n
- **Space**: O(h) - recursion stack height (can be O(n) for skewed)

## Follow-up Questions

1. How would you handle kth largest instead?
2. What if you need to query kth smallest multiple times?
3. Can you solve with O(1) space using Morris traversal?