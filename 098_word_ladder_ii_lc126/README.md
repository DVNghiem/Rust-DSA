# Word Ladder II - LeetCode 126

## Problem

Given a begin word, end word, and a dictionary of words, find all shortest transformation sequences from begin to end where:
- Each transformation changes exactly one character
- Each intermediate word must exist in the dictionary
- Only words of the same length are considered

```
Example:
beginWord = "hit"
endWord = "cog"
dictionary = ["hot","dot","dog","lot","log","cog"]

Result: [["hit","hot","dot","dog","cog"], ["hit","hot","lot","log","cog"]]
Transformation length is 5 (hit->hot->dot->dog->cog)
```

## Algorithm Overview

### Key Insight: BFS for Shortest Path

BFS naturally finds shortest paths in unweighted graphs. For word ladder:
- Each word is a node
- Edges connect words that differ by exactly one character
- BFS from begin word finds shortest path to end word

### Challenge: Reconstruct All Shortest Paths

Standard BFS stops when we find the target. But we need ALL shortest paths.
Solution: Record ALL parents for nodes at each level, then reconstruct paths via backtracking.

### Algorithm Steps

1. **Build word graph efficiently**: For each position in each word, create generalized pattern like `h_t` for `hit`. Words sharing a pattern are connected.

2. **BFS from begin word**: 
   - Process level by level
   - For each word, find all valid neighbors via patterns
   - Track visited words and their parent(s)

3. **Build result by backtracking**: From end word, recursively trace parents back to begin word.

### Visual Walkthrough

```
beginWord = "hit", endWord = "cog"
Dictionary: ["hot","dot","dog","lot","log","cog"]

Build patterns:
  hit -> *it, h*t, hi*
  hot -> *ot, h*t, ho*

Connection via patterns:
  hot connects to: dot (*ot), lot (*ot), not hit (h*t), cog (ho*)

BFS Levels:
  Level 0: hit
  Level 1: hot (one change from hit)
  Level 2: dot, lot (one change from hot)
  Level 3: dog, log (one change from dot/lot)
  Level 4: cog (one change from dog/log) - FOUND!

Parents recorded:
  dot: hot
  lot: hot
  dog: dot
  log: lot
  cog: dog, log

Reconstruction:
  cog <- dog <- dot <- hot <- hit
  cog <- log <- lot <- hot <- hit
```

## Topics Covered
- BFS graph traversal
- Word transformation graphs
- Path reconstruction
- Backtracking
- Pattern matching for graph building

## Approaches

### Approach 1: BFS + Backtracking (Recommended)

Time: O(N × L² × 26) | Space: O(N × L)
- Build generalized patterns to find neighbors efficiently
- BFS to find shortest path length and all nodes at each level
- Backtrack from end to begin to reconstruct all paths

### Approach 2: Standard BFS with Parent Tracking

Time: O(N × L² × 26) | Space: O(N × L)
- More straightforward but similar approach
- Track parent of each node during BFS
- Reconstruct by following parents

### Approach 3: Bidirectional BFS

Time: O(N × L² × 26) | Space: O(N × L)
- Start BFS from both begin and end
- Meet in the middle for faster search on large graphs
- More complex path reconstruction

## Complexity Analysis

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| BFS + Backtrack | O(N × L² × 26) | O(N × L) | Most practical |
| Standard BFS | O(N × L² × 26) | O(N × L) | Simple but slower |
| Bidirectional | O(N × L² × 26) | O(N × L) | Best for large N |

N = number of words in dictionary, L = word length

## Additional Notes

- The BFS approach guarantees shortest path length
- Recording ALL parents at each level enables finding ALL shortest paths
- Pattern-based neighbor finding avoids checking all N words for each word
- Space optimized by not storing full graph, just patterns
