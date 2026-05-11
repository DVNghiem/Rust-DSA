/// Sliding Window Maximum - LeetCode 239
/// Find maximum in each sliding window using monotonic deque.

use std::collections::VecDeque;

/// Approach: Monotonic decreasing deque
/// Store indices, front = max, remove outdated and smaller elements
pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n = nums.len();
    if n == 0 || k == 0 { return vec![]; }

    let mut result = Vec::new();
    let mut deque: VecDeque<usize> = VecDeque::new();

    for i in 0..n {
        // Remove indices outside current window
        while !deque.is_empty() && deque[0] <= i.saturating_sub(k) {
            deque.pop_front();
        }

        // Remove smaller elements from back (they can never be max)
        while !deque.is_empty() && nums[*deque.back().unwrap()] < nums[i] {
            deque.pop_back();
        }

        deque.push_back(i);

        // Add max to result when window is ready
        if i >= k - 1 {
            result.push(nums[*deque.front().unwrap()]);
        }
    }

    result
}

/// Alternative: Brute force O(n*k) for comparison
pub fn max_sliding_window_brute(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n = nums.len();
    if n == 0 || k == 0 { return vec![]; }

    let mut result = Vec::new();
    for i in 0..n - k + 1 {
        let mut max_val = nums[i];
        for j in i..i + k {
            max_val = max_val.max(nums[j]);
        }
        result.push(max_val);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_window_basic() {
        assert_eq!(max_sliding_window(vec![1,3,-1,-3,5,3,6,7], 3), vec![3,3,5,5,6,7]);
    }

    #[test]
    fn test_max_window_empty() {
        assert_eq!(max_sliding_window(vec![], 3), vec![]);
    }

    #[test]
    fn test_max_window_k_one() {
        assert_eq!(max_sliding_window(vec![1,2,3,4], 1), vec![1,2,3,4]);
    }

    #[test]
    fn test_max_window_k_equals_n() {
        assert_eq!(max_sliding_window(vec![1,2,3,4], 4), vec![4]);
    }

    #[test]
    fn test_max_window_k_greater_n() {
        assert_eq!(max_sliding_window(vec![1,2], 3), vec![]);
    }

    #[test]
    fn test_max_window_negative() {
        assert_eq!(max_sliding_window(vec![-1,-2,-3,-1], 2), vec![-1,-2,-3]);
    }

    #[test]
    fn test_max_window_single_element() {
        assert_eq!(max_sliding_window(vec![1], 1), vec![1]);
    }

    #[test]
    fn test_max_window_all_same() {
        assert_eq!(max_sliding_window(vec![5,5,5,5], 2), vec![5,5,5]);
    }

    #[test]
    fn test_max_window_descending() {
        assert_eq!(max_sliding_window(vec![5,4,3,2,1], 3), vec![5,4,3]);
    }

    #[test]
    fn test_max_window_ascending() {
        assert_eq!(max_sliding_window(vec![1,2,3,4,5], 3), vec![3,4,5]);
    }

    #[test]
    fn test_max_window_brute_basic() {
        assert_eq!(max_sliding_window_brute(vec![1,3,-1,-3,5,3,6,7], 3), vec![3,3,5,5,6,7]);
    }

    #[test]
    fn test_max_window_brute_k_one() {
        assert_eq!(max_sliding_window_brute(vec![1,2,3,4], 1), vec![1,2,3,4]);
    }

    #[test]
    fn test_all_same_result() {
        let tests = vec![
            (vec![1,3,-1,-3,5,3,6,7], 3),
            (vec![1,2,3,4], 2),
            (vec![-1,-2,-3,-1], 2),
            (vec![5,4,3,2,1], 3),
        ];
        for (nums, k) in tests {
            let r1 = max_sliding_window(nums.clone(), k);
            let r2 = max_sliding_window_brute(nums, k);
            assert_eq!(r1, r2);
        }
    }

    #[test]
    fn test_max_window_mixed() {
        assert_eq!(max_sliding_window(vec![8,3,5,2,1,7], 3), vec![8,5,5,7]);
    }

    #[test]
    fn test_max_window_large() {
        let nums: Vec<i32> = (0..10000).collect();
        let result = max_sliding_window(nums, 100);
        assert!(!result.is_empty());
        assert_eq!(result.len(), 9901);
    }

    #[test]
    fn test_max_window_edge_1() {
        assert_eq!(max_sliding_window(vec![1], 1), vec![1]);
    }

    #[test]
    fn test_max_window_edge_2() {
        assert_eq!(max_sliding_window(vec![1,2], 1), vec![1,2]);
    }

    #[test]
    fn test_max_window_edge_3() {
        assert_eq!(max_sliding_window(vec![1,2], 2), vec![2]);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("070_sliding_window_max_lc239 exercises - run tests with cargo test");
}
