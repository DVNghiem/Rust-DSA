//! Two Sum Exercises (LeetCode #1)
//!
//! This module contains exercises for the Two Sum problem and its variants.
//! All implementations use todo!() as placeholders.

use std::collections::{HashMap, HashSet};

// ============================================================================
// Exercise 1: Two Sum - One Pass HashMap (Primary Solution)
// ============================================================================

/// Given an array of integers nums and an integer target, return indices of the two numbers
/// such that they add up to target.
///
/// Approach: Use a HashMap to store the complement of each element as we iterate.
/// When we find a complement that exists in the map, we have our pair.
///
/// Time Complexity: O(n)
/// Space Complexity: O(n)
pub fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    todo!("Implement one-pass HashMap solution")
}

// ============================================================================
// Exercise 2: Two Sum - Brute Force (for comparison)
// ============================================================================

/// A brute force O(n²) solution for educational purposes.
///
/// Time Complexity: O(n²)
/// Space Complexity: O(1)
pub fn two_sum_brute_force(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    todo!("Implement brute force solution")
}

// ============================================================================
// Exercise 3: Two Sum - Two Pass HashMap (for comparison)
// ============================================================================

/// Build HashMap first, then check for complements.
///
/// Time Complexity: O(n)
/// Space Complexity: O(n)
pub fn two_sum_two_pass(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    todo!("Implement two-pass HashMap solution")
}

// ============================================================================
// Exercise 4: Two Sum II - Sorted Array (LC 167)
// ============================================================================

/// Given a sorted integer array in non-decreasing order, return the indices (1-indexed)
/// of two numbers such that they add up to target.
///
/// Since the array is sorted, we can use the two-pointer approach:
/// - Start with left at the beginning and right at the end
/// - If sum > target, decrement right to decrease sum
/// - If sum < target, increment left to increase sum
///
/// Time Complexity: O(n)
/// Space Complexity: O(1)
pub fn two_sum_sorted(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    todo!("Implement two-pointer solution for sorted array")
}

// ============================================================================
// Exercise 5: Three Sum (LC 15)
// ============================================================================

/// Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]]
/// such that i != j, i != k, j != k, and nums[i] + nums[j] + nums[k] == 0.
///
/// The solution set must not contain duplicate triplets.
///
/// Time Complexity: O(n²)
/// Space Complexity: O(1) excluding output
pub fn three_sum(nums: &[i32]) -> Vec<Vec<i32>> {
    todo!("Implement three sum solution")
}

// ============================================================================
// Exercise 6: Three Sum Closest (LC 16)
// ============================================================================

/// Given an integer array nums and an integer target, find three integers in nums
/// such that the sum is closest to target.
///
/// Return the sum of the three integers.
///
/// Time Complexity: O(n²)
/// Space Complexity: O(1)
pub fn three_sum_closest(nums: &[i32], target: i32) -> i32 {
    todo!("Implement three sum closest solution")
}

// ============================================================================
// Exercise 7: Four Sum (LC 18)
// ============================================================================

/// Given an array of integers nums and an integer target, return all quadruplets
/// [nums[a], nums[b], nums[c], nums[d]] such that the sum of the four numbers equals target.
///
/// Time Complexity: O(n³) or O(n²) with careful optimization
/// Space Complexity: O(1) excluding output
pub fn four_sum(nums: &[i32], target: i32) -> Vec<Vec<i32>> {
    todo!("Implement four sum solution")
}

// ============================================================================
// Exercise 8: Count Pair with Absolute Difference (LC 2006)
// ============================================================================

/// Given an integer array nums and an integer k, return the number of pairs (i, j)
/// where i < j such that |nums[i] - nums[j]| == k.
///
/// Time Complexity: O(n)
/// Space Complexity: O(n)
pub fn count_pairs_with_diff_k(nums: &[i32], k: i32) -> i32 {
    todo!("Implement count pairs with absolute difference k")
}

// ============================================================================
// Exercise 9: Find All Pairs with Sum (LC 1995)
// ============================================================================

/// Given an integer array nums, find all unique pairs [a, b] where a + b == target.
///
/// Time Complexity: O(n)
/// Space Complexity: O(n)
pub fn find_pairs(nums: &[i32], target: i32) -> Vec<(i32, i32)> {
    todo!("Implement find all pairs with sum")
}

// ============================================================================
// Exercise 10: Two Sum with Data Stream (Custom)
// ============================================================================

