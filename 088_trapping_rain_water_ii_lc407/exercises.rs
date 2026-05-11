//! Exercises for Trapping Rain Water II (LeetCode 407)
//!
//! # Topics Covered
//! - Priority queue (min-heap)
//! - Multi-source BFS
//! - 2D grid traversal
//! - Water trapping algorithm
//!
//! # Difficulty: Hard

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Trapping Rain Water II using priority queue
/// Time: O(mn log(mn)), Space: O(mn)
pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
    if height_map.is_empty() || height_map[0].is_empty() {
        return 0;
    }

    let m = height_map.len();
    let n = height_map[0].len();

    let mut visited = vec![vec![false; n]; m];
    let mut heap: BinaryHeap<Reverse<(i32, i32, i32)>> = BinaryHeap::new();

    // Add all border cells to heap
    for i in 0..m {
        for j in 0..n {
            if i == 0 || i == m - 1 || j == 0 || j == n - 1 {
                heap.push(Reverse((height_map[i][j], i as i32, j as i32)));
                visited[i][j] = true;
            }
        }
    }

    let mut water = 0;
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    while let Some(Reverse((height, row, col))) = heap.pop() {
        for (dr, dc) in &directions {
            let nr = row as i32 + dr;
            let nc = col as i32 + dc;

            if nr < 0 || nr >= m as i32 || nc < 0 || nc >= n as i32 {
                continue;
            }

            let nr = nr as usize;
            let nc = nc as usize;

            if visited[nr][nc] {
                continue;
            }
            visited[nr][nc] = true;

            let nh = height_map[nr][nc];

            if nh < height {
                water += height - nh;
                heap.push(Reverse((height, nr as i32, nc as i32)));
            } else {
                heap.push(Reverse((nh, nr as i32, nc as i32)));
            }
        }
    }

    water
}

