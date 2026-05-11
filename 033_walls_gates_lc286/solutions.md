# Walls and Gates - Solution Analysis

## Problem Overview

Given a grid with gates (0), walls (-1), and empty rooms (INF), fill each empty room with its distance to the nearest gate. Unreachable rooms stay as INF.

## Solution: Multi-Source BFS

### Code Implementation

```rust
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
        let (r, c) = queue.pop_front().unwrap();
        let distance = grid[r][c]; // Current distance (0 for gates)

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

### Line-by-Line Analysis

1. **`const INF: i32 = 2147483647;`**: Large value representing empty room.

2. **`let m = grid.len(); if m == 0 { return; } let n = grid[0].len();`**: Get dimensions, handle empty.

3. **`let mut queue = VecDeque::new();`**: BFS queue.

4. **Gates initialization**: Push all gates (value 0) into queue with their positions.

5. **`let distance = grid[r][c];`**: Current cell's distance value. For gates this is 0.

6. **Four direction exploration**: Check all 4 neighbors.

7. **`if grid[nr][nc] == INF { ... }`**: Only process cells that haven't been reached (still INF).

8. **`grid[nr][nc] = distance + 1;`**: Set distance = parent's distance + 1.

9. **`queue.push_back((nr, nc));`**: Add newly reached cell for next level processing.

### BFS Visualization

```
Initial:
[INF, -1, 0, INF]
[INF, INF, INF, -1]
[INF, -1, INF, -1]
[0, -1, INF, INF]

Queue starts with: [(0,2), (3,0)] ← two gates

Process (0,2) distance=0:
  → (0,1): wall skip
  → (0,3): INF → set to 1, queue
  → (1,2): INF → set to 1, queue
  → (-1,2): skip

Queue: [(3,0), (0,3), (1,2)]
distances now: [INF, -1, 0, 1]

Process (3,0) distance=0:
  → (2,0): INF → set to 1, queue
  → (3,1): wall skip
  → (4,0): skip
  → (3,-1): skip

Queue: [(0,3), (1,2), (2,0)]

Continue... eventually:
[3, -1, 0, 1]
[2,  2, 1, -1]
[1, -1, 2, -1]
[0, -1, 3,  4]
```

## Alternative: Explicit Distance Version

```rust
fn walls_and_gates_v2(grid: &mut Vec<Vec<i32>>) {
    // Stores (row, col, distance)
    let mut queue: Vec<(usize, usize, i32)> = Vec::new();

    // Initialize with all gates at distance 0
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 0 {
                queue.push((i, j, 0));
            }
        }
    }

    // Visited array tracks which gates can reach each cell
    let mut visited = vec![vec![false; n]; m];

    // Mark gates as visited
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 0 {
                visited[i][j] = true;
            }
        }
    }

    let mut head = 0;
    while head < queue.len() {
        let (r, c, dist) = queue[head];
        head += 1;

        // Process neighbors...
    }
}
```

This version explicitly stores distance in queue instead of reading from grid.

## Complexity Comparison

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| BFS (in-place) | O(m×n) | O(m×n) | Modifies grid, no extra space for distance |
| BFS (explicit) | O(m×n) | O(m×n) | Uses visited array |

## Key Insights

1. **Reverse thinking**: Instead of "find nearest gate from each cell", start from gates and spread outward.

2. **Multi-source BFS**: All gates are sources at time 0, so we find shortest distances to ANY gate.

3. **First arrival = shortest**: First time we reach a cell is via shortest path.

4. **Only process INF cells**: Once a cell has distance set, we don't revisit it.

## Why Not Dijkstra?

For unweighted grids, BFS gives shortest paths. Dijkstra with all edge weights = 1 is equivalent to BFS but with more overhead.

## Test Case Analysis

### Test: `test_basic_fill`

```
Initial:
[INF, -1, 0, INF]
[INF, INF, INF, -1]
[INF, -1, INF, -1]
[0, -1, INF, INF]

After BFS:
[3, -1, 0, 1]
[2,  2, 1, -1]
[1, -1, 2, -1]
[0, -1, 3,  4]

Verification:
- (0,0): path to (0,2) = 2, path to (3,0) = 3, nearest = 2? Wait...
  Actually (3,0) is closer! Distance = 3
- (3,0): it's a gate = 0 ✓
- (1,1): adjacent to (1,0)=2, (0,1)=wall, (1,2)=1, (2,1)=wall → nearest = 2 ✓
```

### Test: `test_partially_blocked`

```
[INF, -1]
[-1, INF]

Gates: none
Queue: empty
Nothing processed
All INF remain → unreachable ✓
```

## Edge Cases

1. **No gates**: Queue empty, nothing happens, all remain INF
2. **No empty rooms**: Only gates and walls, nothing to fill
3. **Unreachable rooms**: Stay as INF (never visited)
4. **Multiple gates**: Each room gets distance to NEAREST (first BFS arrival)

## Follow-up Answers

**Q: How to modify for 8-directional movement?**
A: Add diagonals to directions: [(1,1), (1,-1), (-1,1), (-1,-1)]

**Q: Can you track which gate is nearest too?**
A: Store gate ID in cell along with distance, or run separate analysis after computing distances.

**Q: What if gates had different "strengths" (different propagation speeds)?**
A: That would require Dijkstra with varying edge weights.

**Q: Difference from Rotten Oranges problem?**
A: Same BFS pattern but different values interpretation. In rotten oranges, we mark fresh→rotten. Here we compute distances to gates.

**Q: Why use -1 for walls and INF for empty?**
A: Problem constraint - walls block paths, INF is placeholder for unfilled distance.