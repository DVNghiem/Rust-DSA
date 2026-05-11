/// Approach 1: DP with O(1) space (optimal)
///
/// dp[i] = max(dp[i-1], dp[i-2] + nums[i])
/// Only need previous two values.
pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let n = nums.len();
    if n == 1 {
        return nums[0];
    }

    let mut prev2 = 0;  // dp[i-2]
    let mut prev1 = nums[0];  // dp[i-1] = dp[0]

    for i in 1..n {
        let current = std::cmp::max(prev1, prev2 + nums[i]);
        prev2 = prev1;
        prev1 = current;
    }

    prev1
}

/// Approach 2: DP with full array
pub fn rob_with_array(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let n = nums.len();
    if n == 1 {
        return nums[0];
    }

    let mut dp = vec![0i32; n];
    dp[0] = nums[0];
    dp[1] = std::cmp::max(nums[0], nums[1]);

    for i in 2..n {
        dp[i] = std::cmp::max(dp[i - 1], dp[i - 2] + nums[i]);
    }

    dp[n - 1]
}

/// Approach 3: Recursive with memoization
pub fn rob_recursive(nums: Vec<i32>) -> i32 {
    fn helper(nums: &[i32], i: usize, memo: &mut Vec<i32>) -> i32 {
        if i == 0 {
            return nums[0];
        }
        if i == 1 {
            return std::cmp::max(nums[0], nums[1]);
        }
        if memo[i] != 0 {
            return memo[i];
        }
        memo[i] = std::cmp::max(
            helper(nums, i - 1, memo),
            helper(nums, i - 2, memo) + nums[i],
        );
        memo[i]
    }

    if nums.is_empty() {
        return 0;
    }
    let n = nums.len();
    if n == 1 {
        return nums[0];
    }

    let mut memo = vec![0i32; n];
    helper(&nums, n - 1, &mut memo)
}

/// Approach 4: Track which houses are robbed
pub fn rob_with_path(nums: Vec<i32>) -> (i32, Vec<i32>) {
    if nums.is_empty() {
        return (0, vec![]);
    }

    let n = nums.len();
    if n == 1 {
        return (nums[0], vec![0]);
    }

    // dp[i] = max money up to house i
    let mut dp = vec![0i32; n];
    let mut prev_choice = vec![0i32; n]; // 0 = skip, 1 = take

    dp[0] = nums[0];
    prev_choice[0] = 1;

    dp[1] = std::cmp::max(nums[0], nums[1]);
    prev_choice[1] = if nums[1] > nums[0] { 1 } else { 0 };

    for i in 2..n {
        if dp[i - 1] > dp[i - 2] + nums[i] {
            dp[i] = dp[i - 1];
            prev_choice[i] = 0; // skipped
        } else {
            dp[i] = dp[i - 2] + nums[i];
            prev_choice[i] = 1; // took
        }
    }

    // Reconstruct path
    let mut path = Vec::new();
    let mut i = n - 1;
    while i > 0 {
        if prev_choice[i] == 1 {
            path.push(i as i32);
            i -= 2;
        } else {
            i -= 1;
        }
    }
    if i == 0 && prev_choice[0] == 1 {
        path.push(0);
    }

    path.reverse();
    (dp[n - 1], path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        // [2,7,9,3,1] -> 12 (rob 0, 2, 4)
        let nums = vec![2, 7, 9, 3, 1];
        assert_eq!(rob(nums), 12);
    }

    #[test]
    fn test_empty() {
        let nums: Vec<i32> = vec![];
        assert_eq!(rob(nums), 0);
    }

    #[test]
    fn test_single_house() {
        let nums = vec![5];
        assert_eq!(rob(nums), 5);
    }

    #[test]
    fn test_two_houses() {
        let nums = vec![2, 3];
        assert_eq!(rob(nums), 3);
    }

    #[test]
    fn test_all_same() {
        let nums = vec![1, 1, 1, 1];
        // Rob houses 0 and 2 OR 1 and 3
        assert_eq!(rob(nums), 2);
    }

    #[test]
    fn test_alternating_high_low() {
        let nums = vec![1, 2, 3, 1];
        // Rob 0 and 2 = 1 + 3 = 4, or 1 and 3 = 2 + 1 = 3
        assert_eq!(rob(nums), 4);
    }

    #[test]
    fn test_descending() {
        let nums = vec![5, 4, 3, 2, 1];
        // Best is houses 0 and 2 = 5 + 3 = 8
        assert_eq!(rob(nums), 8);
    }

    #[test]
    fn test_ascending() {
        let nums = vec![1, 2, 3, 4, 5];
        // Best is houses 0, 2, 4 = 1 + 3 + 5 = 9
        assert_eq!(rob(nums), 9);
    }

    #[test]
    fn test_large_values() {
        let nums = vec![100, 200, 300, 400, 500];
        // Best: 100 + 300 + 500 = 900 OR 200 + 400 = 600
        assert_eq!(rob(nums), 900);
    }

    #[test]
    fn test_all_array_same_as_recursive() {
        let nums = vec![2, 7, 9, 3, 1];
        let array = rob_with_array(nums.clone());
        let recursive = rob_recursive(nums.clone());
        assert_eq!(array, recursive);
    }

    #[test]
    fn test_two_methods_same() {
        let nums = vec![1, 2, 3, 4, 5];
        let dp = rob(nums.clone());
        let arr = rob_with_array(nums);
        assert_eq!(dp, arr);
    }

    #[test]
    fn test_path_reconstruction() {
        let nums = vec![2, 7, 9, 3, 1];
        let (max_money, path) = rob_with_path(nums);
        assert_eq!(max_money, 12);
        // Path should be indices [0, 2, 4] or similar valid path
        assert_eq!(path.len(), 3);
        // Check no adjacent indices in path
        for i in 0..path.len() {
            if i > 0 {
                assert!(path[i] - path[i - 1] >= 2);
            }
        }
    }

    #[test]
    fn test_path_single_house() {
        let (money, path) = rob_with_path(vec![5]);
        assert_eq!(money, 5);
        assert_eq!(path, vec![0]);
    }

    #[test]
    fn test_path_two_houses() {
        let (money, path) = rob_with_path(vec![2, 3]);
        assert_eq!(money, 3);
        // Should pick house 1 (value 3)
        assert!(path.contains(&1));
    }

    #[test]
    fn test_zero_values() {
        let nums = vec![0, 0, 0, 0];
        assert_eq!(rob(nums), 0);
    }

    #[test]
    fn test_mixed_values() {
        let nums = vec![0, 1, 2, 3, 0];
        // Best: houses 1, 3 = 1 + 3 = 4
        assert_eq!(rob(nums), 4);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("040_house_robber_lc198 exercises - run tests with cargo test");
}
