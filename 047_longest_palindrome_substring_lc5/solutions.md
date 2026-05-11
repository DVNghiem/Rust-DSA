# Longest Palindromic Substring - Solution Analysis

## Problem Overview

Find longest substring that is a palindrome.

## Solution: Expand Around Center

### Code Implementation

```rust
pub fn longest_palindrome(s: String) -> String {
    let bytes = s.as_bytes();
    let n = bytes.len();
    if n <= 1 { return s; }

    let (mut start, mut end) = (0, 0);

    fn expand(s: &[u8], mut l: usize, mut r: usize) -> (usize, usize) {
        while l > 0 && r < s.len() && s[l] == s[r] {
            l -= 1;
            r += 1;
        }
        (l + 1, r - 1)
    }

    for i in 0..n {
        let (s1, e1) = expand(bytes, i, i);     // odd
        let (s2, e2) = expand(bytes, i, i + 1); // even

        if e1 - s1 > end - start { start = s1; end = e1; }
        if e2 - s2 > end - start { start = s2; end = e2; }
    }

    String::from_utf8_lossy(&bytes[start..=end]).to_string()
}
```

### Why Two Centers?

- **Odd palindrome**: "aba" center is 'b'
- **Even palindrome**: "abba" center is between 'b' and 'b'

We check both possibilities for each index.

### Example: "babad"

```
Index 1 (odd center 'a'):
expand(1,1): "a" -> expand -> "bab" ✓

Index 1 (even center between 1,2):
expand(1,2): "ab" -> no expand

Result: "bab" or "aba" ✓
```

## Complexity

| Approach | Time | Space |
|----------|------|-------|
| Expand | O(n²) | O(1) |
| DP | O(n²) | O(n²) |
| Manacher | O(n) | O(n) |

## Follow-up Answers

**Q: Why O(n²) worst case?**
A: For string like "aaaaa...", we expand for each center, each expansion is O(n).

**Q: Manacher's algorithm?**
A: Uses previously computed information to achieve O(n). More complex to implement.