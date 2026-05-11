//! Valid Anagram Exercises (LeetCode #242)
//!
//! This module contains exercises for the Valid Anagram problem and its variants.

use std::collections::{HashMap, HashSet};

// ============================================================================
// Exercise 1: Valid Anagram - HashMap (Primary Solution)
// ============================================================================

/// Given two strings s and t, return true if t is an anagram of s.
///
/// Time Complexity: O(n)
/// Space Complexity: O(k) where k is number of unique characters
pub fn is_anagram(s: &str, t: &str) -> bool {
    todo!("Implement using HashMap character counting")
}

// ============================================================================
// Exercise 2: Valid Anagram - Fixed Size Array
// ============================================================================

/// Given only lowercase English letter strings, use a fixed-size array.
///
/// Time Complexity: O(n)
/// Space Complexity: O(1) - only 26 integers
pub fn is_anagram_array(s: &str, t: &str) -> bool {
    todo!("Implement using fixed-size array for 26 letters")
}

// ============================================================================
// Exercise 3: Valid Anagram - Sort and Compare
// ============================================================================

/// Sort both strings and compare character by character.
///
/// Time Complexity: O(n log n)
/// Space Complexity: O(n)
pub fn is_anagram_sort(s: &str, t: &str) -> bool {
    todo!("Implement by sorting strings")
}

// ============================================================================
// Exercise 4: Check if Two Strings are Equivalent (LC 953)
// ============================================================================

/// Two strings are equivalent if they can be made identical by appending
/// characters to either string. Return true if equivalent.
///
/// Time Complexity: O(n)
/// Space Complexity: O(n)
pub fn are_strings_equivalent(s1: &str, s2: &str) -> bool {
    todo!("Check if strings are equivalent")
}

// ============================================================================
// Exercise 5: Find Minimum Steps to Make Two Strings Anagram (LC 1347)
// ============================================================================

/// Given two strings s and t, return the minimum number of steps to make
/// t an anagram of s. You may modify string t by replacing a character.
///
/// Time Complexity: O(n)
/// Space Complexity: O(1) with fixed array
pub fn min_steps_to_make_anagram(s: &str, t: &str) -> i32 {
    todo!("Find minimum steps to make t an anagram of s")
}

// ============================================================================
// Exercise 6: Group Anagrams (LC 49)
// ============================================================================

/// Given an array of strings, group anagrams together.
///
/// Time Complexity: O(n * k log k) where k is max string length
/// Space Complexity: O(n * k)
pub fn group_anagrams(strs: &[&str]) -> Vec<Vec<String>> {
    todo!("Group anagrams together")
}

// ============================================================================
// Exercise 7: Find Anagram Mappings (LC 760)
// ============================================================================

/// Given two arrays A and B, find an index mapping P such that
/// A[i] is an anagram of B[P[i]].
///
/// Time Complexity: O(n)
/// Space Complexity: O(n)
pub fn anagram_mappings(a: &[i32], b: &[i32]) -> Vec<i32> {
    todo!("Find anagram mappings from A to B")
}

// ============================================================================
// Exercise 8: Valid Palindrome II (LC 680)
// ============================================================================

/// Given a string, return true if it can be made a palindrome by removing
/// at most one character.
///
/// Time Complexity: O(n)
/// Space Complexity: O(1)
pub fn valid_palindrome_deletion(s: &str) -> bool {
    todo!("Check if palindrome possible with one deletion")
}

// ============================================================================
// Exercise 9: Count Anagrams in String (Custom)
// ============================================================================

/// Count how many substrings of length k in string s are anagrams of string p.
///
/// Time Complexity: O(n)
/// Space Complexity: O(k)
pub fn count_anagrams_in_string(s: &str, k: usize) -> i32 {
    todo!("Count anagram substrings of length k")
}

// ============================================================================
// Exercise 10: Minimum Deletions to Make Anagram (Custom)
// ============================================================================

