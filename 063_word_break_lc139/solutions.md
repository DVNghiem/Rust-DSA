# Word Break Solution - LeetCode 139 (Complete)

## Solution Analysis

### Dynamic Programming

```rust
pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let word_set: HashSet<&str> = word_dict.iter().map(|w| w.as_str()).collect();
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut dp = vec![false; n + 1];
    dp[0] = true;

    for i in 1..=n {
        for j in 0..i {
            if dp[j] && word_set.contains(std::str::from_utf8(&bytes[j..i]).unwrap_or("")) {
                dp[i] = true;
                break;
            }
        }
    }

    dp[n]
}
```

## DP Definition

`dp[i]` = true if `s[0..i]` can be segmented into valid words

### Base Case
```rust
dp[0] = true;  // Empty string is always breakable
```

### Transition

For each position i (1 to n):
- Check all possible break points j (0 to i-1)
- If `dp[j]` is true AND `s[j..i]` is in dictionary:
  - Then `dp[i]` is true

## Visual Example

### Input: s = "leetcode", wordDict = ["leet", "code"]

```
dp[0] = true (empty string)

dp[1] = check j=0: dp[0]=true && "l" in dict? false
dp[2] = check j=0: dp[0]=true && "le" in dict? false
        check j=1: dp[1]=false
dp[3] = check j=0: dp[0]=true && "lee" in dict? false
        check j=1: dp[1]=false
        check j=2: dp[2]=false
dp[4] = check j=0: dp[0]=true && "leet" in dict? TRUE! → dp[4]=true

dp[5] = check j=0: dp[0]=true && "leetc" in dict? false
        ...
        check j=4: dp[4]=true && "o" in dict? false
dp[6] = check j=0: dp[0]=true && "leetcod" in dict? false
        ...
        check j=4: dp[4]=true && "od" in dict? false
dp[7] = check j=0: dp[0]=true && "leetcode" in dict? false
        ...
        check j=4: dp[4]=true && "ode" in dict? false
dp[8] = check j=0: dp[0]=true && "leetcode" in dict? TRUE! → dp[8]=true

Return dp[8] = true
```

## Why Break When Found?

```rust
if dp[i] {
    break;  // Don't need to check other j values
}
```

Once we find any valid break point, the substring can be segmented. No need to continue checking.

## Complexity Analysis

| Approach | Time | Space |
|----------|------|-------|
| DP | O(n² × L) | O(n) |
| BFS + Memo | O(n²) | O(n) |

Where n = length of string, L = max word length

## Edge Cases

### Empty String
```rust
Input: s = "", wordDict = ["a"]
Output: true (dp[0]=true)
```

### Empty Dictionary
```rust
Input: s = "a", wordDict = []
Output: false
```

### Single Character
```rust
Input: s = "a", wordDict = ["a"]
Output: true
```

### Word Not in Dictionary
```rust
Input: s = "abc", wordDict = ["ab", "cd"]
Output: false
```

## Why HashSet for Dictionary?

```rust
let word_set: HashSet<&str> = word_dict.iter().map(|w| w.as_str()).collect();
```

HashSet provides O(1) lookup vs O(n) for Vec.
- Checking `s[j..i]` in dictionary needs to be fast
- Dictionary lookup happens O(n²) times

## Common Mistakes

1. **Confusing dp index**: dp[i] corresponds to s[0..i], not including i
2. **Not checking dp[j] first**: Without this, we'd miss the break point
3. **Using string slicing incorrectly**: Need to convert bytes to string for lookup

## Rust-Specific Patterns

1. **`as_bytes()`**: Get string as byte slice for efficient slicing
2. **`from_utf8()`**: Convert bytes back to string for dictionary lookup
3. **`HashSet::collect()`**: Create set from iterator
4. **`unwrap_or("")`**: Handle potential UTF-8 errors gracefully