# Decode Ways - Solution Analysis

## Problem Overview

Given digit string, count ways to decode to letters (1=A, 2=B, ... 26=Z).

## Solution: DP

### Code Implementation

```rust
pub fn num_decodings(s: String) -> i32 {
    if s.is_empty() || s.as_bytes()[0] == b'0' { return 0; }

    let bytes = s.as_bytes();
    let n = bytes.len();

    let mut dp = vec![0i32; n + 1];
    dp[0] = 1;
    dp[1] = 1;

    for i in 2..=n {
        // Single digit (if not '0')
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

### Line-by-Line Analysis

1. **Single digit check**: `bytes[i-1] != b'0'` - 1-9 are valid single digits.
2. **Two digit check**: `two >= 10 && two <= 26` - values 10-26 are valid two-digit decodings.
3. **dp[i] accumulation**: Add from single (dp[i-1]) and two-digit (dp[i-2]).

### DP Table Visualization

```
s = "226"

dp[0] = 1 (empty)
dp[1] = 1 (only "2")

i=2: bytes[0]='2', bytes[1]='2'
  - Single: '2' != '0' → dp[2] += dp[1] = 1
  - Two: 22 is valid → dp[2] += dp[0] = 1
  - dp[2] = 2

i=3: bytes[1]='2', bytes[2]='6'
  - Single: '6' != '0' → dp[3] += dp[2] = 2
  - Two: 26 is valid → dp[3] += dp[1] = 1
  - dp[3] = 3

Result: 3 (BBF, BZ, VF) ✓
```

## Key Insights

1. **dp[i] depends on dp[i-1] and dp[i-2]**: Because we're adding one or two digits.
2. **'0' is invalid alone**: Only valid as part of "10" or "20".
3. **Leading zero check**: "01", "012" are invalid.

## Follow-up Answers

**Q: Why check bytes[i-1] != '0'?**
A: '0' has no single-digit mapping, but "10" and "20" are valid two-digit.

**Q: Why not allow "30", "40", etc.?**
A: Because only 10-26 have letter mappings.

**Q: Space optimization?**
A: Only need previous two values, so O(1) space possible.