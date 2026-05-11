# Word Search II - LeetCode 212

## Problem Overview

Given an `m x n` board of characters and a list of strings `words`, return all words on the board that can be formed by consecutive cells. Each cell can only be used once per word.

**Examples:**
```
Input:
board = [
  ['o','a','a','n'],
  ['e','t','a','e'],
  ['i','h','k','r'],
  ['i','f','l','v']
]
words = ["oath","pea","eat","rain"]

Output: ["eat","oath"]
```

## Theory

### Key Insight: Trie + Backtracking

The naive approach would be to run DFS for each word - O(m × n × word_length × num_words).

**Optimized approach**: Build a Trie from all words, then do a single DFS on the board. While traversing, we can check if we're on a valid Trie path.

### Trie Structure

```
       root
      / | \
     o  e  i
     |  |  |
     a  a  h
     |  |  |
     t  t  f
     |  |
     h  h
     |
     e
```

### Board Traversal

```
Starting from 'o' at (0,0):
- Path: (0,0) → (0,1) → (0,2) → (0,3) [no valid word]
- Path: (0,0) → (1,0) → ... [valid: "oath"]
```

## Implementation

### Data Structures

```rust
struct TrieNode {
    children: HashMap<char, TrieNode>,
    word: Option<String>,
}
```

### Algorithm

1. Build Trie from all words
2. For each cell, run DFS if cell char exists in Trie root
3. Explore 4 directions, mark visited cells
4. When Trie node has word, add to result and mark as visited

## Complexity Analysis

| Operation | Time | Space |
|-----------|------|-------|
| Build Trie | O(total_characters) | O(total_characters) |
| DFS per cell | O(m × n × 4^k) | O(k) stack |
| Total | O(m × n + total_characters) | O(total_characters) |

## Edge Cases

1. **Empty board**: Return []
2. **Empty words**: Return []
3. **No valid words**: Return []
4. **Duplicate words in result**: Remove duplicates

## Test Cases

```rust
#[test]
fn test_word_search_basic() {
    let board = vec![
        vec!['a', 'b'],
        vec!['d', 'c']
    ];
    assert_eq!(find_words(board, vec!["ac".to_string()]), vec!["ac"]);
}
```

## Follow-up Problems

- [LeetCode 79 - Word Search](https://leetcode.com/problems/word-search/) - Single word version