# Decode Ways - LeetCode 91

## Problem Statement

A message containing letters from 'A' to 'Z' is being encoded to numbers using the following mapping:

'A' -> 1, 'B' -> 2, ... 'Z' -> 26

Given a non-empty string s containing only digits, return the number of ways to decode it.

## Visual Walkthrough

```
Example:
s = "12"

Could be:
- '1' (A) + '2' (B) = "AB"
- '12' (L) = "L"

Total: 2 ways

Example 2:
s = "226"

Could be:
- "2" (B), "2" (B), "6" (F) = "BBF"
- "2" (B), "26" (Z) = "BZ"
- "22" (V), "6" (F) = "VF"

Total: 3 ways
```

### Invalid Cases

```
s = "0"
No letter maps to 0, return 0

s = "10"
Only "10" = J, return 1

s = "100"
No valid decoding, return 0
```

## Approach Comparison

| Approach | Time | Space | Description |
|----------|------|-------|-------------|
| DP | O(n) | O(n) | Standard digit-by-digit |
| DP Space Optimized | O(n) | O(1) | Only need previous two |

## Implementation Strategy

```rust
pub fn num_decodings(s: String) -> i32 {
    if s.is_empty() || s.as_bytes()[0] == b'0' { return 0; }

    let bytes = s.as_bytes();
    let n = bytes.len();

    let mut dp = vec![0i32; n + 1];
    dp[0] = 1;  // empty string = 1 way
    dp[1] = 1;  // first character always valid (1-9)

    for i in 2..=n {
        // Single digit (10-26 without leading zero)
        if bytes[i - 1] != b'0' {
            dp[i] += dp[i - 1];
        }

        // Two digits (10-26)
        let two = (bytes[i - 2] as i32 - b'0') * 10 + (bytes[i - 1] as i32 - b'0');
        if two >= 10 && two <= 26 {
            dp[i] += dp[i - 2];
        }
    }

    dp[n]
}
```

## Edge Cases

1. **Empty string**: Return 0 (non-empty per problem)
2. **String starting with '0'**: Invalid, return 0
3. **String with "30"**: Invalid (no 30 mapping)
4. **String with "100"**: Invalid
5. **Large string**: DP handles efficiently

## Test Cases

1. Basic examples
2. String starting with 0
3. Invalid patterns
4. All valid single digits
5. Two-digit patterns

## Solution Explanation

### Key Insight

dp[i] = number of ways to decode s[0..i)
- If s[i-1] is valid single digit (not '0'), dp[i] += dp[i-1]
- If s[i-2..i] is valid two digits (10-26), dp[i] += dp[i-2]

## Complexity Analysis

- **Time**: O(n)
- **Space**: O(n) or O(1) optimized

## Follow-up Questions

1. How to handle strings with invalid patterns?
2. What if we need to return actual decodings?