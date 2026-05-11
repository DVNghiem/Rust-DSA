# Unique Paths - LeetCode 62

## Problem Statement

A robot is located at the top-left corner of an m x n grid. The robot can only move either down or right at any point in time.

Return the number of different possible paths that the robot can take to reach the bottom-right corner.

## Visual Walkthrough

```
m = 3, n = 7

Grid:
S . . . . . .
. . . . . . .
. . . . . . . E

S = start (0,0)
E = end (2,6)

Robot can only move: Down (D) or Right (R)
Each path has (m-1) D's and (n-1) R's = m+n-2 moves total
Number of paths = combinations of arranging D's and R's
```

### Combinatorial Solution

```
Total moves = (m-1) + (n-1)
Choose (m-1) positions for Down moves
Result = C(m+n-2, m-1) or C(m+n-2, n-1)

For m=3, n=7:
Total moves = 2 + 6 = 8
Choose 2 for D: C(8,2) = 8!/(2!6!) = 28
```

## Approach Comparison

| Approach | Time | Space | Description |
|----------|------|-------|-------------|
| Combinatorial | O(min(m,n)) | O(1) | Math formula |
| DP | O(m×n) | O(n) | Classic grid DP |

## Implementation Strategy

```rust
pub fn unique_paths(m: i32, n: i32) -> i32 {
    // Combinatorial: C(m+n-2, m-1)
    let m = m as usize;
    let n = n as usize;

    let k = (m - 1).min(n - 1);
    let total = m + n - 2;

    let mut result: f64 = 1.0;
    for i in 0..k {
        result *= (total - i) as f64 / (i + 1) as f64;
    }

    result as i32
}
```

## Edge Cases

1. **1x1 grid**: Return 1
2. **1 row or 1 column**: Return 1 (only straight line path)
3. **Large m, n**: Handle overflow with f64 or big integers

## Solution Explanation

### Key Insight

Each path is a sequence of (m-1) D's and (n-1) R's. Number of unique paths = number of ways to arrange these moves = binomial coefficient.

## Complexity Analysis

- **Time**: O(min(m, n))
- **Space**: O(1)

## Follow-up Questions

1. What if obstacles are added (LeetCode 63)?
2. How to handle very large numbers?