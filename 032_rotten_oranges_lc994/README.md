# Rotten Oranges - LeetCode 994

## Problem Statement

You are given an `m x n` grid where each cell can have one of three values:
- `0` representing an empty cell,
- `1` representing a fresh orange,
- `2` representing a rotten orange.

Every minute, any fresh orange that is 4-directionally adjacent to a rotten orange becomes rotten.

Return the minimum number of minutes that must elapse until no cell has a fresh orange. If this is impossible, return -1.

## Visual Walkthrough

```
Example 1:
Input: grid = [[2,1,1],[1,1,0],[0,1,1]]
Output: 4

Grid visualization:
Minute 0:     Minute 1:     Minute 2:     Minute 3:     Minute 4:
2 1 1         2 2 1         2 2 2         2 2 2         2 2 2
1 1 0    →    2 1 0    →    2 2 0    →    2 2 0    →    2 2 0
0 1 1         0 2 1         0 2 2         0 2 2         0 2 2

Example 2:
Input: grid = [[2,1,1],[0,1,1],[1,0,1]]
Output: -1

The bottom-left corner (1,0) fresh orange can never become rotten.

Example 3:
Input: grid = [[0,2]]
Output: 0

No fresh oranges, return 0 immediately.
```

### Multi-Source BFS Concept

```
Initial rotten oranges are sources.
BFS spreads rot level by level.
Each minute = one BFS level.

This is essentially "shortest path from multiple sources to all fresh oranges"
where each step is one minute.

If any fresh orange is not reachable, return -1.
```

### BFS Level Visualization

```
Initial:     Sources = {(0,0), (2,2)} (rotten oranges)

Level 0:     Rotting from sources
[2, 1]       →    [2, 2]
[1, 1]             [2, 1]

Level 1:     From newly rotten
[2, 2]       →    [2, 2]
[2, 1]             [2, 2]

Level 2:     All reachable fresh now rotten
[2, 2]       →    [2, 2]
[2, 2]             [2, 2]

Result: 2 minutes
```

## Approach Comparison

| Approach | Time | Space | Description |
|----------|------|-------|-------------|
| Multi-source BFS | O(m×n) | O(m×n) | Start from all rotten at once |
| Sequential simulation | O(m×n × t) | O(m×n) | Simulate minute by minute |

### Why Multi-Source BFS?

- Multiple starting points (all initially rotten)
- BFS naturally handles "spread from multiple sources"
- Level by level gives us the minute count
- Track how many fresh oranges rot each minute

## Implementation Strategy

### Multi-Source BFS

```rust
use std::collections::VecDeque;

pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut grid = grid;
    let mut queue = VecDeque::new();
    let mut fresh = 0;

    // Initialize: find all rotten oranges, count fresh
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 2 {
                queue.push_back((i, j));
            } else if grid[i][j] == 1 {
                fresh += 1;
            }
        }
    }

    if fresh == 0 { return 0; }

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut minutes = 0;

    while !queue.is_empty() && fresh > 0 {
        let level_size = queue.len();
        for _ in 0..level_size {
            // Pop and spread to neighbors
        }
        minutes += 1;
    }

    if fresh == 0 { minutes } else { -1 }
}
```

## Edge Cases

1. **No fresh oranges**: Return 0
2. **No rotten oranges**: Cannot rot anything, return -1
3. **All rotten initially**: Return 0
4. **Isolated fresh orange**: Cannot rot, return -1
5. **Empty grid**: Return 0
6. **Grid with only empty (0)**: Return 0

## Test Cases

1. Basic rotting (Example 1)
2. Impossible case (Example 2)
3. No fresh oranges
4. All rotten
5. Single fresh orange
6. Single rotten orange
7. Isolated fresh
8. Multiple separated clusters

## Solution Explanation

### Key Insight

Treat all initially rotten oranges as BFS sources. Each "level" of BFS represents one minute of time. Track fresh count - when it reaches 0, we've completed the process.

### Algorithm Steps

1. **Initialize queue** with all rotten oranges
2. **Count fresh** oranges
3. **BFS loop** until queue empty or no fresh left:
   - Process one level (one minute)
   - Each rottenorange rots its adjacent fresh oranges
   - Newly rotten oranges added to queue for next minute
4. **Return minutes** if all fresh rotted, else -1

## Complexity Analysis

- **Time**: O(m×n) - each cell processed at most once
- **Space**: O(m×n) - queue can hold all cells in worst case

## Follow-up Questions

1. Can you solve without counting fresh separately?
2. How to track which oranges rot at which minute?
3. What if you need to know the final state?