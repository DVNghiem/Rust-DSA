# Regular Expression Matching - LeetCode 10

## Problem Overview

Implement regular expression matching with support for '.' and '*' where:
- '.' matches any single character
- '*' matches zero or more of the preceding element

**Examples:**
```
Input: s = "aa", p = "a" → false
Input: s = "aa", p = "a*" → true
Input: s = "ab", p = ".*" → true
```

## Theory

### Dynamic Programming

`dp[i][j]` = does `s[0..i]` match `p[0..j]`?

```
Base cases:
- dp[0][0] = true (empty matches empty)
- dp[0][j] = dp[0][j-2] if p[j-1] == '*'

Transitions:
- p[j-1] is letter or '.': dp[i][j] = dp[i-1][j-1] && matches(s[i-1], p[j-1])
- p[j-1] is '*':
  - Zero occurrences: dp[i][j] = dp[i][j-2]
  - One+ occurrences: dp[i][j] |= dp[i-1][j] && matches(s[i-1], p[j-2])
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

    // Handle patterns like a*, a*b*, .* etc
    for j in 2..=n {
        if p[j-1] == b'*' {
            dp[0][j] = dp[0][j-2];
        }
    }

    for i in 1..=m {
        for j in 1..=n {
            match p[j-1] {
                b'*' => {
                    dp[i][j] = dp[i][j-2]; // Zero occurrences
                    if matches(s[i-1], p[j-2]) {
                        dp[i][j] = dp[i][j] || dp[i-1][j];
                    }
                }
                b'.' => dp[i][j] = dp[i-1][j-1],
                c => dp[i][j] = dp[i-1][j-1] && s[i-1] == c,
            }
        }
    }

    dp[m][n]
}

fn matches(s: u8, p: u8) -> bool {
    p == b'.' || s == p
}
```

## Test Cases

```rust
#[test]
fn test_is_match_basic() {
    assert!(is_match("aa".to_string(), "a".to_string()), false);
    assert!(is_match("aa".to_string(), "a*".to_string()), true);
}
```