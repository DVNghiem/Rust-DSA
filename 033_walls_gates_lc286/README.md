# Walls and Gates - LeetCode 286

## Problem Statement

You are given an `m x n` grid where:
- `0` = a wall or an obstacle
- `INF` = an empty room (represented by a large integer like `2^31 - 1 = 2147483647`)
- Gates are denoted by integer `0`

Fill each empty room with the distance to its nearest gate.

If it is impossible to reach a gate, leave the cell as `INF`.

## Visual Walkthrough

```
Example:
[
  [INF, -1,   0,  INF],
  [INF, INF, INF,  -1],
  [INF, -1,  INF,  -1],
  [ 0,  -1,  INF,  INF]
]

After filling:
[
  [  3,  -1,   0,   1],
  [  2,   2,   1,  -1],
  [  1,  -1,   2,  -1],
  [  0,  -1,   3,   4]
]
```

### Distance Visualization

```
Grid with gates (0) and walls (-1):

. -1 . .       .  . . .       .  . 0 .
. . . -1  →    .  . . .   →    .  . . .
. -1 . .       .  . . .       .  . . .

Distance = Manhattan distance to nearest gate
           (only through empty cells, not walls)
```

### BFS from Gates Concept

```
Instead of computing distance from each room to nearest gate,
start from gates and BFS outward, tracking distance.

Gates are at distance 0.
Neighbors are distance 1.
Neighbors of those at distance 2.
etc.

This is essentially "multi-source BFS" in reverse.

For each empty cell, the first time we reach it from any gate
gives us the shortest distance.
```

## Approach Comparison

| Approach | Time | Space | Description |
|----------|------|-------|-------------|
| BFS from gates | O(m×n) | O(m×n) | Multi-source BFS |
| Dijkstra | O(m×n log(m×n)) | O(m×n) | If weighted, but not needed |
| DFS from each cell | O(m²×n²) | O(m×n) | Try each gate, too slow |

### Why BFS Works

- BFS from gates explores level by level
- First time we reach an empty cell is the shortest path
- We don't explore further once distance is set

## Implementation Strategy

### Multi-Source BFS

```rust
use std::collections::VecDeque;

const INF: i32 = 2147483647;

pub fn walls_and_gates(grid: &mut Vec<Vec<i32>>) {
    let m = grid.len();
    if m == 0 { return; }
    let n = grid[0].len();

    let mut queue = VecDeque::new();

    // Initialize: find all gates
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 0 {
                queue.push_back((i, j));
            }
        }
    }

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    while !queue.is_empty() {
        let (r, c) = queue.pop_front();
        let distance = grid[r][c]; // Current distance

        for (dr, dc) in directions.iter() {
            let nr = r as isize + dr;
            let nc = c as isize + dc;

            if nr < 0 || nr >= m as isize || nc < 0 || nc >= n as isize {
                continue;
            }

            let nr = nr as usize;
            let nc = nc as usize;

            // Only process empty rooms (INF)
            if grid[nr][nc] == INF {
                grid[nr][nc] = distance + 1;
                queue.push_back((nr, nc));
            }
        }
    }
}
```

## Edge Cases

1. **No gates**: All rooms remain INF
2. **No empty rooms**: Nothing to fill
3. **Blocked rooms**: Some rooms unreachable → remain INF
4. **Large grid**: BFS handles efficiently
5. **Multiple gates**: All considered in BFS

## Test Cases

1. Basic grid with gates and rooms
2. No gates (all INF)
3. No empty rooms (all 0 and -1)
4. Partially blocked (some unreachable)
5. Dense walls
6. Large grid
7. Gate in corner
8. All gates

## Solution Explanation

### Key Insight

Multi-source BFS from all gates simultaneously. Each gate starts at distance 0, its neighbors at 1, etc. First time we reach a cell gives shortest distance.

### Why Not DFS from Each Cell?

For each empty cell, we'd try BFS to find nearest gate - O(m×n) per cell = O(m²×n²) total. Too slow.

With BFS from gates:
- Each cell visited once → O(m×n)
- All distances computed in single BFS

## Complexity Analysis

- **Time**: O(m×n) - each cell processed at most once
- **Space**: O(m×n) - queue can hold all cells

## Follow-up Questions

1. What if gates had different "strengths"?
2. How to modify for 8-directional movement?
3. Can you track which gate is nearest too?