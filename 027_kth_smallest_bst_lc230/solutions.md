# Kth Smallest Element in BST - Solution Analysis

## Problem Overview

Given a BST and integer k, return the k-th smallest value where 1st smallest is the minimum (leftmost) element.

## Solution 1: Inorder with Early Termination

### Code Implementation

```rust
pub fn kth_smallest(root: Option<&TreeNode>, k: i32) -> i32 {
    let mut count = 0;
    let mut result = 0;

    fn inorder(node: Option<&TreeNode>, k: i32, count: &mut i32, result: &mut i32) {
        if node.is_none() || *count >= k {
            return;
        }

        if let Some(n) = node {
            // Traverse left first (smaller values)
            inorder(n.left.as_deref(), k, count, result);

            // Process current node
            *count += 1;
            if *count == k {
                *result = n.val;
                return;
            }

            // Traverse right (larger values)
            inorder(n.right.as_deref(), k, count, result);
        }
    }

    inorder(root, k, &mut count, &mut result);
    result
}
```

### Line-by-Line Analysis

1. **`let mut count = 0; let mut result = 0;`**: Initialize counter to track nodes visited and result to store answer.

2. **`fn inorder(node: Option<&TreeNode>, k: i32, count: &mut i32, result: &mut i32)`**: Inner recursive function with mutable references to track state across recursive calls.

3. **`if node.is_none() || *count >= k { return; }`**: Early termination - if node is null OR we've already found k-th element, stop.

4. **`if let Some(n) = node { ... }`**: Pattern match to access node fields.

5. **`inorder(n.left.as_deref(), k, count, result);`**: Recurse left first - in BST, all left subtree values are smaller.

6. **`*count += 1;`**: Increment counter after visiting current node.

7. **`if *count == k { *result = n.val; return; }`**: When count equals k, we found the answer - store and return early.

8. **`inorder(n.right.as_deref(), k, count, result);`**: Recurse right for larger values.

### Visualization of Execution

```
Tree:      5         k = 3
          / \
         3   7
        / \
       2   4
      /
     1

Inorder sequence: 1, 2, 3, 4, 5, 7

Step-by-step:
1. Go left to 3
2. Go left to 2  
3. Go left to 1
4. Visit 1: count=1, not k=3
5. Go right from 1 (null)
6. Visit 2: count=2, not k=3
7. Go right from 2 (null)
8. Visit 3: count=3 ← k=3, return val=3

Result: 3 ✓
```

## Solution 2: Collect All Values

```rust
pub fn kth_smallest_array(root: Option<&TreeNode>, k: i32) -> i32 {
    let mut values = Vec::new();

    fn collect(node: Option<&TreeNode>, values: &mut Vec<i32>) {
        if let Some(n) = node {
            collect(n.left.as_deref(), values);
            values.push(n.val);
            collect(n.right.as_deref(), values);
        }
    }

    collect(root, &mut values);
    values[(k - 1) as usize]
}
```

### Line-by-Line Analysis

1. **`let mut values = Vec::new();`**: Create vector to store all inorder values.

2. **`fn collect(node: Option<&TreeNode>, values: &mut Vec<i32>)`**: Helper that fills values vector via inorder.

3. **`collect(n.left.as_deref(), values);`**: Recurse left first.

4. **`values.push(n.val);`**: Visit current node - add to vector.

5. **`collect(n.right.as_deref(), values);`**: Recurse right.

6. **`values[(k - 1) as usize]`**: Direct index access (k is 1-indexed).

### Trade-offs

- Simpler code
- O(n) space (can be problematic for large trees)
- No early termination opportunity

## Solution 3: Iterative Inorder

```rust
pub fn kth_smallest_iterative(root: Option<&TreeNode>, k: i32) -> i32 {
    let mut stack = Vec::new();
    let mut current = root.as_deref();
    let mut count = 0;

    while current.is_some() || !stack.is_empty() {
        // Go to leftmost node
        while let Some(node) = current {
            stack.push(node);
            current = node.left.as_deref();
        }

        // Process node
        if let Some(node) = stack.pop() {
            count += 1;
            if count == k {
                return node.val;
            }
            current = node.right.as_deref();
        }
    }

    -1 // Should never reach here
}
```

### Line-by-Line Analysis

1. **`let mut stack = Vec::new();`**: Stack for iterative DFS.

2. **`let mut current = root.as_deref();`**: Current node pointer.

3. **`while current.is_some() || !stack.is_empty()`**: Loop until all nodes processed.

4. **`while let Some(node) = current { stack.push(node); current = node.left.as_deref(); }`**: Push all left ancestors - reach leftmost node.

5. **`if let Some(node) = stack.pop() { ... }`**: Pop and process node.

6. **`count += 1; if count == k { return node.val; }`**: Same early termination logic.

7. **`current = node.right.as_deref();`**: Move to right subtree.

### Why Iterative?

- No recursion stack overflow risk
- Same time complexity O(n)
- Slightly more verbose but explicit control

## Complexity Comparison

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Counter | O(n) | O(h) | Early termination, O(1) extra |
| Array | O(n) | O(n) | Simple, no early exit |
| Iterative | O(n) | O(h) | No stack overflow |

## Key Insights

1. **BST + Inorder = Sorted**: Inorder traversal of BST produces ascending sequence.

2. **Counter enables early exit**: Don't need to visit all nodes if k is small.

3. **Left subtree contains smallest values**: Always recurse left first for kth smallest.

4. **k-th smallest = k-th in inorder sequence**.

## Test Case Analysis

### Test: `test_basic_kth_smallest`

```
Tree:
    3
   / \
  1   4
   \
    2

Inorder: 1, 2, 3, 4

k=1 → 1st = 1 ✓
k=2 → 2nd = 2 ✓
k=3 → 3rd = 3 ✓
k=4 → 4th = 4 ✓
```

### Test: `test_left_skewed`

```
    4
   /
  3
 /
2
/
1

All nodes are left children. Inorder still gives 1,2,3,4.

k=1 → 1 ✓
k=4 → 4 ✓
```

## Edge Cases Handled

1. **Single node**: count reaches 1 immediately → return that node
2. **Left-skewed**: All nodes visited in order through left pointers
3. **Right-skewed**: Every node has no left child, visit order is root first
4. **k=1**: Immediately returns leftmost node
5. **k=n**: Must traverse all nodes before finding

## Follow-up Answers

**Q: How to find kth largest?**
A: Option 1 - reverse inorder (right, node, left) for descending order. Option 2 - compute kth smallest from end.

**Q: Multiple kth queries?**
A: Use BST iterator or augment tree node with subtree size. O(1) queries after O(n) preprocessing.

**Q: Morris traversal for O(1) space?**
A: Yes - temporarily link nodes back to ancestors to find leftmost without extra stack/recursion.