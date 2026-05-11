# Number of Islands - Solution Analysis

## Problem Overview

Given a 2D grid of '1's (land) and '0's (water), count the number of islands. An island is formed by connecting adjacent land cells horizontally or vertically.

## Solution 1: DFS Recursive

### Code Implementation

```rust
pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    if grid.is_empty() || grid[0].is_empty() { return 0; }

    let m = grid.len();
    let n = grid[0].len();
    let mut grid = grid;
    let mut count = 0;

    fn dfs(grid: &mut Vec<Vec<char>>, i: usize, j: usize, m: usize, n: usize) {
        // Base: out of bounds or water
        if i >= m || j >= n || grid[i][j] == '0' { return; }

        // Mark as visited (flood fill)
        grid[i][j] = '0';

        // Visit all 4 directions
        if i > 0 { dfs(grid, i - 1, j, m, n); }
        if i + 1 < m { dfs(grid, i + 1, j, m, n); }
        if j > 0 { dfs(grid, i, j - 1, m, n); }
        if j + 1 < n { dfs(grid, i, j + 1, m, n); }
    }

    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == '1' {
                count += 1;
                dfs(&mut grid, i, j, m, n);
            }
        }
    }

    count
}
```

### Line-by-Line Analysis

1. **`if grid.is_empty() || grid[0].is_empty() { return 0; }`**: Handle empty grid edge case.

2. **`let m = grid.len(); let n = grid[0].len();`**: Get dimensions.

3. **`let mut grid = grid;`**: Make mutable to modify in-place for visited marking.

4. **`fn dfs(grid: &mut Vec<Vec<char>>, i: usize, j: usize, m: usize, n: usize)`**: Inner recursive function with bounds checking.

5. **`if i >= m || j >= n || grid[i][j] == '0' { return; }`**: Base case - stop if out of bounds or water cell.

6. **`grid[i][j] = '0';`**: Mark current cell as visited (in-place modification).

7. **Four DFS calls**: Recursively visit all 4-connected neighbors (up, down, left, right).

8. **Double loop**: Scan entire grid, when finding unvisited '1', increment count and flood fill.

### DFS Flood Fill Visualization

```
Grid:
1 1 1
1 0 1
1 1 1

Start at (0,0):
  Mark (0,0) → visit up? no | down (1,0) | left? no | right (0,1)
  Mark (0,1) → visit neighbors...
  Mark (1,0) → ...
  Mark (1,1) is '0' → skip
  Mark (1,2) → ...
  Mark (2,0) → ...
  Mark (2,1) → ...
  Mark (2,2) → ...

All connected 1s marked → count = 1
```

## Solution 2: BFS with Queue

### Code Implementation

```rust
pub fn num_islands_bfs(grid: Vec<Vec<char>>) -> i32 {
    if grid.is_empty() || grid[0].is_empty() { return 0; }

    let m = grid.len();
    let n = grid[0].len();
    let mut grid = grid;
    let mut count = 0;
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == '1' {
                count += 1;
                grid[i][j] = '0';
                let mut queue = VecDeque::new();
                queue.push_back((i, j));

                while let Some((r, c)) = queue.pop_front() {
                    for (dr, dc) in directions.iter() {
                        let nr = r as isize + dr;
                        let nc = c as isize + dc;
                        if nr >= 0 && nr < m as isize && nc >= 0 && nc < n as isize {
                            let nr = nr as usize;
                            let nc = nc as usize;
                            if grid[nr][nc] == '1' {
                                grid[nr][nc] = '0';
                                queue.push_back((nr, nc));
                            }
                        }
                    }
                }
            }
        }
    }

    count
}
```

### Line-by-Line Analysis

1. **`let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];`**: Four direction vectors for 4-connectivity.

2. **When '1' found: `count += 1; grid[i][j] = '0'; queue.push_back((i, j));`**: Start new island, mark as visited, add to queue.

3. **`while let Some((r, c)) = queue.pop_front()`**: Process nodes FIFO.

4. **`for (dr, dc) in directions.iter()`**: Check all 4 neighbors.

5. **Bounds check with isize**: Use isize to prevent underflow when subtracting.

6. **Push valid neighbors**: If '1', mark visited and add to queue.

## Solution 3: Iterative DFS (Explicit Stack)

```rust
pub fn num_islands_dfs_iterative(grid: Vec<Vec<char>>) -> i32 {
    // Same structure as BFS but use stack instead of queue
    let mut stack = vec![(i, j)]; // instead of VecDeque
    while let Some((r, c)) = stack.pop() { // pop instead of pop_front
        // rest is similar
    }
}
```

### DFS vs BFS

```
DFS (stack): Process nodes LIFO - goes deep before wide
BFS (queue): Process nodes FIFO - explores level by level

For flood fill of a single island, both visit all connected cells.
DFS may have better cache locality for certain grid patterns.

For finding ALL islands in grid, both have same O(m*n) complexity.
```

## Complexity Comparison

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| DFS Recursive | O(m×n) | O(m×n) | Stack overflow risk |
| BFS | O(m×n) | O(min(m,n)) | No overflow, uses queue |
| DFS Iterative | O(m×n) | O(m×n) | Explicit stack |

## Key Insights

1. **In-place marking saves memory**: Change '1' → '0' after visiting instead of separate visited array.

2. **4-directional only**: Diagonal cells are NOT connected in this problem.

3. **Each '1' belongs to exactly one island**: So marking prevents double counting.

4. **Flood fill analogy**: Like pouring water to fill connected land areas.

## Test Case Analysis

### Test: `test_multiple_islands`

```
Grid:
1 1 0 0 0
1 1 0 0 0
0 0 1 0 0
0 0 0 1 1

Island 1: (0,0), (0,1), (1,0), (1,1) - 4 cells
Island 2: (2,2) - single cell
Island 3: (3,3), (3,4) - 2 cells

Total: 3 islands ✓
```

### Test: `test_diagonal_not_connected`

```
Grid:
1 0 1
0 1 0
1 0 1

No two '1's are adjacent horizontally or vertically.
Each isolated '1' is its own island.

Total: 5 islands (4 corners + 1 center) ✓
```

## Edge Cases

1. **Empty grid**: Return 0 (handled at start)
2. **All water**: No '1's found, count stays 0
3. **All land**: Single DFS marks entire grid, count = 1
4. **Single cell**: Simple case with no neighbors

## Follow-up Answers

**Q: How to handle 8-directional connectivity?**
A: Add 4 more directions: (1,1), (1,-1), (-1,1), (-1,-1)

**Q: Can you use Union-Find?**
A: Yes - DSU to group connected '1's. Slower due to constant factor.

**Q: What about very large grids causing stack overflow?**
A: Use BFS or iterative DFS with explicit stack instead of recursion.

**Q: How to find largest island size?**
A: Track size during DFS, return max instead of count.

**Q: Space complexity analysis?**
A: O(1) if modify in-place, O(m×n) if using separate visited array.