/// Design a data structure that supports:
/// - add(int number): Add a number to the data structure
/// - find(int value): Check if there's a pair that sums to value
///
/// Time Complexity: O(1) for add, O(n) for find
/// Space Complexity: O(n)
#[derive(Default)]
pub struct TwoSum {
    numbers: HashSet<i32>,
    sums: HashSet<i32>,
}

impl TwoSum {
    pub fn new() -> Self {
        todo!("Implement constructor")
    }

    pub fn add(&mut self, number: i32) {
        todo!("Add number to data structure")
    }

    pub fn find(&self, value: i32) -> bool {
        todo!("Check if any pair sums to value")
    }
}

// ============================================================================
// Exercise 11: Minimum Operations to Reduce X to Zero (LC 1658)
// ============================================================================

/// You are given an integer array nums and an integer x. In one operation, you can
/// either remove the first or the last element from the array.
///
/// Return the minimum number of operations to reduce x to exactly zero.
/// If not possible, return -1.
///
/// Hint: This is a two-sum problem on a reversed array (or use two pointers from both ends).
///
/// Time Complexity: O(n)
/// Space Complexity: O(1)
pub fn min_operations(nums: &[i32], x: i32) -> i32 {
    todo!("Implement minimum operations to reduce x to zero")
}

// ============================================================================
// Exercise 12: Subarray Sum Equals K (LC 560)
// ============================================================================

