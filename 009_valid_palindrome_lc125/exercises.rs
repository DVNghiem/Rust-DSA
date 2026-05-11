//! Valid Palindrome Exercises (LeetCode #125)
//!
//! This module contains exercises for the Valid Palindrome problem.

use std::collections::VecDeque;

// ============================================================================
// Exercise 1: Valid Palindrome - Two Pointers (Primary Solution)
// ============================================================================

/// Given a string, return true if it is a palindrome after cleaning.
///
/// Time Complexity: O(n)
/// Space Complexity: O(k) where k is number of alphanumeric chars
pub fn is_palindrome(s: &str) -> bool {
    todo!("Implement two pointers solution")
}

// ============================================================================
// Exercise 2: Valid Palindrome II (LC 680)
// ============================================================================

/// Given a string, return true if it can become a palindrome by removing
/// at most one character.
///
/// Time Complexity: O(n)
/// Space Complexity: O(1)
pub fn valid_palindrome_deletion(s: &str) -> bool {
    todo!("Implement with at most one deletion allowed")
}

// ============================================================================
// Exercise 3: Palindrome Number (LC 9)
// ============================================================================

/// Given an integer, return true if it is a palindrome. Do not use strings.
///
/// Time Complexity: O(n)
/// Space Complexity: O(1)
pub fn is_palindrome_number(x: i32) -> bool {
    todo!("Check if integer is palindrome without string conversion")
}

// ============================================================================
// Exercise 4: Longest Palindromic Substring (LC 5)
// ============================================================================

/// Find the longest palindromic substring in s.
///
/// Time Complexity: O(n²)
/// Space Complexity: O(1)
pub fn longest_palindromic_substring(s: &str) -> String {
    todo!("Find the longest palindromic substring")
}

// ============================================================================
// Exercise 5: Palindrome Linked List (LC 234)
// ============================================================================

/// Given a linked list, determine if it is a palindrome.
///
/// Time Complexity: O(n)
/// Space Complexity: O(1)
pub fn is_palindrome_list(head: Option<Box<ListNode>>) -> bool {
    todo!("Check if linked list is palindrome")
}

// ============================================================================
// Exercise 6: Longest Palindrome (LC 409)
// ============================================================================

/// Given a string, find the length of the longest palindrome that can be
/// built with the letters from the string.
///
/// Time Complexity: O(n)
/// Space Complexity: O(1) with fixed array
pub fn longest_palindrome_len(s: &str) -> usize {
    todo!("Find length of longest palindrome from letters")
}

// ============================================================================
// Exercise 7: Palindrome Pairs (LC 336)
// ============================================================================

/// Given a list of words, find all pairs of indices (i, j) such that
/// words[i] + words[j] is a palindrome.
///
/// Time Complexity: O(n * k²)
/// Space Complexity: O(n)
pub fn palindrome_pairs(words: &[&str]) -> Vec<(usize, usize)> {
    todo!("Find all palindrome pairs")
}

// ============================================================================
// Exercise 8: Check Palindrome After Character Removal
// ============================================================================

/// Check if string can become palindrome after removing all occurrences of char.
///
/// Time Complexity: O(n)
/// Space Complexity: O(1)
pub fn can_make_palindrome_after_removal(s: &str, c: char) -> bool {
    todo!("Check if palindrome possible after removing all c")
}

// ============================================================================
// ListNode definition for Exercise 5
// ============================================================================

