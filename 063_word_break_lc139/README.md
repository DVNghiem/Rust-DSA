# Word Break - LeetCode 139

## Problem Overview

Given a string `s` and a dictionary of strings `wordDict`, return true if `s` can be segmented into a space-separated sequence of dictionary words.

**Examples:**
```
Input: s = "leetcode", wordDict = ["leet","code"]
Output: true

Input: s = "applepenapple", wordDict = ["apple","pen"]
Output: true

Input: s = "catsandog", wordDict = ["cats","dog","sand","and","cat"]
Output: false
```

## Theory

### Dynamic Programming Solution

`dp[i]` = true if `s[0..i]` can be segmented.

```
s = "leetcode", wordDict = ["leet", "code"]

dp[0] = true (empty string)

dp[1] = dp[0] && "l" in dict? = false
dp[2] = dp[1] && "le" in dict? = false
...
dp[4] = dp[0] && "leet" in dict? = true ✓
dp[5] = dp[4] && "e" in dict? = false
...
dp[8] = dp[4] && "code" in dict? = true ✓
```

## Implementation

```rust
pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let word_set: std::collections::HashSet<&str> = word_dict.iter().map(|w| w.as_str()).collect();
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut dp = vec![false; n + 1];
    dp[0] = true;

    for i in 1..=n {
        for j in 0..i {
            if dp[j] && word_set.contains(std::str::from_utf8(&bytes[j..i]).unwrap()) {
                dp[i] = true;
                break;
            }
        }
    }

    dp[n]
}
```

## Test Cases

```rust
#[test]
fn test_word_break_basic() {
    assert!(word_break("leetcode".to_string(), vec!["leet".to_string(), "code".to_string()]));
}
```