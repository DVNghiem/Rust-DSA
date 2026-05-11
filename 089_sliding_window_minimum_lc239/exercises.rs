//! Exercises for Sliding Window Maximum (LeetCode 239)
//!
//! # Topics Covered
//! - Monotonic deque
//! - Sliding window
//! - O(n) algorithm for maximum
//! - Index-based queue management
//!
//! # Difficulty: Hard

use std::collections::VecDeque;

/// Sliding Window Maximum using monotonic deque
/// Time: O(n), Space: O(k)
pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n = nums.len();

    if n == 0 || k == 0 {
        return vec![];
    }

    if k == 1 {
        return nums;
    }

    let mut result = Vec::with_capacity(n - k + 1);
    let mut deque: VecDeque<usize> = VecDeque::new();

    for i in 0..n {
        // Remove indices that are out of the current window
        while !deque.is_empty() && deque.front().copied().unwrap_or(0) <= i.saturating_sub(k) {
            deque.pop_front();
        }

        // Remove indices whose corresponding values are less than nums[i]
        // They can never be the maximum in any window containing i
        while !deque.is_empty() && nums[*deque.back().unwrap()] < nums[i] {
            deque.pop_back();
        }

        // Add current index
        deque.push_back(i);

        // Window is ready when we have at least k elements
        if i >= k - 1 {
            // Front of deque is the maximum
            result.push(nums[*deque.front().unwrap()]);
        }
    }

    result
}

/// Alternative implementation with explicit bounds checking
pub fn max_sliding_window_v2(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n = nums.len();

    if n == 0 || k == 0 {
        return vec![];
    }

    let mut result = Vec::with_capacity(n - k + 1);
    let mut deque: VecDeque<usize> = VecDeque::new();

    for i in 0..n {
        // Remove elements outside the window
        while let Some(&front_idx) = deque.front() {
            if front_idx + k <= i {
                deque.pop_front();
            } else {
                break;
            }
        }

        // Remove smaller elements from the back
        while let Some(&back_idx) = deque.back() {
            if nums[back_idx] < nums[i] {
                deque.pop_back();
            } else {
                break;
            }
        }

        deque.push_back(i);

        // Add to result when window is complete
        if i >= k - 1 {
            result.push(nums[*deque.front().unwrap()]);
        }
    }

    result
}

/// Brute force O(nk) for comparison
pub fn max_sliding_window_brute(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n = nums.len();

    if n == 0 || k == 0 {
        return vec![];
    }

    let mut result = Vec::with_capacity(n - k + 1);

    for i in 0..=n - k {
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
    fn test_basic_example() {
        let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
        let k = 3;
        let result = max_sliding_window(nums, k);
        assert_eq!(result, vec![3, 3, 5, 5, 6, 7]);
    }

    #[test]
    fn test_single_element_window() {
        let nums = vec![1, 2, 3, 4, 5];
        let k = 1;
        let result = max_sliding_window(nums, k);
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_window_equals_array() {
        let nums = vec![1, 2, 3];
        let k = 3;
        let result = max_sliding_window(nums, k);
        assert_eq!(result, vec![3]);
    }

    #[test]
    fn test_decreasing_array() {
        let nums = vec![5, 4, 3, 2, 1];
        let k = 3;
        let result = max_sliding_window(nums, k);
        assert_eq!(result, vec![5, 4, 3]);
    }

    #[test]
    fn test_increasing_array() {
        let nums = vec![1, 2, 3, 4, 5];
        let k = 3;
        let result = max_sliding_window(nums, k);
        assert_eq!(result, vec![3, 4, 5]);
    }

    #[test]
    fn test_all_same_values() {
        let nums = vec![2, 2, 2, 2];
        let k = 2;
        let result = max_sliding_window(nums, k);
        assert_eq!(result, vec![2, 2, 2]);
    }

    #[test]
    fn test_negative_numbers() {
        let nums = vec![-1, -3, -5, -7, -9];
        let k = 2;
        let result = max_sliding_window(nums, k);
        assert_eq!(result, vec![-1, -3, -5, -7]);
    }

    #[test]
    fn test_empty_input() {
        let nums: Vec<i32> = vec![];
        let result = max_sliding_window(nums, 3);
        assert!(result.is_empty());
    }

    #[test]
    fn test_zero_window_size() {
        let nums = vec![1, 2, 3];
        let result = max_sliding_window(nums, 0);
        assert!(result.is_empty());
    }

    #[test]
    fn test_mixed_positive_negative() {
        let nums = vec![-1, 3, -2, 5, 8, -4, 2];
        let k = 3;
        let result = max_sliding_window(nums, k);
        assert_eq!(result, vec![3, 3, 5, 8, 8, 2]);
    }

    #[test]
    fn test_two_elements() {
        let nums = vec![1, -1];
        let k = 2;
        let result = max_sliding_window(nums, k);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_consistency_v1_v2() {
        let test_cases = vec![
            (vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
            (vec![1, 2, 3, 4, 5], 2),
            (vec![5, 4, 3, 2, 1], 3),
            (vec![1, 1, 1, 1], 2),
            (vec![-1, -2, -3], 2),
        ];

        for (nums, k) in test_cases {
            let v1 = max_sliding_window(nums.clone(), k);
            let v2 = max_sliding_window_v2(nums.clone(), k);
            assert_eq!(v1, v2, "v1 and v2 differ");
        }
    }

    #[test]
    fn test_consistency_with_brute() {
        let test_cases = vec![
            (vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
            (vec![1, 2, 3, 4, 5], 2),
            (vec![1, 2, 3, 4, 5], 1),
            (vec![5, 4, 3, 2, 1], 2),
        ];

        for (nums, k) in test_cases {
            let deque = max_sliding_window(nums.clone(), k);
            let brute = max_sliding_window_brute(nums.clone(), k);
            assert_eq!(deque, brute, "deque and brute differ for {:?}, k={}", nums, k);
        }
    }

    #[test]
    fn test_single_element() {
        let nums = vec![1];
        let result = max_sliding_window(nums, 1);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_window_larger_than_array() {
        let nums = vec![1, 2, 3];
        let result = max_sliding_window(nums, 5);
        assert!(result.is_empty());
    }

    #[test]
    fn test_alternating_high_low() {
        let nums = vec![1, 5, 1, 5, 1, 5];
        let k = 2;
        let result = max_sliding_window(nums, k);
        assert_eq!(result, vec![5, 5, 5, 5, 5]);
    }

    #[test]
    fn test_peak_at_end() {
        let nums = vec![1, 2, 3, 2, 1];
        let k = 3;
        let result = max_sliding_window(nums, k);
        assert_eq!(result, vec![3, 3, 3]);
    }

    #[test]
    fn test_peak_at_start() {
        let nums = vec![3, 2, 1, 2, 3];
        let k = 3;
        let result = max_sliding_window(nums, k);
        assert_eq!(result, vec![3, 3, 3]);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("089_sliding_window_minimum_lc239 exercises - run tests with cargo test");
}
