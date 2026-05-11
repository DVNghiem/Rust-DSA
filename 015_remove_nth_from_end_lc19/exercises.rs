//! Remove Nth From End Exercises (LeetCode #19)
//!
//! This module contains exercises for the Remove Nth From End problem.

use std::collections::VecDeque;

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
// Exercise 1: Remove Nth From End - Two Pointers (Primary Solution)
// ============================================================================

/// Remove the nth node from the end of the list.
///
/// Time Complexity: O(n)
/// Space Complexity: O(1)
pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    todo!("Implement two-pointer solution")
}

// ============================================================================
// Exercise 2: Remove Nth From End - Two Pass
// ============================================================================

/// Count length first, then remove.
///
/// Time Complexity: O(n)
/// Space Complexity: O(1)
pub fn remove_nth_two_pass(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    todo!("Implement two-pass solution")
}

// ============================================================================
// Exercise 3: Get Nth From End
// ============================================================================

/// Get the nth node from the end of the list.
///
/// Time Complexity: O(n)
/// Space Complexity: O(1)
pub fn get_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<i32> {
    todo!("Get nth node from end")
}

// ============================================================================
// Exercise 4: Remove Every Kth Node
// ============================================================================

/// Remove every kth node from the list.
///
/// Time Complexity: O(n)
/// Space Complexity: O(1)
pub fn remove_every_kth(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    todo!("Remove every kth node")
}

// ============================================================================
// Exercise 5: Swap Nth Node with 1st
// ============================================================================

/// Swap the nth node with the first node.
///
/// Time Complexity: O(n)
/// Space Complexity: O(1)
pub fn swap_nth_with_first(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    todo!("Swap nth node with first")
}

// ============================================================================
// Exercise 6: Find Middle and Remove
// ============================================================================

/// Find the middle node and remove it.
///
/// Time Complexity: O(n)
/// Space Complexity: O(1)
pub fn remove_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    todo!("Remove the middle node")
}

// ============================================================================
// Exercise 7: Remove Last K Nodes
// ============================================================================

/// Remove the last k nodes from the list.
///
/// Time Complexity: O(n)
/// Space Complexity: O(1)
pub fn remove_last_k_nodes(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    todo!("Remove last k nodes")
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Exercise 1: Basic Remove Nth Tests
    #[test]
    fn test_remove_nth_basic() {
        let head = ListNode::from_slice(&[1, 2, 3, 4, 5]);
        let result = remove_nth_from_end(head, 2);
        assert_eq!(ListNode::to_vec(&result), vec![1, 2, 3, 5]);
    }

    #[test]
    fn test_remove_nth_first() {
        let head = ListNode::from_slice(&[1, 2, 3, 4, 5]);
        let result = remove_nth_from_end(head, 5);
        assert_eq!(ListNode::to_vec(&result), vec![2, 3, 4, 5]);
    }

    #[test]
    fn test_remove_nth_last() {
        let head = ListNode::from_slice(&[1, 2, 3, 4, 5]);
        let result = remove_nth_from_end(head, 1);
        assert_eq!(ListNode::to_vec(&result), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_remove_nth_single() {
        let head = ListNode::from_slice(&[1]);
        let result = remove_nth_from_end(head, 1);
        assert!(result.is_none());
    }

    #[test]
    fn test_remove_nth_two_nodes() {
        let head = ListNode::from_slice(&[1, 2]);
        let result = remove_nth_from_end(head, 1);
        assert_eq!(ListNode::to_vec(&result), vec![1]);
    }

    #[test]
    fn test_remove_nth_two_nodes_first() {
        let head = ListNode::from_slice(&[1, 2]);
        let result = remove_nth_from_end(head, 2);
        assert_eq!(ListNode::to_vec(&result), vec![2]);
    }

    // Exercise 2: Two Pass Tests
    #[test]
    fn test_remove_nth_two_pass_basic() {
        let head = ListNode::from_slice(&[1, 2, 3, 4, 5]);
        let result = remove_nth_two_pass(head, 2);
        assert_eq!(ListNode::to_vec(&result), vec![1, 2, 3, 5]);
    }

    // Exercise 3: Get Nth From End Tests
    #[test]
    fn test_get_nth_from_end() {
        let head = ListNode::from_slice(&[1, 2, 3, 4, 5]);
        assert_eq!(get_nth_from_end(head, 2), Some(4));
    }

    #[test]
    fn test_get_nth_from_end_first() {
        let head = ListNode::from_slice(&[1, 2, 3]);
        assert_eq!(get_nth_from_end(head, 3), Some(1));
    }

    #[test]
    fn test_get_nth_from_end_last() {
        let head = ListNode::from_slice(&[1, 2, 3]);
        assert_eq!(get_nth_from_end(head, 1), Some(3));
    }

    // Exercise 4: Remove Every Kth Tests
    #[test]
    fn test_remove_every_kth_basic() {
        let head = ListNode::from_slice(&[1, 2, 3, 4, 5, 6, 7, 8]);
        let result = remove_every_kth(head, 3);
        assert_eq!(ListNode::to_vec(&result), vec![1, 2, 4, 5, 7, 8]);
    }

    // Exercise 5: Swap Nth with First Tests
    #[test]
    fn test_swap_nth_with_first_basic() {
        let head = ListNode::from_slice(&[1, 2, 3, 4, 5]);
        let result = swap_nth_with_first(head, 3);
        assert_eq!(ListNode::to_vec(&result), vec![3, 2, 1, 4, 5]);
    }

    // Exercise 6: Remove Middle Tests
    #[test]
    fn test_remove_middle_odd() {
        let head = ListNode::from_slice(&[1, 2, 3, 4, 5]);
        let result = remove_middle(head);
        assert_eq!(ListNode::to_vec(&result), vec![1, 2, 4, 5]);
    }

    #[test]
    fn test_remove_middle_even() {
        let head = ListNode::from_slice(&[1, 2, 3, 4]);
        let result = remove_middle(head);
        assert_eq!(ListNode::to_vec(&result), vec![1, 2, 4]);
    }

    // Exercise 7: Remove Last K Tests
    #[test]
    fn test_remove_last_k_basic() {
        let head = ListNode::from_slice(&[1, 2, 3, 4, 5]);
        let result = remove_last_k_nodes(head, 2);
        assert_eq!(ListNode::to_vec(&result), vec![1, 2, 3]);
    }

    #[test]
    fn test_remove_last_k_all() {
        let head = ListNode::from_slice(&[1, 2, 3]);
        let result = remove_last_k_nodes(head, 3);
        assert!(result.is_none());
    }
}
// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("$name exercises - run tests with cargo test");
}
