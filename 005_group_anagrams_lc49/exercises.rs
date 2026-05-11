//! Group Anagrams Exercises (LeetCode #49)
//!
//! This module contains exercises for the Group Anagrams problem and its variants.

use std::collections::{HashMap, HashSet};

// ============================================================================
// Exercise 1: Group Anagrams - Sorted Key (Primary Solution)
// ============================================================================

/// Given an array of strings, group the anagrams together.
///
/// Time Complexity: O(n * k log k)
/// Space Complexity: O(n * k)
pub fn group_anagrams(strs: &[String]) -> Vec<Vec<String>> {
    todo!("Implement using sorted key approach")
}

// ============================================================================
// Exercise 2: Group Anagrams - Character Count
// ============================================================================

/// Use character frequency as the grouping key.
///
/// Time Complexity: O(n * k)
/// Space Complexity: O(n * k)
pub fn group_anagrams_count(strs: &[String]) -> Vec<Vec<String>> {
    todo!("Implement using character count key")
}

// ============================================================================
// Exercise 3: Group Anagrams - Prime Product
// ============================================================================

/// Use prime number multiplication for a potentially smaller key.
///
/// Time Complexity: O(n * k)
/// Space Complexity: O(n * k)
pub fn group_anagrams_prime(strs: &[String]) -> Vec<Vec<String>> {
    todo!("Implement using prime product key")
}

// ============================================================================
// Exercise 4: Count Number of Groups
// ============================================================================

/// Return the number of distinct anagram groups.
///
/// Time Complexity: O(n * k log k)
/// Space Complexity: O(n * k)
pub fn count_anagram_groups(strs: &[String]) -> i32 {
    todo!("Count the number of anagram groups")
}

// ============================================================================
// Exercise 5: Find Largest Anagram Group Size
// ============================================================================

/// Find the size of the largest anagram group.
///
/// Time Complexity: O(n * k log k)
/// Space Complexity: O(n * k)
pub fn largest_anagram_group_size(strs: &[String]) -> usize {
    todo!("Find the size of the largest group")
}

// ============================================================================
// Exercise 6: Find Groups with Minimum Size
// ============================================================================

/// Given an integer k, return groups with at least k members.
///
/// Time Complexity: O(n * k log k)
/// Space Complexity: O(n * k)
pub fn find_groups_with_min_size(strs: &[String], k: usize) -> Vec<Vec<String>> {
    todo!("Find groups with at least k members")
}

// ============================================================================
// Exercise 7: Check if String is Anagram of Any in List
// ============================================================================

/// Check if a string is an anagram of any string in the list.
///
/// Time Complexity: O(n * k)
/// Space Complexity: O(k)
pub fn is_anagram_of_any(s: &str, strs: &[String]) -> bool {
    todo!("Check if s is anagram of any string in list")
}

// ============================================================================
// Exercise 8: Find Minimum Group Difference Size
// ============================================================================

/// Find the minimum difference in size between the largest and smallest groups.
///
/// Time Complexity: O(n * k log k)
/// Space Complexity: O(n * k)
pub fn min_group_difference(strs: &[String]) -> usize {
    todo!("Find minimum difference between group sizes")
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Exercise 1: Basic Group Anagrams Tests
    #[test]
    fn test_group_anagrams_basic() {
        let strs = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        let result = group_anagrams(&strs);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_group_anagrams_empty() {
        let strs: Vec<String> = vec![];
        let result = group_anagrams(&strs);
        assert!(result.is_empty());
    }

    #[test]
    fn test_group_anagrams_single() {
        let strs = vec!["a".to_string()];
        let result = group_anagrams(&strs);
        assert_eq!(result.len(), 1);
        assert!(result[0].contains(&"a".to_string()));
    }

    #[test]
    fn test_group_anagrams_all_same() {
        let strs = vec!["a".to_string(), "a".to_string(), "a".to_string()];
        let result = group_anagrams(&strs);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 3);
    }

    #[test]
    fn test_group_anagrams_empty_string() {
        let strs = vec!["".to_string(), "a".to_string()];
        let result = group_anagrams(&strs);
        // Empty string and "a" should be in separate groups
        assert!(result.iter().any(|g| g.contains(&"".to_string())));
        assert!(result.iter().any(|g| g.contains(&"a".to_string())));
    }

    // Exercise 2: Character Count Tests
    #[test]
    fn test_group_anagrams_count_basic() {
        let strs = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
        ];
        let result = group_anagrams_count(&strs);
        assert_eq!(result.len(), 2); // "eat","tea" together, "tan" alone
    }

    #[test]
    fn test_group_anagrams_count_all_same() {
        let strs = vec!["abc".to_string(), "bca".to_string(), "cab".to_string()];
        let result = group_anagrams_count(&strs);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 3);
    }

    // Exercise 3: Prime Product Tests
    #[test]
    fn test_group_anagrams_prime_basic() {
        let strs = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
        ];
        let result = group_anagrams_prime(&strs);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_group_anagrams_prime_all_same() {
        let strs = vec!["abc".to_string(), "bca".to_string()];
        let result = group_anagrams_prime(&strs);
        assert_eq!(result.len(), 1);
    }

    // Exercise 4: Count Groups Tests
    #[test]
    fn test_count_groups_basic() {
        let strs = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        assert_eq!(count_anagram_groups(&strs), 3);
    }

    #[test]
    fn test_count_groups_none() {
        let strs = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        assert_eq!(count_anagram_groups(&strs), 3);
    }

    #[test]
    fn test_count_groups_empty() {
        let strs: Vec<String> = vec![];
        assert_eq!(count_anagram_groups(&strs), 0);
    }

    // Exercise 5: Largest Group Size Tests
    #[test]
    fn test_largest_group_size_basic() {
        let strs = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        assert_eq!(largest_anagram_group_size(&strs), 3);
    }

    #[test]
    fn test_largest_group_size_single() {
        let strs = vec!["a".to_string()];
        assert_eq!(largest_anagram_group_size(&strs), 1);
    }

    // Exercise 6: Min Size Groups Tests
    #[test]
    fn test_find_groups_min_size_basic() {
        let strs = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        let result = find_groups_with_min_size(&strs, 2);
        assert_eq!(result.len(), 2); // "eat,tea,ate" and "tan,nat"
    }

    #[test]
    fn test_find_groups_min_size_none() {
        let strs = vec![
            "eat".to_string(),
            "tea".to_string(),
            "bat".to_string(),
        ];
        let result = find_groups_with_min_size(&strs, 3);
        assert_eq!(result.len(), 1); // Only one group has 3 members
    }

    // Exercise 7: Is Anagram of Any Tests
    #[test]
    fn test_is_anagram_of_any_true() {
        let strs = vec!["eat".to_string(), "tea".to_string()];
        assert!(is_anagram_of_any("ate", &strs));
    }

    #[test]
    fn test_is_anagram_of_any_false() {
        let strs = vec!["eat".to_string(), "tea".to_string()];
        assert!(!is_anagram_of_any("xyz", &strs));
    }

    #[test]
    fn test_is_anagram_of_any_empty() {
        let strs: Vec<String> = vec![];
        assert!(!is_anagram_of_any("a", &strs));
    }

    // Exercise 8: Min Group Difference Tests
    #[test]
    fn test_min_group_difference_basic() {
        let strs = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        assert_eq!(min_group_difference(&strs), 1); // "bat" has 1, others have 3 or 2
    }

    #[test]
    fn test_min_group_difference_equal() {
        let strs = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
        ];
        assert_eq!(min_group_difference(&strs), 0); // All groups have size 1
    }
}
// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("$name exercises - run tests with cargo test");
}
