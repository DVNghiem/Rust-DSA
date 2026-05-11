# Longest Palindromic Substring - LeetCode 5

## Problem Statement

Given a string `s`, return the longest palindromic substring in `s`.

## Visual Walkthrough

```
Example:
s = "babad"

Output: "bab" or "aba" (both valid)

Example 2:
s = "cbbd"

Output: "bb"
```

### Expand Around Center

```
For each center, expand while characters match:

s = "babad"
Index 0: "b" -> expand -> "bab" (length 3)
Index 1: "a" -> expand -> "aba" (length 3)
Index 2: "b" -> expand -> "b" (length 1)
Index 3: "a" -> expand -> "ada" (length 3)
Index 4: "d" -> expand -> "d" (length 1)

Longest: "bab" or "aba" (length 3)
```

## Approach Comparison

| Approach | Time | Space | Description |
|----------|------|-------|-------------|
| Expand Around Center | O(n²) | O(1) | For each center, expand |
| Manacher's Algorithm | O(n) | O(n) | Advanced, linear |
| DP | O(n²) | O(n²) | dp[i][j] = palindrome check |

## Implementation Strategy

```rust
pub fn longest_palindrome(s: String) -> String {
    let bytes = s.as_bytes();
    let n = bytes.len();
    if n <= 1 { return s; }

    let (mut start, mut end) = (0, 0);

    for i in 0..n {
        // Odd length: center at i
        let (s1, e1) = expand(bytes, i, i);
        // Even length: center between i and i+1
        let (s2, e2) = expand(bytes, i, i + 1);

        if e1 - s1 > end - start { (start, end) = (s1, e1); }
        if e2 - s2 > end - start { (start, end) = (s2, e2); }
    }

    String::from_utf8_lossy(&bytes[start..=end]).to_string()
}

fn expand(s: &[u8], mut l: usize, mut r: usize) -> (usize, usize) {
    while l > 0 && r < s.len() && s[l] == s[r] {
        l -= 1;
        r += 1;
    }
    (l + 1, r - 1) // inclusive bounds
}
```

## Complexity Analysis

- **Time**: O(n²) worst case
- **Space**: O(1)

## Follow-up Questions

1. What is Manacher's algorithm?
2. How to return all palindromic substrings?