#[derive(Debug, PartialEq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }

    pub fn from_slice(vals: &[i32]) -> Option<Box<ListNode>> {
        if vals.is_empty() {
            return None;
        }
        let mut head = Box::new(ListNode::new(vals[0]));
        let mut current = &mut head;
        for &val in &vals[1..] {
            current.next = Some(Box::new(ListNode::new(val)));
            current = current.next.as_mut().unwrap();
        }
        Some(head)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Exercise 1: Basic Palindrome Tests
    #[test]
    fn test_is_palindrome_true() {
        assert!(is_palindrome("A man, a plan, a canal: Panama"));
    }

    #[test]
    fn test_is_palindrome_false() {
        assert!(!is_palindrome("race a car"));
    }

    #[test]
    fn test_is_palindrome_empty() {
        assert!(is_palindrome(""));
    }

    #[test]
    fn test_is_palindrome_single() {
        assert!(is_palindrome("a"));
    }

    #[test]
    fn test_is_palindrome_numbers() {
        assert!(is_palindrome("A1a"));
    }

    #[test]
    fn test_is_palindrome_only_non_alphanumeric() {
        assert!(is_palindrome("!@#$%^&*()"));
    }

    #[test]
    fn test_is_palindrome_spaces() {
        assert!(is_palindrome("a b a"));
    }

    // Exercise 2: Valid Palindrome II Tests
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
    fn test_valid_palindrome_deletion_abcba() {
        assert!(valid_palindrome_deletion("abcba"));
    }

    // Exercise 3: Palindrome Number Tests
    #[test]
    fn test_is_palindrome_number_true() {
        assert!(is_palindrome_number(121));
    }

    #[test]
    fn test_is_palindrome_number_false() {
        assert!(!is_palindrome_number(123));
    }

    #[test]
    fn test_is_palindrome_number_negative() {
        assert!(!is_palindrome_number(-121));
    }

    #[test]
    fn test_is_palindrome_number_single() {
        assert!(is_palindrome_number(7));
    }

    #[test]
    fn test_is_palindrome_number_zero() {
        assert!(is_palindrome_number(0));
    }

    // Exercise 4: Longest Palindromic Substring Tests
    #[test]
    fn test_longest_palindromic_substring_basic() {
        assert_eq!(longest_palindromic_substring("babad"), "bab");
    }

    #[test]
    fn test_longest_palindromic_substring_palindrome() {
        assert_eq!(longest_palindromic_substring("cbbd"), "bb");
    }

    #[test]
    fn test_longest_palindromic_substring_single() {
        assert_eq!(longest_palindromic_substring("a"), "a");
    }

    // Exercise 5: Palindrome Linked List Tests
    #[test]
    fn test_is_palindrome_list_true() {
        let head = ListNode::from_slice(&[1, 2, 1]);
        assert!(is_palindrome_list(head));
    }

    #[test]
    fn test_is_palindrome_list_false() {
        let head = ListNode::from_slice(&[1, 2, 3]);
        assert!(!is_palindrome_list(head));
    }

    #[test]
    fn test_is_palindrome_list_single() {
        let head = ListNode::from_slice(&[1]);
        assert!(is_palindrome_list(head));
    }

    #[test]
    fn test_is_palindrome_list_empty() {
        assert!(is_palindrome_list(None));
    }

    // Exercise 6: Longest Palindrome Tests
    #[test]
    fn test_longest_palindrome_len_basic() {
        assert_eq!(longest_palindrome_len("abccccdd"), 7);
    }

    #[test]
    fn test_longest_palindrome_len_single() {
        assert_eq!(longest_palindrome_len("a"), 1);
    }

    #[test]
    fn test_longest_palindrome_len_all_same() {
        assert_eq!(longest_palindrome_len("aaa"), 3);
    }

    // Exercise 7: Palindrome Pairs Tests
    #[test]
    fn test_palindrome_pairs_basic() {
        let pairs = palindrome_pairs(&["bat", "tab", "cat"]);
        assert!(pairs.contains(&(0, 1)));
        assert!(pairs.contains(&(1, 0)));
    }

    #[test]
    fn test_palindrome_pairs_empty() {
        assert!(palindrome_pairs(&[]).is_empty());
    }

    // Exercise 8: Check After Removal Tests
    #[test]
    fn test_can_make_palindrome_after_removal_true() {
        assert!(can_make_palindrome_after_removal("abcz", 'z'));
    }

    #[test]
    fn test_can_make_palindrome_after_removal_false() {
        assert!(!can_make_palindrome_after_removal("abc", 'z'));
    }
}
// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("$name exercises - run tests with cargo test");
}
