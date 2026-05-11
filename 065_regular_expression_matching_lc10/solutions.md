# Regular Expression Matching Solution - LeetCode 10 (Complete)

## Solution Analysis

### 2D Dynamic Programming

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
        if p[j - 1] == b'*' {
            dp[0][j] = dp[0][j - 2];
        }
    }

    for i in 1..=m {
        for j in 1..=n {
            match p[j - 1] {
                b'*' => {
                    dp[i][j] = dp[i][j - 2]; // Zero occurrences
                    if matches(s[i - 1], p[j - 2]) {
                        dp[i][j] = dp[i][j] || dp[i - 1][j];
                    }
                }
                b'.' => dp[i][j] = dp[i - 1][j - 1],
                c => dp[i][j] = dp[i - 1][j - 1] && s[i - 1] == c,
            }
        }
    }

    dp[m][n]
}

fn matches(s: u8, p: u8) -> bool {
    p == b'.' || s == p
}
```

## DP Definition

`dp[i][j]` = does `s[0..i]` match `p[0..j]`?

## Base Cases

### Empty string matches empty pattern
```rust
dp[0][0] = true;
```

### Empty string with patterns like "a*", "a*b*"
```rust
for j in 2..=n {
    if p[j - 1] == b'*' {
        dp[0][j] = dp[0][j - 2];
    }
}
```
- "a*" matches empty string (zero occurrences)
- "a*b*" matches empty string (zero occurrences)

## Transitions

### When p[j-1] is '*'
```rust
b'*' => {
    // Zero occurrences of preceding element
    dp[i][j] = dp[i][j - 2];

    // One or more occurrences (if preceding matches current char)
    if matches(s[i - 1], p[j - 2]) {
        dp[i][j] = dp[i][j] || dp[i - 1][j];
    }
}
```

'*' means "zero or more of preceding element":
1. **Zero occurrences**: Skip the "x*" entirely → dp[i][j-2]
2. **One or more**: If preceding element matches current s[i-1], we can extend → dp[i-1][j]

### When p[j-1] is '.'
```rust
b'.' => dp[i][j] = dp[i - 1][j - 1];
```
'.' matches any single character, so just consume both.

### When p[j-1] is a regular character
```rust
c => dp[i][j] = dp[i - 1][j - 1] && s[i - 1] == c;
```
Must match exactly.

## Visual Example

### Input: s = "aab", p = "c*a*b"

```
Pattern: c* a* b
c* = zero or more 'c'
a* = zero or more 'a'
b = literal 'b'

Step by step:
- c* matches empty (zero c's)
- a* matches "aa" (two a's)
- b matches "b"

Result: true
```

### DP Table for "aab" and "c*a*b"

```
     ""  c  *  a  *  b
""   T  F  T  F  T  F  T
a    F  F  F  F  T  F  F
a    F  F  F  F  T  F  F
b    F  F  F  F  T  F  T
```

## Why dp[i-1][j] for '*'?

When '*' matches one more occurrence:
- We're extending a match, so we look at dp[i-1][j]
- This represents "can s[0..i-1] match p[0..j] with one more '*' usage?"
- We don't consume p[j-1] because '*' still needs to potentially match more

## Complexity Analysis

| Approach | Time | Space |
|----------|------|-------|
| DP | O(mn) | O(mn) |
| Recursive + Memo | O(mn) | O(mn) |

## Edge Cases

### Empty s and p
```rust
Input: s = "", p = ""
Output: true
```

### Empty s with pattern
```rust
Input: s = "", p = "a*"
Output: true (a* can match empty)
```

### Single character match
```rust
Input: s = "a", p = "."
Output: true
```

### No match
```rust
Input: s = "aa", p = "a"
Output: false
```

## Why Initialize dp[0][j] for '*' Patterns?

Pattern "a*" can match empty string:
- "a*" = zero 'a's
- "a*b*" = zero 'a's, zero 'b's
- etc.

This is why we initialize: `dp[0][j] = dp[0][j-2]` for '*' patterns.

## Common Mistakes

1. **Not initializing dp[0][j]**: Would miss patterns like "a*"
2. **Confusing '*' with '?'**: '*' is zero or more, '?' is zero or one
3. **Forgetting to check preceding element**: '*' depends on p[j-2]