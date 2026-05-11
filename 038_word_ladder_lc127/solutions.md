# Word Ladder - Solution Analysis

## Problem Overview

Given beginWord, endWord, and wordList, find shortest transformation sequence where:
- Each step changes exactly one letter
- Each intermediate word must be in wordList
- Return 0 if no such sequence exists

## Solution 1: BFS Naive

### Code Implementation

```rust
pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
    let mut word_set: HashSet<String> = word_list.into_iter().collect();

    if !word_set.contains(&end_word) {
        return 0;
    }

    let mut queue = VecDeque::new();
    queue.push_back((begin_word, 1)); // (word, level)

    let mut visited = HashSet::new();
    visited.insert(begin_word.clone());

    while let Some((word, level)) = queue.pop_front() {
        if word == end_word {
            return level;
        }

        let chars: Vec<char> = word.chars().collect();

        for i in 0..chars.len() {
            let original_char = chars[i];

            for c in b'a'..=b'z' {
                let c = c as char;
                if c == original_char { continue; }

                let mut new_chars = chars.clone();
                new_chars[i] = c;
                let new_word: String = new_chars.into_iter().collect();

                if word_set.contains(&new_word) && !visited.contains(&new_word) {
                    visited.insert(new_word.clone());
                    queue.push_back((new_word, level + 1));
                }
            }
        }
    }

    0
}
```

### Line-by-Line Analysis

1. **`let mut word_set: HashSet<String> = word_list.into_iter().collect();`**: Convert wordList to HashSet for O(1) lookup.

2. **`if !word_set.contains(&end_word) { return 0; }`**: If endWord not in list, impossible.

3. **`queue.push_back((begin_word, 1));`**: Start BFS with beginWord at level 1.

4. **`visited.insert(begin_word.clone());`**: Mark beginWord as visited to avoid cycles.

5. **`if word == end_word { return level; }`**: If we reached endWord, return current level.

6. **`for c in b'a'..=b'z' { ... }`**: Try all 25 possible replacements for each position.

7. **`if word_set.contains(&new_word) && !visited.contains(&new_word) { ... }`**: If new word is valid and not visited, add to queue.

8. **`queue.push_back((new_word, level + 1));`**: Add to queue with incremented level.

### BFS Visualization

```
Begin: "hit", End: "cog"
Word list contains: hot, dot, dog, lot, log, cog

Queue initially: [("hit", 1)]

Process "hit" (level 1):
  Generate variants: xit, hit, hxt, hix, hi_ (wait, only one letter change)
  Actually try all positions:
    i=0: "ait", "bit", "cit", ... "zit"
    i=1: "hat", "hbt", ... "hzt"
    i=2: "hia", "hib", ... "hiz"
  Only "hot" matches and is in wordList
  Add ("hot", 2) to queue

Process "hot" (level 2):
  Variants lead to "dot" and "lot"
  Add ("dot", 3), ("lot", 3) to queue

Process "dot" (level 3):
  Leads to "dog"
  Add ("dog", 4) to queue

Process "dog" (level 4):
  Leads to "cog"
  Add ("cog", 5) to queue

Process "cog" (level 5):
  Matches endWord!
  Return 5 ✓
```

## Solution 2: Pattern-Based

Instead of generating 25 variants for each position, generate patterns like "_og" and find all matching words. More efficient when many words share patterns.

```rust
// Build pattern_map: pattern -> [words]
// For word "dog" with length 3:
// "_og" -> ["dog"]
// "d_g" -> ["dog"]
// "do_" -> ["dog"]

// When at word "dig", generate pattern "_ig":
// Find all words matching "_ig": ["dig", "pig", "big", ...]
// These are all valid neighbors
```

## Solution 3: Bidirectional BFS

Two BFS from both ends - meet in middle. Often faster because problem space is smaller.

```rust
// Maintain begin_set and end_set
// Expand smaller set each time
// If sets meet, found path
```

## Complexity Comparison

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Naive BFS | O(L × 25 × N) | O(N) | Simple |
| Pattern BFS | O(L × N) avg | O(N) | Good for dense patterns |
| Bidirectional | O(L × 25 × N/2) avg | O(N) | Faster when sets meet early |

Where L = word length, N = word list size

## Key Insights

1. **Word as node, one-letter-diff as edge**: Graph representation makes BFS natural.

2. **BFS finds shortest path**: Unweighted edges → level = number of transformations.

3. **Visited set prevents cycles**: Can't revisit words within same transformation sequence.

4. **Early termination**: Return immediately when endWord reached.

## Test Case Analysis

### Test: `test_no_path`

```
Begin: "hit", End: "cog"
WordList: hot, dot, dog, lot, log (NO "cog"!)

Queue: [("hit", 1)]
Process "hit" → "hot" is neighbor
Queue: [("hot", 2)]
Process "hot" → "dot", "lot" are neighbors
Queue: [("dot", 3), ("lot", 3)]
Process "dot" → "dog" is neighbor
Queue: [("lot", 3), ("dog", 4)]
Process "lot" → "log" is neighbor
Queue: [("dog", 4), ("log", 4)]
Process "dog" → no new neighbors in wordList
Process "log" → no new neighbors
Queue empty
Return 0 ✓
```

## Follow-up Answers

**Q: Why return level not edges?**
A: Level = number of words in sequence. "hit" → "hot" is 2 words (level 2).

**Q: Can we use Dijkstra?**
A: Not needed - all edges weight 1, BFS gives shortest path in unweighted graph.

**Q: When is bidirectional BFS faster?**
A: When beginWord and endWord are close to each other, sets meet early, halving search space.

**Q: Why 25 not 26 in loop?**
A: We skip the original character (c == original). Changing to same letter = no change, not a valid transformation.

**Q: Time complexity L × 25 × N?**
A: For each of L positions, try 25 letters, check N words in worst case. But word_set lookup is O(1), so total O(L × 25 × N).

**Q: Memory O(N)?**
A: word_set O(N), visited O(N), queue O(N) worst case. Total O(N).