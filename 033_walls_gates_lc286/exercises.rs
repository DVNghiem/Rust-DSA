use std::collections::VecDeque;

/// INF represents an empty room (2^31 - 1)
const INF: i32 = 2147483647;

/// Multi-source BFS from all gates
///
/// Start BFS from all gates (distance 0).
/// Each step increases distance by 1.
/// Only fill cells that haven't been visited (remain INF).
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

            // Bounds check
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

/// Alternative: BFS with explicit distance tracking
pub fn walls_and_gates_v2(grid: &mut Vec<Vec<i32>>) {
    let m = grid.len();
    if m == 0 { return; }
    let n = grid[0].len();

    let mut queue: Vec<(usize, usize, i32)> = Vec::new();

    // Initialize
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 0 {
                queue.push((i, j, 0));
            }
        }
    }

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
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

        for (dr, dc) in directions.iter() {
            let nr = r as isize + dr;
            let nc = c as isize + dc;

            if nr < 0 || nr >= m as isize || nc < 0 || nc >= n as isize {
                continue;
            }

            let nr = nr as usize;
            let nc = nc as usize;

            if !visited[nr][nc] && grid[nr][nc] == INF {
                grid[nr][nc] = dist + 1;
                visited[nr][nc] = true;
                queue.push((nr, nc, dist + 1));
            }
        }
    }
}

