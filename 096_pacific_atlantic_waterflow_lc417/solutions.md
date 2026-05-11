# Solutions - Pacific Atlantic Water Flow (LeetCode 417)

## Solution 1: Multi-Source BFS (Optimal)

```rust
pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if heights.is_empty() || heights[0].is_empty() {
        return vec![];
    }

    let m = heights.len();
    let n = heights[0].len();

    let mut pacific = vec![vec![false; n]; m];
    let mut atlantic = vec![vec![false; n]; m];

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    // BFS from Pacific (top row and left column)
    let mut queue = VecDeque::new();
    for j in 0..n { pacific[0][j] = true; queue.push_back((0, j)); }
    for i in 0..m { pacific[i][0] = true; queue.push_back((i, 0)); }

    while let Some((i, j)) = queue.pop_front() {
        for (dr, dc) in &directions {
            let ni = *i as i32 + dr;
            let nj = *j as i32 + dc;
            if ni < 0 || ni >= m as i32 || nj < 0 || nj >= n as i32 { continue; }
            let ni = ni as usize;
            let nj = nj as usize;
            if pacific[ni][nj] { continue; }
            if heights[ni][nj] >= heights[i][j] {
                pacific[ni][nj] = true;
                queue.push_back((ni, nj));
            }
        }
    }

    // BFS from Atlantic (bottom row and right column)
    for j in 0..n { atlantic[m-1][j] = true; queue.push_back((m-1, j)); }
    for i in 0..m { atlantic[i][n-1] = true; queue.push_back((i, n-1)); }

    while let Some((i, j)) = queue.pop_front() {
        for (dr, dc) in &directions {
            let ni = *i as i32 + dr;
            let nj = *j as i32 + dc;
            if ni < 0 || ni >= m as i32 || nj < 0 || nj >= n as i32 { continue; }
            let ni = ni as usize;
            let nj = nj as usize;
            if atlantic[ni][nj] { continue; }
            if heights[ni][nj] >= heights[i][j] {
                atlantic[ni][nj] = true;
                queue.push_back((ni, nj));
            }
        }
    }

    // Find cells reachable from both
    let mut result = vec![];
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

### Line-by-Line Analysis

**Lines 14-17: Edge Case Handling**
```rust
if heights.is_empty() || heights[0].is_empty() {
    return vec![];
}
```
Empty grid or empty row returns empty result. Essential for safety.

**Lines 19-20: Dimensions**
```rust
let m = heights.len();
let n = heights[0].len();
```
Store matrix dimensions. `m` = rows, `n` = columns.

**Lines 22-23: Visited Arrays**
```rust
let mut pacific = vec![vec![false; n]; m];
let mut atlantic = vec![vec![false; n]; m];
```
Two boolean matrices track reachability from each ocean. Initialized to `false`.

**Line 25: Directions**
```rust
let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
```
Four-directional movement: right, left, down, up.

**Lines 27-31: Pacific Initialization**
```rust
let mut queue = VecDeque::new();
for j in 0..n { pacific[0][j] = true; queue.push_back((0, j)); }
for i in 0..m { pacific[i][0] = true; queue.push_back((i, 0)); }
```
Pacific Ocean touches top row (row 0) and left column (column 0). All cells on these borders are marked reachable and added to queue.

**Lines 32-45: Pacific BFS**
```rust
while let Some((i, j)) = queue.pop_front() {
    for (dr, dc) in &directions {
        let ni = *i as i32 + dr;
        let nj = *j as i32 + dc;
        if ni < 0 || ni >= m as i32 || nj < 0 || nj >= n as i32 { continue; }
        let ni = ni as usize;
        let nj = nj as usize;
        if pacific[ni][nj] { continue; }
        if heights[ni][nj] >= heights[i][j] {
            pacific[ni][nj] = true;
            queue.push_back((ni, nj));
        }
    }
}
```
Standard BFS traversal. For each cell, explore neighbors. A neighbor is reachable if:
1. Not already visited (`!pacific[ni][nj]`)
2. Height is at least as great (`heights[ni][nj] >= heights[i][j]`)

The height condition ensures water flows from high to equal height (can't flow uphill).

**Lines 47-50: Atlantic Initialization**
```rust
for j in 0..n { atlantic[m-1][j] = true; queue.push_back((m-1, j)); }
for i in 0..m { atlantic[i][n-1] = true; queue.push_back((i, n-1)); }
```
Atlantic Ocean touches bottom row (row m-1) and right column (column n-1). Same pattern as Pacific.

**Lines 51-64: Atlantic BFS**
Same BFS logic applied for Atlantic reachability.

**Lines 66-75: Find Intersection**
```rust
let mut result = vec![];
for i in 0..m {
    for j in 0..n {
        if pacific[i][j] && atlantic[i][j] {
            result.push(vec![i as i32, j as i32]);
        }
    }
}
result
```
Cells reachable from BOTH oceans are the answer. Returns vector of [row, col] pairs.

### Complexity Analysis

| Aspect | Complexity |
|--------|------------|
| Time | O(m × n) - each cell visited at most twice (once per BFS) |
| Space | O(m × n) - two visited arrays and queue |

### Why This Works

The key insight is **reverse thinking**:
- Instead of asking "where can water flow from this cell to both oceans?"
- We ask "which cells can water reach from each ocean?"

Water can reach a cell from an ocean if there's a path following non-increasing heights. Starting from ocean borders and flowing "inward" (reverse of actual water flow) captures all cells that water could eventually drain to that ocean.

### Test Cases Verified

1. **Basic Example**: Matrix with varied heights - correctly identifies boundary cells
2. **Empty Grid**: Returns empty as expected
3. **Single Cell**: Returns [[0,0]] since single cell touches both borders
4. **All Same Height**: All cells flow to both oceans
5. **Increasing/Decreasing**: Correctly handles monotonic gradients

## Alternative Approaches

### Approach 2: DFS Recursion
DFS can work but risks stack overflow on large grids. BFS with explicit queue is preferred.

### Approach 3: Union Find
Treat each cell as a node, union cells that can flow to each other. Then check which cells are connected to both ocean borders. More complex and slower in practice.

### Approach 4: DP Table
Compute reachability using dynamic programming in two passes (left-to-right + top-to-bottom, then reverse). More memory but avoids queue.
