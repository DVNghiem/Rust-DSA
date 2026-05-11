# Word Search - Solution Analysis

## Problem Overview

Given an m×n grid of characters and a word, determine if the word can be formed by sequentially adjacent cells (horizontally or vertically). Each cell can only be used once per word.

## Solution 1: Backtracking DFS with Visited Array

### Code Implementation

```rust
pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    if board.is_empty() || board[0].is_empty() {
        return word.is_empty();
    }

    let m = board.len();
    let n = board[0].len();
    let bytes = word.as_bytes();

    if bytes.is_empty() { return true; }
    if bytes.len() > m * n { return false; }

    fn dfs(
        board: &[Vec<char>],
        bytes: &[u8],
        i: usize,
        j: usize,
        idx: usize,
        visited: &mut Vec<Vec<bool>>,
    ) -> bool {
        // Base case: matched all characters
        if idx == bytes.len() { return true; }

        // Bounds and character check
        if i >= board.len() || j >= board[0].len() { return false; }
        if visited[i][j] { return false; }
        if board[i][j] != bytes[idx] as char { return false; }

        // Mark as visited
        visited[i][j] = true;

        // Try all 4 directions
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        for (dr, dc) in directions.iter() {
            let ni = i as isize + dr;
            let nj = j as isize + dc;

            if ni >= 0 && ni < board.len() as isize && nj >= 0 && nj < board[0].len() as isize {
                if dfs(board, bytes, ni as usize, nj as usize, idx + 1, visited) {
                    return true;
                }
            }
        }

        // Backtrack: unmark
        visited[i][j] = false;

        false
    }

    let mut visited = vec![vec![false; n]; m];
    for i in 0..m {
        for j in 0..n {
            if board[i][j] == bytes[0] as char {
                visited[i][j] = true;
                if dfs(&board, bytes, i, j, 1, &mut visited) {
                    return true;
                }
                visited[i][j] = false;
            }
        }
    }

    false
}
```

### Line-by-Line Analysis

1. **`if board.is_empty() || board[0].is_empty() { return word.is_empty(); }`**: Handle empty grid.

2. **`let bytes = word.as_bytes();`**: Convert string to bytes for efficient indexing.

3. **`if bytes.is_empty() { return true; }`**: Empty word is always found.

4. **`if bytes.len() > m * n { return false; }`**: If word longer than cells, impossible.

5. **`fn dfs(...) -> bool`**: Inner recursive function with all parameters needed for backtracking.

6. **`if idx == bytes.len() { return true; }`**: All characters matched successfully.

7. **`if visited[i][j] { return false; }`**: Cell already used in current path.

8. **`if board[i][j] != bytes[idx] as char { return false; }`**: Character doesn't match.

9. **`visited[i][j] = true;`**: Mark current cell as used.

10. **`let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];`**: Four directions.

11. **Recursive calls**: Try each direction for remaining characters.

12. **`visited[i][j] = false;`**: Unmark after exploring all paths (backtrack).

### Backtracking Visualization

```
Grid: "ABCE", "SFCS", "ADEE"  Word: "ABCCED"

Start at (0,0)='A':
  visited = {(0,0)}

  At (0,0), try (0,1)='B':
    visited = {(0,0), (0,1)}

    At (0,1), try (0,2)='C':
      visited = {(0,0), (0,1), (0,2)}

      At (0,2), try (1,2)='C':
        visited = {(0,0), (0,1), (0,2), (1,2)}

        At (1,2), try (2,2)='E':
          visited = {(0,0), (0,1), (0,2), (1,2), (2,2)}

          At (2,2), try (2,1)='D':
            Match! word complete ✓

If first path fails, backtrack and try other directions.
```

## Solution 2: In-Place Modification (No Visited Array)

```rust
pub fn exist_inplace(board: Vec<Vec<char>>, word: String) -> bool {
    fn dfs(
        board: &mut Vec<Vec<char>>,
        bytes: &[u8],
        i: usize,
        j: usize,
        idx: usize,
    ) -> bool {
        if idx == bytes.len() { return true; }
        if board[i][j] != bytes[idx] as char { return false; }

        // Mark with sentinel
        let temp = board[i][j];
        board[i][j] = '#';

        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        for (dr, dc) in directions.iter() {
            let ni = i as isize + dr;
            let nj = j as isize + dc;
            if ni >= 0 && ni < board.len() as isize && nj >= 0 && nj < board[0].len() as isize {
                if dfs(board, bytes, ni as usize, nj as usize, idx + 1) {
                    return true;
                }
            }
        }

        board[i][j] = temp; // Restore
        false
    }
    // Same outer loop structure...
}
```

### In-Place Approach

- Instead of visited array, mark cell with sentinel character ('#')
- Restore original character when backtracking
- Avoids separate visited array, but requires cloning board for each start position

## Solution 3: Optimized with Early Pruning

```rust
fn can_form(board: &[Vec<char>], word: &[u8]) -> bool {
    // If any character in word appears more times in word than in board, impossible
    let mut board_count = [0i32; 256];
    let mut word_count = [0i32; 256];

    for row in board {
        for &c in row {
            board_count[c as usize] += 1;
        }
    }
    for &c in word {
        word_count[c as usize] += 1;
    }

    for i in 0..256 {
        if word_count[i] > board_count[i] {
            return false;
        }
    }
    true
}
```

### Why Early Pruning?

- If word contains more of character X than board contains of X, impossible
- Quick check before expensive DFS
- O(m×n) to count vs potentially O(m×n×4^L) for DFS

## Complexity Analysis

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Basic | O(m×n×4^L) | O(m×n) | Visited array |
| In-place | O(m×n×4^L) | O(L) | No visited array |
| Optimized | O(m×n×4^L) + O(m×n) | O(m×n) | Prunes early |

Where L = word length

## Key Insights

1. **Backtracking essential**: Multiple paths need exploration, one failure doesn't mean overall failure.

2. **Mark visited prevents reuse**: Within same path, can't reuse cells. But different paths can use same cell.

3. **4 directions, no diagonals**: Only horizontally/vertically adjacent.

4. **Time complexity is rough upper bound**: In practice, much lower due to pruning.

## Test Case Analysis

### Test: `test_abcb_not_exist`

```
Grid: "ABCE", "SFCS", "ADEE"
Word: "ABCB"

Trace:
(0,0)='A' → (0,1)='B' → (0,2)='C' → now need 'B'
Neighbors of (0,2): (0,3)='E', (1,2)='C', (0,1) visited, (-1,2) out
No 'B' found → backtrack

Try different path:
(0,0)='A' → (1,0)='S' → ... never finds B

All paths fail → return false ✓
```

### Test: `test_cannot_reuse_cell`

```
Grid: "AA" (2 cells)
Word: "AAA" (needs 3 A's)

Only 2 cells, can't use any cell twice, impossible ✓
```

## Follow-up Answers

**Q: Why use bytes instead of chars?**
A: bytes is more efficient for indexing and comparison.

**Q: Can we solve with BFS?**
A: BFS doesn't naturally support backtracking needed here.

**Q: How to handle multiple words efficiently?**
A: Use Trie to share prefixes across words.

**Q: Space optimization?**
A: Use in-place modification instead of visited array.

**Q: Time complexity 4^L?**
A: Each character has up to 4 choices, but this decreases as we hit walls/boundaries. Actual is usually much lower.

**Q: Can we use path length tracking instead of visited?**
A: Only if we ensure path doesn't cross itself, which is harder to verify without visited array.