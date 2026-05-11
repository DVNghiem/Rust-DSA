//! Product of Array Except Self Exercises (LeetCode #238)
//!
//! This module contains exercises for the Product Except Self problem.

use std::collections::HashMap;

// ============================================================================
// Exercise 1: Product Except Self - Prefix Suffix (Primary Solution)
// ============================================================================

/// Given an array nums, return an array where answer[i] equals the product
/// of all elements except nums[i].
///
/// Time Complexity: O(n)
/// Space Complexity: O(n)
pub fn product_except_self(nums: &[i32]) -> Vec<i32> {
    todo!("Implement using prefix and suffix products")
}

// ============================================================================
// Exercise 2: Product Except Self - With Division
// ============================================================================

/// Use division to calculate (requires handling zeros).
///
/// Time Complexity: O(n)
/// Space Complexity: O(n)
pub fn product_except_self_with_divide(nums: &[i32]) -> Vec<i32> {
    todo!("Implement using division (handle zeros)")
}

// ============================================================================
// Exercise 3: Product Except Self - Explicit Zero Handling
// ============================================================================

/// Handle zeros explicitly without division.
///
/// Time Complexity: O(n)
/// Space Complexity: O(n)
pub fn product_except_self_with_zeros(nums: &[i32]) -> Vec<i32> {
    todo!("Handle zeros explicitly")
}

// ============================================================================
// Exercise 4: Find Zero Positions
// ============================================================================

/// Return the indices of all zeros in the array.
///
/// Time Complexity: O(n)
/// Space Complexity: O(k) where k is number of zeros
pub fn find_zero_positions(nums: &[i32]) -> Vec<usize> {
    todo!("Find all positions where nums[i] == 0")
}

// ============================================================================
// Exercise 5: Product of Range
// ============================================================================

/// Return the product of all elements in the range [left, right] inclusive.
///
/// Time Complexity: O(n) preprocessing, O(1) per query
/// Space Complexity: O(n)
pub struct ProductQuery {
    prefix_products: Vec<i64>,
}

impl ProductQuery {
    pub fn new(nums: &[i32]) -> Self {
        todo!("Build prefix products for range queries")
    }

    pub fn product(&self, left: usize, right: usize) -> i64 {
        todo!("Return product of nums[left..=right]")
    }
}

// ============================================================================
// Exercise 6: Product of Self with i64
// ============================================================================

/// Same as product_except_self but using i64 to avoid intermediate overflow.
///
/// Time Complexity: O(n)
/// Space Complexity: O(n)
pub fn product_except_self_i64(nums: &[i32]) -> Vec<i64> {
    todo!("Use i64 to handle potential overflow")
}

// ============================================================================
// Exercise 7: Check if Product Matches
// ============================================================================

/// Check if the given result array matches the expected product except self.
///
/// Time Complexity: O(n)
/// Space Complexity: O(1)
pub fn verify_product_except_self(nums: &[i32], result: &[i32]) -> bool {
    todo!("Verify that result[i] equals product of all nums except nums[i]")
}

// ============================================================================
// Exercise 8: Find Subarray with Product K
// ============================================================================

