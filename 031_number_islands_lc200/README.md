# Number of Islands - LeetCode 200

## Problem Statement

Given an `m x n` 2D binary grid which represents a map of '1's (land) and '0's (water), return the number of islands.

An island is surrounded by water and is formed by connecting adjacent lands horizontally or vertically. You may assume all four edges of the grid are surrounded by water.

## Visual Walkthrough

```
Example 1:
Input: grid = [
  ["1","1","1","1","0"],
  ["1","1","0","1","0"],
  ["1","1","0","0","0"],
  ["0","0","0","0","0"]
]
Output: 1

Example 2:
Input: grid = [
  ["1","1","0","0","0"],
  ["1","1","0","0","0"],
  ["0","0","1","0","0"],
  ["0","0","0","1","1"]
]
Output: 3
```

### Island Visualization

```
Grid 1:
1 1 1 1 0
1 1 0 1 0
1 1 0 0 0
0 0 0 0 0

All 1s are connected → 1 island

Grid 2:
1 1 0 0 0
1 1 0 0 0
0 0 1 0 0
0 0 0 1 1

Three separate groups → 3 islands

  Island 1     Island 2     Island 3
  [0,0]-[1,1]    [2,2]    [3,3]-[3,4]
```

### DFS Flood Fill Concept

```
Starting from a '1', mark all connected '1's as visited.
Then move to next unvisited '1'.

Step-by-step for Grid 2:

. . . . .    Initial: find first '1' at (0,0)
1 1 . . .
1 1 . . .    DFS from (0,0): mark (0,0), (0,1), (1,0), (1,1)
. . . . .    count = 1
. . . . .

. . . . .    Continue: find '1' at (2,2)
. . . . .    DFS: mark (2,2)
. . 1 . .    count = 2
. . . . .

. . . . .    Continue: find '1' at (3,3)
. . . . .    DFS: mark (3,3), (3,4)
. . . . .    count = 3
. . . 1 1

Result: 3 islands
```

## Approach Comparison

| Approach | Time | Space | Description |
|----------|------|-------|-------------|
| DFS | O(m×n) | O(m×n) | Flood fill from each island |
| BFS | O(m×n) | O(min(m,n)) | Queue-based flood fill |
| Union-Find | O(m×n) | O(m×n) | DSU to count distinct components |
| DFS (In-place) | O(m×n) | O(m×n) | Modify grid to mark visited |

### Why DFS Works

- Starting from any land cell, DFS/BFS visits all connected land
- Connection is only 4-directional (not diagonal)
- After visiting one island, mark all its cells as visited
- Next unvisited land must be a different island

## Implementation Strategy

### Approach 1: DFS Recursive

```rust
pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    if grid.is_empty() { return 0; }
    let (m, n) = (grid.len(), grid[0].len());
    let mut count = 0;

    fn dfs(grid: &[Vec<char>], i: usize, j: usize, m: usize, n: usize) {
        if i >= m || j >= n || grid[i][j] == '0' { return; }
        // Mark as visited
        // Recurse in 4 directions
    }

    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == '1' {
                count += 1;
                dfs(&grid, i, j, m, n);
            }
        }
    }
    count
}
```

### Approach 2: BFS with Queue

```rust
use std::collections::VecDeque;

pub fn num_islands_bfs(grid: Vec<Vec<char>>) -> i32 {
    // BFS from each unvisited land
}
```

## Edge Cases

1. **Empty grid**: Return 0
2. **All water**: Return 0
3. **All land**: Return 1 (single island)
4. **Single cell land**: Return 1
5. **Multiple islands of size 1**: Count each
6. **Large grid**: Handle efficiently

## Test Cases

1. Basic islands
2. No islands (all water)
3. All land (one island)
4. Single cell island
5. Multiple isolated cells
6. Large connected land mass
7. Diagonal cells (not connected)
8. Grid with various island sizes

## Solution Explanation

### Key Insight

Each '1' belongs to exactly one island. When we find an unvisited '1', we start DFS/BFS to mark all cells in that island as visited, then increment count.

### Marking Visited

We can either:
1. Use a separate visited array
2. Modify grid in-place ('1' → '0' after visiting)

Modifying in-place saves space but requires careful handling.

## Complexity Analysis

- **Time**: O(m×n) - each cell visited at most once
- **Space**: O(m×n) worst case for recursion stack or visited array

## Follow-up Questions

1. Can you solve with Union-Find?
2. How to handle 8-directional connectivity?
3. What if you need to find the largest island?