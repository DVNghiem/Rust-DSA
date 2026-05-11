# Wildcard Matching Solution - LeetCode 44 (Complete)

## Solution Analysis

### DP Approach

```rust
pub fn is_match(s: String, p: String) -> bool {
    let s = s.as_bytes();
    let p = p.as_bytes();
    let m = s.len();
    let n = p.len();
    let mut dp = vec![vec![false; n + 1]; m + 1];
    dp[0][0] = true;

    // Initialize first row (empty string)
    for j in 1..=n {
        if p[j - 1] == b'*' {
            dp[0][j] = dp[0][j - 1];
        }
    }

    for i in 1..=m {
        for j in 1..=n {
            match p[j - 1] {
                b'?' => dp[i][j] = dp[i - 1][j - 1],
                b'*' => dp[i][j] = dp[i][j - 1] || dp[i - 1][j],
                c => dp[i][j] = dp[i - 1][j - 1] && s[i - 1] == c,
            }
        }
    }

    dp[m][n]
}
```

## DP Definition

`dp[i][j]` = does `s[0..i]` match `p[0..j]`?

## Key Differences from Regular Expression

| Feature | Regex (LeetCode 10) | Wildcard (LeetCode 44) |
|---------|---------------------|------------------------|
| '*' | Zero or more of preceding | Zero or more of ANY |
| '?' | Not in Regex | Matches any single |
| '.' | Matches any single | Not used here |

### Wildcard '*' is simpler
- '*' matches ANY sequence (including empty)
- No need to look at preceding character

## Transitions

### When p[j-1] is '?'
```rust
b'?' => dp[i][j] = dp[i - 1][j - 1];
```
'?' matches exactly one character, so we consume both.

### When p[j-1] is '*'
```rust
b'*' => dp[i][j] = dp[i][j - 1] || dp[i - 1][j];
```
'*' can match:
- **Zero characters**: dp[i][j-1] (skip '*' in pattern)
- **One or more characters**: dp[i-1][j] (use '*' to match s[i-1] and continue)

### When p[j-1] is regular character
```rust
c => dp[i][j] = dp[i - 1][j - 1] && s[i - 1] == c;
```
Must match exactly.

## Visual Example

### Input: s = "adceb", p = "*a*b"

```
Pattern: *a*b
- * matches any sequence (including empty)
- 'a' matches 'a'
- * matches any sequence
- 'b' matches 'b'

Matching process:
- '*' at start matches "" (empty)
- Then 'a' matches 'a'
- Then '*' matches "dce" (zero or more)
- Then 'b' matches 'b'

Result: true
```

### DP Table for "adceb" and "*a*b"

```
     ""  *  a  *  b
""   T  T  F  T  F
a    F  F  T  T  F
d    F  F  F  T  F
c    F  F  F  T  F
e    F  F  F  T  F
b    F  F  F  T  T
```

## Greedy Alternative (Optimal O(n+m))

```rust
pub fn is_match_greedy(s: String, p: String) -> bool {
    let s = s.as_bytes();
    let p = p.as_bytes();
    let mut s_idx = 0;
    let mut p_idx = 0;
    let mut star_idx = None;
    let mut s_tmp_idx = None;

    while s_idx < s.len() {
        if p_idx < p.len() && (p[p_idx] == b'?' || p[p_idx] == s[s_idx]) {
            s_idx += 1;
            p_idx += 1;
        } else if p_idx < p.len() && p[p_idx] == b'*' {
            star_idx = Some(p_idx);
            s_tmp_idx = Some(s_idx);
            p_idx += 1;
        } else if let Some(star) = star_idx {
            p_idx = star + 1;
            s_idx = s_tmp_idx.unwrap() + 1;
            s_tmp_idx = Some(s_idx);
        } else {
            return false;
        }
    }

    while p_idx < p.len() && p[p_idx] == b'*' {
        p_idx += 1;
    }

    p_idx == p.len()
}
```

### Greedy Algorithm Intuition

1. When we see '*', remember its position and where we are in s
2. When we can't match, try extending the last '*'
3. Keep track of where '*' could match different portions

## Complexity Analysis

| Approach | Time | Space |
|----------|------|-------|
| DP | O(mn) | O(mn) |
| Greedy | O(m+n) | O(1) |

## Edge Cases

### Empty s and p
```rust
Input: s = "", p = ""
Output: true
```

### Empty s with '*'
```rust
Input: s = "", p = "*"
Output: true
```

### '?' matches nothing
```rust
Input: s = "", p = "?"
Output: false
```

## Why Greedy Works for Wildcard

Unlike regex where '*' depends on preceding element, wildcard '*' always matches any sequence. This allows us to:

1. **Try to consume as little as possible** with '*'
2. **Only extend '*' when we must**
3. **Backtrack by extending last seen '*' when needed**

## Common Mistakes

1. **Confusing with regex**: Wildcard '*' is simpler (any character)
2. **Not handling trailing '*'**: Multiple '*' at end should all be consumed
3. **Off-by-one in loops**: Remember dp[i][j] is for s[0..i] and p[0..j]