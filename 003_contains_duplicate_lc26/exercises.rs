//! Contains Duplicate Exercises (LeetCode #26)
//!
//! This module contains exercises for the Contains Duplicate problem and its variants.

use std::collections::{HashMap, HashSet};

// ============================================================================
// Exercise 1: Contains Duplicate - HashSet (Primary Solution)
// ============================================================================

/// Given an integer array, return true if any value appears at least twice.
/// Return false if every element is distinct.
///
/// Time Complexity: O(n)
/// Space Complexity: O(n)
pub fn contains_duplicate(nums: &[i32]) -> bool {
    todo!("Implement using HashSet")
}

// ============================================================================
// Exercise 2: Contains Duplicate - Sort First
// ============================================================================

/// Sort the array first, then check adjacent elements for duplicates.
///
/// Time Complexity: O(n log n)
/// Space Complexity: O(n) for the sorted copy
pub fn contains_duplicate_sort(nums: &[i32]) -> bool {
    todo!("Implement using sort and adjacent comparison")
}

// ============================================================================
// Exercise 3: Contains Duplicate - Brute Force
// ============================================================================

/// Check every pair of elements for equality.
///
/// Time Complexity: O(n²)
/// Space Complexity: O(1)
pub fn contains_duplicate_brute_force(nums: &[i32]) -> bool {
    todo!("Implement brute force comparison")
}

// ============================================================================
// Exercise 4: Contains Duplicate II (LC 219)
// ============================================================================

/// Given an integer array and an integer k, return true if there are two distinct
/// indices i and j such that nums[i] == nums[j] and abs(i - j) <= k.
///
/// Time Complexity: O(n)
/// Space Complexity: O(min(n, k))
pub fn contains_nearby_duplicate(nums: &[i32], k: i32) -> bool {
    todo!("Implement contains nearby duplicate")
}

// ============================================================================
// Exercise 5: Contains Duplicate III (LC 220)
// ============================================================================

/// Given an integer array, an integer k, and an integer t, return true if there are
/// two distinct indices i and j such that abs(nums[i] - nums[j]) <= t and
/// abs(i - j) <= k.
///
/// Time Complexity: O(n log k)
/// Space Complexity: O(k)
pub fn contains_nearby_almost_duplicate(nums: &[i32], k: i32, t: i32) -> bool {
    todo!("Implement contains nearby almost duplicate")
}

// ============================================================================
// Exercise 6: Count Distinct Elements
// ============================================================================

/// Count how many distinct elements are in the array.
///
/// Time Complexity: O(n)
/// Space Complexity: O(n)
pub fn count_distinct(nums: &[i32]) -> usize {
    todo!("Count distinct elements using HashSet")
}

// ============================================================================
// Exercise 7: Find All Duplicate Indices
// ============================================================================

/// Given an array of integers where 1 <= nums[i] <= n (n = size of array),
/// find all duplicates. Each number appears once or twice.
///
/// Time Complexity: O(n)
/// Space Complexity: O(1) without considering output
pub fn find_all_duplicates(nums: &[i32]) -> Vec<i32> {
    todo!("Find all duplicate elements")
}

// ============================================================================
// Exercise 8: Contains Any Duplicate in Range
// ============================================================================

/// Check if the array contains any duplicate in the range [left, right].
///
/// Time Complexity: O(n)
/// Space Complexity: O(n)
pub fn contains_duplicate_in_range(nums: &[i32], left: i32, right: i32) -> bool {
    todo!("Check duplicates within specific range")
}

// ============================================================================
// Exercise 9: Number of Unique Elements
// ============================================================================

/// Return the count of unique elements in the array.
///
/// Time Complexity: O(n)
/// Space Complexity: O(n)
pub fn count_unique(nums: &[i32]) -> usize {
    todo!("Count unique elements")
}

// ============================================================================
// Exercise 10: Check if Array Has All Duplicates
// ============================================================================

