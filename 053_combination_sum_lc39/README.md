# Combination Sum - LeetCode 39

## Problem Overview

Given an array of distinct integers `candidates` and a target integer `target`, return all unique combinations of candidates where they sum to target. The same number may be used unlimited times.

**Examples:**
```
Input: candidates = [2,3,6,7], target = 7
Output: [[2,2,3], [7]]

Input: candidates = [2,3,5], target = 8
Output: [[2,2,2,2], [2,3,3], [3,5]]

Input: candidates = [2], target = 1
Output: []
```

## Theory

### Key Differences from Permutations

- **Order doesn't matter**: [2,2,3] is same as [3,2,2]
- **Unlimited usage**: Each element can be used multiple times
- **Unique combinations**: No duplicate result arrays

### Backtracking Tree Visualization

```
candidates = [2, 3, 5], target = 8

                        [] (sum=0)
            /           |           \
         [2]            [3]          [5]
       sum=2          sum=3        sum=5
       /    \          |              |
   [2,2]   [2,3]     [3,3]          skip
   sum=4   sum=5     sum=6
     |       |         |
   [2,2,2] [2,2,3]   [3,3,2]        [3,5]
   sum=6   sum=7     sum=8 ✓        sum=8 ✓
     |
   [2,2,2,2]
   sum=8 ✓
```

## Approaches

### Approach 1: Backtracking with Index (Standard)

```rust
pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut path = Vec::new();

    fn backtrack(start: usize, candidates: &Vec<i32>, target: i32, path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if target == 0 {
            result.push(path.clone());
            return;
        }
        for i in start..candidates.len() {
            if candidates[i] <= target {
                path.push(candidates[i]);
                backtrack(i, candidates, target - candidates[i], path, result);
                path.pop();
            }
        }
    }

    backtrack(0, &candidates, target, &mut path, &mut result);
    result
}
```

### Approach 2: Sorting + Pruning

```rust
pub fn combination_sum_sorted(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut candidates = candidates;
    candidates.sort();
    let mut result = Vec::new();
    let mut path = Vec::new();

    fn backtrack(start: usize, candidates: &Vec<i32>, target: i32, path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if target == 0 {
            result.push(path.clone());
            return;
        }
        for i in start..candidates.len() {
            if candidates[i] > target {
                break; // sorted, so all subsequent will also be > target
            }
            path.push(candidates[i]);
            backtrack(i, candidates, target - candidates[i], path, result);
            path.pop();
        }
    }

    backtrack(0, &candidates, target, &mut path, &mut result);
    result
}
```

## Complexity Analysis

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Backtrack (unsorted) | O(n^target) | O(target) | Exponential |
| Backtrack (sorted) | O(n × target) | O(target) | Pruning helps |

## Edge Cases

1. **No solution**: target too small or candidates too large
2. **Empty candidates**: []
3. **Target is zero**: [[]]
4. **Single candidate equals target**: [2], target=2 → [[2]]
5. **Single candidate less than target**: [2], target=1 → []

## Visual: Why Index Tracking Matters

```
candidates = [2, 3], target = 6

WITH index (start=i):
- [2,2,2] ✓
- [2,3] ✓
- [3,2] ✗ (not included, preserves uniqueness)

WITHOUT index:
- [2,2,2] ✓
- [2,3] ✓
- [3,2] ✓ (duplicate!)
- [3,3] ✓
```

## Test Cases

```rust
#[test]
fn test_combination_sum_basic() {
    let result = combination_sum(vec![2,3,6,7], 7);
    assert!(result.contains(&vec![2,2,3]));
    assert!(result.contains(&vec![7]));
}
```

## Follow-up Problems

- [LeetCode 40 - Combination Sum II](https://leetcode.com/problems/combination-sum-ii/) - Each number can only be used once