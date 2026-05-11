/// Approach: Greedy - track maximum reachable index
pub fn can_jump(nums: Vec<i32>) -> bool {
    if nums.is_empty() {
        return false;
    }

    let mut max_reach = 0;

    for i in 0..nums.len() {
        if i as i32 > max_reach {
            return false;
        }
        max_reach = max_reach.max(i as i32 + nums[i]);
    }

    true
}

/// Alternative: Dynamic Programming (bottom-up)
pub fn can_jump_dp(nums: Vec<i32>) -> bool {
    if nums.is_empty() {
        return false;
    }

    let n = nums.len();
    let mut dp = vec![false; n];
    dp[0] = true;

    for i in 0..n {
        if !dp[i] {
            continue;
        }
        let max_jump = nums[i] as usize;
        for j in 1..=max_jump.min(n - 1 - i) {
            dp[i + j] = true;
        }
    }

    dp[n - 1]
}

/// Alternative: Backward greedy
pub fn can_jump_backward(nums: Vec<i32>) -> bool {
    if nums.is_empty() {
        return false;
    }

    let mut target = nums.len() - 1;

    for i in (0..target).rev() {
        if i as i32 + nums[i] >= target as i32 {
            target = i;
        }
    }

    target == 0
}

/// Find minimum jumps to reach end (LC 45 - Jump Game II)
pub fn min_jump(nums: Vec<i32>) -> i32 {
    if nums.is_empty() || nums.len() == 1 {
        return 0;
    }

    let mut jumps = 0;
    let mut current_end: usize = 0;
    let mut furthest: usize = 0;

    for i in 0..nums.len() - 1 {
        let reach = (i as i32 + nums[i]) as usize;
        furthest = furthest.max(reach);
        if i == current_end {
            jumps += 1;
            current_end = furthest;
        }
    }

    jumps as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let nums = vec![2, 3, 1, 1, 4];
        assert!(can_jump(nums));
    }

    #[test]
    fn test_cannot_reach() {
        let nums = vec![3, 2, 1, 0, 4];
        assert!(!can_jump(nums));
    }

    #[test]
    fn test_empty() {
        let nums: Vec<i32> = vec![];
        assert!(!can_jump(nums));
    }

    #[test]
    fn test_single_element() {
        let nums = vec![0];
        assert!(can_jump(nums));
    }

    #[test]
    fn test_single_zero() {
        let nums = vec![0];
        assert!(can_jump(nums));
    }

    #[test]
    fn test_all_ones() {
        let nums = vec![1, 1, 1, 1, 1];
        assert!(can_jump(nums));
    }

    #[test]
    fn test_first_is_zero() {
        let nums = vec![0, 2, 3];
        assert!(!can_jump(nums));
    }

    #[test]
    fn test_last_is_zero_but_reachable() {
        let nums = vec![1, 1, 0, 1];
        assert!(!can_jump(nums));
    }

    #[test]
    fn test_linear_increment() {
        let nums = vec![1, 1, 1, 1, 1];
        assert!(can_jump(nums));
    }

    #[test]
    fn test_large_jump_at_start() {
        let nums = vec![5, 0, 0, 0, 0];
        assert!(can_jump(nums));
    }

    #[test]
    fn test_dp_basic() {
        let nums = vec![2, 3, 1, 1, 4];
        assert!(can_jump_dp(nums));
    }

    #[test]
    fn test_dp_cannot_reach() {
        let nums = vec![3, 2, 1, 0, 4];
        assert!(!can_jump_dp(nums));
    }

    #[test]
    fn test_dp_empty() {
        let nums: Vec<i32> = vec![];
        assert!(!can_jump_dp(nums));
    }

    #[test]
    fn test_backward_basic() {
        let nums = vec![2, 3, 1, 1, 4];
        assert!(can_jump_backward(nums));
    }

    #[test]
    fn test_backward_cannot_reach() {
        let nums = vec![3, 2, 1, 0, 4];
        assert!(!can_jump_backward(nums));
    }

    #[test]
    fn test_backward_first_zero() {
        let nums = vec![0, 2, 3];
        assert!(!can_jump_backward(nums));
    }

    #[test]
    fn test_min_jump_basic() {
        let nums = vec![2, 3, 1, 1, 4];
        assert_eq!(min_jump(nums), 2);
    }

    #[test]
    fn test_min_jump_one_jump() {
        let nums = vec![1, 1, 1, 1, 1];
        assert_eq!(min_jump(nums), 4);
    }

    #[test]
    fn test_min_jump_large_jump() {
        let nums = vec![5, 0, 0, 0, 0];
        assert_eq!(min_jump(nums), 1);
    }

    #[test]
    fn test_min_jump_single() {
        let nums = vec![0];
        assert_eq!(min_jump(nums), 0);
    }

    #[test]
    fn test_min_jump_two_elements() {
        let nums = vec![2, 0];
        assert_eq!(min_jump(nums), 1);
    }

    #[test]
    fn test_min_jump_three_elements() {
        let nums = vec![1, 2, 1];
        assert_eq!(min_jump(nums), 2);
    }

    #[test]
    fn test_all_zeros_except_last() {
        let nums = vec![1, 0, 0, 0];
        assert!(!can_jump(nums));
    }

    #[test]
    fn test_staggered_reachability() {
        let nums = vec![2, 0, 0, 0, 2];
        assert!(!can_jump(nums));
    }

    #[test]
    fn test_large_input() {
        let nums = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        assert!(can_jump(nums));
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("050_jump_game_lc55 exercises - run tests with cargo test");
}
