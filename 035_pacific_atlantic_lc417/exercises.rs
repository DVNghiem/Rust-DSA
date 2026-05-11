use std::collections::VecDeque;

/// Multi-source BFS from both oceans
///
/// Start BFS from each ocean edge and find all cells that can flow to it.
/// A cell can flow to neighbor only if neighbor's height <= current (water flows downhill).
pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if heights.is_empty() || heights[0].is_empty() {
        return vec![];
    }

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
    // Left column (skip (0,0) already added from top row)
    for i in 1..m {
        pacific[i][0] = true;
        queue.push_back((i, 0));
    }

    bfs(&heights, &mut pacific, &mut queue);

    // BFS from Atlantic edges (bottom row + right column)
    queue.clear();
    for j in 0..n {
        atlantic[m - 1][j] = true;
        queue.push_back((m - 1, j));
    }
    // Right column (skip (m-1, n-1) already added from bottom row)
    for i in 0..m - 1 {
        atlantic[i][n - 1] = true;
        queue.push_back((i, n - 1));
    }

    bfs(&heights, &mut atlantic, &mut queue);

    // Find cells that can reach both oceans
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

            // Bounds check
            if nr < 0 || nr >= m as isize || nc < 0 || nc >= n as isize {
                continue;
            }

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

/// Alternative DFS approach
pub fn pacific_atlantic_dfs(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if heights.is_empty() || heights[0].is_empty() {
        return vec![];
    }

    let m = heights.len();
    let n = heights[0].len();

    let mut pacific = vec![vec![false; n]; m];
    let mut atlantic = vec![vec![false; n]; m];

    // DFS from Pacific edges
    for j in 0..n {
        dfs(&heights, 0, j, i32::MIN, &mut pacific);
    }
    for i in 1..m {
        dfs(&heights, i, 0, i32::MIN, &mut pacific);
    }

    // DFS from Atlantic edges
    for j in 0..n {
        dfs(&heights, m - 1, j, i32::MIN, &mut atlantic);
    }
    for i in 0..m - 1 {
        dfs(&heights, i, n - 1, i32::MIN, &mut atlantic);
    }

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

fn dfs(heights: &[Vec<i32>], r: usize, c: usize, height: i32, visited: &mut Vec<Vec<bool>>) {
    let m = heights.len();
    let n = heights[0].len();

    // Check bounds and if this cell can be visited
    if r >= m || c >= n || visited[r][c] || heights[r][c] < height {
        return;
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    fn create_matrix(v: Vec<&[i32]>) -> Vec<Vec<i32>> {
        v.iter().map(|row| row.to_vec()).collect()
    }

    #[test]
    fn test_basic_example() {
        let heights = create_matrix(vec![
            &[1, 2, 2, 3, 1],
            &[3, 2, 3, 4, 4],
            &[2, 4, 5, 3, 1],
            &[6, 7, 1, 4, 5],
            &[5, 1, 1, 2, 4],
        ]);
        let result = pacific_atlantic(heights);

        // Expected: [[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[3,2],[4,4]]
        assert_eq!(result.len(), 8);
        let result_coords: Vec<(i32, i32)> = result.iter()
            .map(|v| (v[0], v[1]))
            .collect();
        assert!(result_coords.contains(&(0, 4)));
        assert!(result_coords.contains(&(1, 3)));
        assert!(result_coords.contains(&(3, 0)));
    }

    #[test]
    fn test_single_row() {
        let heights = create_matrix(vec![
            &[1, 2, 3, 4, 5],
        ]);
        let result = pacific_atlantic(heights);

        // All cells touch both oceans (single row)
        // Top edge = Pacific, bottom edge = also this row
        // Left = Pacific, right = Atlantic
        // Since single row, all touch both
        assert_eq!(result.len(), 5);
    }

    #[test]
    fn test_single_column() {
        let heights = create_matrix(vec![
            &[1],
            &[2],
            &[3],
            &[4],
            &[5],
        ]);
        let result = pacific_atlantic(heights);

        // All cells touch both oceans (single column)
        assert_eq!(result.len(), 5);
    }

    #[test]
    fn test_no_cells_reach_both() {
        // Mountain in center - only peak might reach both
        let heights = create_matrix(vec![
            &[1, 3, 1],
            &[3, 100, 3],
            &[1, 3, 1],
        ]);
        let result = pacific_atlantic(heights);

        // Center (1,1) height 100 can reach both
        assert!(result.iter().any(|v| v[0] == 1 && v[1] == 1));
    }

    #[test]
    fn test_all_cells_reach_both() {
        // Flat grid - all cells flow to both
        let heights = create_matrix(vec![
            &[1, 1],
            &[1, 1],
        ]);
        let result = pacific_atlantic(heights);
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_bfs_and_dfs_same() {
        let heights = create_matrix(vec![
            &[1, 2, 2, 3, 1],
            &[3, 2, 3, 4, 4],
            &[2, 4, 5, 3, 1],
            &[6, 7, 1, 4, 5],
            &[5, 1, 1, 2, 4],
        ]);
        let bfs_result = pacific_atlantic(heights.clone());
        let dfs_result = pacific_atlantic_dfs(heights);
        assert_eq!(bfs_result.len(), dfs_result.len());
    }

    #[test]
    fn test_empty_grid() {
        let heights: Vec<Vec<i32>> = vec![];
        let result = pacific_atlantic(heights);
        assert!(result.is_empty());
    }

    #[test]
    fn test_increasing_height() {
        // Height increases toward Atlantic
        // Only the highest cells on right/bottom might reach both
        let heights = create_matrix(vec![
            &[1, 2, 3],
            &[1, 2, 3],
            &[1, 2, 3],
        ]);
        let result = pacific_atlantic(heights);
        // All cells can flow to both - water flows from high to low
        // Since all same height, water can flow both directions
        assert_eq!(result.len(), 9);
    }

    #[test]
    fn test_decreasing_height() {
        // Height decreases toward Atlantic
        let heights = create_matrix(vec![
            &[3, 2, 1],
            &[3, 2, 1],
            &[3, 2, 1],
        ]);
        let result = pacific_atlantic(heights);
        // Only edge cells on left/top can reach Pacific
        // Only edge cells on right/bottom can reach Atlantic
        // Corner cells should be in result
        assert!(result.iter().any(|v| v[0] == 0 && v[1] == 0)); // (0,0)
        assert!(result.iter().any(|v| v[0] == 2 && v[1] == 2)); // (2,2)
    }

    #[test]
    fn test_diamond_shape() {
        // High in middle, low on edges
        //      1
        //     2 2
        //    3 3 3
        //   4 4 4 4
        //  5 5 5 5 5
        let heights = create_matrix(vec![
            &[0, 0, 0, 0, 0],
            &[0, 1, 1, 1, 0],
            &[0, 1, 2, 1, 0],
            &[0, 1, 1, 1, 0],
            &[0, 0, 0, 0, 0],
        ]);
        let result = pacific_atlantic(heights);
        // Center should reach both
        assert!(result.iter().any(|v| v[0] == 2 && v[1] == 2));
    }

    #[test]
    fn test_two_by_two() {
        let heights = create_matrix(vec![
            &[1, 2],
            &[3, 4],
        ]);
        let result = pacific_atlantic(heights);

        // All 4 cells should reach both - each touches both oceans in 2x2
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_stairs() {
        // Ascending stairs from top-left to bottom-right
        let heights = create_matrix(vec![
            &[1, 2, 3, 4],
            &[2, 3, 4, 5],
            &[3, 4, 5, 6],
            &[4, 5, 6, 7],
        ]);
        let result = pacific_atlantic(heights);
        // Many cells should reach both
        assert!(result.len() > 0);
    }

    #[test]
    fn test_valley() {
        // Low in middle, high on edges
        // 5 5 5 5 5
        // 5 1 1 1 5
        // 5 1 1 1 5
        // 5 5 5 5 5
        let heights = create_matrix(vec![
            &[5, 5, 5, 5, 5],
            &[5, 1, 1, 1, 5],
            &[5, 1, 1, 1, 5],
            &[5, 5, 5, 5, 5],
        ]);
        let result = pacific_atlantic(heights);
        // All cells (including edges) should reach both
        assert_eq!(result.len(), 16);
    }

    #[test]
    fn test_peak() {
        // Single peak in center
        // 1 1 1
        // 1 5 1
        // 1 1 1
        let heights = create_matrix(vec![
            &[1, 1, 1],
            &[1, 5, 1],
            &[1, 1, 1],
        ]);
        let result = pacific_atlantic(heights);
        // Center (1,1) should reach both as peak
        assert!(result.iter().any(|v| v[0] == 1 && v[1] == 1));
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("035_pacific_atlantic_lc417 exercises - run tests with cargo test");
}
