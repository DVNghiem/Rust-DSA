# Minimum Path Sum - LeetCode 64

## Problem Statement

Given an m x n grid filled with non-negative numbers, find a path from top-left to bottom-right that minimizes the sum of all numbers along its path.

You can only move either down or right at any point in time.

## Visual Walkthrough

```
Grid:
[1, 3, 1]
[1, 5, 1]
[4, 2, 1]

Optimal path: 1→3→1→1→1 = 7

Path:
(0,0) → (0,1) → (0,2)
         ↓
       (1,2) → (2,2)

Sum: 1 + 3 + 1 + 1 + 1 = 7
```

### DP Approach

```
dp[i][j] = grid[i][j] + min(dp[i-1][j], dp[i][j-1])

dp[0][0] = grid[0][0]
First row: can only come from left
First column: can only come from above
```

## Approach Comparison

| Approach | Time | Space | Description |
|----------|------|-------|-------------|
| DP 2D | O(m×n) | O(m×n) | Full dp array |
| DP In-place | O(m×n) | O(1) | Modify grid |

## Implementation Strategy

```rust
pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut dp = vec![vec![0i32; n]; m];

    for i in 0..m {
        for j in 0..n {
            if i == 0 && j == 0 {
                dp[i][j] = grid[i][j];
            } else if i == 0 {
                dp[i][j] = dp[i][j-1] + grid[i][j];
            } else if j == 0 {
                dp[i][j] = dp[i-1][j] + grid[i][j];
            } else {
                dp[i][j] = grid[i][j] + dp[i-1][j].min(dp[i][j-1]);
            }
        }
    }

    dp[m-1][n-1]
}
```

## Edge Cases

1. **1x1 grid**: Return single element
2. **1 row or 1 column**: Only one path possible
3. **All zeros**: Return 0

## Complexity Analysis

- **Time**: O(m×n)
- **Space**: O(m×n) or O(1) with in-place modification

## Follow-up Questions

1. How to reconstruct the path?
2. What if multiple paths have same minimum sum?