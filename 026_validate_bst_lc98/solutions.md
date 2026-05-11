# Validate Binary Search Tree - Solution Analysis

## Problem Overview

Given a binary tree root, determine if it is a valid Binary Search Tree (BST). A BST satisfies:
- All nodes in left subtree have values < node.val
- All nodes in right subtree have values > node.val
- Both subtrees must also be BSTs

## Solution 1: Recursive DFS with Min/Max Bounds

### Code Implementation

```rust
pub fn is_valid_bst_recursive(root: Option<&TreeNode>) -> bool {
    fn validate(node: Option<&TreeNode>, min: Option<i64>, max: Option<i64>) -> bool {
        match node {
            None => true,
            Some(n) => {
                let val = n.val as i64;
                // Check if current value violates bounds
                if let Some(min_val) = min {
                    if val <= min_val {
                        return false;  // Value must be strictly greater than min
                    }
                }
                if let Some(max_val) = max {
                    if val >= max_val {
                        return false;  // Value must be strictly less than max
                    }
                }
                // Recurse on left subtree with updated max
                // Recurse on right subtree with updated min
                validate(n.left.as_deref(), min, Some(val))
                    && validate(n.right.as_deref(), Some(val), max)
            }
        }
    }
    validate(root, None, None)
}
```

### Line-by-Line Analysis

1. **`pub fn is_valid_bst_recursive(root: Option<&TreeNode>) -> bool`**: Public API taking an optional reference to TreeNode. Using `Option<&TreeNode>` allows us to pass `None` for empty tree without ownership issues.

2. **`fn validate(node: Option<&TreeNode>, min: Option<i64>, max: Option<i64>) -> bool`**: Inner recursive function with two additional parameters tracking the valid range. Using `i64` prevents integer overflow when comparing values.

3. **`match node { None => true, ... }`**: Base case - empty node is always valid (null children are considered valid BSTs by definition).

4. **`let val = n.val as i64;`**: Convert to i64 to safely handle edge cases near i32 boundaries and prevent overflow in comparisons.

5. **`if let Some(min_val) = min) if val <= min_val { return false; }`**: Check lower bound. If min is set and current value is not strictly greater, BST property is violated.

6. **`if let Some(max_val) = max) if val >= max_val { return false; }`**: Check upper bound symmetrically.

7. **`validate(n.left.as_deref(), min, Some(val))`**: Recurse left. The max becomes current node's value - all left descendants must be < current node.

8. **`validate(n.right.as_deref(), Some(val), max)`**: Recurse right. The min becomes current node's value - all right descendants must be > current node.

9. **`&&` operator ensures BOTH subtrees must be valid for the whole tree to be valid.

### Visualization of Bounds Propagation

```
        5 (range: -∞, +∞)
       / \
      3   7 (range: 5, +∞)
     / \ / \
    2  4 6  8

At node 5: val=5, min=None, max=None → valid
At node 3: val=3, min=None, max=5 → 3 < 5 → valid
At node 7: val=7, min=5, max=None → 7 > 5 → valid
At node 2: val=2, min=None, max=3 → 2 < 3 → valid
At node 4: val=4, min=None, max=3 → 4 > 3 → valid
At node 6: val=6, min=5, max=7 → 5 < 6 < 7 → valid
At node 8: val=8, min=7, max=None → 8 > 7 → valid

All nodes valid → is_valid_bst = true
```

### Edge Case: Invalid BST Detection

```
        5
       / \
      3   7
       \
        4  <- should be in right subtree of 3 (should be > 3 and < 5)

At node 4: val=4, min=None, max=3
           4 > 3 → but wait, we're in left subtree of 5!
           Actually, parent is 3, not 5.
           But 4 > 3 is fine for being in right subtree of 3.
           However, 4 < 5 means it CAN be in left subtree of 5.
           So this is actually VALID!

Let me show a truly invalid case:

        5
       / \
      3   7
       \
        6  <- 6 > 3 (parent), but 6 < 7 is fine...
           Actually this is valid too because 6 > 3 and 6 < 7.

Invalid case:
        5
       / \
      6   7   <- 6 is in left subtree of 5, but 6 > 5, violates BST!

At node 6: val=6, min=None, max=5
           6 > 5? YES! But 6 is NOT < 5.
           Since 6 >= 5 (max), return false.
```

## Solution 2: Inorder Traversal

### Code Implementation

```rust
pub fn is_valid_bst_inorder(root: Option<&TreeNode>) -> bool {
    fn inorder(node: Option<&TreeNode>, prev: &mut Option<i64>) -> bool {
        match node {
            None => true,
            Some(n) => {
                // Traverse left first (inorder)
                if !inorder(n.left.as_deref(), prev) {
                    return false;
                }
                // Process current node - check if greater than previous
                if let Some(p) = *prev {
                    if n.val as i64 <= p {
                        return false;  // Not strictly greater
                    }
                }
                *prev = Some(n.val as i64);
                // Traverse right
                inorder(n.right.as_deref(), prev)
            }
        }
    }
    inorder(root, &mut None)
}
```

### Line-by-Line Analysis

