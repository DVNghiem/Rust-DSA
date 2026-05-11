# Word Search - LeetCode 79

## Problem Statement

Given an `m x n` grid of characters `board` and a string `word`, return `true` if `word` exists in the grid.

The word can be constructed from letters of sequentially adjacent cells, where adjacent cells are horizontally or vertically neighboring. The same cell may not be used more than once in the same word.

## Visual Walkthrough

```
Example 1:
Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCCED"
Output: true

Grid:
A B C E
S F C S
A D E E

Path for "ABCCED": A→B→C→C→E→D (backwards)
Sequential adjacent cells, used once each.

Example 2:
Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "SEE"
Output: true

Example 3:
Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCB"
Output: false

"ABCB" cannot be formed - would need to reuse 'B' at (0,0)
```

### Backtracking Visualization

```
Grid:         Word: "ABCCED"
A B C E       Path: (0,0)→(0,1)→(0,2)→(0,2)→(1,2)→(2,2) - wait
S F C S              can't use (0,2) twice

Let's trace:
(0,0) = 'A' ✓
Neighbors: (0,1)='B', (1,0)='S'
Pick (0,1) = 'B' ✓
Neighbors: (0,2)='C', (1,1)='F', (0,0) already used
Pick (0,2) = 'C' ✓
Neighbors: (0,3)='E', (1,2)='C', (0,1) used
Pick (1,2) = 'C' ✓
Neighbors: (0,2) used, (1,3)='S', (2,2)='E', (1,1)='F'
Pick (2,2) = 'E' ✓
Neighbors: (1,2) used, (2,3)='E', (3,2) out of bounds
Pick (2,3) = 'E' ✓

Word found! ✓
```

## Approach Comparison

| Approach | Time | Space | Description |
|----------|------|-------|-------------|
| Backtracking DFS | O(m×n×4^L) | O(L) | Try all paths, backtrack |
| Backtracking with pruning | O(m×n×3^L) | O(L) | Prune dead ends earlier |

### Why Backtracking?

- Word construction requires sequential path
- Multiple possible starting points
- Must track visited cells within one path (can't reuse)
- DFS naturally explores all paths

### Time Complexity Analysis

- Each character position can be visited multiple times (different paths)
- At each step, up to 4 choices (3 after first, since we came from one direction)
- L = word length

## Implementation Strategy

### Backtracking with DFS

```rust
pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let m = board.len();
    let n = board[0].len();

    fn dfs(board: &[Vec<char>], word: &[u8], i: usize, j: usize, idx: usize) -> bool {
        if idx == word.len() { return true; }

        let c = word[idx] as char;
        if board[i][j] != c { return false; }

        // Mark visited (use a sentinel value to avoid separate visited array)
        let original = board[i][j];
        // Can't actually modify board in this approach...

        // Actually need to mark visited and backtrack
    }
}
```

## Edge Cases

1. **Empty word**: Return true (empty string is substring of anything)
2. **Empty board**: Return false unless word is empty
3. **Single cell board**: Word must be single char matching that cell
4. **Word longer than board**: Impossible, return false
5. **All same letter**: Must respect path constraint (no reusing cell)
6. **Word not found**: Return false

## Test Cases

1. Basic word exists
2. Word not found
3. Empty board
4. Single cell match
5. Single cell no match
6. Word needs backtracking
7. Word longer than board
8. All same letter grid

## Solution Explanation

### Key Insight

Treat each cell as potential starting point. DFS explores paths matching word, marking visited cells to prevent reuse within same path. Backtrack when dead end reached.

### Backtracking Algorithm

1. For each cell, start DFS if it matches word[0]
2. At each step, check if current cell matches expected character
3. Mark cell as visited (cannot reuse)
4. Try all 4 neighbors that match next character
5. If dead end, unmark (backtrack) and try other paths
6. Return true if complete word matched

## Complexity Analysis

- **Time**: O(m×n×4^L) worst case where L = word length
- **Space**: O(L) for recursion stack and path tracking

## Follow-up Questions

1. How to optimize pruning?
2. Can you solve with BFS instead?
3. How to handle multiple words efficiently?