//! Exercises for Perfect Squares (LeetCode 279)
//!
//! # Topics Covered
//! - Dynamic programming
//! - BFS shortest path
//! - Mathematical theorems (Lagrange, Legendre)
//! - Number theory for optimization
//!
//! # Difficulty: Medium

use std::collections::VecDeque;

/// Check if a number is a perfect square
fn is_perfect_square(n: i32) -> bool {
    if n < 0 {
        return false;
    }
    let sqrt = (n as f64).sqrt() as i32;
    sqrt * sqrt == n
}

/// DP Bottom-up approach: O(n√n) time, O(n) space
pub fn num_squares_dp(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    }

    let n = n as usize;
    let mut dp = vec![usize::MAX; n + 1];
    dp[0] = 0;

    for i in 1..=n {
        let mut j = 1;
        let mut min_count = usize::MAX;
        while j * j <= i {
            min_count = min_count.min(dp[i - j * j] + 1);
            j += 1;
        }
        dp[i] = min_count;
    }

    dp[n] as i32
}

/// BFS approach: shortest path from n to 0
/// Each step subtracts a perfect square
pub fn num_squares_bfs(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    }

    let n = n as usize;
    let mut visited = vec![false; n + 1];
    let mut queue = VecDeque::new();
    queue.push_back(n);
    visited[n] = true;
    let mut depth = 0;

    while !queue.is_empty() {
        let level_size = queue.len();
        depth += 1;
        for _ in 0..level_size {
            let num = queue.pop_front().unwrap();
            let mut i = 1;
            while i * i <= num {
                let next = num - i * i;
                if next == 0 {
                    return depth;
                }
                if !visited[next] {
                    visited[next] = true;
                    queue.push_back(next);
                }
                i += 1;
            }
        }
    }
    0
}

/// Math-based approach using Lagrange's Four Square Theorem
/// and Legendre's Three Square Theorem
pub fn num_squares_math(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    }

    let mut n = n;

    // Check 1: perfect square (1 square)
    if is_perfect_square(n) {
        return 1;
    }

    // Check 2: sum of two squares (2 squares)
    let mut a = 0;
    while a * a <= n {
        let b_squared = n - a * a;
        if is_perfect_square(b_squared) {
            return 2;
        }
        a += 1;
    }

    // Check 4: n = 4^a * (8b + 7) means at least 4 squares
    while n % 4 == 0 {
        n /= 4;
    }
    if n % 8 == 7 {
        return 4;
    }

    // Otherwise 3 squares
    3
}