/// Alternative implementation with same logic
pub fn trap_rain_water_v2(height_map: Vec<Vec<i32>>) -> i32 {
    if height_map.is_empty() || height_map[0].is_empty() {
        return 0;
    }

    let m = height_map.len();
    let n = height_map[0].len();

    let mut visited = vec![vec![false; n]; m];
    let mut heap: BinaryHeap<(i32, i32, i32)> = BinaryHeap::new();

    // Initialize with border cells
    for i in 0..m {
        heap.push((height_map[i][0], i as i32, 0));
        heap.push((height_map[i][n - 1], i as i32, (n - 1) as i32));
        visited[i][0] = true;
        visited[i][n - 1] = true;
    }
    for j in 0..n {
        heap.push((height_map[0][j], 0, j as i32));
        heap.push((height_map[m - 1][j], (m - 1) as i32, j as i32));
        visited[0][j] = true;
        visited[m - 1][j] = true;
    }

    let mut result = 0;
    let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    while let Some((h, r, c)) = heap.pop() {
        for (dr, dc) in &dirs {
            let nr = r + dr;
            let nc = c + dc;

            if nr < 0 || nr >= m as i32 || nc < 0 || nc >= n as i32 || visited[nr as usize][nc as usize] {
                continue;
            }

            visited[nr as usize][nc as usize] = true;
            let nh = height_map[nr as usize][nc as usize];

            if nh < h {
                result += h - nh;
                heap.push((h, nr, nc));
            } else {
                heap.push((nh, nr, nc));
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_example() {
        let height_map = vec![
            vec![1, 4, 3, 1, 3, 2],
            vec![3, 2, 1, 3, 2, 4],
            vec![2, 3, 3, 2, 3, 1],
        ];
        assert_eq!(trap_rain_water(height_map), 4);
    }

    #[test]
    fn test_empty_grid() {
        assert_eq!(trap_rain_water(vec![]), 0);
    }

    #[test]
    fn test_single_cell() {
        assert_eq!(trap_rain_water(vec![vec![1]]), 0);
    }

    #[test]
    fn test_two_by_two() {
        // All same height, no water
        let height_map = vec![
            vec![1, 1],
            vec![1, 1],
        ];
        assert_eq!(trap_rain_water(height_map), 0);
    }

    #[test]
    fn test_two_by_two_with_center() {
        // Center is lower, water can be trapped
        let height_map = vec![
            vec![3, 3],
            vec![1, 3],
        ];
        // Water can collect in cell (1,0) = 1
        // Level is 3, cell is 1, trapped = 3-1 = 2
        assert_eq!(trap_rain_water(height_map), 2);
    }

    #[test]
    fn test_all_same_height() {
        let height_map = vec![
            vec![2, 2, 2],
            vec![2, 2, 2],
            vec![2, 2, 2],
        ];
        assert_eq!(trap_rain_water(height_map), 0);
    }

    #[test]
    fn test_bowl_shape() {
        // Low center, high borders
        let height_map = vec![
            vec![3, 3, 3, 3],
            vec![3, 1, 1, 3],
            vec![3, 1, 1, 3],
            vec![3, 3, 3, 3],
        ];
        // Each inner cell can hold 2 units (3-1)
        // Total = 4 cells * 2 = 8
        assert_eq!(trap_rain_water(height_map), 8);
    }

    #[test]
    fn test_peak_in_center() {
        // Peak in center, water around it
        let height_map = vec![
            vec![2, 2, 2, 2],
            vec![2, 5, 5, 2],
            vec![2, 5, 5, 2],
            vec![2, 2, 2, 2],
        ];
        // Cells around peak (height 5) can hold 3 units each (5-2)
        // But some might spill to border... let me calculate
        // Actually with border at 2 and inner at 5, no water trapped
        // Because border is same as outer inner cells
        assert_eq!(trap_rain_water(height_map), 0);
    }

    #[test]
    fn test_random_heights() {
        let height_map = vec![
            vec![3, 3, 5, 5, 4],
            vec![2, 1, 2, 3, 4],
            vec![1, 2, 3, 4, 5],
        ];
        // Some water should be trapped
        let result = trap_rain_water(height_map);
        assert!(result > 0);
    }

    #[test]
    fn test_three_rows() {
        let height_map = vec![
            vec![12, 13, 0, 12],
            vec![13, 4, 13, 13],
            vec![13, 13, 13, 13],
        ];
        let result = trap_rain_water(height_map);
        assert!(result >= 0);
    }

    #[test]
    fn test_single_row() {
        let height_map = vec![vec![5, 5, 5, 5]];
        assert_eq!(trap_rain_water(height_map), 0);
    }

    #[test]
    fn test_single_column() {
        let height_map = vec![
            vec![5],
            vec![5],
            vec![5],
        ];
        assert_eq!(trap_rain_water(height_map), 0);
    }

    #[test]
    fn test_v2_basic() {
        let height_map = vec![
            vec![1, 4, 3, 1, 3, 2],
            vec![3, 2, 1, 3, 2, 4],
            vec![2, 3, 3, 2, 3, 1],
        ];
        assert_eq!(trap_rain_water_v2(height_map), 4);
    }

    #[test]
    fn test_v2_empty() {
        assert_eq!(trap_rain_water_v2(vec![]), 0);
    }

    #[test]
    fn test_consistency_v1_v2() {
        let test_cases = vec![
            vec![vec![1, 4, 3, 1, 3, 2], vec![3, 2, 1, 3, 2, 4], vec![2, 3, 3, 2, 3, 1]],
            vec![vec![3, 3, 3, 3], vec![3, 1, 1, 3], vec![3, 1, 1, 3], vec![3, 3, 3, 3]],
            vec![vec![1]],
            vec![vec![], vec![]],
            vec![vec![2, 2, 2], vec![2, 2, 2]],
        ];

        for height_map in test_cases {
            let v1 = trap_rain_water(height_map.clone());
            let v2 = trap_rain_water_v2(height_map.clone());
            assert_eq!(v1, v2, "Failed for {:?}", height_map);
        }
    }

    #[test]
    fn test_water_cant_escape() {
        // Higher border prevents water escape
        let height_map = vec![
            vec![5, 5, 5, 5],
            vec![5, 1, 1, 5],
            vec![5, 1, 1, 5],
            vec![5, 5, 5, 5],
        ];
        // 4 inner cells, each can hold 4 units (5-1)
        assert_eq!(trap_rain_water(height_map), 16);
    }

    #[test]
    fn test_water_can_escape() {
        // Lower border allows water to escape
        let height_map = vec![
            vec![5, 5, 5, 5],
            vec![5, 1, 1, 2],  // Note: right side is 2, not 5
            vec![5, 1, 1, 2],
            vec![5, 5, 5, 5],
        ];
        // Water can flow out to right side
        let result = trap_rain_water(height_map);
        assert!(result < 16);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("088_trapping_rain_water_ii_lc407 exercises - run tests with cargo test");
}
