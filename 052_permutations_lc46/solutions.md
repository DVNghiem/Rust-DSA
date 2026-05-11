# Permutations Solution - LeetCode 46 (Complete)

## Solution Analysis

### Backtracking with Used Array

```rust
pub fn permutations_backtrack(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut path = Vec::new();
    let mut used = vec![false; nums.len()];

    fn backtrack(nums: &Vec<i32>, path: &mut Vec<i32>, used: &mut Vec<bool>, result: &mut Vec<Vec<i32>>) {
        if path.len() == nums.len() {
            result.push(path.clone());
            return;
        }
        for i in 0..nums.len() {
            if !used[i] {
                used[i] = true;
                path.push(nums[i]);
                backtrack(nums, path, used, result);
                path.pop();
                used[i] = false;
            }
        }
    }

    backtrack(&nums, &mut path, &mut used, &mut result);
    result
}
```

**Key Insight**: Track used elements with boolean array. At each position, try each unused element.

### Swap-Based Approach

```rust
pub fn permutations_swap(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut nums = nums;

    fn backtrack(nums: &mut Vec<i32>, start: usize, result: &mut Vec<Vec<i32>>) {
        if start == nums.len() {
            result.push(nums.clone());
            return;
        }
        for i in start..nums.len() {
            nums.swap(start, i);
            backtrack(nums, start + 1, result);
            nums.swap(start, i); // backtrack
        }
    }

    backtrack(&mut nums, 0, &mut result);
    result
}
```

**Key Insight**: Fix position `start`, try all elements from `start` onward, swap back to restore.

## Why Both Approaches Work

1. **Used Array**: Explicit tracking of which elements are in current path
2. **Swap**: Implicit tracking through position fixing

Both generate n! permutations because:
- At position 0: n choices
- At position 1: n-1 choices
- ...total: n × (n-1) × ... × 1 = n!

## Step-by-Step Trace for [1,2,3]

Using backtracking:
```
[]
├── [1] → [] → [1,2] → [] → [1,2,3] ✓
│        ↓
│        [1,3] ✓
│
├── [2] → [] → [2,1] ✓
│        ↓
│        [2,3] ✓
│
└── [3] → [] → [3,1] ✓
         ↓
         [3,2] ✓
```

Result: 6 permutations

## Complexity Analysis

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Backtrack + Used | O(n × n!) | O(n) | + O(n!) for result |
| Swap-Based | O(n × n!) | O(n) | + O(n!) for result |

## Edge Cases

1. **Empty input**: Returns `[[]]`
2. **Single element**: Returns `[[element]]`
3. **Two elements**: Returns 2 permutations

## Common Mistakes

1. **Forgetting to unmark used**: Element would be "lost"
2. **Forgetting to pop**: Path contains extra elements
3. **Not cloning before push**: Corrupts stored permutations

## Rust-Specific Patterns

1. **Nested functions**: `fn backtrack(...)` captures variables
2. **Boolean array**: `vec![false; n]` for tracking
3. **Clone for preservation**: `path.clone()` essential