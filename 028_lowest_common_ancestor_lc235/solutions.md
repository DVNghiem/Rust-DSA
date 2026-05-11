# Lowest Common Ancestor - Solution Analysis

## Problem Overview

Given a BST and two node values p and q, find the Lowest Common Ancestor (LCA) - the node that is ancestor of both where no child of that node is also an ancestor of both.

## Solution 1: Recursive BST-Based

### Code Implementation

```rust
pub fn lowest_common_ancestor(
    root: Option<&TreeNode>,
    p: i32,
    q: i32,
) -> Option<i32> {
    fn find_lca(node: Option<&TreeNode>, p: i32, q: i32) -> Option<i32> {
        match node {
            None => None,
            Some(n) => {
                // Both in left subtree
                if p < n.val && q < n.val {
                    return find_lca(n.left.as_deref(), p, q);
                }
                // Both in right subtree
                if p > n.val && q > n.val {
                    return find_lca(n.right.as_deref(), p, q);
                }
                // On different sides or one is current → current is LCA
                Some(n.val)
            }
        }
    }
    find_lca(root, p, q)
}
```

### Line-by-Line Analysis

1. **`pub fn lowest_common_ancestor(root: Option<&TreeNode>, p: i32, q: i32) -> Option<i32>`**: Takes tree root and two node values, returns value of LCA.

2. **`fn find_lca(node: Option<&TreeNode>, p: i32, q: i32) -> Option<i32>`**: Inner recursive helper.

3. **`match node { None => None, Some(n) => { ... } }`**: Pattern match on node - base case returns None for empty tree.

4. **`if p < n.val && q < n.val { return find_lca(n.left.as_deref(), p, q); }`**: If both values are less than current, both nodes must be in left subtree. Recurse left.

5. **`if p > n.val && q > n.val { return find_lca(n.right.as_deref(), p, q); }`**: If both values are greater, both nodes must be in right subtree. Recurse right.

6. **`Some(n.val)`**: If neither condition is true, current node is on the path to both nodes. This means:
   - One is on left, one on right, OR
   - One equals current node
   In both cases, current is the LCA.

### Visualization

```
        6           Find LCA of 2 and 8
       / \
      2   8
     / \ / \
    0  4 7  9

Step 1: At 6
  - p=2 < 6 and q=8 > 6
  - Not both left, not both right
  - Return 6 (LCA!)

        6           Find LCA of 0 and 4
       / \
      2   8
     / \
    0   4

Step 1: At 6
  - 0 < 6 and 4 < 6
  - Both left → go left

Step 2: At 2
  - 0 < 2 and 4 > 2
  - Different sides
  - Return 2 (LCA!)
```

## Solution 2: Iterative

### Code Implementation

```rust
pub fn lowest_common_ancestor_iterative(
    root: Option<&TreeNode>,
    p: i32,
    q: i32,
) -> Option<i32> {
    let mut current = root.as_deref();

    while let Some(node) = current {
        if p < node.val && q < node.val {
            current = node.left.as_deref();
        } else if p > node.val && q > node.val {
            current = node.right.as_deref();
        } else {
            return Some(node.val);
        }
    }

    None
}
```

### Line-by-Line Analysis

1. **`let mut current = root.as_deref();`**: Start at root.

2. **`while let Some(node) = current { ... }`**: Loop while we have a node.

3. **`if p < node.val && q < node.val { current = node.left.as_deref(); }`**: Both smaller → go left.

4. **`else if p > node.val && q > node.val { current = node.right.as_deref(); }`**: Both larger → go right.

5. **`else { return Some(node.val); }`**: Different sides or one matches → found LCA.

### Why Iterative Works

- Same logic as recursive
- No stack overflow risk
- O(h) iterations at most

## Solution 3: Path-Based (General Binary Tree)

### Code Implementation

