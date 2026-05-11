# Decode Ways II - LeetCode 639

## Problem Overview

A message containing letters A-Z is encoded using the following mapping: 'A' -> 1, ... 'Z' -> 26. Given an encoded message, count the number of ways to decode it. '*' can be any digit (1-9).

**Examples:**
```
Input: s = "*"
Output: 9 (1-9)

Input: s = "1*"
Output: 18 (11-19, plus 1 and * individually)
```

## Theory

### DP with Special '*' Handling

`dp[i]` = number of ways to decode `s[0..i]`

For each position:
- Single digit: if valid, add dp[i-1]
- Double digit: if valid, add dp[i-2]

Special cases for '*': 9 for single, 15 for double (11-19 except 10, 20)

## Implementation

```rust
pub fn num_decodings(s: String) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut dp = vec![0i64; n + 1];
    dp[0] = 1;

    // First character
    match bytes[0] {
        b'*' => dp[1] = 9,
        b'0' => return 0,
        _ => dp[1] = 1,
    }

    for i in 2..=n {
        // Single digit
        match bytes[i-1] {
            b'*' => dp[i] += dp[i-1] * 9,
            b'0' => {}
            _ => dp[i] += dp[i-1],
        }
        dp[i] %= MOD;

        // Double digit
        match (bytes[i-2], bytes[i-1]) {
            (b'1', b'*') => dp[i] += dp[i-2] * 9,
            (b'2', b'*') => dp[i] += dp[i-2] * 6,
            (b'*', b'*') => dp[i] += dp[i-2] * 15,
            (b'1', _) => dp[i] += dp[i-2],
            (b'2', b'0'..=b'6') => dp[i] += dp[i-2],
            _ => {}
        }
        dp[i] %= MOD;
    }

    dp[n] as i32
}
```

## Test Cases

```rust
#[test]
fn test_decode_ways_basic() {
    assert_eq!(num_decodings("12".to_string()), 2);
}
```