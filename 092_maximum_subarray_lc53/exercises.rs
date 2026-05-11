//! Exercises for Maximum Subarray (LeetCode 53)
//!
//! # Topics Covered
//! - Kadane's algorithm
//! - Dynamic programming
//! - O(n) time, O(1) space
//! - Subarray maximum finding
//!
//! # Difficulty: Medium

/// Kadane's algorithm for maximum subarray
/// Time: O(n), Space: O(1)
pub fn max_subarray(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut max_sum = nums[0];
    let mut current_sum = nums[0];

    for num in nums.iter().skip(1) {
        current_sum = (*num).max(current_sum + *num);
        max_sum = max_sum.max(current_sum);
    }

    max_sum
}

/// Divide and conquer approach
/// Time: O(n log n), Space: O(log n)
pub fn max_subarray_divide_conquer(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    fn helper(nums: &[i32], left: usize, right: usize) -> i32 {
        if left == right {
            return nums[left];
        }

        let mid = (left + right) / 2;

        let left_max = helper(nums, left, mid);
        let right_max = helper(nums, mid + 1, right);

        // Max crossing subarray
        let mut left_sum = i32::MIN;
        let mut sum = 0;
        for i in (left..=mid).rev() {
            sum += nums[i];
            left_sum = left_sum.max(sum);
        }

        let mut right_sum = i32::MIN;
        sum = 0;
        for i in mid + 1..=right {
            sum += nums[i];
            right_sum = right_sum.max(sum);
        }

        let cross_max = left_sum + right_sum;

        left_max.max(right_max).max(cross_max)
    }

    helper(&nums, 0, nums.len() - 1)
}

/// Brute force O(n²)
pub fn max_subarray_brute(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut max_sum = nums[0];

    for i in 0..nums.len() {
        let mut sum = 0;
        for j in i..nums.len() {
            sum += nums[j];
            max_sum = max_sum.max(sum);
        }
    }

    max_sum
}

/// Find indices of maximum subarray
pub fn max_subarray_indices(nums: Vec<i32>) -> (usize, usize, i32) {
    if nums.is_empty() {
        return (0, 0, 0);
    }

    let mut max_sum = nums[0];
    let mut max_start = 0;
    let mut max_end = 0;

    let mut current_start = 0;
    let mut current_sum = nums[0];

    for i in 1..nums.len() {
        if current_sum + nums[i] < nums[i] {
            current_start = i;
            current_sum = nums[i];
        } else {
            current_sum += nums[i];
        }

        if current_sum > max_sum {
            max_sum = current_sum;
            max_start = current_start;
            max_end = i;
        }
    }

    (max_start, max_end, max_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_example() {
        assert_eq!(max_subarray(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(max_subarray_divide_conquer(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(max_subarray_brute(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    }

    #[test]
    fn test_all_positive() {
        assert_eq!(max_subarray(vec![1, 2, 3, 4, 5]), 15);
        assert_eq!(max_subarray_divide_conquer(vec![1, 2, 3, 4, 5]), 15);
        assert_eq!(max_subarray_brute(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn test_all_negative() {
        assert_eq!(max_subarray(vec![-1, -2, -3, -4]), -1);
        assert_eq!(max_subarray_divide_conquer(vec![-1, -2, -3, -4]), -1);
        assert_eq!(max_subarray_brute(vec![-1, -2, -3, -4]), -1);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(max_subarray(vec![5]), 5);
        assert_eq!(max_subarray_divide_conquer(vec![5]), 5);
        assert_eq!(max_subarray_brute(vec![5]), 5);
    }

    #[test]
    fn test_empty() {
        assert_eq!(max_subarray(vec![]), 0);
    }

    #[test]
    fn test_two_elements() {
        assert_eq!(max_subarray(vec![1, -2]), 1);
        assert_eq!(max_subarray(vec![-1, 2]), 2);
    }

    #[test]
    fn test_mixed() {
        assert_eq!(max_subarray(vec![-2, -1]), -1);
    }

    #[test]
    fn test_consistency_all_approaches() {
        let test_cases = vec![
            vec![-2, 1, -3, 4, -1, 2, 1, -5, 4],
            vec![1, 2, 3, 4, 5],
            vec![-1, -2, -3, -4],
            vec![5],
            vec![],
            vec![1, -2, 3, 4, -1, 2, 1],
            vec![-2, -1, -3],
        ];

        for nums in test_cases {
            let kadane = max_subarray(nums.clone());
            let divide = max_subarray_divide_conquer(nums.clone());
            let brute = max_subarray_brute(nums.clone());
            assert_eq!(kadane, divide, "kadane vs divide for {:?}", nums);
            assert_eq!(divide, brute, "divide vs brute for {:?}", nums);
        }
    }

    #[test]
    fn test_indices_basic() {
        let (start, end, sum) = max_subarray_indices(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]);
        assert_eq!(sum, 6);
        // [4, -1, 2, 1] is indices 3 to 6
        assert_eq!(start, 3);
        assert_eq!(end, 6);
    }

    #[test]
    fn test_indices_all_positive() {
        let (start, end, sum) = max_subarray_indices(vec![1, 2, 3, 4, 5]);
        assert_eq!(sum, 15);
        assert_eq!(start, 0);
        assert_eq!(end, 4);
    }

    #[test]
    fn test_indices_all_negative() {
        let (start, end, sum) = max_subarray_indices(vec![-1, -2, -3]);
        assert_eq!(sum, -1);
        assert_eq!(start, 0);
        assert_eq!(end, 0);
    }

    #[test]
    fn test_start_at_zero() {
        let (start, _, _) = max_subarray_indices(vec![5, -1, -2, 3, 4]);
        assert_eq!(start, 0);
    }

    #[test]
    fn test_ending_at_last() {
        let (start, end, _) = max_subarray_indices(vec![-1, -2, 5]);
        assert_eq!(end, 2);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("092_maximum_subarray_lc53 exercises - run tests with cargo test");
}
