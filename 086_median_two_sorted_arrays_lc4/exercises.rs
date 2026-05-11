//! Exercises for Median of Two Sorted Arrays (LeetCode 4)
//!
//! # Topics Covered
//! - Binary search
//! - Partition-based median finding
//! - O(log n) algorithm design
//! - Edge case handling
//!
//! # Difficulty: Hard

/// Simple merge approach - O(m+n) time, O(m+n) space
pub fn find_median_merge(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let m = nums1.len();
    let n = nums2.len();
    let total = m + n;

    let mut merged = Vec::with_capacity(total);
    let mut i = 0;
    let mut j = 0;

    while i < m && j < n {
        if nums1[i] <= nums2[j] {
            merged.push(nums1[i]);
            i += 1;
        } else {
            merged.push(nums2[j]);
            j += 1;
        }
    }

    while i < m {
        merged.push(nums1[i]);
        i += 1;
    }

    while j < n {
        merged.push(nums2[j]);
        j += 1;
    }

    if total % 2 == 0 {
        (merged[total / 2 - 1] as f64 + merged[total / 2] as f64) / 2.0
    } else {
        merged[total / 2] as f64
    }
}

/// Two pointers approach - O(m+n) time, O(1) space
/// Only tracks the two middle elements
pub fn find_median_two_pointers(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let m = nums1.len();
    let n = nums2.len();
    let total = m + n;

    let mut left = 0;
    let mut right = 0;

    let mut i = 0;
    let mut j = 0;

    for _ in 0..(total + 1) / 2 {
        left = right;
        if i < m && (j >= n || nums1[i] <= nums2[j]) {
            right = nums1[i];
            i += 1;
        } else {
            right = nums2[j];
            j += 1;
        }
    }

    if total % 2 == 0 {
        (left as f64 + right as f64) / 2.0
    } else {
        right as f64
    }
}

/// Binary search approach - O(log m) time, O(1) space
pub fn find_median_binary_search(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let m = nums1.len();
    let n = nums2.len();

    // Ensure nums1 is the smaller array for simpler binary search
    let (a, b, m, n) = if m > n {
        (nums2, nums1, n, m)
    } else {
        (nums1, nums2, m, n)
    };

    let mut left = 0;
    let mut right = m;

    while left <= right {
        let partition_x = (left + right) / 2;
        let partition_y = (m + n + 1) / 2 - partition_x;

        let max_left_x = if partition_x == 0 { i32::MIN } else { a[partition_x - 1] };
        let min_right_x = if partition_x == m { i32::MAX } else { a[partition_x] };

        let max_left_y = if partition_y == 0 { i32::MIN } else { b[partition_y - 1] };
        let min_right_y = if partition_y == n { i32::MAX } else { b[partition_y] };

        if max_left_x <= min_right_y && max_left_y <= min_right_x {
            // Found correct partition
            if (m + n) % 2 == 0 {
                let left_max = max_left_x.max(max_left_y);
                let right_min = min_right_x.min(min_right_y);
                return (left_max as f64 + right_min as f64) / 2.0;
            } else {
                return max_left_x.max(max_left_y) as f64;
            }
        } else if max_left_x > min_right_y {
            right = partition_x - 1;
        } else {
            left = partition_x + 1;
        }
    }

    0.0 // Should never reach here
}

