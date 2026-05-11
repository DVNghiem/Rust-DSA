//! Exercises for Burst Balloons (LeetCode 312)
//!
//! # Topics Covered
//! - Interval dynamic programming
//! - Divide and conquer optimization
//! - Memoization
//! - Burst order optimization
//!
//! # Difficulty: Hard

use std::collections::HashMap;

/// Interval DP approach with bottom-up filling
pub fn max_coins_interval_dp(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n == 0 {
        return 0;
    }

    // Add virtual boundaries (1 at both ends)
    let mut extended = vec![1; n + 2];
    for i in 0..n {
        extended[i + 1] = nums[i];
    }

    let len = extended.len();
    let mut dp = vec![vec![0; len]; len];

    // dp[i][j] = max coins from interval (i, j) exclusive
    // i and j are boundaries, balloons are in (i, j)
    // We consider bursting balloon k last in (i, j)

    // len is the distance between boundaries
    // We need at least 2 distance (one balloon between)
    for gap in 2..len {
        for i in 0..len - gap {
            let j = i + gap;
            for k in i + 1..j {
                dp[i][j] = dp[i][j].max(
                    dp[i][k] + dp[k][j] + extended[i] * extended[k] * extended[j]
                );
            }
        }
    }

    dp[0][len - 1]
}

/// Memoized recursion with DFS
pub fn max_coins_memoization(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n == 0 {
        return 0;
    }

    let mut extended = vec![1; n + 2];
    for i in 0..n {
        extended[i + 1] = nums[i];
    }

    let mut memo = vec![vec![Option::<i32>::None; n + 2]; n + 2];
    dfs_memo(0, n + 1, &extended, &mut memo)
}

fn dfs_memo(i: usize, j: usize, nums: &[i32], memo: &mut Vec<Vec<Option<i32>>>) -> i32 {
    if i + 1 >= j {
        return 0;
    }

    if let Some(val) = memo[i][j] {
        return val;
    }

    let mut result = 0;
    for k in i + 1..j {
        let coins = dfs_memo(i, k, nums, memo)
            + dfs_memo(k, j, nums, memo)
            + nums[i] * nums[k] * nums[j];
        result = result.max(coins);
    }

    memo[i][j] = Some(result);
    result
}

/// HashMap-based memoization for comparison
pub fn max_coins_hash_memo(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n == 0 {
        return 0;
    }

    let mut extended = vec![1; n + 2];
    for i in 0..n {
        extended[i + 1] = nums[i];
    }

    let mut memo = HashMap::new();
    dfs_hash(0, n + 1, &extended, &mut memo)
}

