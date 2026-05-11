//! Reverse Linked List Exercises (LeetCode #206)
//!
//! This module contains exercises for the Reverse Linked List problem.

use std::collections::VecDeque;

// ============================================================================
// ListNode definition
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

    pub fn to_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut current = head;
        while let Some(node) = current {
            result.push(node.val);
            current = &node.next;
        }
        result
    }
}

// ============================================================================
// Exercise 1: Reverse Linked List - Iterative (Primary Solution)
// ============================================================================

/// Given head of singly linked list, reverse the list in place.
///
/// Time Complexity: O(n)
/// Space Complexity: O(1)
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    todo!("Implement iterative reverse")
}

// ============================================================================
// Exercise 2: Reverse Linked List - Recursive
// ============================================================================

/// Reverse the list using recursion.
///
/// Time Complexity: O(n)
/// Space Complexity: O(n) call stack
pub fn reverse_list_recursive(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    todo!("Implement recursive reverse")
}

// ============================================================================
// Exercise 3: Reverse Linked List in Groups
// ============================================================================

/// Reverse nodes in groups of k. If nodes remain less than k, don't reverse.
///
/// Time Complexity: O(n)
/// Space Complexity: O(1)
pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    todo!("Reverse in groups of k")
}

// ============================================================================
// Exercise 4: Reverse Between (LC 92)
// ============================================================================

/// Reverse nodes from left to right (1-indexed).
///
/// Time Complexity: O(n)
/// Space Complexity: O(1)
pub fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
    todo!("Reverse nodes between left and right")
}

// ============================================================================
// Exercise 5: Is Palindrome (LC 234)
// ============================================================================

/// Check if linked list is a palindrome.
///
/// Time Complexity: O(n)
/// Space Complexity: O(1)
pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    todo!("Check if list is palindrome")
}

// ============================================================================
// Exercise 6: Reverse Nodes Even Positions
// ============================================================================

/// Reverse only the nodes at even positions (2nd, 4th, 6th, etc.).
///
/// Time Complexity: O(n)
/// Space Complexity: O(1)
pub fn reverse_even_positions(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    todo!("Reverse nodes at even positions only")
}

// ============================================================================
// Exercise 7: Sum of Two Linked Lists
// ============================================================================

/// Given two linked lists representing numbers, return sum as linked list.
///
/// Time Complexity: O(n)
/// Space Complexity: O(n)
pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    todo!("Add two numbers represented as linked lists")
}

// ============================================================================
// Exercise 8: Rotate Linked List Right
// ============================================================================

