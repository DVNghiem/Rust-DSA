use std::collections::VecDeque;

/// Multi-source BFS approach
///
/// Start with all rotten oranges at time 0.
/// Each BFS level represents one minute.
/// Return minutes elapsed or -1 if impossible.
pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }

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
    if fresh == 0 {
        return 0;
    }

    // No rotten oranges - can't rot anything
    if queue.is_empty() {
        return -1;
    }

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

                // Check bounds
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

    // If fresh > 0, some oranges couldn't be rotted
    if fresh > 0 {
        -1
    } else {
        minutes
    }
}

/// Alternative implementation with explicit level tracking
pub fn oranges_rotting_v2(grid: Vec<Vec<i32>>) -> i32 {
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }

    let m = grid.len();
    let n = grid[0].len();
    let mut grid = grid;
    let mut queue: Vec<(usize, usize)> = Vec::new();
    let mut fresh = 0;

    // Find all rotten oranges, count fresh
    for i in 0..m {
        for j in 0..n {
            match grid[i][j] {
                2 => { queue.push((i, j)); }
                1 => { fresh += 1; }
                _ => {}
            }
        }
    }

    if fresh == 0 { return 0; }
    if queue.is_empty() { return -1; }

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut minutes = 0;

    // Process queue as BFS
    let mut head = 0;
    while head < queue.len() {
        let start = head;
        let end = queue.len();

        // Process all nodes at current level (same minute)
        while head < end {
            let (r, c) = queue[head];
            head += 1;

            for (dr, dc) in directions.iter() {
                let nr = r as isize + dr;
                let nc = c as isize + dc;

                if nr < 0 || nr >= m as isize || nc < 0 || nc >= n as isize {
                    continue;
                }

                let nr = nr as usize;
                let nc = nc as usize;

                if grid[nr][nc] == 1 {
                    grid[nr][nc] = 2;
                    fresh -= 1;
                    queue.push((nr, nc));
                }
            }
        }

        minutes += 1;

        if fresh == 0 {
            return minutes;
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_grid(v: Vec<&[i32]>) -> Vec<Vec<i32>> {
        v.iter().map(|row| row.to_vec()).collect()
    }

    #[test]
    fn test_basic_case() {
        let grid = create_grid(vec![
            &[2, 1, 1],
            &[1, 1, 0],
            &[0, 1, 1]
        ]);
        assert_eq!(oranges_rotting(grid), 4);
    }

    #[test]
    fn test_impossible_case() {
        let grid = create_grid(vec![
            &[2, 1, 1],
            &[0, 1, 1],
            &[1, 0, 1]
        ]);
        assert_eq!(oranges_rotting(grid), -1);
    }

    #[test]
    fn test_no_fresh_oranges() {
        let grid = create_grid(vec![
            &[2, 0],
            &[0, 0]
        ]);
        assert_eq!(oranges_rotting(grid), 0);
    }

    #[test]
    fn test_all_rotten() {
        let grid = create_grid(vec![
            &[2, 2],
            &[2, 2]
        ]);
        assert_eq!(oranges_rotting(grid), 0);
    }

    #[test]
    fn test_single_fresh() {
        let grid = create_grid(vec![
            &[1]
        ]);
        assert_eq!(oranges_rotting(grid), -1);
    }

    #[test]
    fn test_single_rotten() {
        let grid = create_grid(vec![
            &[2]
        ]);
        assert_eq!(oranges_rotting(grid), 0);
    }

    #[test]
    fn test_isolated_fresh() {
        let grid = create_grid(vec![
            &[2, 0, 1]
        ]);
        assert_eq!(oranges_rotting(grid), -1);
    }

    #[test]
    fn test_simple_case() {
        // 2 at (0,0), fresh at (0,1), fresh at (1,0)
        // After 1 minute: all become rotten
        let grid = create_grid(vec![
            &[2, 1],
            &[1, 0]
        ]);
        assert_eq!(oranges_rotting(grid), 1);
    }

    #[test]
    fn test_two_by_two() {
        let grid = create_grid(vec![
            &[2, 1],
            &[1, 1]
        ]);
        assert_eq!(oranges_rotting(grid), 1);
    }

    #[test]
    fn test_multiple_rotten_sources() {
        // Two rotten oranges far apart
        let grid = create_grid(vec![
            &[2, 0, 0],
            &[0, 0, 0],
            &[0, 0, 2]
        ]);
        // No fresh oranges can be reached
        assert_eq!(oranges_rotting(grid), -1);
    }

    #[test]
    fn test_line_configuration() {
        let grid = create_grid(vec![
            &[2, 1, 1, 1, 1]
        ]);
        // Rot spreads from left to right
        // 2 -> 1 (minute 1) -> 1 (minute 2) -> 1 (minute 3) -> 1 (minute 4)
        assert_eq!(oranges_rotting(grid), 4);
    }

    #[test]
    fn test_column_configuration() {
        let grid = create_grid(vec![
            &[2],
            &[1],
            &[1],
            &[1]
        ]);
        // Rot spreads downward
        assert_eq!(oranges_rotting(grid), 3);
    }

    #[test]
    fn test_both_versions_match() {
        let grid1 = create_grid(vec![
            &[2, 1, 1],
            &[1, 1, 0],
            &[0, 1, 1]
        ]);
        let grid2 = create_grid(vec![
            &[2, 1, 1],
            &[1, 1, 0],
            &[0, 1, 1]
        ]);
        assert_eq!(oranges_rotting(grid1), oranges_rotting_v2(grid2));
    }

    #[test]
    fn test_empty_grid() {
        let grid: Vec<Vec<i32>> = vec![];
        assert_eq!(oranges_rotting(grid), 0);
    }

    #[test]
    fn test_empty_row_grid() {
        let grid: Vec<Vec<i32>> = vec![vec![]];
        assert_eq!(oranges_rotting(grid), 0);
    }

    #[test]
    fn test_only_fresh() {
        let grid = create_grid(vec![
            &[1, 1],
            &[1, 1]
        ]);
        assert_eq!(oranges_rotting(grid), -1);
    }

    #[test]
    fn test_rotten_in_corner() {
        let grid = create_grid(vec![
            &[2, 1, 1, 1, 1]
        ]);
        // 2 at (0,0), 4 fresh to the right
        assert_eq!(oranges_rotting(grid), 4);
    }

    #[test]
    fn test_rotten_in_center() {
        let grid = create_grid(vec![
            &[1, 1, 1],
            &[1, 2, 1],
            &[1, 1, 1]
        ]);
        // All 8 fresh around the center rotten
        assert_eq!(oranges_rotting(grid), 1);
    }

    #[test]
    fn test_large_grid() {
        let mut grid = vec![vec![0; 100]; 100];
        grid[0][0] = 2;
        grid[50][50] = 2;
        grid[1][0] = 1;
        grid[0][1] = 1;
        grid[50][51] = 1;
        grid[51][50] = 1;
        // Should complete without issue
        let result = oranges_rotting(grid);
        assert!(result >= 0);
    }

    #[test]
    fn test_diagonal_not_spread() {
        // Diagonal adjacency doesn't count
        let grid = create_grid(vec![
            &[2, 0],
            &[0, 1]
        ]);
        // Fresh at (1,1) cannot be reached
        assert_eq!(oranges_rotting(grid), -1);
    }

    #[test]
    fn test_cascade_spread() {
        // Fresh -> Fresh -> Rotten chain
        let grid = create_grid(vec![
            &[1, 1, 2]
        ]);
        // (2,0) rots (1,0) at minute 1, which rots (0,0) at minute 2
        assert_eq!(oranges_rotting(grid), 2);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("032_rotten_oranges_lc994 exercises - run tests with cargo test");
}
