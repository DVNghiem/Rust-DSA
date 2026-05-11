# Edit Distance - LeetCode 72

## Problem Statement

Given two strings word1 and word2, return the minimum number of operations required to convert word1 to word2.

You have three operations:
- Insert a character
- Delete a character
- Replace a character

## Visual Walkthrough

```
Example:
word1 = "horse", word2 = "ros"

horse → ros (3 operations)
1. Replace 'h' with 'r' → ros (but we removed h, need r)
Actually:
h→r (replace)
o→o (keep)
r→s (replace)
s→r (replace)
e→ (delete)

Let's trace properly:
horse → rorse (replace h with r)
rorse → rose (replace s with nothing? No delete s)
rose → ros (delete e)

That's 3 operations. ✓
```

### DP State

```
dp[i][j] = minimum operations to convert word1[0..i) to word2[0..j)

If word1[i-1] == word2[j-1]:
  dp[i][j] = dp[i-1][j-1]
Else:
  dp[i][j] = 1 + min(
    dp[i-1][j]    (delete from word1)
    dp[i][j-1]    (insert into word1)
    dp[i-1][j-1]  (replace)
  )
```

## Approach Comparison

| Approach | Time | Space | Description |
|----------|------|-------|-------------|
| DP 2D | O(m×n) | O(m×n) | Standard |
| DP 1D | O(m×n) | O(min(m,n)) | Space optimized |

## Implementation Strategy

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

## Complexity Analysis

- **Time**: O(m×n)
- **Space**: O(m×n) or O(min(m,n)) optimized

## Follow-up Questions

1. How to track which operations?
2. What if we need all possible edit sequences?