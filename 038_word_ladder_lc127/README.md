# Word Ladder - LeetCode 127

## Problem Statement

Given two words `beginWord` and `endWord`, and a dictionary `wordList`, return the length of the shortest transformation sequence from `beginWord` to `endWord`.

Only one letter can be changed at a time.
Each transformed word must exist in the wordList.
Return 0 if no such transformation sequence exists.

## Visual Walkthrough

```
Example 1:
Input: beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log","cog"]
Output: 5

Transformation: "hit" → "hot" → "dot" → "dog" → "cog"
Length = 5 (5 words in sequence)

Example 2:
Input: beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log"]
Output: 0

No path exists from "hit" to "cog" since "cog" not in wordList.
```

### Graph Model

```
Each word is a node.
Edge between words if they differ by exactly one letter.

"hit" --differs by 1--> "hot" --differs by 1--> "dot" --differs by 1--> "dog" --differs by 1--> "cog"

BFS from "hit" to find shortest path to "cog".
```

### BFS Visualization

```
Start: "hit"
Level 0: ["hit"]

Neighbors of "hit" (diff by 1 letter): "hot"
Level 1: ["hot"]

Neighbors of "hot" (in wordList, not visited): "dot", "lot"
Level 2: ["dot", "lot"]

Neighbors of "dot": "dog" (from "dot")
Level 3: ["dog"]

Neighbors of "dog": "cog" (found!)
Level 4: ["cog"] → Return path length = 5
```

## Approach Comparison

| Approach | Time | Space | Description |
|----------|------|-------|-------------|
| BFS with word list | O(L × 26 × N) | O(N) | Standard BFS |
| BFS with neighbors optimization | O(L × 26 × N) | O(N) | Generate patterns |
| Bidirectional BFS | O(L × 26 × N) | O(N) | Start from both ends |

### Why BFS?

- Finding shortest path in unweighted graph
- All edges have weight 1
- BFS guarantees shortest path in terms of number of edges

## Implementation Strategy

### BFS with Pattern Generation

```rust
use std::collections::{HashSet, VecDeque};

pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
    let word_set: HashSet<String> = word_list.into_iter().collect();

    if !word_set.contains(&end_word) { return 0; }

    let mut queue = VecDeque::new();
    queue.push_back((begin_word, 1)); // (word, level)

    let mut visited = HashSet::new();
    visited.insert(begin_word);

    while let Some((word, level)) = queue.pop_front() {
        if word == end_word { return level; }

        // Generate all one-letter variations
        let chars: Vec<char> = word.chars().collect();
        for i in 0..chars.len() {
            let original = chars[i];
            for c in 'a'..='z' {
                if c == original { continue; }
                chars[i] = c;
                let new_word: String = chars.iter().collect();

                if word_set.contains(&new_word) && !visited.contains(&new_word) {
                    visited.insert(new_word.clone());
                    queue.push_back((new_word, level + 1));
                }
            }
            chars[i] = original;
        }
    }

    0
}
```

## Edge Cases

1. **endWord not in wordList**: Return 0
2. **beginWord == endWord**: Return 1 (or 0 depending on definition)
3. **Empty wordList**: Return 0
4. **Single character words**: Handle correctly
5. **No path exists**: Return 0

## Test Cases

1. Basic path exists
2. No path (end not in list)
3. Direct transformation (diff by 1 letter)
4. Begin word in list
5. Single letter words
6. Long words
7. Duplicate words in list

## Solution Explanation

### Key Insight

Treat each word as node. Create edges between words that differ by exactly one letter. BFS from beginWord finds shortest path.

### BFS Algorithm

1. Add beginWord to queue with level 1
2. Mark beginWord as visited
3. While queue not empty:
   - Dequeue word
   - Generate all one-letter variations
   - If variation is endWord, return level
   - If variation is in wordList and not visited, add to queue
4. Return 0 if no path found

## Complexity Analysis

- **Time**: O(L × 26 × N) where L = word length, N = word list size
- **Space**: O(N) for queue, visited set, word set

## Follow-up Questions

1. How to optimize neighbor generation?
2. What is bidirectional BFS and when to use it?
3. How to return actual transformation sequence?