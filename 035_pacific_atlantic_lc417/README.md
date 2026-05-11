# Pacific Atlantic Water Flow - LeetCode 417

## Problem Statement

Given an `m x n` matrix of non-negative integers representing the height of terrain at each cell, water can flow from a cell to another cell if the height is less than or equal to the current cell.

Pacific Ocean touches the left and top edges, and Atlantic Ocean touches the right and bottom edges.

Return a list of cell coordinates where water can flow to BOTH the Pacific and Atlantic oceans.

## Visual Walkthrough

```
Example:
 heights = [
   [1,2,2,3,1],
   [3,2,3,4,4],
   [2,4,5,3,1],
   [6,7,1,4,5],
   [5,1,1,2,4]
 ]

Pacific touches: top row + left column
Atlantic touches: bottom row + right column

Result: [[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[3,2],[4,4]]

Visualization:
P . . . A    P = Pacific side
P P . . A    A = Atlantic side
P P P . A
P P P P A
P P P P A
```

### Water Flow Concept

```
A cell can flow to ocean if:
1. It's on the edge (directly touches ocean), OR
2. It can flow to a neighbor that can reach the ocean

Water flows from HIGH to LOW or EQUAL height.

If cell can reach both oceans → it's in result.
```

### Multi-Source BFS/DFS

```
Instead of checking from each cell if it can reach both oceans,
reverse the problem:

1. From Pacific edge (top+left), BFS/DFS to find all cells that can reach Pacific
2. From Atlantic edge (bottom+right), BFS/DFS to find all cells that can reach Atlantic
3. Cells reachable from BOTH → result

Water flows from higher to lower (or equal).
So during BFS, we can only move to cells with height <= current.
```

## Approach Comparison

| Approach | Time | Space | Description |
|----------|------|-------|-------------|
| BFS from oceans | O(m×n) | O(m×n) | Two BFS passes |
| DFS from oceans | O(m×n) | O(m×n) | Two DFS passes |

### Why Reverse Approach?

- Forward: Check from each cell if it can reach both oceans (expensive)
- Reverse: Find all cells that can reach each ocean, then intersect

## Implementation Strategy

### Multi-Source BFS

```rust
pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = heights.len();
    let n = heights[0].len();

    let mut pacific = vec![vec![false; n]; m];
    let mut atlantic = vec![vec![false; n]; m];

    // BFS from Pacific edges (top row + left column)
    let mut queue = VecDeque::new();

    // Top row
    for j in 0..n {
        pacific[0][j] = true;
        queue.push_back((0, j));
    }
    // Left column (skip (0,0) already added)
    for i in 1..m {
        pacific[i][0] = true;
        queue.push_back((i, 0));
    }

    // BFS to mark all cells that can reach Pacific
    bfs(&heights, &mut pacific, &mut queue);

    // BFS from Atlantic edges (bottom row + right column)
    queue.clear();
    for j in 0..n {
        atlantic[m-1][j] = true;
        queue.push_back((m-1, j));
    }
    for i in 0..m-1 {
        atlantic[i][n-1] = true;
        queue.push_back((i, n-1));
    }

    bfs(&heights, &mut atlantic, &mut queue);

    // Find cells that can reach both
    let mut result = Vec::new();
    for i in 0..m {
        for j in 0..n {
            if pacific[i][j] && atlantic[i][j] {
                result.push(vec![i as i32, j as i32]);
            }
        }
    }

    result
}
```

## Edge Cases

1. **Empty grid**: Return empty result
2. **Single cell**: Check if it touches both (rare) or only one
3. **All same height**: All edge cells reach both, interior depends
4. **Descending heights**: Edge cells only
5. **Ascending heights**: Interior may reach both

## Test Cases

1. Basic example from problem
2. No cells reach both
3. All cells reach both
4. Single row
5. Single column
6. Varying heights

## Solution Explanation

### Key Insight

Start BFS from ocean edges. Water flows from high to low (or equal), so BFS can only traverse to cells with height <= current. This finds all cells that CAN flow TO that ocean (reverse of "ocean flows to cell").

### BFS Algorithm

1. Start with all Pacific-edge cells (queue)
2. Mark reachable Pacific cells
3. From each visited cell, try neighbors with height <= current
4. Repeat for Atlantic

## Complexity Analysis

- **Time**: O(m×n) - each cell visited at most twice
- **Space**: O(m×n) - visited arrays and queue

## Follow-up Questions

1. Can you solve with DFS instead?
2. What if water can flow both ways (high to low and low to high)?
3. How to track the actual paths?