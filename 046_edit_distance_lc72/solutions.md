# Edit Distance - Solution Analysis

## Problem Overview

Given two strings, find minimum operations to convert one to another. Operations: insert, delete, replace.

## Solution: DP

### Code Implementation

```rust
pub fn min_distance(word1: String, word2: String) -> i32 {
    let w1 = word1.as_bytes();
    let w2 = word2.as_bytes();
    let m = w1.len();
    let n = w2.len();

    let mut dp = vec![vec![0i32; n + 1]; m + 1];

    for i in 0..=m { dp[i][0] = i as i32; }
    for j in 0..=n { dp[0][j] = j as i32; }

    for i in 1..=m {
        for j in 1..=n {
            if w1[i-1] == w2[j-1] {
                dp[i][j] = dp[i-1][j-1];
            } else {
                dp[i][j] = 1 + dp[i-1][j].min(dp[i][j-1]).min(dp[i-1][j-1]);
            }
        }
    }

    dp[m][n]
}
```

### Key Points

- **dp[i][j]** = min operations to convert w1[0..i) to w2[0..j)
- **Base cases**: converting to/from empty string
- **Transition**: match or min(delete, insert, replace) + 1

### Example: "horse" to "ros"

```
dp table:
     ""  r   o   s
""   0   1   2   3
h    1   1   2   3
o    2   2   1   2
r    3   2   2   2
s    4   3   3   2
e    5   4   4   3

Result: dp[5][3] = 3 ✓
```

## Complexity

- **Time**: O(m×n)
- **Space**: O(m×n) or O(min(m,n)) optimized

## Follow-up Answers

**Q: Why 3 operations?**
A: Insert/delete/replace. The Levenshtein distance.

**Q: Time complexity O(m×n)?**
A: Yes, must fill entire dp table. Can't do better due to problem structure.