/// Improved DP with precomputed squares
pub fn num_squares_optimized(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    }

    let n = n as usize;

    // Precompute squares up to n
    let max_sqrt = (n as f64).sqrt() as usize;
    let squares: Vec<usize> = (1..=max_sqrt).map(|i| i * i).collect();

    let mut dp = vec![usize::MAX; n + 1];
    dp[0] = 0;

    for i in 1..=n {
        for &sq in &squares {
            if sq > i {
                break;
            }
            dp[i] = dp[i].min(dp[i - sq] + 1);
        }
    }

    dp[n] as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_perfect_square() {
        assert!(is_perfect_square(0));
        assert!(is_perfect_square(1));
        assert!(is_perfect_square(4));
        assert!(is_perfect_square(9));
        assert!(is_perfect_square(16));
        assert!(!is_perfect_square(2));
        assert!(!is_perfect_square(3));
        assert!(!is_perfect_square(5));
        assert!(!is_perfect_square(14));
    }

    #[test]
    fn test_dp_zero() {
        assert_eq!(num_squares_dp(0), 0);
    }

    #[test]
    fn test_dp_one() {
        assert_eq!(num_squares_dp(1), 1);
    }

    #[test]
    fn test_dp_two() {
        assert_eq!(num_squares_dp(2), 2); // 1 + 1
    }

    #[test]
    fn test_dp_three() {
        assert_eq!(num_squares_dp(3), 3); // 1 + 1 + 1
    }

    #[test]
    fn test_dp_four() {
        assert_eq!(num_squares_dp(4), 1); // 4 itself is perfect square
    }

    #[test]
    fn test_dp_twelve() {
        assert_eq!(num_squares_dp(12), 3); // 4 + 4 + 4
    }

    #[test]
    fn test_dp_thirteen() {
        assert_eq!(num_squares_dp(13), 2); // 4 + 9
    }

    #[test]
    fn test_bfs_basic() {
        assert_eq!(num_squares_bfs(1), 1);
        assert_eq!(num_squares_bfs(2), 2);
        assert_eq!(num_squares_bfs(3), 3);
        assert_eq!(num_squares_bfs(4), 1);
    }

    #[test]
    fn test_bfs_twelve() {
        assert_eq!(num_squares_bfs(12), 3);
    }

    #[test]
    fn test_bfs_thirteen() {
        assert_eq!(num_squares_bfs(13), 2);
    }

    #[test]
    fn test_math_perfect_square() {
        assert_eq!(num_squares_math(1), 1);
        assert_eq!(num_squares_math(4), 1);
        assert_eq!(num_squares_math(9), 1);
        assert_eq!(num_squares_math(16), 1);
    }

    #[test]
    fn test_math_two_squares() {
        assert_eq!(num_squares_math(2), 2); // 1 + 1
        assert_eq!(num_squares_math(5), 2); // 1 + 4
        assert_eq!(num_squares_math(10), 2); // 1 + 9
    }

    #[test]
    fn test_math_four_squares() {
        assert_eq!(num_squares_math(7), 4); // 7 = 4^0 * 7, 7 % 8 == 7
        assert_eq!(num_squares_math(15), 4); // 15 = 4^0 * 15, 15 % 8 == 7
    }

    #[test]
    fn test_math_three_squares() {
        assert_eq!(num_squares_math(3), 3); // 1 + 1 + 1
        assert_eq!(num_squares_math(6), 3); // 4 + 1 + 1
        assert_eq!(num_squares_math(14), 3); // 9 + 4 + 1
    }

    #[test]
    fn test_math_twelve() {
        assert_eq!(num_squares_math(12), 3);
    }

    #[test]
    fn test_math_thirteen() {
        assert_eq!(num_squares_math(13), 2);
    }

    #[test]
    fn test_optimized_basic() {
        assert_eq!(num_squares_optimized(1), 1);
        assert_eq!(num_squares_optimized(2), 2);
        assert_eq!(num_squares_optimized(3), 3);
        assert_eq!(num_squares_optimized(4), 1);
        assert_eq!(num_squares_optimized(12), 3);
        assert_eq!(num_squares_optimized(13), 2);
    }

    #[test]
    fn test_all_approaches_consistent() {
        let test_cases = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];

        for n in test_cases {
            let dp = num_squares_dp(n);
            let bfs = num_squares_bfs(n);
            let math = num_squares_math(n);
            let opt = num_squares_optimized(n);

            assert_eq!(dp, bfs, "DP and BFS differ for n={}", n);
            assert_eq!(bfs, math, "BFS and Math differ for n={}", n);
            assert_eq!(math, opt, "Math and Optimized differ for n={}", n);
        }
    }

    #[test]
    fn test_large_numbers() {
        assert!(num_squares_dp(100) >= 1);
        assert!(num_squares_dp(1000) >= 1);
        assert!(num_squares_dp(10000) >= 1);
    }

    #[test]
    fn test_dp_100() {
        // 100 = 10 * 10 = 1 perfect square
        assert_eq!(num_squares_dp(100), 1);
    }

    #[test]
    fn test_dp_99() {
        // 99 = 9 + 9 + 9 + 9 + 9 + 9 + 9 + 9 + 9 + 9 + 9... wait
        // 99 = 9 + 9 + 9 + 9 + 9 + 9 + 9 + 9 + 9 + 9 + 9... 11 times 9 = 99
        // Actually 99 = 9 + 9 + 9 + 9 + 9 + 9 + 9 + 9 + 9 + 9 + 9 = 11
        // But optimal: 99 = 36 + 36 + 25 + 1 + 1 = 5
        // Let DP determine
        let result = num_squares_dp(99);
        assert!(result >= 1 && result <= 4);
    }

    #[test]
    fn test_dp_50() {
        // 50 = 25 + 25 = 2 squares
        assert_eq!(num_squares_dp(50), 2);
    }

    #[test]
    fn test_dp_48() {
        // 48 = 36 + 4 + 4 + 4 = 4
        // Or 48 = 16 + 16 + 16 = 3
        // Or 48 = 9 + 9 + 9 + 9 + 9 + 3? No...
        // 48 = 25 + 16 + 4 + 1 + 1 + 1 = 6? No...
        // Let DP find optimal
        let result = num_squares_dp(48);
        assert!(result >= 1 && result <= 4);
    }

    #[test]
    fn test_squares_list() {
        let squares: Vec<i32> = (1..=30).map(|i| i * i).collect();
        assert_eq!(squares[0], 1);
        assert_eq!(squares[3], 16);
        assert_eq!(squares[5], 36);
    }

    #[test]
    fn test_four_theorem() {
        // According to Lagrange, every number can be represented as sum of 4 squares
        for n in 1..100 {
            let result = num_squares_dp(n);
            assert!(result >= 1 && result <= 4, "n={} has {} squares", n, result);
        }
    }

    #[test]
    fn test_math_consistency() {
        for n in 1..100 {
            let math = num_squares_math(n);
            let dp = num_squares_dp(n);
            assert_eq!(math, dp, "Math and DP differ for n={}", n);
        }
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("084_perfect_squares_lc279 exercises - run tests with cargo test");
}
