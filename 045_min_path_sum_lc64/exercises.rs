/// Approach 1: DP with full array
pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    if m == 0 { return 0; }
    let n = grid[0].len();

    let mut dp = vec![vec![0i32; n]; m];

    for i in 0..m {
        for j in 0..n {
            if i == 0 && j == 0 {
                dp[i][j] = grid[i][j];
            } else if i == 0 {
                dp[i][j] = dp[i][j-1] + grid[i][j];
            } else if j == 0 {
                dp[i][j] = dp[i-1][j] + grid[i][j];
            } else {
                dp[i][j] = grid[i][j] + dp[i-1][j].min(dp[i][j-1]);
            }
        }
    }

    dp[m-1][n-1]
}

/// Approach 2: In-place modification (O(1) space)
pub fn min_path_sum_inplace(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    if m == 0 { return 0; }
    let n = grid[0].len();

    let mut grid = grid;

    // First row
    for j in 1..n {
        grid[0][j] += grid[0][j-1];
    }

    // First column
    for i in 1..m {
        grid[i][0] += grid[i-1][0];
    }

    // Rest of grid
    for i in 1..m {
        for j in 1..n {
            grid[i][j] += grid[i-1][j].min(grid[i][j-1]);
        }
    }

    grid[m-1][n-1]
}

/// Approach 3: Return path as well
pub fn min_path_sum_with_path(grid: Vec<Vec<i32>>) -> (i32, Vec<(usize, usize)>) {
    let m = grid.len();
    if m == 0 { return (0, vec![]); }
    let n = grid[0].len();

    let mut dp = vec![vec![0i32; n]; m];

    for i in 0..m {
        for j in 0..n {
            if i == 0 && j == 0 {
                dp[i][j] = grid[i][j];
            } else if i == 0 {
                dp[i][j] = dp[i][j-1] + grid[i][j];
            } else if j == 0 {
                dp[i][j] = dp[i-1][j] + grid[i][j];
            } else {
                dp[i][j] = grid[i][j] + dp[i-1][j].min(dp[i][j-1]);
            }
        }
    }

    // Reconstruct path
    let mut path = Vec::new();
    let mut i = m - 1;
    let mut j = n - 1;
    path.push((i, j));

    while i > 0 || j > 0 {
        if i == 0 {
            j -= 1;
        } else if j == 0 {
            i -= 1;
        } else if dp[i-1][j] < dp[i][j-1] {
            i -= 1;
        } else {
            j -= 1;
        }
        path.push((i, j));
    }

    path.reverse();
    (dp[m-1][n-1], path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let grid = vec![
            vec![1, 3, 1],
            vec![1, 5, 1],
            vec![4, 2, 1],
        ];
        // Path: 1→3→1→1→1 = 7
        assert_eq!(min_path_sum(grid), 7);
    }

    #[test]
    fn test_single_cell() {
        let grid = vec![vec![5]];
        assert_eq!(min_path_sum(grid), 5);
    }

    #[test]
    fn test_single_row() {
        let grid = vec![vec![1, 2, 3]];
        assert_eq!(min_path_sum(grid), 6);
    }

    #[test]
    fn test_single_column() {
        let grid = vec![vec![1], vec![2], vec![3]];
        assert_eq!(min_path_sum(grid), 6);
    }

    #[test]
    fn test_all_zeros() {
        let grid = vec![vec![0, 0], vec![0, 0]];
        assert_eq!(min_path_sum(grid), 0);
    }

    #[test]
    fn test_inplace_same_as_array() {
        let grid1 = vec![
            vec![1, 3, 1],
            vec![1, 5, 1],
            vec![4, 2, 1],
        ];
        let grid2 = grid1.clone();
        let arr = min_path_sum(grid1);
        let inplace = min_path_sum_inplace(grid2);
        assert_eq!(arr, inplace);
    }

    #[test]
    fn test_with_path() {
        let grid = vec![
            vec![1, 3, 1],
            vec![1, 5, 1],
            vec![4, 2, 1],
        ];
        let (sum, path) = min_path_sum_with_path(grid);
        assert_eq!(sum, 7);
        assert_eq!(path[0], (0, 0));
        assert_eq!(path[path.len() - 1], (2, 2));
        // Check path is contiguous
        for i in 1..path.len() {
            let (r1, c1) = path[i-1];
            let (r2, c2) = path[i];
            assert!((r1 == r2 && c1 + 1 == c2) || (c1 == c2 && r1 + 1 == r2));
        }
    }

    #[test]
    fn test_2x2() {
        let grid = vec![
            vec![1, 2],
            vec![3, 4],
        ];
        // Paths: 1→2→4=7 or 1→3→4=8, min=7
        assert_eq!(min_path_sum(grid), 7);
    }

    #[test]
    fn test_large_grid() {
        let grid = vec![vec![1; 10]; 10];
        // Only one path: 10 steps of 1 = 10
        assert_eq!(min_path_sum(grid), 10);
    }

    #[test]
    fn test_increasing_grid() {
        let grid = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        // Path: 1→2→3→6→9=21 (other paths: 1→4→5→6→9=25, etc.)
        assert_eq!(min_path_sum(grid), 21);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("045_min_path_sum_lc64 exercises - run tests with cargo test");
}