```rust
fn find_path(root: Option<&TreeNode>, target: i32, path: &mut Vec<i32>) -> bool {
    match root {
        None => false,
        Some(n) => {
            path.push(n.val);
            if n.val == target { return true; }
            if target < n.val {
                if find_path(n.left.as_deref(), target, path) { return true; }
            } else {
                if find_path(n.right.as_deref(), target, path) { return true; }
            }
            path.pop();
            false
        }
    }
}

pub fn lowest_common_ancestor_path(
    root: Option<&TreeNode>,
    p: i32,
    q: i32,
) -> Option<i32> {
    let mut path_p = Vec::new();
    let mut path_q = Vec::new();

    if !find_path(root, p, &mut path_p) || !find_path(root, q, &mut path_q) {
        return None;
    }

    let mut lca = path_p[0];
    for i in 0..path_p.len().min(path_q.len()) {
        if path_p[i] == path_q[i] {
            lca = path_p[i];
        } else {
            break;
        }
    }

    Some(lca)
}
```

### Line-by-Line Analysis

1. **`fn find_path(...) -> bool`**: DFS to find path from root to target. Pushes nodes onto path, pops back if dead end.

2. **`if n.val == target { return true; }`**: Found target, path contains root-to-target.

3. **`if target < n.val { find_path(n.left...) } else { find_path(n.right...) }`**: BST property guides search.

4. **`path.pop();`**: Backtrack if this path didn't lead to target.

5. **`let mut lca = path_p[0]; for i in 0..path_p.len().min(path_q.len())`**: Walk both paths simultaneously, last matching node is LCA.

### Trade-offs of Path Method

- Works for ANY binary tree (not just BST)
- Requires two full traversals O(n)
- Stores paths O(h)
- Less efficient than BST-based single traversal

## Complexity Comparison

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| BST Recursive | O(n) worst, O(h) avg | O(h) | Optimal for BST |
| BST Iterative | O(n) worst, O(h) avg | O(1) | No stack overflow |
| Path-Based | O(n) | O(h) | Works for any tree |

## Key Insights

1. **BST allows pruning**: By comparing values, we know which subtree to search.

2. **LCA found when p and q are on different sides**: One left, one right means current is LCA.

3. **Ancestor case**: If current equals p or q, and other is in subtree, current is LCA.

4. **Single traversal**: No need to find actual nodes, just compare values.

## Test Case Analysis

### Test: `test_p_q_on_different_sides`

```
Tree:
      6
     / \
    2   8

p=2, q=8

At root 6:
  - 2 < 6 and 8 > 6
  - Different sides → LCA = 6 ✓
```

### Test: `test_p_ancestor_of_q`

```
Tree with node 2 having child 4

p=2, q=4

At node 2:
  - p == 2 (current)
  - 4 > 2
  - Not both left, not both right → LCA = 2 ✓
```

### Test: `test_both_in_left_subtree`

```
        6
       /
      2
     / \
    0   4

p=0, q=4

At 6: both < 6 → go left
At 2: 0 < 2 and 4 > 2 → different sides → LCA = 2 ✓
```

## Why BST Property is Key

```
For general binary tree (LeetCode 236), we must:
1. Find paths to both nodes (O(n))
2. Compare paths (O(h))

For BST (LeetCode 235), we can:
1. Compare values at each node (O(h))
2. Single traversal, no extra space

Value comparison tells us which subtree to search.
This is why BST LCA is O(h) vs general O(n).
```

## Follow-up Answers

**Q: Difference from LeetCode 236 (general binary tree)?**
A: 236 must find actual nodes first, then paths. 235 uses BST property to prune search based on values.

**Q: Can we solve with parent pointers?**
A: Yes - ascend both until they meet. O(h) time, O(1) space if parent pointers available.

**Q: What if p or q is not in tree?**
A: Return None (handled by path-based approach). BST approach would return first node where search ends.

**Q: Time complexity for skewed tree?**
A: O(n) since we might traverse all nodes (height = n for skewed tree).

**Q: Space complexity for balanced tree?**
A: O(log n) recursion/iteration depth.