1. **Inorder of BST produces sorted sequence**: Left subtree → Node → Right subtree gives values in ascending order for valid BST.

2. **`fn inorder(node: Option<&TreeNode>, prev: &mut Option<i64>) -> bool`**: Mutable reference to previous value allows tracking last seen value across recursive calls.

3. **`if !inorder(n.left.as_deref(), prev) { return false; }`**: Recurse left first. If that subtree is invalid, propagate failure.

4. **`if let Some(p) = *prev { if n.val as i64 <= p { return false; } }`**: Check current node against previous. For valid BST, each value must be STRICTLY greater than the previous.

5. **`*prev = Some(n.val as i64);`**: Update previous to current value for next comparison.

6. **`inorder(n.right.as_deref(), prev)`**: Recurse right after processing current node.

### Why Inorder Works

```
Inorder traversal sequence for BST = sorted ascending sequence

    5
   / \
  3   7
 / \ / \
1  4 6  8

Inorder: 1 → 3 → 4 → 5 → 6 → 7 → 8 (sorted!)

If any pair is out of order, BST is invalid.
```

## Solution 3: Iterative with Stack

### Code Implementation

```rust
pub fn is_valid_bst_iterative(root: Option<&TreeNode>) -> bool {
    let mut stack: Vec<(&TreeNode, Option<i64>, Option<i64>)> = Vec::new();
    if let Some(r) = root.as_deref() {
        stack.push((r, None, None));  // (node, min, max)
    }

    while let Some((node, min, max)) = stack.pop() {
        let val = node.val as i64;
        if let Some(min_val) = min {
            if val <= min_val { return false; }
        }
        if let Some(max_val) = max {
            if val >= max_val { return false; }
        }
        // Push children with updated bounds
        if let Some(ref left) = node.left {
            stack.push((left.as_deref(), min, Some(val)));
        }
        if let Some(ref right) = node.right {
            stack.push((right.as_deref(), Some(val), max));
        }
    }
    true
}
```

### Line-by-Line Analysis

1. **`let mut stack: Vec<(&TreeNode, Option<i64>, Option<i64>)> = Vec::new();`**: Stack stores tuples of (node, min_bound, max_bound).

2. **`if let Some(r) = root.as_deref() { stack.push((r, None, None)); }`**: Initialize with root node and no bounds.

3. **`while let Some((node, min, max)) = stack.pop()`**: Process nodes LIFO. Works similarly to DFS.

4. **Bounds checking identical to recursive version**.

5. **`if let Some(ref left) = node.left { stack.push((left.as_deref(), min, Some(val))); }`**: Push left child with updated max bound.

6. **`if let Some(ref right) = node.right { stack.push((right.as_deref(), Some(val), max)); }`**: Push right child with updated min bound.

### Why Iterative?

- Avoids recursion stack overflow for very deep trees
- Same time complexity, slightly more memory due to explicit stack
- Easier to debug and step through

## Complexity Comparison

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Recursive | O(n) | O(h) | Clean, elegant, stack overflow risk |
| Inorder | O(n) | O(h) | Leverages BST property |
| Iterative | O(n) | O(h) | No stack overflow, explicit stack |

Where n = number of nodes, h = tree height

## Key Insights

1. **Bounds propagation**: Each node must satisfy min < val < max where bounds come from ancestors.

2. **Strict inequality**: BST uses < and > not ≤ and ≥. Duplicates are invalid.

3. **Integer overflow**: Using i64 for bounds prevents issues near i32 boundaries.

4. **Empty = valid**: null/None nodes are valid BSTs (simplifies recursion).

5. **Inorder trick**: Valid BST produces strictly increasing inorder sequence.

## Test Case Analysis

### Test: `test_invalid_bst`
```
Input: [5,1,4,null,null,3,6]
Tree:
        5
       / \
      1   4
         / \
        3   6

At node 4 (value=4):
- Parent is 5, it's in right subtree → min=5
- But 4 < 5 → violates bound → return false

Expected: false ✓
```

### Test: `test_duplicates_invalid`
```
Input: [2,2,3]
Tree:
    2
   / \
  2   3

At root 2: valid
At left child 2:
- Parent is 2, it's in left subtree → max=2
- But 2 >= 2 → violates max bound → return false

Expected: false ✓
```

## Follow-up Questions & Answers

**Q: Why use i64 instead of i32 for bounds?**
A: Consider tree with nodes at i32::MAX and i32::MAX-1. When checking if i32::MAX is valid, its left child i32::MAX-1 must be < i32::MAX. But comparing with i32 may overflow in edge cases. Using i64 provides safe headroom.

**Q: Can we solve without recursion?**
A: Yes - use iterative approach with explicit stack (Solution 3) or use BFS with queue.

**Q: How does inorder detection work for duplicates?**
A: Inorder checks if current.val <= prev.val. For duplicate 2, we get 2 <= 2 → false → invalid.

**Q: What's the worst case space?**
A: O(n) for skewed tree (essentially a linked list). O(log n) for balanced tree.

**Q: Can we early terminate?**
A: Yes - any violation immediately returns false. No need to explore remaining nodes. So best case O(1) if root is invalid.