/// Find if there exists a subarray whose product equals k.
///
/// Time Complexity: O(n)
/// Space Complexity: O(1)
pub fn has_subarray_product_k(nums: &[i32], k: i32) -> bool {
    todo!("Find if any subarray product equals k")
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Exercise 1: Basic Product Except Self Tests
    #[test]
    fn test_product_except_self_basic() {
        assert_eq!(product_except_self(&[1, 2, 3, 4]), vec![24, 12, 8, 6]);
    }

    #[test]
    fn test_product_except_self_single() {
        assert_eq!(product_except_self(&[5]), vec![1]);
    }

    #[test]
    fn test_product_except_self_two() {
        assert_eq!(product_except_self(&[1, 2]), vec![2, 1]);
    }

    #[test]
    fn test_product_except_self_negative() {
        assert_eq!(product_except_self(&[-1, 1, 0, -3, 3]), vec![0, 0, 9, 0, 0]);
    }

    #[test]
    fn test_product_except_self_zeros() {
        assert_eq!(product_except_self(&[0, 2, 3]), vec![6, 0, 0]);
    }

    #[test]
    fn test_product_except_self_multiple_zeros() {
        assert_eq!(product_except_self(&[0, 0, 5]), vec![0, 0, 0]);
    }

    #[test]
    fn test_product_except_self_empty() {
        assert_eq!(product_except_self(&[]), vec![]);
    }

    // Exercise 2: Division Approach Tests
    #[test]
    fn test_with_divide_basic() {
        assert_eq!(product_except_self_with_divide(&[1, 2, 3, 4]), vec![24, 12, 8, 6]);
    }

    #[test]
    fn test_with_divide_single_zero() {
        assert_eq!(product_except_self_with_divide(&[1, 0, 3, 4]), vec![0, 12, 0, 0]);
    }

    #[test]
    fn test_with_divide_multiple_zeros() {
        assert_eq!(product_except_self_with_divide(&[0, 2, 0, 4]), vec![0, 0, 0, 0]);
    }

    // Exercise 3: Explicit Zero Handling Tests
    #[test]
    fn test_with_zeros_basic() {
        assert_eq!(product_except_self_with_zeros(&[1, 2, 3, 4]), vec![24, 12, 8, 6]);
    }

    #[test]
    fn test_with_zeros_single_zero() {
        assert_eq!(product_except_self_with_zeros(&[0, 1, 2]), vec![2, 0, 0]);
    }

    // Exercise 4: Find Zero Positions Tests
    #[test]
    fn test_find_zero_positions_basic() {
        assert_eq!(find_zero_positions(&[1, 0, 3, 0]), vec![1, 3]);
    }

    #[test]
    fn test_find_zero_positions_none() {
        assert_eq!(find_zero_positions(&[1, 2, 3]), vec![]);
    }

    #[test]
    fn test_find_zero_positions_all() {
        assert_eq!(find_zero_positions(&[0, 0, 0]), vec![0, 1, 2]);
    }

    // Exercise 5: Product Range Tests
    #[test]
    fn test_product_range_basic() {
        let query = ProductQuery::new(&[1, 2, 3, 4]);
        assert_eq!(query.product(0, 3), 24);
        assert_eq!(query.product(1, 2), 6);
        assert_eq!(query.product(0, 0), 1);
    }

    #[test]
    fn test_product_range_single() {
        let query = ProductQuery::new(&[5]);
        assert_eq!(query.product(0, 0), 5);
    }

    #[test]
    fn test_product_range_negative() {
        let query = ProductQuery::new(&[-1, 2, -3, 4]);
        assert_eq!(query.product(0, 3), 24);
    }

    // Exercise 6: i64 Version Tests
    #[test]
    fn test_product_except_self_i64_basic() {
        assert_eq!(product_except_self_i64(&[1, 2, 3, 4]), vec![24, 12, 8, 6]);
    }

    #[test]
    fn test_product_except_self_i64_large() {
        let nums = vec![10, 20, 30, 40, 50];
        let result = product_except_self_i64(&nums);
        assert_eq!(result[0], 20 * 30 * 40 * 50);
    }

    // Exercise 7: Verification Tests
    #[test]
    fn test_verify_correct() {
        assert!(verify_product_except_self(&[1, 2, 3, 4], &[24, 12, 8, 6]));
    }

    #[test]
    fn test_verify_incorrect() {
        assert!(!verify_product_except_self(&[1, 2, 3, 4], &[24, 12, 8, 7]));
    }

    #[test]
    fn test_verify_with_zero() {
        assert!(verify_product_except_self(&[1, 0, 3], &[0, 3, 0]));
    }

    // Exercise 8: Subarray Product Tests
    #[test]
    fn test_has_subarray_product_k_true() {
        assert!(has_subarray_product_k(&[1, 2, 3, 4], 12));
    }

    #[test]
    fn test_has_subarray_product_k_false() {
        assert!(!has_subarray_product_k(&[1, 2, 3, 4], 100));
    }

    #[test]
    fn test_has_subarray_product_k_single() {
        assert!(has_subarray_product_k(&[5], 5));
    }

    #[test]
    fn test_has_subarray_product_k_with_zero() {
        assert!(has_subarray_product_k(&[1, 0, 2, 3], 0));
    }
}
// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("$name exercises - run tests with cargo test");
}
