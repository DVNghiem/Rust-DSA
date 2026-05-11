# Lowest Common Ancestor - LeetCode 235

## Problem Statement

Given a binary search tree (BST), find the lowest common ancestor (LCA) of two given nodes in the tree.

The lowest common ancestor is defined as the lowest node that has both p and q as descendants (where we allow a node to be a descendant of itself).

## Visual Walkthrough

```
Example 1:
Input: root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 8
        6
       / \
      2   8
     / \ / \
    0  4 7  9
      / \
     3   5

Output: 6
Explanation: LCA of nodes 2 and 8 is 6.

Example 2:
Input: root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 4
        6
       / \
      2   8
     / \ / \
    0  4 7  9

Output: 2
Explanation: LCA of 2 and 4 is 2.
A node can be a descendant of itself.
```

### Understanding LCA

```
        A
       / \
      B   C
     / \
    D   E

LCA of D and E is B.
LCA of D and C is A.
LCA of B and C is A.
LCA of B and B is B (node is descendant of itself).
```

### BST Property for LCA

Since it's a BST:
- All nodes in left subtree < node.val
- All nodes in right subtree > node.val

This allows us to find LCA in single traversal:
- If both p and q are < current, go left
- If both p and q are > current, go right
- Otherwise, current is LCA (p and q are on different sides)

## Approach Comparison

| Approach | Time | Space | Description |
|----------|-------|-------|-------------|
| Recursive | O(n) | O(h) | BST property allows pruning |
| Iterative | O(n) | O(h) | Same as recursive but iterative |
| Naive | O(n) | O(n) | Find paths, then last common node |

### Why BST Property Helps

```
        6           p=2, q=4
       / \          Both < 6, go left
      2   8         Both < 6? No, 8 > 6, go right... wait
     / \           Actually 2 and 4 are both < 6, so go left
    0   4
```

## Edge Cases

1. **p or q is ancestor of other**: Return the ancestor
2. **p and q on different sides**: Return current
3. **p = q**: Return that node
4. **Root is one of p or q**: Return root
5. **p and q both in left subtree**: Return left child result
6. **p and q both in right subtree**: Return right child result

## Test Cases

1. p and q on different sides of root
2. p is ancestor of q
3. q is ancestor of p
4. p = q (both reference same node)
5. Root is LCA
6. LCA is deep in tree
7. p and q both in left subtree
8. p and q both in right subtree

## Solution Explanation

### BST-Based Recursive Solution

```rust
fn lowest_common_ancestor(root: &TreeNode, p: &TreeNode, q: &TreeNode) -> &TreeNode
```

Key insight: For BST, we can determine direction based on values:
- If both values < current.val → LCA is in left subtree
- If both values > current.val → LCA is in right subtree  
- Otherwise → current is LCA

### Why This Works

```
If p.val < root.val and q.val < root.val:
  Both nodes are in left subtree
  LCA must be in left subtree
  
If p.val > root.val and q.val > root.val:
  Both nodes are in right subtree
  LCA must be in right subtree

If one < root.val and one > root.val:
  Nodes are on different sides
  root is the lowest node that has both as descendants
```

## Complexity Analysis

- **Time**: O(n) worst case (skewed tree), O(log n) average (balanced)
- **Space**: O(h) recursion stack, O(1) iterative

## Follow-up Questions

1. What if it's not a BST (just binary tree)?
2. Can you solve iteratively?
3. How does this differ from LeetCode 236 (general binary tree)?