/// Check if the array contains only duplicates (no unique elements).
/// Return true if every element appears at least twice.
///
/// Time Complexity: O(n)
/// Space Complexity: O(n)
pub fn has_all_duplicates(nums: &[i32]) -> bool {
    todo!("Check if all elements are duplicates")
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Exercise 1: Basic Contains Duplicate Tests
    #[test]
    fn test_contains_duplicate_true() {
        assert!(contains_duplicate(&[1, 2, 3, 1]));
    }

    #[test]
    fn test_contains_duplicate_false() {
        assert!(!contains_duplicate(&[1, 2, 3, 4]));
    }

    #[test]
    fn test_contains_duplicate_empty() {
        assert!(!contains_duplicate(&[]));
    }

    #[test]
    fn test_contains_duplicate_single() {
        assert!(!contains_duplicate(&[1]));
    }

    #[test]
    fn test_contains_duplicate_all_same() {
        assert!(contains_duplicate(&[1, 1, 1, 1]));
    }

    #[test]
    fn test_contains_duplicate_negative() {
        assert!(contains_duplicate(&[-1, -1, 0, 1]));
    }

    #[test]
    fn test_contains_duplicate_two_elements() {
        assert!(contains_duplicate(&[1, 1]));
    }

    // Exercise 2: Sort-based Tests
    #[test]
    fn test_contains_duplicate_sort_true() {
        assert!(contains_duplicate_sort(&[1, 2, 3, 1]));
    }

    #[test]
    fn test_contains_duplicate_sort_false() {
        assert!(!contains_duplicate_sort(&[1, 2, 3, 4]));
    }

    // Exercise 3: Brute Force Tests
    #[test]
    fn test_brute_force_true() {
        assert!(contains_duplicate_brute_force(&[1, 2, 3, 1]));
    }

    #[test]
    fn test_brute_force_false() {
        assert!(!contains_duplicate_brute_force(&[1, 2, 3, 4]));
    }

    // Exercise 4: Contains Nearby Duplicate Tests
    #[test]
    fn test_nearby_duplicate_true() {
        assert!(contains_nearby_duplicate(&[1, 2, 3, 1], 3));
    }

    #[test]
    fn test_nearby_duplicate_false() {
        assert!(!contains_nearby_duplicate(&[1, 2, 3, 4, 5], 2));
    }

    #[test]
    fn test_nearby_duplicate_k_zero() {
        assert!(contains_nearby_duplicate(&[1, 1], 0));
    }

    #[test]
    fn test_nearby_duplicate_k_one() {
        assert!(contains_nearby_duplicate(&[1, 2, 1], 1));
    }

    #[test]
    fn test_nearby_duplicate_no_duplicate() {
        assert!(!contains_nearby_duplicate(&[1, 2, 3, 4], 3));
    }

    // Exercise 5: Contains Nearby Almost Duplicate Tests
    #[test]
    fn test_nearby_almost_duplicate_true() {
        assert!(contains_nearby_almost_duplicate(&[1, 2, 3, 1], 3, 0));
    }

    #[test]
    fn test_nearby_almost_duplicate_false() {
        assert!(!contains_nearby_almost_duplicate(&[1, 5, 9, 1], 2, 2));
    }

    // Exercise 6: Count Distinct Tests
    #[test]
    fn test_count_distinct_basic() {
        assert_eq!(count_distinct(&[1, 2, 3, 1, 2, 4]), 4);
    }

    #[test]
    fn test_count_distinct_all_same() {
        assert_eq!(count_distinct(&[1, 1, 1, 1]), 1);
    }

    #[test]
    fn test_count_distinct_all_different() {
        assert_eq!(count_distinct(&[1, 2, 3, 4]), 4);
    }

    #[test]
    fn test_count_distinct_empty() {
        assert_eq!(count_distinct(&[]), 0);
    }

    // Exercise 7: Find All Duplicates Tests
    #[test]
    fn test_find_all_duplicates_basic() {
        let result = find_all_duplicates(&[4, 3, 2, 7, 8, 2, 3, 1]);
        assert!(result.contains(&2));
        assert!(result.contains(&3));
    }

    #[test]
    fn test_find_all_duplicates_no_duplicates() {
        assert!(find_all_duplicates(&[1, 2, 3, 4]).is_empty());
    }

    // Exercise 8: Range-based Duplicates Tests
    #[test]
    fn test_range_duplicates_true() {
        assert!(contains_duplicate_in_range(&[1, 2, 3, 2], 1, 3));
    }

    #[test]
    fn test_range_duplicates_false() {
        assert!(!contains_duplicate_in_range(&[1, 2, 3, 4], 1, 3));
    }

    // Exercise 9: Count Unique Tests
    #[test]
    fn test_count_unique_basic() {
        assert_eq!(count_unique(&[1, 2, 3, 1, 2, 4]), 4);
    }

    #[test]
    fn test_count_unique_all_same() {
        assert_eq!(count_unique(&[1, 1, 1, 1]), 1);
    }

    // Exercise 10: All Duplicates Tests
    #[test]
    fn test_has_all_duplicates_true() {
        assert!(has_all_duplicates(&[1, 1, 2, 2]));
    }

    #[test]
    fn test_has_all_duplicates_false() {
        assert!(!has_all_duplicates(&[1, 2, 1, 2]));
    }

    #[test]
    fn test_has_all_duplicates_all_same() {
        assert!(has_all_duplicates(&[1, 1, 1]));
    }

    #[test]
    fn test_has_all_duplicates_mixed() {
        assert!(!has_all_duplicates(&[1, 1, 2, 2, 3]));
    }
}
// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("$name exercises - run tests with cargo test");
}
