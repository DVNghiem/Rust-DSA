# Subsets - LeetCode 78

## Problem Overview

Given an integer array `nums` of unique elements, return all possible subsets (the power set). The solution set cannot contain duplicate subsets.

**Examples:**
```
Input: nums = [1, 2, 3]
Output: [[], [1], [2], [3], [1,2], [1,3], [2,3], [1,2,3]]

Input: nums = [0]
Output: [[], [0]]
```

## Theory

### Power Set Mathematics

A set with `n` elements has exactly `2^n` subsets. This forms the foundation of our approach:

- For `n=3`: subsets = `2^3 = 8`
- For `n=10`: subsets = `2^10 = 1024`

### Visual Walkthrough - Bit Manipulation

```
nums = [1, 2, 3]

Binary  | Subsets
--------|-----------
000     | []           (empty set)
001     | [1]
010     | [2]
011     | [1,2]
100     | [3]
101     | [1,3]
110     | [2,3]
111     | [1,2,3]
```

Each bit position corresponds to whether an element is included:
- Bit 0 (LSB): Include element at index 0?
- Bit 1: Include element at index 1?
- Bit 2 (MSB): Include element at index 2?

### Backtracking Tree Visualization

```
                    []
           /        |        \
         [1]       [2]       [3]
        /  \       /  \        |
    [1,2] [1,3] [2,3]  []     [3]
       |
    [1,2,3]
```

## Approaches

### Approach 1: Iterative Building (Prepending)

```rust
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![vec![]];
    for num in nums {
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

**Process:**
1. Start with `[[]]`
2. Add `1`: `[[], [1]]`
3. Add `2`: `[[], [1], [2], [1,2]]`
4. Add `3`: `[[], [1], [2], [1,2], [3], [1,3], [2,3], [1,2,3]]`

### Approach 2: Bit Manipulation

```rust
pub fn subsets_bit(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let n = nums.len();
    let total = 1 << n;
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

### Approach 3: Backtracking

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

### Approach 4: Recursive (Different Branching)

```rust
pub fn subsets_recursive(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn helper(idx: usize, nums: &Vec<i32>, current: Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if idx == nums.len() {
            result.push(current);
            return;
        }
        // Exclude current element
        helper(idx + 1, nums, current.clone(), result);
        // Include current element
        let mut with_element = current;
        with_element.push(nums[idx]);
        helper(idx + 1, nums, with_element, result);
    }

    let mut result = Vec::new();
    helper(0, &nums, vec![], &mut result);
    result
}
```

## Complexity Analysis

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Iterative | O(n * 2^n) | O(2^n) | Most intuitive |
| Bit Manipulation | O(n * 2^n) | O(2^n) | Explicit control |
| Backtracking | O(n * 2^n) | O(n) | Memory efficient |
| Recursive | O(n * 2^n) | O(n) | Same as backtrack |

## Edge Cases

1. **Empty array**: `[]` -> `[[]]`
2. **Single element**: `[1]` -> `[[], [1]]`
3. **Duplicate elements**: Not in this problem (unique elements)
4. **All same elements**: `[1,1,1]` -> All 8 subsets
5. **Large arrays**: `[1..10]` -> 1024 subsets

## Test Cases

```rust
#[test]
fn test_subsets_basic() {
    assert_eq!(subsets(vec![1,2]), vec![vec![], vec![1], vec![2], vec![1,2]]);
}

#[test]
fn test_subsets_single() {
    assert_eq!(subsets(vec![1]), vec![vec![], vec![1]]);
}

#[test]
fn test_subsets_empty() {
    assert_eq!(subsets(vec![]), vec![vec![]]);
}
```

## Implementation Notes

1. **No Duplicates**: Problem states "unique elements", so no need to handle duplicates in subsets
2. **Order Matters**: Subsets can be in any order in result
3. **Immutable**: Prefer `clone()` over mutation for clarity
4. **Size Calculation**: Use `1 << n` for `2^n`, but beware of overflow for large n

## Follow-up Problems

- [LeetCode 90 - Subsets II](https://leetcode.com/problems/subsets-ii/) - With duplicate elements
- [LeetCode 78 - Subsets](https://leetcode.com/problems/subsets/) - This problem