/// Given an integer array nums and an integer k, return the total number of subarrays
/// whose sum equals k.
///
/// Time Complexity: O(n)
/// Space Complexity: O(n)
pub fn subarray_sum_equals_k(nums: &[i32], k: i32) -> i32 {
    todo!("Implement subarray sum equals k")
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Exercise 1: Basic Two Sum Tests
    #[test]
    fn test_two_sum_basic() {
        assert_eq!(two_sum(&[2, 7, 11, 15], 9), Some((0, 1)));
    }

    #[test]
    fn test_two_sum_duplicate_values() {
        assert_eq!(two_sum(&[3, 3], 6), Some((0, 1)));
    }

    #[test]
    fn test_two_sum_negative() {
        assert_eq!(two_sum(&[-1, -2, 3], 1), Some((0, 2)));
    }

    #[test]
    fn test_two_sum_middle_elements() {
        assert_eq!(two_sum(&[1, 2, 3, 4, 5], 9), Some((3, 4)));
    }

    #[test]
    fn test_two_sum_no_solution() {
        assert_eq!(two_sum(&[1, 2, 3], 10), None);
    }

    #[test]
    fn test_two_sum_empty() {
        assert_eq!(two_sum(&[], 5), None);
    }

    #[test]
    fn test_two_sum_single_element() {
        assert_eq!(two_sum(&[5], 10), None);
    }

    // Exercise 2: Brute Force Tests
    #[test]
    fn test_brute_force_basic() {
        assert_eq!(two_sum_brute_force(&[2, 7, 11, 15], 9), Some((0, 1)));
    }

    #[test]
    fn test_brute_force_no_match() {
        assert_eq!(two_sum_brute_force(&[1, 2, 3], 10), None);
    }

    // Exercise 3: Two-Pass HashMap Tests
    #[test]
    fn test_two_pass_basic() {
        assert_eq!(two_sum_two_pass(&[2, 7, 11, 15], 9), Some((0, 1)));
    }

    #[test]
    fn test_two_pass_negative() {
        assert_eq!(two_sum_two_pass(&[-1, -2, 3], 1), Some((0, 2)));
    }

    // Exercise 4: Two Sum Sorted Tests
    #[test]
    fn test_two_sum_sorted_basic() {
        assert_eq!(two_sum_sorted(&[2, 3, 4], 6), Some((1, 3)));
    }

    #[test]
    fn test_two_sum_sorted_negative() {
        assert_eq!(two_sum_sorted(&[-1, 0, 1, 2, -1, -4], -1), Some((2, 5)));
    }

    #[test]
    fn test_two_sum_sorted_no_match() {
        assert_eq!(two_sum_sorted(&[1, 2, 3], 10), None);
    }

    // Exercise 5: Three Sum Tests
    #[test]
    fn test_three_sum_basic() {
        let result = three_sum(&[-1, 0, 1, 2, -1, -4]);
        assert!(result.contains(&vec![-1, -1, 2]));
        assert!(result.contains(&vec![-1, 0, 1]));
    }

    #[test]
    fn test_three_sum_no_triplet() {
        assert!(three_sum(&[1, 2, 3]).is_empty());
    }

    #[test]
    fn test_three_sum_zeros() {
        let result = three_sum(&[0, 0, 0, 0]);
        assert!(result.contains(&vec![0, 0, 0]));
    }

    #[test]
    fn test_three_sum_large() {
        let result = three_sum(&[-1, 0, 1, 0, -1, 0, 1, 0]);
        let expected: Vec<Vec<i32>> = vec![
            vec![-1, 0, 1],
            vec![-1, 0, 1],
            vec![-1, 0, 0],
            vec![0, 0, 0],
        ];
        // Just check we get some valid results
        for triplet in &expected {
            assert!(result.contains(triplet));
        }
    }

    // Exercise 6: Three Sum Closest Tests
    #[test]
    fn test_three_sum_closest_basic() {
        assert_eq!(three_sum_closest(&[-1, 2, 1, -4], 1), 2);
    }

    #[test]
    fn test_three_sum_closest_exact() {
        assert_eq!(three_sum_closest(&[1, 2, 5, 10], 12), 13);
    }

    // Exercise 7: Four Sum Tests
    #[test]
    fn test_four_sum_basic() {
        let result = four_sum(&[1, 0, -1, 0, -2, 2], 0);
        assert!(result.contains(&vec![-2, -1, 1, 2]));
        assert!(result.contains(&vec![-2, 0, 0, 2]));
        assert!(result.contains(&vec![0, 0, 0, 0]));
    }

    #[test]
    fn test_four_sum_no_match() {
        assert!(four_sum(&[1, 2, 3, 4], 100).is_empty());
    }

    // Exercise 8: Count Pairs with Diff K Tests
    #[test]
    fn test_count_pairs_basic() {
        assert_eq!(count_pairs_with_diff_k(&[1, 2, 4, 5], 3), 2);
    }

    #[test]
    fn test_count_pairs_negative() {
        assert_eq!(count_pairs_with_diff_k(&[-1, -1, -1], 0), 3);
    }

    #[test]
    fn test_count_pairs_no_match() {
        assert_eq!(count_pairs_with_diff_k(&[1, 2, 3], 10), 0);
    }

    // Exercise 9: Find All Pairs Tests
    #[test]
    fn test_find_pairs_basic() {
        let pairs = find_pairs(&[1, 2, 3, 2, 5], 5);
        assert!(pairs.contains(&(2, 3)));
        assert!(pairs.contains(&(2, 2)));
        assert!(pairs.contains(&(3, 2)));
    }

    #[test]
    fn test_find_pairs_no_duplicates() {
        let pairs = find_pairs(&[1, 2, 3, 2, 5], 5);
        // Should not have duplicate pairs
        for (a, b) in &pairs {
            assert_ne!(a, b);
        }
    }

    // Exercise 10: Two Sum Data Stream Tests
    #[test]
    fn test_two_sum_stream_basic() {
        let mut stream = TwoSum::new();
        stream.add(1);
        stream.add(3);
        stream.add(5);
        assert!(stream.find(4));  // 1 + 3 = 4
        assert!(stream.find(6)); // 1 + 5 = 6 or 3 + 5 = 8? No, 1+5=6
        assert!(!stream.find(10));
    }

    #[test]
    fn test_two_sum_stream_update() {
        let mut stream = TwoSum::new();
        stream.add(1);
        assert!(stream.find(2));  // 1 + 1 = 2
        stream.add(2);
        assert!(stream.find(3));  // 1 + 2 = 3
    }

    // Exercise 11: Minimum Operations Tests
    #[test]
    fn test_min_operations_basic() {
        assert_eq!(min_operations(&[1, 1, 4, 2, 3], 5), 2);
    }

    #[test]
    fn test_min_operations_impossible() {
        assert_eq!(min_operations(&[1, 3, 5, 7], 100), -1);
    }

    // Exercise 12: Subarray Sum Equals K Tests
    #[test]
    fn test_subarray_sum_basic() {
        assert_eq!(subarray_sum_equals_k(&[1, 1, 1], 2), 2);
    }

    #[test]
    fn test_subarray_sum_negative() {
        assert_eq!(subarray_sum_equals_k(&[3, 4, -7, 3, 1, 3, 1, 1], 8), 3);
    }

    #[test]
    fn test_subarray_sum_zero() {
        assert_eq!(subarray_sum_equals_k(&[0, 0, 0, 0], 0), 10);
    }
}
// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("$name exercises - run tests with cargo test");
}
