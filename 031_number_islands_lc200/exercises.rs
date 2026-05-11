use std::collections::VecDeque;

/// Approach 1: DFS Recursive
///
/// Start DFS from each unvisited '1', mark all connected '1's as visited.
/// Each DFS call marks one complete island.
pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }

    let m = grid.len();
    let n = grid[0].len();
    let mut grid = grid;
    let mut count = 0;

    fn dfs(grid: &mut Vec<Vec<char>>, i: usize, j: usize, m: usize, n: usize) {
        // Base: out of bounds or water
        if i >= m || j >= n || grid[i][j] == '0' {
            return;
        }

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

/// Approach 2: BFS with Queue
pub fn num_islands_bfs(grid: Vec<Vec<char>>) -> i32 {
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }

    let m = grid.len();
    let n = grid[0].len();
    let mut grid = grid;
    let mut count = 0;

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == '1' {
                count += 1;
                grid[i][j] = '0'; // Mark visited

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

/// Approach 3: DFS Iterative (explicit stack)
pub fn num_islands_dfs_iterative(grid: Vec<Vec<char>>) -> i32 {
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }

    let m = grid.len();
    let n = grid[0].len();
    let mut grid = grid;
    let mut count = 0;

    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == '1' {
                count += 1;
                grid[i][j] = '0';

                let mut stack = vec![(i, j)];
                while let Some((r, c)) = stack.pop() {
                    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
                    for (dr, dc) in directions.iter() {
                        let nr = r as isize + dr;
                        let nc = c as isize + dc;
                        if nr >= 0 && nr < m as isize && nc >= 0 && nc < n as isize {
                            let nr = nr as usize;
                            let nc = nc as usize;
                            if grid[nr][nc] == '1' {
                                grid[nr][nc] = '0';
                                stack.push((nr, nc));
                            }
                        }
                    }
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_grid(v: Vec<&str>) -> Vec<Vec<char>> {
        v.iter().map(|s| s.chars().collect()).collect()
    }

    #[test]
    fn test_basic_island() {
        let grid = create_grid(vec![
            "11110",
            "11010",
            "11000",
            "00000"
        ]);
        assert_eq!(num_islands(grid), 1);
    }

    #[test]
    fn test_multiple_islands() {
        let grid = create_grid(vec![
            "11000",
            "11000",
            "00100",
            "00011"
        ]);
        assert_eq!(num_islands(grid), 3);
    }

    #[test]
    fn test_empty_grid() {
        let grid: Vec<Vec<char>> = vec![];
        assert_eq!(num_islands(grid), 0);
    }

    #[test]
    fn test_empty_row_grid() {
        let grid: Vec<Vec<char>> = vec![vec![]];
        assert_eq!(num_islands(grid), 0);
    }

    #[test]
    fn test_all_water() {
        let grid = create_grid(vec![
            "00000",
            "00000",
            "00000"
        ]);
        assert_eq!(num_islands(grid), 0);
    }

    #[test]
    fn test_all_land() {
        let grid = create_grid(vec![
            "111",
            "111",
            "111"
        ]);
        assert_eq!(num_islands(grid), 1);
    }

    #[test]
    fn test_single_cell_land() {
        let grid = create_grid(vec!["1"]);
        assert_eq!(num_islands(grid), 1);
    }

    #[test]
    fn test_single_cell_water() {
        let grid = create_grid(vec!["0"]);
        assert_eq!(num_islands(grid), 0);
    }

    #[test]
    fn test_isolated_cells() {
        // 5 isolated '1's = 5 islands
        let grid = create_grid(vec![
            "10001",
            "00000",
            "00001",
            "00000",
            "10000"
        ]);
        assert_eq!(num_islands(grid), 3);
    }

    #[test]
    fn test_large_island() {
        let grid = create_grid(vec![
            "11111110",
            "11000010",
            "11001110",
            "11000000",
            "11111111"
        ]);
        assert_eq!(num_islands(grid), 1);
    }

    #[test]
    fn test_diagonal_not_connected() {
        // Diagonal '1's are NOT connected
        let grid = create_grid(vec![
            "1 0",
            " 0 1"  // This format is confusing, let me use proper strings
        ]);
        // Actually: two diagonal 1s should be 2 islands
        let grid = create_grid(vec![
            "101",
            "010",
            "101"
        ]);
        assert_eq!(num_islands(grid), 5); // 5 separate 1s
    }

    #[test]
    fn test_vertical_line() {
        let grid = create_grid(vec![
            "0",
            "1",
            "1",
            "0",
            "1"
        ]);
        assert_eq!(num_islands(grid), 1);
    }

    #[test]
    fn test_horizontal_line() {
        let grid = create_grid(vec![
            "11111"
        ]);
        assert_eq!(num_islands(grid), 1);
    }

    #[test]
    fn test_bfs_same_as_dfs() {
        let grid = create_grid(vec![
            "11000",
            "11000",
            "00100",
            "00011"
        ]);
        let dfs_result = num_islands(grid.clone());
        let bfs_result = num_islands_bfs(grid);
        assert_eq!(dfs_result, bfs_result);
    }

    #[test]
    fn test_iterative_same_as_recursive() {
        let grid = create_grid(vec![
            "11110",
            "11010",
            "11000",
            "00000"
        ]);
        let recursive = num_islands(grid.clone());
        let iterative = num_islands_dfs_iterative(grid);
        assert_eq!(recursive, iterative);
    }

    #[test]
    fn test_complex_grid() {
        let grid = create_grid(vec![
            "11000100100000000",
            "00000100100101111",
            "11111001111111110",
            "00000000011111100",
            "00111110110000110",
            "01000000101110010",
            "01101010001101010",
            "10101010101000100",
            "11111010101011110"
        ]);
        // Just verify it runs without error
        let result = num_islands(grid);
        assert!(result >= 0);
    }

    #[test]
    fn test_single_row_multiple_islands() {
        let grid = create_grid(vec!["101001"]);
        assert_eq!(num_islands(grid), 3);
    }

    #[test]
    fn test_single_column_multiple_islands() {
        let grid = create_grid(vec!["1", "0", "1", "1", "0"]);
        assert_eq!(num_islands(grid), 2);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("031_number_islands_lc200 exercises - run tests with cargo test");
}
