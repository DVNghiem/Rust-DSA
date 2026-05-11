# Pacific Atlantic Water Flow - Solution Analysis

## Problem Overview

Given heights matrix where water flows from high to low/equal heights, find all cells that can flow to BOTH Pacific Ocean (top+left edges) and Atlantic Ocean (bottom+right edges).

## Solution 1: Multi-Source BFS

### Code Implementation

```rust
pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if heights.is_empty() || heights[0].is_empty() { return vec![]; }

    let m = heights.len();
    let n = heights[0].len();
    let mut pacific = vec![vec![false; n]; m];
    let mut atlantic = vec![vec![false; n]; m];
    let mut queue = VecDeque::new();

    // Pacific: top row + left column
    for j in 0..n { pacific[0][j] = true; queue.push_back((0, j)); }
    for i in 1..m { pacific[i][0] = true; queue.push_back((i, 0)); }
    bfs(&heights, &mut pacific, &mut queue);

    // Atlantic: bottom row + right column
    queue.clear();
    for j in 0..n { atlantic[m-1][j] = true; queue.push_back((m-1, j)); }
    for i in 0..m-1 { atlantic[i][n-1] = true; queue.push_back((i, n-1)); }
    bfs(&heights, &mut atlantic, &mut queue);

    // Find cells reachable from both
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

fn bfs(heights: &[Vec<i32>], visited: &mut Vec<Vec<bool>>, queue: &mut VecDeque<(usize, usize)>) {
    let m = heights.len();
    let n = heights[0].len();
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    while let Some((r, c)) = queue.pop_front() {
        let current_height = heights[r][c];

        for (dr, dc) in directions.iter() {
            let nr = r as isize + dr;
            let nc = c as isize + dc;

            if nr < 0 || nr >= m as isize || nc < 0 || nc >= n as isize { continue; }
            let nr = nr as usize;
            let nc = nc as usize;

            // Can only flow to equal or lower height
            // AND cell must not be visited yet
            if !visited[nr][nc] && heights[nr][nc] >= current_height {
                visited[nr][nc] = true;
                queue.push_back((nr, nc));
            }
        }
    }
}
```

### Line-by-Line Analysis

1. **Initialize visited arrays**: `pacific` and `atlantic` matrices to track which cells can reach each ocean.

2. **Pacific initialization**: Top row (row 0) and left column (col 0) are edges touching Pacific.

3. **`bfs(&heights, &mut pacific, &mut queue);`**: BFS from all Pacific edge cells to find all cells that can reach Pacific.

4. **BFS logic**: Water flows from high to low/equal, so we can only move to cells with height >= current cell's height.

5. **`if !visited[nr][nc] && heights[nr][nc] >= current_height { ... }`**: Key condition - neighbor must be unvisited AND neighbor's height is greater than or equal to current (water can flow uphill to it).

6. **Repeat for Atlantic**: Same process starting from Atlantic edges.

7. **Intersect results**: Cells marked in both arrays can reach both oceans.

### BFS Visualization

```
Heights:
[1, 2, 2, 3, 1]
[3, 2, 3, 4, 4]
[2, 4, 5, 3, 1]
[6, 7, 1, 4, 5]
[5, 1, 1, 2, 4]

Pacific start cells: (0,0)-(0,4), (1,0)-(4,0)
Atlantic start cells: (4,0)-(4,4), (0,4)-(3,4)

From (0,0) height=1:
- Can flow to (0,1) height=2? 2 >= 1 YES
- Can flow to (1,0) height=3? 3 >= 1 YES

From (4,4) height=4:
- Can flow to (4,3) height=2? 2 >= 4 NO
- Can flow to (3,4) height=5? 5 >= 4 YES

After BFS from both sides, intersect to find cells that can reach both.
```

## Solution 2: DFS Approach

```rust
fn dfs(heights: &[Vec<i32>], r: usize, c: usize, height: i32, visited: &mut Vec<Vec<bool>>) {
    let m = heights.len();
    let n = heights[0].len();

    if r >= m || c >= n || visited[r][c] || heights[r][c] < height { return; }

    visited[r][c] = true;

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    for (dr, dc) in directions.iter() {
        let nr = r as isize + dr;
        let nc = c as isize + dc;

        if nr >= 0 && nr < m as isize && nc >= 0 && nc < n as isize {
            dfs(heights, nr as usize, nc as usize, heights[r][c], visited);
        }
    }
}
```

### DFS vs BFS

- BFS: Uses explicit queue, good for level-based processing
- DFS: Recursive, good for exploring deep paths
- Both achieve same result for this problem

## Complexity Comparison

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| BFS | O(m×n) | O(m×n) | Two passes |
| DFS | O(m×n) | O(m×n) | Two passes |

## Key Insights

1. **Reverse thinking**: Instead of "can this cell reach both oceans", find "which cells can reach each ocean" and intersect.

2. **Multi-source BFS**: Start from all ocean-edge cells simultaneously.

3. **Height constraint**: Water flows from high to low or equal, so BFS moves to cells with height >= current.

4. **First-time visited = can reach ocean**: Each cell marked true means it can flow to that ocean.

## Test Case Analysis

### Test: `test_basic_example`

```
Heights matrix from problem:
[1,2,2,3,1]
[3,2,3,4,4]
[2,4,5,3,1]
[6,7,1,4,5]
[5,1,1,2,4]

Expected result has 8 cells:
- (0,4): Height 1, can flow to both
- (1,3): Height 4, etc.

All cells on valid diagonal from high terrain can reach both.
```

### Test: `test_decreasing_height`

```
[3,2,1]
[3,2,1]
[3,2,1]

Cells on anti-diagonal (0,2), (1,1), (2,0) can reach both.
Higher cells (3) on left/top can only reach Pacific.
Lower cells (1) on right/bottom can only reach Atlantic.
```

## Edge Cases

1. **Empty grid**: Return empty
2. **Single cell**: May or may not touch both oceans depending on position
3. **All same height**: All can reach both (water flows freely)
4. **Mountain peak**: Only high cells near both edges reach both

## Follow-up Answers

**Q: Why >= and not <= for height comparison?**
A: Water flows from higher to lower. But we start from ocean and go "upstream" - so we can only reach cells with height >= current. This is reverse direction.

**Q: Why intersection and not union?**
A: We need cells that reach BOTH oceans, not EITHER ocean.

**Q: What if water could flow uphill?**
A: The problem specifies water flows from high to low. If uphill was allowed, all cells would reach all oceans (connected graph).

**Q: Time complexity?**
A: Each cell visited at most twice (once per ocean). O(m×n) total.

**Q: Could we use single BFS?**
A: No, because we need to know which ocean each cell can reach. Separate tracking needed.