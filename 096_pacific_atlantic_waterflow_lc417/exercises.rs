//! Exercises for Pacific Atlantic Water Flow (LeetCode 417)
//!
//! # Topics Covered
//! - Multi-source BFS
//! - Matrix traversal
//! - Water flow simulation
//! - Cell reachability
//!
//! # Difficulty: Medium

use std::collections::VecDeque;

/// Pacific Atlantic Water Flow using multi-source BFS
pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if heights.is_empty() || heights[0].is_empty() {
        return vec![];
    }

    let m = heights.len();
    let n = heights[0].len();

    let mut pacific = vec![vec![false; n]; m];
    let mut atlantic = vec![vec![false; n]; m];

    let directions: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    // BFS from Pacific (top row and left column)
    let mut queue = VecDeque::new();
    for j in 0..n { pacific[0][j] = true; queue.push_back((0, j)); }
    for i in 0..m { pacific[i][0] = true; queue.push_back((i, 0)); }

    while let Some((i, j)) = queue.pop_front() {
        for (dr, dc) in &directions {
            let ni = i as i32 + *dr;
            let nj = j as i32 + *dc;
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
            let ni = i as i32 + *dr;
            let nj = j as i32 + *dc;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_example() {
        let heights = vec![
            vec![1, 2, 2, 3, 5],
            vec![3, 2, 3, 4, 4],
            vec![2, 4, 5, 3, 1],
            vec![6, 7, 1, 4, 5],
            vec![5, 1, 1, 2, 4],
        ];
        let result = pacific_atlantic(heights);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_empty_grid() {
        let heights: Vec<Vec<i32>> = vec![];
        let result = pacific_atlantic(heights);
        assert!(result.is_empty());
    }

    #[test]
    fn test_single_cell() {
        let heights = vec![vec![1]];
        let result = pacific_atlantic(heights);
        assert_eq!(result, vec![vec![0, 0]]);
    }

    #[test]
    fn test_all_same_height() {
        let heights = vec![vec![1, 1], vec![1, 1]];
        let result = pacific_atlantic(heights);
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_increasing_height() {
        let heights = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let result = pacific_atlantic(heights);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_decreasing_height() {
        let heights = vec![vec![9, 8, 7], vec![6, 5, 4], vec![3, 2, 1]];
        let result = pacific_atlantic(heights);
        assert!(!result.is_empty());
    }
}
// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("$name exercises - run tests with cargo test");
}
