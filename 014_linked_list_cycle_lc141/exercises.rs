//! Linked List Cycle Exercises (LeetCode #141)
//!
//! This module contains exercises for the Linked List Cycle problem.

use std::collections::{HashMap, HashSet};

// ============================================================================
// ListNode definition
// ============================================================================

#[derive(Debug, PartialEq, Clone)]
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
// Exercise 1: Linked List Cycle - Floyd's Algorithm (Primary Solution)
// ============================================================================

/// Given a linked list, determine if it has a cycle in it.
///
/// Time Complexity: O(n)
/// Space Complexity: O(1)
pub fn has_cycle(head: Option<Box<ListNode>>) -> bool {
    todo!("Implement Floyd's cycle detection")
}

// ============================================================================
// Exercise 2: Find Cycle Start Index (LC 142)
// ============================================================================

/// Find the index (0-based) of the node where cycle begins. Return None if no cycle.
///
/// Time Complexity: O(n)
/// Space Complexity: O(1)
pub fn detect_cycle_start(head: Option<Box<ListNode>>) -> Option<usize> {
    todo!("Find where the cycle starts")
}

// ============================================================================
// Exercise 3: Cycle Length
// ============================================================================

/// Find the length of the cycle. Return 0 if no cycle.
///
/// Time Complexity: O(n)
/// Space Complexity: O(1)
pub fn cycle_length(head: Option<Box<ListNode>>) -> usize {
    todo!("Find the length of the cycle")
}

// ============================================================================
// Exercise 4: Happy Number (LC 202)
// ============================================================================

/// A happy number is defined by the process:
/// Starting with any positive integer, replace the number by the sum of the squares of its digits,
/// and repeat the process until the number equals 1 (where it will stay),
/// or it loops endlessly in a cycle which does not include 1.
/// Return true if happy, false if not.
///
/// Time Complexity: O(log n) or O(n) worst case
/// Space Complexity: O(1)
pub fn is_happy(n: i32) -> bool {
    todo!("Check if number is happy using cycle detection")
}

// ============================================================================
// Exercise 5: Has Cycle with HashSet
// ============================================================================

/// Use HashSet to detect cycles.
///
/// Time Complexity: O(n)
/// Space Complexity: O(n)
pub fn has_cycle_hashset(head: Option<Box<ListNode>>) -> bool {
    todo!("Implement using HashSet")
}

// ============================================================================
// Exercise 6: Remove Cycle
// ============================================================================

/// Remove the cycle from the linked list if it exists.
/// Return true if cycle was removed, false otherwise.
///
/// Time Complexity: O(n)
/// Space Complexity: O(1)
pub fn remove_cycle(head: &mut Option<Box<ListNode>>) -> bool {
    todo!("Remove cycle from linked list")
}

// ============================================================================
// Exercise 7: Is Circular Linked List
// ============================================================================

/// Check if the linked list is circular (every node points to another).
///
/// Time Complexity: O(n)
/// Space Complexity: O(1)
pub fn is_circular(head: Option<Box<ListNode>>) -> bool {
    todo!("Check if every node is part of a cycle")
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Exercise 1: Basic Cycle Detection Tests
    #[test]
    fn test_has_cycle_true() {
        // Create list [3,2,0,-4] with cycle back to index 1
        let mut head = Box::new(ListNode::new(3));
        let mut node1 = Box::new(ListNode::new(2));
        let mut node2 = Box::new(ListNode::new(0));
        let node3 = Box::new(ListNode::new(-4));

        head.next = Some(node1.clone());
        node1.next = Some(node2.clone());
        node2.next = Some(node3.clone());
        node3.next = Some(node1.clone()); // cycle back to node1

        assert!(has_cycle(Some(head)));
    }

    #[test]
    fn test_has_cycle_false() {
        let head = ListNode::from_slice(&[1, 2, 3, 4, 5]);
        assert!(!has_cycle(head));
    }

    #[test]
    fn test_has_cycle_empty() {
        assert!(!has_cycle(None));
    }

    #[test]
    fn test_has_cycle_single_no_cycle() {
        let head = ListNode::from_slice(&[1]);
        assert!(!has_cycle(head));
    }

    #[test]
    fn test_has_cycle_single_cycle_self() {
        let mut node = Box::new(ListNode::new(1));
        node.next = Some(node.clone());
        assert!(has_cycle(Some(node)));
    }

    // Exercise 2: Find Cycle Start Tests
    #[test]
    fn test_detect_cycle_start_basic() {
        // Create list [3,2,0,-4] with cycle at index 1
        let mut head = Box::new(ListNode::new(3));
        let mut node1 = Box::new(ListNode::new(2));
        let mut node2 = Box::new(ListNode::new(0));
        let node3 = Box::new(ListNode::new(-4));

        head.next = Some(node1.clone());
        node1.next = Some(node2.clone());
        node2.next = Some(node3.clone());
        node3.next = Some(node1.clone());

        assert_eq!(detect_cycle_start(Some(head)), Some(1));
    }

    #[test]
    fn test_detect_cycle_start_none() {
        let head = ListNode::from_slice(&[1, 2, 3]);
        assert_eq!(detect_cycle_start(head), None);
    }

    // Exercise 3: Cycle Length Tests
    #[test]
    fn test_cycle_length_basic() {
        // Create list [1,2,3,4] with cycle of length 3 (2->3->4->2)
        let mut head = Box::new(ListNode::new(1));
        let mut node2 = Box::new(ListNode::new(2));
        let mut node3 = Box::new(ListNode::new(3));
        let node4 = Box::new(ListNode::new(4));

        head.next = Some(node2.clone());
        node2.next = Some(node3.clone());
        node3.next = Some(node4.clone());
        node4.next = Some(node2.clone());

        assert_eq!(cycle_length(Some(head)), 3);
    }

    #[test]
    fn test_cycle_length_no_cycle() {
        let head = ListNode::from_slice(&[1, 2, 3]);
        assert_eq!(cycle_length(head), 0);
    }

    // Exercise 4: Happy Number Tests
    #[test]
    fn test_is_happy_true() {
        assert!(is_happy(19));
        assert!(is_happy(1));
        assert!(is_happy(7));
        assert!(is_happy(10));
    }

    #[test]
    fn test_is_happy_false() {
        assert!(!is_happy(2));
        assert!(!is_happy(3));
        assert!(!is_happy(4));
        assert!(!is_happy(5));
    }

    #[test]
    fn test_is_happy_one() {
        assert!(is_happy(1));
    }

    // Exercise 5: HashSet Tests
    #[test]
    fn test_has_cycle_hashset_true() {
        let mut head = Box::new(ListNode::new(1));
        let node2 = Box::new(ListNode::new(2));
        head.next = Some(node2.clone());
        node2.next = Some(head.clone());

        assert!(has_cycle_hashset(Some(head)));
    }

    #[test]
    fn test_has_cycle_hashset_false() {
        let head = ListNode::from_slice(&[1, 2, 3]);
        assert!(!has_cycle_hashset(head));
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("Linked List Cycle exercises - run tests with cargo test");
}
