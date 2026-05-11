# Solutions - Regular Expression Matching (LeetCode 10)

## Solution 1: 2D Dynamic Programming

```rust
pub fn is_match(text: String, pattern: String) -> bool {
    let m = text.len();
    let n = pattern.len();
    let text = text.as_bytes();
    let pattern = pattern.as_bytes();

    let mut dp = vec![vec![false; n + 1]; m + 1];
    dp[0][0] = true;

    // Handle patterns like a*, a*b*, a*b*c* that can match empty string
    for j in 2..=n {
        if pattern[j - 1] == b'*' {
            dp[0][j] = dp[0][j - 2];
        }
    }

    for i in 1..=m {
        for j in 1..=n {
            let curr_p = pattern[j - 1];
            let curr_t = text[i - 1];

            if curr_p == b'*' {
                dp[i][j] = dp[i][j - 2];

                if pattern[j - 2] == b'.' || pattern[j - 2] == curr_t {
                    dp[i][j] = dp[i][j] || dp[i - 1][j];
                }
            } else if curr_p == b'.' || curr_p == curr_t {
                dp[i][j] = dp[i - 1][j - 1];
            }
        }
    }

    dp[m][n]
}
```

### Line-by-Line Analysis

**Lines 14-18: Setup**
```rust
let m = text.len();
let n = pattern.len();
let text = text.as_bytes();
let pattern = pattern.as_bytes();
```
Convert to bytes for efficient indexing. `m` = text length, `n` = pattern length.

**Line 20: DP Table Initialization**
```rust
let mut dp = vec![vec![false; n + 1]; m + 1];
```
Create (m+1) × (n+1) table. Extra row/column handles empty strings.

**Line 21: Base Case**
```rust
dp[0][0] = true;
```
Empty pattern matches empty text.

**Lines 24-27: Handle Empty Text with Star Patterns**
```rust
for j in 2..=n {
    if pattern[j - 1] == b'*' {
        dp[0][j] = dp[0][j - 2];
    }
}
```
Patterns like `a*`, `a*b*`, `.*` can match empty text. The `j-2` skips the `x*` pair when treating it as zero occurrences.

**Lines 29-44: Fill DP Table**
```rust
for i in 1..=m {
    for j in 1..=n {
        let curr_p = pattern[j - 1];
        let curr_t = text[i - 1];

        if curr_p == b'*' {
            // Two cases handled below
        } else if curr_p == b'.' || curr_p == curr_t {
            dp[i][j] = dp[i - 1][j - 1];
        }
    }
}
```
Two main cases:
- `*` operator (lines 32-39)
- Direct match with `.` or exact character (lines 40-42)

**Lines 33-35: Star Case - Zero Occurrences**
```rust
dp[i][j] = dp[i][j - 2];
```
If we use `*` as zero occurrences of the preceding element, the match status is the same as if the `x*` pair wasn't there.

**Lines 36-39: Star Case - One+ Occurrences**
```rust
if pattern[j - 2] == b'.' || pattern[j - 2] == curr_t {
    dp[i][j] = dp[i][j] || dp[i - 1][j];
}
```
If the preceding element (`pattern[j-2]`) matches the current text character (`curr_t`), we can also match by consuming this text character and keeping the `*` pattern (recursively check `dp[i-1][j]`).

### Complexity Analysis

| Aspect | Complexity |
|--------|------------|
| Time | O(m × n) - fill entire DP table |
| Space | O(m × n) - DP table |

### Visual Example

```
Pattern: "a*"
Text: "aaa"

DP Table Construction:
        ""  a   *   (pattern positions: 0, 1, 2)
    +---+---+---+---+
    |"" | T | F | T |   dp[0][2] = dp[0][0] = T (a* matches empty)
    +---+---+---+---+
    | a | F | T | T |   dp[1][2] = dp[1][0] = T (a* matches "a")
    +---+---+---+---+
    |aa | F | T | T |   dp[2][2] = dp[2][0] = T (a* matches "aa")
    +---+---+---+---+
    |aaa| F | T | T |   dp[3][2] = dp[3][0] = T (a* matches "aaa")
    +---+---+---+---+

Row 1, Col 1: pattern[0]='a', text[0]='a' match -> dp[1][1] = dp[0][0] = T
Row 1, Col 2: pattern[1]='*', use as zero -> dp[1][2] = dp[1][0] = F
                    use as one+: pattern[0]='a' matches text[0]='a'
                    -> dp[1][2] = F || dp[0][2] = F || T = T
```

## Solution 2: Memoized Recursion

```rust
fn dp_recursive(
    i: usize,
    j: usize,
    text: &[u8],
    pattern: &[u8],
    memo: &mut Vec<Vec<Option<bool>>>,
) -> bool {
    if j >= pattern.len() {
        return i == text.len();
    }

    if let Some(result) = memo[i][j] {
        return result;
    }

    let mut result: bool;

    if j + 1 < pattern.len() && pattern[j + 1] == b'*' {
        result = dp_recursive(i, j + 2, text, pattern, memo);

        if pattern[j] == b'.' || pattern[j] == text[i] {
            result = result || dp_recursive(i + 1, j, text, pattern, memo);
        }
    } else {
        result = (pattern[j] == b'.' || pattern[j] == text[i])
            && dp_recursive(i + 1, j + 1, text, pattern, memo);
    }

    memo[i][j] = Some(result);
    result
}
```

### Line-by-Line Analysis

**Lines 66-68: Base Case**
```rust
if j >= pattern.len() {
    return i == text.len();
}
```
If we've consumed the entire pattern, we're done only if we've also consumed all text.

**Lines 70-72: Memoization Check**
```rust
if let Some(result) = memo[i][j] {
    return result;
}
```
Return cached result if we've computed this subproblem before.

**Lines 74-82: Star Pattern Handling**
```rust
if j + 1 < pattern.len() && pattern[j + 1] == b'*' {
    result = dp_recursive(i, j + 2, text, pattern, memo);

    if pattern[j] == b'.' || pattern[j] == text[i] {
        result = result || dp_recursive(i + 1, j, text, pattern, memo);
    }
}
```
Two choices when we see `x*`:
1. Use zero occurrences: skip past `x*` entirely (j + 2)
2. Use one+ occurrences: if `x` matches current text, consume text and keep `x*` (i + 1, j stays)

**Lines 83-86: Non-Star Pattern**
```rust
else {
    result = (pattern[j] == b'.' || pattern[j] == text[i])
        && dp_recursive(i + 1, j + 1, text, pattern, memo);
}
```
For non-star patterns, must match current character exactly and recurse to next position.

## Test Cases Verified

1. **Exact match**: `is_match("aa", "aa")` = true
2. **Dot wildcard**: `is_match("ab", "..")` = true
3. **Star zero**: `is_match("", "a*")` = true
4. **Star one+**: `is_match("aaa", "a*")` = true
5. **Dot-star**: `is_match("aab", "a.*")` = true
6. **Complex**: `is_match("mississippi", "mis*is*ip*.")` = true
7. **Recursive vs Iterative**: Both approaches produce identical results
