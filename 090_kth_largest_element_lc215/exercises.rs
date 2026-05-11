//! Exercises for Kth Largest Element (LeetCode 215)
//!
//! # Topics Covered
//! - Quickselect algorithm
//! - Heap-based approach
//! - Sort-based solution
//! - O(n) average time complexity
//!
//! # Difficulty: Medium

use std::collections::BinaryHeap;

/// Sort-based approach: O(n log n) time, O(1) space
pub fn find_kth_largest_sort(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    nums.sort_by(|a, b| b.cmp(a)); // Descending
    nums[(k as usize) - 1]
}

/// Quickselect approach: O(n) average, O(n²) worst case, O(1) space
pub fn find_kth_largest_quickselect(mut nums: Vec<i32>, k: i32) -> i32 {
    let k = (k as usize) - 1; // Convert to 0-indexed
    let len = nums.len();
    quickselect(&mut nums, 0, len - 1, k);
    nums[k]
}

fn quickselect(arr: &mut [i32], left: usize, right: usize, target: usize) {
    if left >= right {
        return;
    }

    let pivot_idx = partition(arr, left, right);

    if pivot_idx == target {
        return;
    } else if pivot_idx < target {
        quickselect(arr, pivot_idx + 1, right, target);
    } else {
        quickselect(arr, left, pivot_idx - 1, target);
    }
}

fn partition(arr: &mut [i32], left: usize, right: usize) -> usize {
    let pivot = arr[right];
    let mut i = left;

    for j in left..right {
        if arr[j] >= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, right);
    i
}

/// Min-heap approach: O(n log k) time, O(k) space
pub fn find_kth_largest_heap(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut heap: BinaryHeap<std::cmp::Reverse<i32>> = BinaryHeap::new();

    for num in nums {
        heap.push(std::cmp::Reverse(num));
        if heap.len() > k {
            heap.pop();
        }
    }

    heap.peek().map(|x| x.0).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_example() {
        assert_eq!(find_kth_largest_quickselect(vec![3, 2, 1, 5, 6, 4], 2), 5);
        assert_eq!(find_kth_largest_sort(vec![3, 2, 1, 5, 6, 4], 2), 5);
        assert_eq!(find_kth_largest_heap(vec![3, 2, 1, 5, 6, 4], 2), 5);
    }

    #[test]
    fn test_k_1() {
        assert_eq!(find_kth_largest_quickselect(vec![3, 2, 1, 5, 6, 4], 1), 6);
        assert_eq!(find_kth_largest_sort(vec![3, 2, 1, 5, 6, 4], 1), 6);
        assert_eq!(find_kth_largest_heap(vec![3, 2, 1, 5, 6, 4], 1), 6);
    }

    #[test]
    fn test_k_equals_length() {
        let nums = vec![3, 2, 1, 5, 6, 4];
        let n = nums.len() as i32;
        assert_eq!(find_kth_largest_quickselect(nums.clone(), n), 1);
        assert_eq!(find_kth_largest_sort(nums.clone(), n), 1);
        assert_eq!(find_kth_largest_heap(nums.clone(), n), 1);
    }

    #[test]
    fn test_duplicates() {
        assert_eq!(find_kth_largest_quickselect(vec![3, 3, 3, 3], 2), 3);
        assert_eq!(find_kth_largest_sort(vec![3, 3, 3, 3], 2), 3);
        assert_eq!(find_kth_largest_heap(vec![3, 3, 3, 3], 2), 3);
    }

    #[test]
    fn test_negative_numbers() {
        let nums = vec![-1, -2, -3, -4, -5];
        assert_eq!(find_kth_largest_quickselect(nums.clone(), 1), -1);
        assert_eq!(find_kth_largest_sort(nums.clone(), 1), -1);
        assert_eq!(find_kth_largest_heap(nums.clone(), 1), -1);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(find_kth_largest_quickselect(vec![5], 1), 5);
        assert_eq!(find_kth_largest_sort(vec![5], 1), 5);
        assert_eq!(find_kth_largest_heap(vec![5], 1), 5);
    }

    #[test]
    fn test_two_elements() {
        assert_eq!(find_kth_largest_quickselect(vec![1, 2], 1), 2);
        assert_eq!(find_kth_largest_quickselect(vec![1, 2], 2), 1);
    }

    #[test]
    fn test_already_sorted_descending() {
        assert_eq!(find_kth_largest_quickselect(vec![6, 5, 4, 3, 2, 1], 3), 4);
    }

    #[test]
    fn test_already_sorted_ascending() {
        assert_eq!(find_kth_largest_quickselect(vec![1, 2, 3, 4, 5, 6], 4), 3);
    }

    #[test]
    fn test_consistency_all_approaches() {
        let test_cases = vec![
            (vec![3, 2, 1, 5, 6, 4], 2),
            (vec![3, 3, 3, 3], 2),
            (vec![-1, -2, -3, -4], 2),
            (vec![5], 1),
            (vec![1, 2], 1),
            (vec![1, 2], 2),
        ];

        for (nums, k) in test_cases {
            let sort = find_kth_largest_sort(nums.clone(), k);
            let quick = find_kth_largest_quickselect(nums.clone(), k);
            let heap = find_kth_largest_heap(nums.clone(), k);
            assert_eq!(sort, quick, "sort vs quick for {:?}, k={}", nums, k);
            assert_eq!(quick, heap, "quick vs heap for {:?}, k={}", nums, k);
        }
    }

    #[test]
    fn test_large_k() {
        let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(find_kth_largest_quickselect(nums.clone(), 10), 1);
        assert_eq!(find_kth_largest_sort(nums.clone(), 10), 1);
        assert_eq!(find_kth_largest_heap(nums.clone(), 10), 1);
    }

    #[test]
    fn test_middle_k() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(find_kth_largest_quickselect(nums.clone(), 3), 3);
    }

    #[test]
    fn test_mixed_positive_negative() {
        let nums = vec![-10, 5, 3, 2, -5];
        assert_eq!(find_kth_largest_quickselect(nums.clone(), 2), 3);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("Kth Largest Element exercises - run tests with cargo test");
}