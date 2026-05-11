# Decode Ways II Solution - LeetCode 639 (Complete)

## Solution Analysis

### DP with '*' Handling

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
        // Single character decode
        match bytes[i - 1] {
            b'*' => dp[i] = (dp[i] + dp[i - 1] * 9) % MOD,
            b'0' => {}
            _ => dp[i] = (dp[i] + dp[i - 1]) % MOD,
        }

        // Two character decode
        match (bytes[i - 2], bytes[i - 1]) {
            (b'1', b'*') => dp[i] = (dp[i] + dp[i - 2] * 9) % MOD,
            (b'2', b'*') => dp[i] = (dp[i] + dp[i - 2] * 6) % MOD,
            (b'*', b'*') => dp[i] = (dp[i] + dp[i - 2] * 15) % MOD,
            (b'1', _) => dp[i] = (dp[i] + dp[i - 2]) % MOD,
            (b'2', b'0'..=b'6') => dp[i] = (dp[i] + dp[i - 2]) % MOD,
            _ => {}
        }
    }

    dp[n] as i32
}
```

## DP Definition

`dp[i]` = number of ways to decode `s[0..i]`

## '*' Handling

### What is '*'?
- Represents any single digit 1-9
- So '*' can be decoded 9 ways as single character (1-9)

### '*' in Two-Digit Context

| Pattern | Interpretation | Number of Ways |
|---------|---------------|----------------|
| "1*" | "11" to "19" | 9 |
| "2*" | "21" to "26" | 6 |
| "**" | Any two digits forming valid number | 15 (11-19, 21-26) |

### Why 15 for "**"?
- 11-19: 9 combinations
- 21-26: 6 combinations
- Total: 15 valid two-digit numbers

## Line-by-Line Analysis

### Initialize dp[0]
```rust
dp[0] = 1;
```
- Empty string has exactly one decoding (do nothing)

### Handle First Character
```rust
match bytes[0] {
    b'*' => dp[1] = 9,    // * can be 1-9
    b'0' => return 0,    // Invalid: 0 can't be decoded alone
    _ => dp[1] = 1,      // Normal digit: only 1 way
}
```

### Single Character Decoding
```rust
match bytes[i - 1] {
    b'*' => dp[i] = (dp[i] + dp[i - 1] * 9) % MOD,
    // 9 ways: * could be 1-9
    b'0' => {}  // 0 alone is invalid
    _ => dp[i] = (dp[i] + dp[i - 1]) % MOD,
    // Normal digit: 1 way
}
```

### Two Character Decoding
```rust
match (bytes[i - 2], bytes[i - 1]) {
    (b'1', b'*') => dp[i] = (dp[i] + dp[i - 2] * 9) % MOD,
    // 1* → 11-19, 9 ways
    (b'2', b'*') => dp[i] = (dp[i] + dp[i - 2] * 6) % MOD,
    // 2* → 21-26, 6 ways
    (b'*', b'*') => dp[i] = (dp[i] + dp[i - 2] * 15) % MOD,
    // ** → 15 valid combinations
    (b'1', _) => dp[i] = (dp[i] + dp[i - 2]) % MOD,
    // 1X → valid if X is any digit
    (b'2', b'0'..=b'6') => dp[i] = (dp[i] + dp[i - 2]) % MOD,
    // 20-26 → valid
    _ => {}  // Invalid two-digit combination
}
```

## Visual Example

### Input: s = "*"

```
dp[0] = 1
dp[1] = 9 (* could be 1-9)
Return 9
```

### Input: s = "1*"

```
dp[0] = 1

bytes[0] = '1' → dp[1] = 1

i=2:
- Single: bytes[1] = '*' → dp[2] += dp[1] * 9 = 9
- Two: (bytes[0], bytes[1]) = ('1', '*') → dp[2] += dp[0] * 9 = 9

dp[2] = 18
Return 18
```

Verifies: 11-19 (9 ways) + 1 alone + * alone (9 ways) = 11 + 9 + 9? 

Wait, let me recalculate:
- "11" to "19": 9 ways
- "1" + "*": * can be 1-9 → 9 ways
- Total: 18 ways ✓

## Complexity Analysis

| Approach | Time | Space |
|----------|------|-------|
| DP | O(n) | O(n) |
| Optimized DP | O(n) | O(1) |

## Edge Cases

### Leading Zero
```rust
Input: s = "0"
Output: 0 (invalid)
```

### Adjacent Zeros
```rust
Input: s = "00"
Output: 0
```

### Empty String
```rust
Input: s = ""
Output: 1 (empty has one way)
```

## Why Modulo?

Results can be very large (2^31 possibilities). We use modulo 10^9 + 7 to prevent overflow.

## Common Mistakes

1. **Not handling leading zero**: "0" alone is invalid
2. **Confusing single vs double decode**: Need to check both conditions
3. **Forgetting modulo**: Causes integer overflow

## '*' as Single Digit

When '*' is decoded alone, it can be 1-9 (not 0), so 9 possibilities.

When '*' is part of "1*" or "2*", we count valid two-digit numbers:
- "1*" = 11, 12, ..., 19 → 9 ways
- "2*" = 21, 22, ..., 26 → 6 ways (because 27+ are invalid)