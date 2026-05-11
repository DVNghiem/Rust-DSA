# Unique Paths - Solution Analysis

## Problem Overview

Grid m×n, robot at top-left, can only move down/right. Find number of paths to bottom-right.

## Solution: Combinatorial

### Code Implementation

```rust
pub fn unique_paths(m: i32, n: i32) -> i32 {
    let m = m as usize;
    let n = n as usize;

    let total = m + n - 2;
    let k = (m - 1).min(n - 1);

    let mut result: f64 = 1.0;
    for i in 0..k {
        result *= (total - i) as f64 / (i + 1) as f64;
    }

    result as i32
}
```

### Why This Works

- Each path has (m-1) D's and (n-1) R's
- Total moves = (m-1)+(n-1)
- Choose positions for D's (or R's): C(m+n-2, m-1)

### Example: m=3, n=7

```
Total moves = 2 + 6 = 8
Choose 2 positions for D: C(8,2) = 28 paths
```

## DP Alternative

```rust
dp[i][j] = dp[i-1][j] + dp[i][j-1]
```

## Complexity Comparison

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Combinatorial | O(min(m,n)) | O(1) | Optimal |
| DP 2D | O(m×n) | O(m×n) | Intuitive |
| DP 1D | O(m×n) | O(n) | Space optimized |

## Follow-up Answers

**Q: Why combinatorics works?**
A: Each path is unique arrangement of D and R moves.

**Q: What if obstacles added?**
A: DP approach, set dp[i][j]=0 if blocked.