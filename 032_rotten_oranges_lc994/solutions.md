# Rotten Oranges - Solution Analysis

## Problem Overview

Given a grid with fresh oranges (1), rotten oranges (2), and empty cells (0). Every minute, fresh oranges adjacent to rotten ones become rotten. Find minimum minutes to rot all fresh oranges, or -1 if impossible.

## Solution: Multi-Source BFS

### Code Implementation

```rust
pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
    if grid.is_empty() || grid[0].is_empty() { return 0; }

    let m = grid.len();
    let n = grid[0].len();
    let mut grid = grid;
    let mut queue = VecDeque::new();
    let mut fresh = 0;

    // Initialize: find all rotten oranges and count fresh
    for i in 0..m {
        for j in 0..n {
            match grid[i][j] {
                2 => { queue.push_back((i, j)); }
                1 => { fresh += 1; }
                _ => {}
            }
        }
    }

    // No fresh oranges - already done
    if fresh == 0 { return 0; }
    // No rotten oranges - can't rot anything
    if queue.is_empty() { return -1; }

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut minutes = 0;

    // BFS: process level by level (minute by minute)
    while !queue.is_empty() && fresh > 0 {
        let level_size = queue.len();

        for _ in 0..level_size {
            let (r, c) = queue.pop_front().unwrap();

            for (dr, dc) in directions.iter() {
                let nr = r as isize + dr;
                let nc = c as isize + dc;

                if nr < 0 || nr >= m as isize || nc < 0 || nc >= n as isize {
                    continue;
                }

                let nr = nr as usize;
                let nc = nc as usize;

                // If fresh, rot it!
                if grid[nr][nc] == 1 {
                    grid[nr][nc] = 2;
                    fresh -= 1;
                    queue.push_back((nr, nc));
                }
            }
        }

        minutes += 1;
    }

    if fresh > 0 { -1 } else { minutes }
}
```

### Line-by-Line Analysis

1. **`if grid.is_empty() || grid[0].is_empty() { return 0; }`**: Handle empty grid.

2. **`let m = grid.len(); let n = grid[0].len();`**: Get dimensions.

3. **`let mut queue = VecDeque::new(); let mut fresh = 0;`**: Initialize queue for BFS and counter for fresh oranges.

4. **`match grid[i][j] { 2 => { queue.push_back((i, j)); } 1 => { fresh += 1; } _ => {} }`**: First pass: collect all rotten oranges as starting points, count fresh oranges.

5. **`if fresh == 0 { return 0; }`**: No fresh oranges to rot.

6. **`if queue.is_empty() { return -1; }`**: No rotten to start spreading.

7. **`let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];`**: Four 4-directional vectors.

8. **`while !queue.is_empty() && fresh > 0 { ... }`**: Main BFS loop - continue while we have rotten sources and fresh oranges to rot.

9. **`let level_size = queue.len();`**: Number of rotten oranges that exist at the START of this minute (current BFS level).

10. **`for _ in 0..level_size { ... }`**: Process all rotten oranges from current minute before incrementing minutes.

11. **`let (r, c) = queue.pop_front().unwrap();`**: Get a rotten orange to spread from.

12. **`for (dr, dc) in directions.iter() { ... }`**: Check all 4 neighbors.

13. **`if grid[nr][nc] == 1 { grid[nr][nc] = 2; fresh -= 1; queue.push_back((nr, nc)); }`**: If neighbor is fresh, rot it (mark as 2), decrement fresh count, add to queue for next minute.

14. **`minutes += 1;`**: After processing entire level, increment minute count.

15. **`if fresh > 0 { -1 } else { minutes }`**: If any fresh remain, they couldn't be reached.

### BFS Level Visualization

```
Initial grid:
[2, 1, 1]
[1, 1, 0]
[0, 1, 1]

Queue initially: [(0,0)]  ← all rotten at start

Minute 0:
  Processing (0,0)
  Rot adjacent fresh: (0,1), (1,0)
  Queue now: [(0,1), (1,0)]

Minute 1:
  level_size = 2
  Process (0,1): rot (0,2), (1,1)
  Process (1,0): rot (2,0)
  Queue: [(1,1), (2,0)] (newly added)
  minutes = 1

Minute 2:
  level_size = 2
  Process (1,1): rot (1,2) (2,1 already rotten?)
  Process (2,0): nothing new
  Queue: [(1,2)]
  minutes = 2

Minute 3:
  level_size = 1
  Process (1,2): rot (2,2)
  Queue: [(2,2)]
  minutes = 3

Minute 4:
  level_size = 1
  Process (2,2): nothing new (fresh count becomes 0)
  Queue: []
  minutes = 4

Result: 4 ✓
```

## Key Insights

1. **Multi-source BFS**: All initially rotten oranges are sources at time 0.
2. **Level = Minute**: Each BFS level corresponds to one minute.
3. **Track fresh count**: When fresh reaches 0, we're done.
4. **Level processing**: Process all nodes at current level before incrementing time.

## Complexity Analysis

| Metric | Value |
|--------|-------|
| Time | O(m × n) - each cell visited at most once |
| Space | O(m × n) - queue can hold all cells |

## Test Case Analysis

### Test: `test_impossible_case`

```
Grid:
[2, 1, 1]
[0, 1, 1]
[1, 0, 1]

Fresh at (0,1) adjacent to (0,0) rotten → will rot
Fresh at (1,1) adjacent to (0,1), (1,2) → will rot eventually
Fresh at (1,2) adjacent to (0,2)? no, (1,1)? no, (2,2)? no, (0,2)? no
  Actually (1,2) adjacent to (1,1) which becomes rotten → will rot
Fresh at (2,2) adjacent to (2,1)? no (empty), (1,2)? not rotten yet
  Wait, (2,2) is fresh with no adjacent rotten initially
  Cannot reach it through empty cells (1,2) initially empty (value 0)

Actually looking more carefully:
- (2,2) at (row=2, col=2) has neighbors: (1,2), (2,1), (2,3), (3,2)
- Only (2,1) exists and is empty (0)
- So (2,2) can never be reached!

Result: -1 ✓
```

### Test: `test_simple_case`

```
Grid:
[2, 1]
[1, 0]

Initial rotten: (0,0)
Fresh: (0,1), (1,0)

Minute 0:
  (0,0) rots (0,1) and (1,0)

Minute 1:
  All fresh are now rotten
  fresh = 0

Result: 1 ✓
```

## Alternative Implementation (V2)

The second version uses array indexing instead of VecDeque, but same principle:

```rust
let mut head = 0;
while head < queue.len() {
    let start = head;
    let end = queue.len();
    // process all from start to end (current level)
    while head < end {
        // process queue[head]
        head += 1;
    }
    minutes += 1;
}
```

## Edge Cases

1. **No fresh**: Return 0 immediately
2. **No rotten**: Return -1 (can't start)
3. **All rotten**: Return 0 (already done)
4. **Isolated fresh**: Return -1 (can't reach)

## Follow-up Answers

**Q: Why track fresh count instead of checking grid?**
A: Checking grid would require scanning entire grid each minute. Fresh count is O(1) update per rotting event.

**Q: Can we use DFS instead?**
A: Yes but DFS doesn't naturally give us "minute" count by level. BFS with level tracking is more natural.

**Q: What if we need to know which oranges rot at which minute?**
A: Store (row, col, time) in queue instead of just (row, col).

**Q: Can we solve with fewer space?**
A: Not easily - we need to track which rotten oranges to process each minute.

**Q: Relationship to shortest path?**
A: This IS shortest path from multiple sources to all fresh nodes, where each edge weight is 1 minute.