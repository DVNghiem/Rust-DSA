# Word Break II - LeetCode 140

## Problem Statement

Given a string s and a dictionary of strings wordDict, add spaces in s to construct a sentence where each word exists in the dictionary.

```
Example:
Input: s = "catsanddog", wordDict = ["cat", "cats", "and", "sand", "dog"]
Output: ["cats and dog", "cat sand dog"]
```

## Understanding the Problem

We need to segment the string into words from the dictionary and return all possible sentences.

```
s = "catsanddog"
        ↓
Possible segmentations:
1. "cats" + "and" + "dog" → "cats and dog"
2. "cat" + "sand" + "dog" → "cat sand dog"
```

## Visual Walkthrough

```
s = "catsanddog"
wordDict = {cat, cats, and, sand, dog}

DP approach:

dp[0] = [""] (empty string, valid)

dp[1] = "" (t, no word matches)
dp[2] = "" (ca, no word matches)
dp[3] = "" (cat, matches "cat" → add "cat " to dp[0])
dp[4] = "" (cats, matches "cats" → add "cats " to dp[0])
         matches "cats" from dp[3]? No, need check...
         
Actually we build from left to right.

For each position i, we check all substrings s[j..i] that are in dictionary.
For each match, we combine all sentences from dp[j] with the new word.
```

## Four Approaches

| Approach | Time | Space | Description |
|----------|------|-------|-------------|
| DFS + Memoization | O(n² + output) | O(n²) | Top-down with cache |
| DP + Backtracking | O(n² + output) | O(n²) | Bottom-up DP |
| Trie + DP | O(n² + dict size) | O(n²) | Trie-based word matching |
| BFS | O(n² + output) | O(n²) | Breadth-first traversal |

## DFS + Memoization Approach

```rust
fn word_break(s: &str, word_dict: &HashSet<String>) -> Vec<String> {
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut memo = vec![Option::<Vec<String>>::None; n + 1];
    dfs(0, bytes, word_dict, &mut memo)
}

fn dfs(start: usize, bytes: &[u8], dict: &HashSet<String>, memo: &mut Vec<Option<Vec<String>>>) -> Vec<String> {
    if start == bytes.len() {
        return vec![String::new()];
    }

    if let Some(result) = &memo[start] {
        return result.clone();
    }

    let mut results = Vec::new();
    let mut end = start + 1;

    while end <= bytes.len() {
        let word = &bytes[start..end];
        if dict.contains(word) {
            let suffix_sentences = dfs(end, bytes, dict, memo);
            for mut sentence in suffix_sentences {
                if sentence.is_empty() {
                    results.push(String::from_utf8(word.to_vec()).unwrap());
                } else {
                    results.push(format!("{} {}", String::from_utf8(word.to_vec()).unwrap(), sentence));
                }
            }
        }
        end += 1;
    }

    memo[start] = Some(results.clone());
    results
}
```

## Visual Trace

```
s = "catsanddog"
wordDict = {cat, cats, and, sand, dog}

dfs(0):
  Check substrings starting at 0:
  - "c": not in dict
  - "ca": not in dict
  - "cat": in dict!
    dfs(3) -> "and dog", "sand dog"
    Add "cat " prefix -> ["cat and dog", "cat sand dog"]
  - "cats": in dict!
    dfs(4) -> "and dog"
    Add "cats " prefix -> ["cats and dog"]
  - "catsa": not in dict
  ...

Merge both results:
["cat and dog", "cat sand dog", "cats and dog"]
```

## DP Bottom-Up Approach

```rust
pub fn word_break_dp(s: String, word_dict: HashSet<String>) -> Vec<String> {
    let bytes = s.as_bytes();
    let n = bytes.len();

    // dp[i] = all sentences for s[0..i]
    let mut dp: Vec<Vec<String>> = vec![Vec::new(); n + 1];
    dp[0] = vec![String::new()];

    for i in 1..=n {
        let mut sentences = Vec::new();
        for j in 0..i {
            let word = &bytes[j..i];
            if word_dict.contains(word) {
                for prefix in &dp[j] {
                    if prefix.is_empty() {
                        sentences.push(String::from_utf8(word.to_vec()).unwrap());
                    } else {
                        sentences.push(format!("{} {}", prefix, String::from_utf8(word.to_vec()).unwrap()));
                    }
                }
            }
        }
        dp[i] = sentences;
    }

    dp[n].clone()
}
```

## Complexity Analysis

| Approach | Time | Space |
|----------|------|-------|
| DFS + Memo | O(n² × output) | O(n²) |
| DP Bottom-up | O(n² × output) | O(n²) |

Where n = length of string, output = total characters in all sentences.

## Edge Cases

### Empty String

```rust
s = ""
dp[0] = [""]  // one empty sentence
Result: [""]  // one sentence (empty string)
```

### No Valid Segmentation

```rust
s = "applepen"
wordDict = {"apple", "pen"}
dp[8] will be empty, return []
```

### All Same Word

```rust
s = "aaaaaaaa"
wordDict = {"a", "aa", "aaa"}
Many possible combinations...
```

## Test Cases Design

### Basic Cases
1. Simple two-word sentence
2. Multiple valid sentences
3. Single word match

### Edge Cases
4. Empty string
5. No valid segmentation (return empty list)
6. One character words

### Complex Cases
7. Many possible segmentations
8. Long string with many dictionary words
9. Overlapping words in dictionary

## Related Problems

1. **LeetCode 140**: Word Break II (this problem)
2. **LeetCode 139**: Word Break (similar but only returns if possible)
3. **LeetCode 127**: Word Ladder

## Time to Complete

**Target**: 45 minutes
**Optimal**: 30 minutes with memoization