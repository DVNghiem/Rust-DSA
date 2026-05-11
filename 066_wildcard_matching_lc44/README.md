# Wildcard Matching - LeetCode 44

## Problem Overview

Implement wildcard pattern matching supporting '?' and '*' where:
- '?' matches any single character
- '*' matches any sequence (including empty)

**Examples:**
```
Input: s = "aa", p = "*" → true
Input: s = "cb", p = "?a" → false
Input: s = "adceb", p = "*a*b" → true
```

## Theory

### DP Approach

`dp[i][j]` = does `s[0..i]` match `p[0..j]`?

```
Base cases:
- dp[0][0] = true
- dp[0][j] = dp[0][j-1] if p[j-1] == '*'

Transitions:
- p[j-1] == '?': dp[i][j] = dp[i-1][j-1]
- p[j-1] == '*': dp[i][j] = dp[i][j-1] || dp[i-1][j]
- else: dp[i][j] = dp[i-1][j-1] && s[i-1] == p[j-1]
```

## Implementation

```rust
pub fn is_match(s: String, p: String) -> bool {
    let s = s.as_bytes();
    let p = p.as_bytes();
    let m = s.len();
    let n = p.len();
    let mut dp = vec![vec![false; n + 1]; m + 1];
    dp[0][0] = true;

    // Handle patterns starting with '*'
    for j in 1..=n {
        if p[j-1] == b'*' {
            dp[0][j] = dp[0][j-1];
        }
    }

    for i in 1..=m {
        for j in 1..=n {
            match p[j-1] {
                b'?' => dp[i][j] = dp[i-1][j-1],
                b'*' => dp[i][j] = dp[i][j-1] || dp[i-1][j],
                c => dp[i][j] = dp[i-1][j-1] && s[i-1] == c,
            }
        }
    }

    dp[m][n]
}
```

## Test Cases

```rust
#[test]
fn test_is_match_star() {
    assert!(is_match("aa".to_string(), "*".to_string()));
}
```