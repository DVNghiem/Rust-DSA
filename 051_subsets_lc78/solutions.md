# Subsets Solution - LeetCode 78 (Complete)

## Solution Analysis

### Iterative Approach Line-by-Line

```rust
pub fn subsets_iterative(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![vec![]];
    for num in nums {
        // For each number, create new subsets by adding it to existing ones
        let new_subsets: Vec<Vec<i32>> = result
            .iter()
            .map(|subset| {
                let mut new = subset.clone();
                new.push(num);
                new
            })
            .collect();
        result.extend(new_subsets);
    }
    result
}
```

**Key Points:**
1. Start with `[[]]` - one empty subset
2. For each number, clone all existing subsets and add the new number
3. Extend result with all new subsets
4. Time: O(n × 2^n), Space: O(2^n)

### Bit Manipulation Approach

```rust
pub fn subsets_bit(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let n = nums.len();
    let total = 1 << n;  // 2^n subsets
    let mut result = Vec::new();

    for mask in 0..total {
        let mut subset = Vec::new();
        for i in 0..n {
            if mask & (1 << i) != 0 {
                subset.push(nums[i]);
            }
        }
        result.push(subset);
    }
    result
}
```

**Key Points:**
1. Each mask from 0 to 2^n-1 represents a subset
2. Bit i is set means include nums[i]
3. Clean, explicit approach

### Backtracking Approach

```rust
pub fn subsets_backtrack(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut path = Vec::new();

    fn backtrack(start: usize, nums: &Vec<i32>, path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        result.push(path.clone());
        for i in start..nums.len() {
            path.push(nums[i]);
            backtrack(i + 1, nums, path, result);
            path.pop();
        }
    }

    backtrack(0, &nums, &mut path, &mut result);
    result
}
```

**Key Points:**
1. Each recursion level adds current path to result
2. Try including each element, recurse, then backtrack
3. Start index prevents duplicates

## Why Clone is Essential

When using iterative approach:
```rust
let mut new = subset.clone();  // MUST clone
new.push(num);                  // Modify clone
```

Without clone, we'd be modifying the original subset in result.

## Complexity Summary

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Iterative | O(n × 2^n) | O(2^n) | Most intuitive |
| Bit Manipulation | O(n × 2^n) | O(2^n) | Explicit control |
| Backtracking | O(n × 2^n) | O(n) | Memory efficient |

## Edge Cases

1. **Empty input**: Returns `[[]]` - one empty subset
2. **Single element**: Returns `[[], [element]]`
3. **Large input**: Works up to reasonable n (2^n subsets)

## Common Patterns

1. **Building from empty**: Start with base case and grow
2. **Clone-before-modify**: Essential for correctness
3. **Two-dimensional output**: Vec<Vec<T>> pattern