fn dfs_hash(i: usize, j: usize, nums: &[i32], memo: &mut HashMap<(usize, usize), i32>) -> i32 {
    if i + 1 >= j {
        return 0;
    }

    if let Some(&val) = memo.get(&(i, j)) {
        return val;
    }

    let mut result = 0;
    for k in i + 1..j {
        let coins = dfs_hash(i, k, nums, memo)
            + dfs_hash(k, j, nums, memo)
            + nums[i] * nums[k] * nums[j];
        result = result.max(coins);
    }

    memo.insert((i, j), result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_example() {
        // From the problem: [3,1,5,8] -> 167
        let result = max_coins_interval_dp(vec![3, 1, 5, 8]);
        assert_eq!(result, 167);
    }

    #[test]
    fn test_empty() {
        let result = max_coins_interval_dp(vec![]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_single_balloon() {
        let result = max_coins_interval_dp(vec![3]);
        // coins = 1 * 3 * 1 = 3
        assert_eq!(result, 3);
    }

    #[test]
    fn test_two_balloons() {
        let result = max_coins_interval_dp(vec![3, 1]);
        // Burst 3: 1*3*1 = 3, then burst 1: 1*1*1 = 1, total = 4
        // Or burst 1: 1*1*3 = 3, then burst 3: 1*3*1 = 3, total = 6
        // Best: 6
        assert_eq!(result, 6);
    }

    #[test]
    fn test_three_balloons() {
        let result = max_coins_interval_dp(vec![1, 2, 3]);
        // Optimal: burst 2 first -> 1*2*3=6, then 1 -> 1*1*3=3, then 3 -> 1*3*1=3, total=12
        assert_eq!(result, 12);
    }

    #[test]
    fn test_all_ones() {
        let result = max_coins_interval_dp(vec![1, 1, 1, 1]);
        // With all 1s, order doesn't matter much
        assert_eq!(result, 16);
    }

    #[test]
    fn test_zeros() {
        let result = max_coins_interval_dp(vec![0, 0]);
        // Any balloon adjacent to 0 gives 0 coins
        assert_eq!(result, 0);
    }

    #[test]
    fn test_mixed_values() {
        let result = max_coins_interval_dp(vec![1, 2, 3, 4]);
        // Should produce some positive result
        assert!(result > 0);
    }

    #[test]
    fn test_large_values() {
        let result = max_coins_interval_dp(vec![10, 10, 10, 10]);
        assert!(result > 0);
    }

    // Memoization tests
    #[test]
    fn test_memo_basic() {
        let result = max_coins_memoization(vec![3, 1, 5, 8]);
        assert_eq!(result, 167);
    }

    #[test]
    fn test_memo_empty() {
        let result = max_coins_memoization(vec![]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_memo_single() {
        let result = max_coins_memoization(vec![3]);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_memo_two() {
        let result = max_coins_memoization(vec![3, 1]);
        assert_eq!(result, 6);
    }

    // Hash memo tests
    #[test]
    fn test_hash_memo_basic() {
        let result = max_coins_hash_memo(vec![3, 1, 5, 8]);
        assert_eq!(result, 167);
    }

    #[test]
    fn test_hash_memo_empty() {
        let result = max_coins_hash_memo(vec![]);
        assert_eq!(result, 0);
    }

    // Consistency tests
    #[test]
    fn test_all_approaches_same_result() {
        let test_cases = [
            vec![3, 1, 5, 8],
            vec![],
            vec![3],
            vec![3, 1],
            vec![1, 2, 3],
            vec![1, 1, 1, 1],
            vec![0, 0],
            vec![1, 2, 3, 4],
            vec![10, 10, 10, 10],
        ];

        for nums in test_cases.iter() {
            let result1 = max_coins_interval_dp(nums.clone());
            let result2 = max_coins_memoization(nums.clone());
            let result3 = max_coins_hash_memo(nums.clone());

            assert_eq!(result1, result2, "Interval DP vs Memoization differ for {:?}", nums);
            assert_eq!(result2, result3, "Memoization vs HashMemo differ for {:?}", nums);
        }
    }

    #[test]
    fn test_boundaries_matter() {
        // Value 1 at boundaries always
        let result = max_coins_interval_dp(vec![2]);
        assert_eq!(result, 2); // 1*2*1 = 2
    }

    #[test]
    fn test_negative_values() {
        let result = max_coins_interval_dp(vec![-1, -2, -3]);
        // Negative values can reduce coins
        assert!(result < 0);
    }

    #[test]
    fn test_five_balloons() {
        let result = max_coins_interval_dp(vec![1, 2, 3, 4, 5]);
        assert!(result > 0);
    }

    #[test]
    fn test_six_balloons() {
        let result = max_coins_interval_dp(vec![1, 2, 3, 4, 5, 6]);
        assert!(result > 0);
    }

    #[test]
    fn test_optimal_order_matters() {
        // [3, 1] - we computed 6 is optimal
        let result = max_coins_interval_dp(vec![3, 1]);
        assert_eq!(result, 6);

        // [1, 3] - should also give 6
        let result = max_coins_interval_dp(vec![1, 3]);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_palindrome_like_order() {
        // Optimal for [1, 2, 3] is 12
        let result = max_coins_interval_dp(vec![1, 2, 3]);
        assert_eq!(result, 12);
    }

    #[test]
    fn test_specific_case_from_reddit() {
        // Some specific cases for verification
        let result = max_coins_interval_dp(vec![2, 4, 8]);
        assert!(result > 0);
    }

    #[test]
    fn test_alternating_high_low() {
        let result = max_coins_interval_dp(vec![1, 10, 1, 10, 1]);
        assert!(result > 0);
    }

    #[test]
    fn test_first_last_burst_strategy() {
        // For [3, 1, 5, 8], bursting 1 first with neighbors 3 and 5: 3*1*5=15
        // Then bursting 3: 1*3*8=24
        // etc...
        let result = max_coins_interval_dp(vec![3, 1, 5, 8]);
        assert_eq!(result, 167);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("083_burst_balloons_lc312 exercises - run tests with cargo test");
}