/// Rotate the list to the right by k places.
///
/// Time Complexity: O(n)
/// Space Complexity: O(1)
pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    todo!("Rotate list right by k positions")
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Exercise 1: Basic Reverse Tests
    #[test]
    fn test_reverse_basic() {
        let head = ListNode::from_slice(&[1, 2, 3, 4, 5]);
        let result = reverse_list(head);
        assert_eq!(ListNode::to_vec(&result), vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_reverse_two() {
        let head = ListNode::from_slice(&[1, 2]);
        let result = reverse_list(head);
        assert_eq!(ListNode::to_vec(&result), vec![2, 1]);
    }

    #[test]
    fn test_reverse_single() {
        let head = ListNode::from_slice(&[1]);
        let result = reverse_list(head);
        assert_eq!(ListNode::to_vec(&result), vec![1]);
    }

    #[test]
    fn test_reverse_empty() {
        let head: Option<Box<ListNode>> = None;
        let result = reverse_list(head);
        assert!(result.is_none());
    }

    #[test]
    fn test_reverse_same_values() {
        let head = ListNode::from_slice(&[1, 1, 1, 1]);
        let result = reverse_list(head);
        assert_eq!(ListNode::to_vec(&result), vec![1, 1, 1, 1]);
    }

    // Exercise 2: Recursive Reverse Tests
    #[test]
    fn test_reverse_recursive_basic() {
        let head = ListNode::from_slice(&[1, 2, 3, 4, 5]);
        let result = reverse_list_recursive(head);
        assert_eq!(ListNode::to_vec(&result), vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_reverse_recursive_single() {
        let head = ListNode::from_slice(&[1]);
        let result = reverse_list_recursive(head);
        assert_eq!(ListNode::to_vec(&result), vec![1]);
    }

    // Exercise 3: Reverse K Group Tests
    #[test]
    fn test_reverse_k_group_basic() {
        let head = ListNode::from_slice(&[1, 2, 3, 4, 5, 6]);
        let result = reverse_k_group(head, 2);
        assert_eq!(ListNode::to_vec(&result), vec![2, 1, 4, 3, 6, 5]);
    }

    #[test]
    fn test_reverse_k_group_exact() {
        let head = ListNode::from_slice(&[1, 2, 3]);
        let result = reverse_k_group(head, 3);
        assert_eq!(ListNode::to_vec(&result), vec![3, 2, 1]);
    }

    #[test]
    fn test_reverse_k_group_partial() {
        let head = ListNode::from_slice(&[1, 2, 3, 4, 5]);
        let result = reverse_k_group(head, 3);
        assert_eq!(ListNode::to_vec(&result), vec![3, 2, 1, 4, 5]);
    }

    // Exercise 4: Reverse Between Tests
    #[test]
    fn test_reverse_between_basic() {
        let head = ListNode::from_slice(&[1, 2, 3, 4, 5]);
        let result = reverse_between(head, 2, 4);
        assert_eq!(ListNode::to_vec(&result), vec![1, 4, 3, 2, 5]);
    }

    #[test]
    fn test_reverse_between_all() {
        let head = ListNode::from_slice(&[1, 2, 3]);
        let result = reverse_between(head, 1, 3);
        assert_eq!(ListNode::to_vec(&result), vec![3, 2, 1]);
    }

    // Exercise 5: Palindrome Tests
    #[test]
    fn test_is_palindrome_true() {
        let head = ListNode::from_slice(&[1, 2, 3, 2, 1]);
        assert!(is_palindrome(head));
    }

    #[test]
    fn test_is_palindrome_false() {
        let head = ListNode::from_slice(&[1, 2, 3, 4, 5]);
        assert!(!is_palindrome(head));
    }

    #[test]
    fn test_is_palindrome_single() {
        let head = ListNode::from_slice(&[1]);
        assert!(is_palindrome(head));
    }

    #[test]
    fn test_is_palindrome_even() {
        let head = ListNode::from_slice(&[1, 2, 2, 1]);
        assert!(is_palindrome(head));
    }

    // Exercise 6: Reverse Even Positions Tests
    #[test]
    fn test_reverse_even_positions_basic() {
        let head = ListNode::from_slice(&[1, 2, 3, 4, 5, 6]);
        let result = reverse_even_positions(head);
        assert_eq!(ListNode::to_vec(&result), vec![1, 6, 3, 4, 5, 2]);
    }

    #[test]
    fn test_reverse_even_positions_five() {
        let head = ListNode::from_slice(&[1, 2, 3, 4, 5]);
        let result = reverse_even_positions(head);
        assert_eq!(ListNode::to_vec(&result), vec![1, 5, 3, 4, 5]);
    }

    // Exercise 7: Add Two Numbers Tests
    #[test]
    fn test_add_two_numbers_basic() {
        let l1 = ListNode::from_slice(&[2, 4, 3]);
        let l2 = ListNode::from_slice(&[5, 6, 4]);
        let result = add_two_numbers(l1, l2);
        assert_eq!(ListNode::to_vec(&result), vec![7, 0, 8]);
    }

    #[test]
    fn test_add_two_numbers_carry() {
        let l1 = ListNode::from_slice(&[9, 9, 9]);
        let l2 = ListNode::from_slice(&[1]);
        let result = add_two_numbers(l1, l2);
        assert_eq!(ListNode::to_vec(&result), vec![0, 0, 0, 1]);
    }

    // Exercise 8: Rotate Right Tests
    #[test]
    fn test_rotate_right_basic() {
        let head = ListNode::from_slice(&[1, 2, 3, 4, 5]);
        let result = rotate_right(head, 2);
        assert_eq!(ListNode::to_vec(&result), vec![4, 5, 1, 2, 3]);
    }

    #[test]
    fn test_rotate_right_single() {
        let head = ListNode::from_slice(&[1]);
        let result = rotate_right(head, 1);
        assert_eq!(ListNode::to_vec(&result), vec![1]);
    }
}
// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("$name exercises - run tests with cargo test");
}
