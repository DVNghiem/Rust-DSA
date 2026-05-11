# Regular Expression Matching - LeetCode 10

## Problem

Implement regular expression matching with support for:
- `.` - Matches any single character
- `*` - Matches zero or more of the preceding element

```
Example:
Pattern: "aa"
Text: "aab"
Result: false (aa does not match aab)

Pattern: "a*"
Text: "aaa"
Result: true (a* matches zero or more 'a's)

Pattern: ".*"
Text: "aab"
Result: true (.* matches any sequence)

Pattern: "ab.*"
Text: "aabb"
Result: true
```

## Dynamic Programming Approach

### Key Insight: 2D DP Table

Define `dp[i][j]` = whether pattern `p[0..j]` matches text `t[0..i]`

Two main cases:
1. **No `*` in pattern**: `dp[i][j] = dp[i-1][j-1]` if characters match or pattern is `.`
2. **`*` in pattern**: More complex, depends on whether we use `*` as zero or more matches

### Visual Walkthrough

```
Pattern: "a*"
Text: "aaa"

DP Table:
        ""  a   a   a
    +---+---+---+---+---+
    |"" | T | F | F | F |   <- empty pattern
    +---+---+---+---+---+
    | a | T | T | T | T |   <- a* matches empty or more 'a's
    +---+---+---+---+---+

For "a*" matching "aaa":
- dp[0][1] = true (a* matches empty string)
- dp[1][1] = true (a* matches "a")
- dp[2][1] = true (a* matches "aa")
- dp[3][1] = true (a* matches "aaa")
```

```
Pattern: ".*"
Text: "aab"

DP Table (partial):
        ""  .   *
    +---+---+---+---+
    |"" | T | F | T |   <- .* matches empty
    +---+---+---+---+
    | a | T | T | T |
    +---+---+---+---+
```

## Topics Covered
- Dynamic programming (2D DP)
- String matching
- Pattern matching with wildcards
- State transitions
- Bottom-up tabulation

## Approaches

### Approach 1: 2D Dynamic Programming (Recommended)

Time: O(m × n) | Space: O(m × n)
- Build DP table bottom-up
- Handle `.` and `*` explicitly
- Most efficient for practical inputs

### Approach 2: Recursive with Memoization

Time: O(m × n) | Space: O(m × n)
- Top-down recursion
- Cache results for overlapping subproblems
- Same complexity, different implementation

### Approach 3: Space-Optimized DP

Time: O(m × n) | Space: O(min(m, n))
- Optimize to 1D array if pattern fits in memory
- More complex due to `*` handling

## Complexity Analysis

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| 2D DP | O(m × n) | O(m × n) | Most readable |
| Memoized Recursion | O(m × n) | O(m × n) | Elegant, recursive |
| Space-Optimized | O(m × n) | O(n) | Best memory |

m = length of text, n = length of pattern

## Additional Notes

- The `*` operator always follows a character (either literal or `.`)
- Empty pattern matches empty text
- When in doubt, `.` matches any single character including nothing... wait, no - `.` matches exactly one character
- `*` modifies the preceding element: `a*` means zero or more 'a's, `.*` means zero or more of anything
