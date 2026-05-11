# Validate Binary Search Tree - LeetCode 98

## Problem Statement

Given the root of a binary tree, determine if it is a valid binary search tree (BST).

A binary search tree is defined as follows:
- The left subtree of a node contains only nodes with keys **less than** the node's key.
- The right subtree of a node contains only nodes with keys **greater than** the node's key.
- Both the left and right subtrees must also be binary search trees.

## Visual Walkthrough

```
Example 1:
Input: root = [2,1,3]
        2
       / \
      1   3

Output: true

Example 2:
Input: root = [5,1,4,null,null,3,6]
        5
       / \
      1   4
         / \
        3   6

Output: false (3 is in the right subtree of 5 but 3 < 5)
```

### Key Insight: Min/Max Bounds

```
        5
       / \
      1   7
         / \
        6   8

When checking node 6:
- We know it's in the right subtree of 5, so it must be > 5
- We know it's in the left subtree of 7, so it must be < 7
- Therefore, valid range for 6 is (5, 7)

         5
        / \
       3   7
      / \ / \
     2  4 6  8

When checking node 4:
- In left subtree of 5 → must be > 5? NO! 4 < 5
- This violates the BST property
```

### Algorithm Visualization

```
Tree: [5,3,7,2,4,6,8]

        5 (range: -∞, +∞)
       / \
      3   7 (range: 5, +∞)
     / \ / \
    2  4 6  8 (range: 3, 7) (range: 5, 7) (range: 7, +∞)

For node 4:
- Parent is 3, which is in left subtree of 5 → min = 5
- But 4 < 5 → INVALID BST
```

## Approach Comparison

| Approach | Time Complexity | Space Complexity | Description |
|----------|-----------------|------------------|-------------|
| Recursive with Min/Max | O(n) | O(h) | Track bounds, check each node |
| Inorder Traversal | O(n) | O(h) | BST inorder produces sorted sequence |
| Iteration with Stack | O(n) | O(h) | Iterative version of approach 1 |
| Validate with Range | O(n) | O(n) | Use Option<i64> for bounds |

### Why Min/Max Approach Works

The key insight is that every node in a BST must satisfy:
- All nodes in left subtree < current node value
- All nodes in right subtree > current node value

When we recursively traverse, we pass down the valid range:
- For left child: range becomes (min, node.val)
- For right child: range becomes (node.val, max)

A node is valid if: min < node.val < max

## Implementation Strategy

### Approach 1: Recursive DFS with Bounds

```rust
fn is_valid_bst_recursive(root: Option<&TreeNode>, min: Option<i64>, max: Option<i64>) -> bool
```

For each node:
1. If null, return true
2. Check if node.val is within (min, max)
3. Recursively validate left child with bounds (min, node.val)
4. Recursively validate right child with bounds (node.val, max)

### Approach 2: Inorder Traversal

A valid BST produces a strictly increasing sequence in inorder traversal.

```rust
fn is_valid_bst_inorder(root: Option<&TreeNode>) -> bool {
    let mut prev = Option::None;
    // Inorder produces sorted sequence for BST
}
```

## Edge Cases

1. **Empty tree**: Single node is always a valid BST
2. **Single node**: Valid BST
3. **Negative values**: BST can have negative numbers
4. **Duplicates**: Not allowed in BST (strict inequality)
5. **Large values**: Use i64 to avoid overflow
6. **Integer overflow**: Use Option<i64> to represent ±∞

## Test Cases

1. Basic valid BST
2. Invalid BST with value in wrong position
3. Empty tree (valid)
4. Single node (valid)
5. Strictly increasing inorder (valid)
6. Duplicates in tree (invalid)
7. Negative numbers
8. Large values (near i32::MAX)
9. Left-skewed tree
10. Right-skewed tree

## Solution Explanation

The recursive solution with bounds is the most intuitive:

1. Base case: empty node is valid
2. Check node against current bounds
3. Recurse left with max = node.val
4. Recurse right with min = node.val

This ensures every node satisfies the BST property relative to its ancestors.

## Complexity Analysis

- **Time**: O(n) - visit each node exactly once
- **Space**: O(h) - recursion stack depth where h is tree height
  - Worst case O(n) for skewed tree
  - Best case O(log n) for balanced tree

## Follow-up Questions

1. Can you solve using inorder traversal?
2. How would you handle integer overflow with i32?
3. Can you solve iteratively without recursion?