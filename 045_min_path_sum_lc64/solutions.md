# Minimum Path Sum - Solution Analysis

## Problem Overview

Grid of non-negative numbers. Find path from top-left to bottom-right minimizing sum. Can only move down/right.

## Solution: DP

### Code Implementation

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

### Line-by-Line Analysis

1. **`if i == 0 && j == 0`**: Starting cell, sum = grid value.
2. **`else if i == 0`**: First row, can only come from left.
3. **`else if j == 0`**: First column, can only come from above.
4. **`dp[i][j] = grid[i][j] + dp[i-1][j].min(dp[i][j-1])`**: Take minimum of two possible predecessors, add current cell.

### DP Table Visualization

```
Grid:
[1, 3, 1]
[1, 5, 1]
[4, 2, 1]

dp:
[1, 4, 5]
[2, 7, 6]
[6, 8, 7]

dp[2][2] = 7 (answer) ✓

Paths:
1+3+1+1+1 = 7 (optimal)
1+1+5+1+1 = 9
1+1+4+2+1 = 9
etc.
```

## Complexity

| Approach | Time | Space |
|----------|------|-------|
| DP | O(m×n) | O(m×n) |
| In-place | O(m×n) | O(1) |

## Follow-up Answers

**Q: Path reconstruction?**
A: Work backwards from dp[m-1][n-1], at each step choose direction that gives smaller predecessor.

**Q: Multiple optimal paths?**
A: Yes, but we just return the minimum sum, not which path.