/// Helper to get kth element using binary search
pub fn find_kth_binary_search(nums1: &[i32], nums2: &[i32], k: usize) -> f64 {
    let (m, n) = (nums1.len(), nums2.len());

    // Ensure m <= n
    let (a, b) = if m > n { (nums2, nums1) } else { (nums1, nums2) };

    let mut k = k;

    let mut left = 0;
    let mut right = a.len();

    while left <= right {
        let partition_a = (left + right) / 2;
        let partition_b = k - partition_a;

        let max_left_a = if partition_a == 0 { i32::MIN } else { a[partition_a - 1] };
        let min_right_a = if partition_a == a.len() { i32::MAX } else { a[partition_a] };

        let max_left_b = if partition_b == 0 { i32::MIN } else { b[partition_b - 1] };
        let min_right_b = if partition_b == b.len() { i32::MAX } else { b[partition_b] };

        if max_left_a <= min_right_b && max_left_b <= min_right_a {
            // Found correct partition
            if k % 2 == 1 {
                return max_left_a.max(max_left_b) as f64;
            } else {
                let left_max = max_left_a.max(max_left_b);
                let right_min = min_right_a.min(min_right_b);
                return (left_max + right_min) as f64 / 2.0;
            }
        } else if max_left_a > min_right_b {
            right = partition_a - 1;
        } else {
            left = partition_a + 1;
        }
    }

    0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_basic() {
        assert_eq!(find_median_merge(vec![1, 3], vec![2]), 2.0);
    }

    #[test]
    fn test_merge_two_elements_each() {
        assert_eq!(find_median_merge(vec![1, 2], vec![3, 4]), 2.5);
    }

    #[test]
    fn test_merge_empty_first() {
        assert_eq!(find_median_merge(vec![], vec![1]), 1.0);
    }

    #[test]
    fn test_merge_empty_second() {
        assert_eq!(find_median_merge(vec![1], vec![]), 1.0);
    }

    #[test]
    fn test_merge_both_empty() {
        assert_eq!(find_median_merge(vec![], vec![]), 0.0);
    }

    #[test]
    fn test_merge_odd_total() {
        assert_eq!(find_median_merge(vec![1, 2, 3], vec![]), 2.0);
    }

    #[test]
    fn test_merge_even_total() {
        assert_eq!(find_median_merge(vec![1, 2], vec![]), 1.5);
    }

    #[test]
    fn test_twoway_basic() {
        assert_eq!(find_median_two_pointers(vec![1, 3], vec![2]), 2.0);
    }

    #[test]
    fn test_twoway_two_each() {
        assert_eq!(find_median_two_pointers(vec![1, 2], vec![3, 4]), 2.5);
    }

    #[test]
    fn test_twoway_empty_first() {
        assert_eq!(find_median_two_pointers(vec![], vec![1]), 1.0);
    }

    #[test]
    fn test_binary_basic() {
        assert_eq!(find_median_binary_search(vec![1, 3], vec![2]), 2.0);
    }

    #[test]
    fn test_binary_two_each() {
        assert_eq!(find_median_binary_search(vec![1, 2], vec![3, 4]), 2.5);
    }

    #[test]
    fn test_binary_empty_first() {
        assert_eq!(find_median_binary_search(vec![], vec![1]), 1.0);
    }

    #[test]
    fn test_binary_empty_second() {
        assert_eq!(find_median_binary_search(vec![1], vec![]), 1.0);
    }

    #[test]
    fn test_binary_diff_sizes() {
        assert_eq!(find_median_binary_search(vec![1, 3], vec![2, 4, 5, 6]), 3.5);
    }

    #[test]
    fn test_binary_longer_first() {
        assert_eq!(find_median_binary_search(vec![1, 2, 3, 4], vec![5, 6]), 3.5);
    }

    #[test]
    fn test_consistency_all_approaches() {
        let test_cases = [
            (vec![1, 3], vec![2]),
            (vec![1, 2], vec![3, 4]),
            (vec![], vec![1]),
            (vec![1], vec![]),
            (vec![1, 2, 3], vec![]),
            (vec![1, 2], vec![]),
            (vec![1, 3], vec![2, 4, 5, 6]),
            (vec![1, 2, 3, 4], vec![5, 6]),
            (vec![1, 2, 3], vec![4, 5, 6]),
            (vec![1, 1, 1], vec![1, 1]),
            (vec![1, 2, 5], vec![3, 4]),
            (vec![2, 2], vec![2, 2]),
        ];

        for (a, b) in test_cases {
            let merge = find_median_merge(a.clone(), b.clone());
            let twoway = find_median_two_pointers(a.clone(), b.clone());
            let binary = find_median_binary_search(a.clone(), b.clone());

            // Allow small floating point differences
            assert!((merge - twoway).abs() < 1e-9, "Merge vs TwoPointers differ: {:?}, {:?}", a, b);
            assert!((twoway - binary).abs() < 1e-9, "TwoPointers vs Binary differ: {:?}, {:?}", a, b);
        }
    }

    #[test]
    fn test_negative_numbers() {
        let result = find_median_binary_search(vec![-5, -3, -1], vec![-2, 0]);
        assert!(result >= -2.0 && result <= 0.0);
    }

    #[test]
    fn test_large_arrays() {
        let a: Vec<i32> = (0..1000).collect();
        let b: Vec<i32> = (1000..2000).collect();
        let result = find_median_binary_search(a, b);
        assert_eq!(result, 999.5);
    }

    #[test]
    fn test_single_element_each() {
        assert_eq!(find_median_binary_search(vec![1], vec![2]), 1.5);
    }

    #[test]
    fn test_same_elements() {
        assert_eq!(find_median_binary_search(vec![1, 1, 1], vec![1, 1, 1]), 1.0);
    }

    #[test]
    fn test_unbalanced_both() {
        // [1,2,3,4,5] and [6]
        // Combined: [1,2,3,4,5,6], median = (3+4)/2 = 3.5
        assert_eq!(find_median_binary_search(vec![1, 2, 3, 4, 5], vec![6]), 3.5);
    }

    #[test]
    fn test_unbalanced_second() {
        // [] and [1,2,3,4,5]
        assert_eq!(find_median_binary_search(vec![], vec![1, 2, 3, 4, 5]), 3.0);
    }

    #[test]
    fn test_kth_basic() {
        // k=1 should return smallest
        assert_eq!(find_kth_binary_search(&[1], &[2], 1), 1.0);
    }

    #[test]
    fn test_kth_middle() {
        // k=3 for [1,3] and [2]
        // Combined: [1,2,3], k=3rd is 2
        assert_eq!(find_kth_binary_search(&[1, 3], &[2], 3), 2.0);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("086_median_two_sorted_arrays_lc4 exercises - run tests with cargo test");
}