/// Check if grid has any unreachable rooms (rooms still at INF)
pub fn count_unreachable(grid: &[Vec<i32>]) -> i32 {
    grid.iter()
        .flat_map(|row| row.iter())
        .filter(|&&v| v == INF)
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_grid(v: Vec<&[i32]>) -> Vec<Vec<i32>> {
        v.iter().map(|row| row.to_vec()).collect()
    }

    #[test]
    fn test_basic_fill() {
        let mut grid = create_grid(vec![
            &[INF, -1, 0, INF],
            &[INF, INF, INF, -1],
            &[INF, -1, INF, -1],
            &[0, -1, INF, INF]
        ]);
        walls_and_gates(&mut grid);

        // Expected distances:
        // [3, -1, 0, 1]
        // [2,  2, 1, -1]
        // [1, -1, 2, -1]
        // [0, -1, 3,  4]
        assert_eq!(grid[0][0], 3);
        assert_eq!(grid[0][2], 0);
        assert_eq!(grid[1][2], 1);
        assert_eq!(grid[3][0], 0);
    }

    #[test]
    fn test_no_gates() {
        let mut grid = create_grid(vec![
            &[INF, INF],
            &[INF, INF]
        ]);
        walls_and_gates(&mut grid);

        // All should remain INF
        assert_eq!(grid[0][0], INF);
        assert_eq!(grid[1][1], INF);
    }

    #[test]
    fn test_all_gates() {
        let mut grid = create_grid(vec![
            &[0, 0],
            &[0, 0]
        ]);
        walls_and_gates(&mut grid);

        // All remain 0
        assert_eq!(grid[0][0], 0);
        assert_eq!(grid[1][1], 0);
    }

    #[test]
    fn test_partially_blocked() {
        let mut grid = create_grid(vec![
            &[INF, -1],
            &[-1, INF]
        ]);
        walls_and_gates(&mut grid);

        // (0,0) blocked, (1,1) blocked
        assert_eq!(grid[0][0], INF);
        assert_eq!(grid[1][1], INF);
    }

    #[test]
    fn test_single_gate() {
        let mut grid = create_grid(vec![
            &[INF, INF, INF],
            &[INF, 0, INF],
            &[INF, INF, INF]
        ]);
        walls_and_gates(&mut grid);

        // All cells distance 1 from center
        assert_eq!(grid[0][0], 2);
        assert_eq!(grid[0][1], 1);
        assert_eq!(grid[1][0], 1);
        assert_eq!(grid[1][2], 1);
    }

    #[test]
    fn test_gate_corner() {
        let mut grid = create_grid(vec![
            &[0, INF, INF],
            &[INF, INF, INF],
            &[INF, INF, INF]
        ]);
        walls_and_gates(&mut grid);

        // Gate at (0,0)
        assert_eq!(grid[0][0], 0);
        assert_eq!(grid[0][1], 1);
        assert_eq!(grid[1][0], 1);
        assert_eq!(grid[1][1], 2);
    }

    #[test]
    fn test_multiple_gates_same_distance() {
        let mut grid = create_grid(vec![
            &[0, INF, 0],
            &[INF, INF, INF],
            &[0, INF, 0]
        ]);
        walls_and_gates(&mut grid);

        // Four corners are gates
        // Center (1,1) should be 2 (distance to any corner)
        assert_eq!(grid[1][1], 2);
    }

    #[test]
    fn test_dense_walls() {
        let mut grid = create_grid(vec![
            &[0, -1, INF],
            &[-1, -1, -1],
            &[INF, -1, INF]
        ]);
        walls_and_gates(&mut grid);

        // Only reachable cell from (0,0) is (0,2) with distance 2
        assert_eq!(grid[0][2], 2);
        // (2,0) and (2,2) blocked by walls
        assert_eq!(grid[2][0], INF);
        assert_eq!(grid[2][2], INF);
    }

    #[test]
    fn test_large_grid() {
        let mut grid = vec![vec![INF; 100]; 100];
        grid[0][0] = 0;
        grid[50][50] = 0;
        walls_and_gates(&mut grid);
        assert!(grid[25][25] < INF);
    }

    #[test]
    fn test_both_versions_same() {
        let mut grid1 = create_grid(vec![
            &[INF, -1, 0, INF],
            &[INF, INF, INF, -1],
            &[INF, -1, INF, -1],
            &[0, -1, INF, INF]
        ]);
        let mut grid2 = grid1.clone();
        walls_and_gates(&mut grid1);
        walls_and_gates_v2(&mut grid2);

        assert_eq!(grid1, grid2);
    }

    #[test]
    fn test_count_unreachable() {
        let grid = create_grid(vec![
            &[INF, -1, 0],
            &[INF, INF, -1],
            &[-1, INF, INF]
        ]);
        // Only (2,2) should be unreachable
        assert_eq!(count_unreachable(&grid), 1);
    }

    #[test]
    fn test_no_walls() {
        let mut grid = create_grid(vec![
            &[0, INF, INF],
            &[INF, INF, INF],
            &[INF, INF, 0]
        ]);
        walls_and_gates(&mut grid);

        // Should fill symmetric distances
        assert_eq!(grid[0][1], 1);
        assert_eq!(grid[0][2], 2);
        assert_eq!(grid[1][0], 1);
        assert_eq!(grid[1][1], 2);
        assert_eq!(grid[2][0], 2);
        assert_eq!(grid[2][1], 1);
    }

    #[test]
    fn test_single_cell_gate() {
        let mut grid = create_grid(vec![[0]]);
        walls_and_gates(&mut grid);
        assert_eq!(grid[0][0], 0);
    }

    #[test]
    fn test_single_cell_empty() {
        let mut grid = create_grid(vec![[INF]]);
        walls_and_gates(&mut grid);
        assert_eq!(grid[0][0], INF);
    }

    #[test]
    fn test_line_configuration() {
        let mut grid = create_grid(vec![
            &[0, INF, INF, INF, INF]
        ]);
        walls_and_gates(&mut grid);

        assert_eq!(grid[0][0], 0);
        assert_eq!(grid[0][1], 1);
        assert_eq!(grid[0][2], 2);
        assert_eq!(grid[0][3], 3);
        assert_eq!(grid[0][4], 4);
    }

    #[test]
    fn test_multiple_gates_walls() {
        let mut grid = create_grid(vec![
            &[0, -1, INF],
            &[INF, INF, 0],
            &[-1, INF, -1]
        ]);
        walls_and_gates(&mut grid);

        // (0,2) should be distance 1 from gate at (0,0)? No, wall at (0,1)
        // Distance through (1,2) = 2
        assert_eq!(grid[0][2], 2);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("Walls and Gates exercises - run tests with cargo test");
}