/// Find minimum deletions needed to make strings s and t anagrams of each other.
///
/// Time Complexity: O(n)
/// Space Complexity: O(1) with fixed array
pub fn min_deletions_to_anagram(s: &str, t: &str) -> i32 {
    todo!("Find minimum deletions required")
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Exercise 1: Basic Valid Anagram Tests
    #[test]
    fn test_is_anagram_true() {
        assert!(is_anagram("anagram", "nagaram"));
    }

    #[test]
    fn test_is_anagram_false() {
        assert!(!is_anagram("rat", "car"));
    }

    #[test]
    fn test_is_anagram_empty() {
        assert!(is_anagram("", ""));
    }

    #[test]
    fn test_is_anagram_single_char() {
        assert!(is_anagram("a", "a"));
    }

    #[test]
    fn test_is_anagram_different_length() {
        assert!(!is_anagram("abc", "ab"));
    }

    #[test]
    fn test_is_anagram_case_sensitive() {
        // Assuming case-sensitive as per LeetCode 242
        assert!(!is_anagram("Anagram", "nagaram"));
    }

    // Exercise 2: Array-based Tests
    #[test]
    fn test_is_anagram_array_true() {
        assert!(is_anagram_array("anagram", "nagaram"));
    }

    #[test]
    fn test_is_anagram_array_false() {
        assert!(!is_anagram_array("rat", "car"));
    }

    #[test]
    fn test_is_anagram_array_length_mismatch() {
        assert!(!is_anagram_array("abc", "ab"));
    }

    // Exercise 3: Sort-based Tests
    #[test]
    fn test_is_anagram_sort_true() {
        assert!(is_anagram_sort("anagram", "nagaram"));
    }

    #[test]
    fn test_is_anagram_sort_false() {
        assert!(!is_anagram_sort("rat", "car"));
    }

    // Exercise 4: String Equivalence Tests
    #[test]
    fn test_are_equivalent_true() {
        assert!(are_strings_equivalent("abc", "abc"));
    }

    #[test]
    fn test_are_equivalent_append() {
        assert!(are_strings_equivalent("ab", "bab"));
    }

    #[test]
    fn test_are_equivalent_false() {
        assert!(!are_strings_equivalent("abc", "xyz"));
    }

    // Exercise 5: Min Steps Tests
    #[test]
    fn test_min_steps_basic() {
        assert_eq!(min_steps_to_make_anagram("abc", "abc"), 0);
    }

    #[test]
    fn test_min_steps_one_change() {
        assert_eq!(min_steps_to_make_anagram("abc", "def"), 3);
    }

    #[test]
    fn test_min_steps_partial() {
        assert_eq!(min_steps_to_make_anagram("bab", "aba"), 1);
    }

    // Exercise 6: Group Anagrams Tests
    #[test]
    fn test_group_anagrams_basic() {
        let result = group_anagrams(&["eat", "tea", "tan", "ate", "nat", "bat"]);
        assert!(result.iter().any(|g| g.contains(&"eat".to_string()) &&
                                   g.contains(&"tea".to_string()) &&
                                   g.contains(&"ate".to_string())));
    }

    #[test]
    fn test_group_anagrams_empty() {
        assert!(group_anagrams(&[]).is_empty());
    }

    #[test]
    fn test_group_anagrams_single() {
        let result = group_anagrams(&["hello"]);
        assert_eq!(result.len(), 1);
    }

    // Exercise 7: Anagram Mappings Tests
    #[test]
    fn test_anagram_mappings_basic() {
        let a = vec![12, 28, 22, 92, 22];
        let b = vec![22, 92, 22, 12, 28];
        let result = anagram_mappings(&a, &b);
        assert_eq!(result.len(), 5);
    }

    #[test]
    fn test_anagram_mappings_simple() {
        let a = vec![1, 2, 3];
        let b = vec![2, 3, 1];
        let result = anagram_mappings(&a, &b);
        // Check that mapping produces valid anagram pairs
        for i in 0..a.len() {
            let mut a_copy = vec![a[i]];
            let mut b_copy = vec![b[result[i] as usize]];
            a_copy.sort();
            b_copy.sort();
            assert_eq!(a_copy, b_copy);
        }
    }

    // Exercise 8: Valid Palindrome II Tests
    #[test]
    fn test_valid_palindrome_deletion_true() {
        assert!(valid_palindrome_deletion("aba"));
    }

    #[test]
    fn test_valid_palindrome_deletion_one_del() {
        assert!(valid_palindrome_deletion("abca"));
    }

    #[test]
    fn test_valid_palindrome_deletion_false() {
        assert!(!valid_palindrome_deletion("abc"));
    }

    #[test]
    fn test_valid_palindrome_deletion_no_del() {
        assert!(valid_palindrome_deletion("racecar"));
    }

    // Exercise 9: Count Anagrams Tests
    #[test]
    fn test_count_anagrams_basic() {
        assert_eq!(count_anagrams_in_string("abc", 2), 2);
    }

    #[test]
    fn test_count_anagrams_none() {
        assert_eq!(count_anagrams_in_string("abc", 3), 1);
    }

    #[test]
    fn test_count_anagrams_overlapping() {
        // "ab" and "bc" are both anagrams of "ab", "bc"
        assert_eq!(count_anagrams_in_string("abc", 2), 2);
    }

    // Exercise 10: Min Deletions Tests
    #[test]
    fn test_min_deletions_no_deletion() {
        assert_eq!(min_deletions_to_anagram("abc", "abc"), 0);
    }

    #[test]
    fn test_min_deletions_basic() {
        assert_eq!(min_deletions_to_anagram("abc", "adbc"), 1);
    }

    #[test]
    fn test_min_deletions_multiple() {
        assert_eq!(min_deletions_to_anagram("abc", "def"), 6);
    }
}
// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("$name exercises - run tests with cargo test");
}
