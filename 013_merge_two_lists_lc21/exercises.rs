//! Merge Two Sorted Lists Exercises (LeetCode #21)
//!
//! This module contains exercises for the Merge Two Sorted Lists problem.

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
// Exercise 1: Merge Two Sorted Lists - Iterative (Primary Solution)
// ============================================================================

/// Merge two sorted linked lists in ascending order.
///
/// Time Complexity: O(n + m)
/// Space Complexity: O(1)
pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    todo!("Implement iterative merge")
}

// ============================================================================
// Exercise 2: Merge Two Sorted Lists - Recursive
// ============================================================================

/// Merge using recursion (no dummy head needed).
///
/// Time Complexity: O(n + m)
/// Space Complexity: O(n + m) call stack
pub fn merge_two_lists_recursive(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    todo!("Implement recursive merge")
}

// ============================================================================
// Exercise 3: Merge K Sorted Lists (LC 23)
// ============================================================================

/// Merge k sorted linked lists into one sorted list.
///
/// Time Complexity: O(n log k)
/// Space Complexity: O(k)
pub fn merge_k_lists(lists: &[Option<Box<ListNode>>]) -> Option<Box<ListNode>> {
    todo!("Merge k sorted lists")
}

// ============================================================================
// Exercise 4: Merge Two Lists In Place
// ============================================================================

/// Merge l2 into l1 in place without creating new nodes.
///
/// Time Complexity: O(n + m)
/// Space Complexity: O(1)
pub fn merge_in_place(l1: &mut Option<Box<ListNode>>, l2: &mut Option<Box<ListNode>>) {
    todo!("Merge l2 into l1 in place")
}

// ============================================================================
// Exercise 5: Merge Sorted Array
// ============================================================================

/// Merge two sorted arrays into one sorted array.
///
/// Time Complexity: O(n + m)
/// Space Complexity: O(n + m)
pub fn merge_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    todo!("Merge two sorted arrays")
}

// ============================================================================
// Exercise 6: Find Middle of Merged List
// ============================================================================

/// Find the middle node of the merged list.
///
/// Time Complexity: O(n + m)
/// Space Complexity: O(1)
pub fn find_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    todo!("Find middle node using slow/fast pointers")
}

// ============================================================================
// Exercise 7: Sorted Merge with Limit
// ============================================================================

/// Merge up to k nodes from each list, then stop.
///
/// Time Complexity: O(k * (n/k + m/k))
/// Space Complexity: O(k)
pub fn merge_limited(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, limit: i32) -> Option<Box<ListNode>> {
    todo!("Merge limited number of nodes from each list")
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Exercise 1: Basic Merge Tests
    #[test]
    fn test_merge_basic() {
        let l1 = ListNode::from_slice(&[1, 2, 4]);
        let l2 = ListNode::from_slice(&[1, 3, 4]);
        let result = merge_two_lists(l1, l2);
        assert_eq!(ListNode::to_vec(&result), vec![1, 1, 2, 3, 4, 4]);
    }

    #[test]
    fn test_merge_both_empty() {
        let l1: Option<Box<ListNode>> = None;
        let l2: Option<Box<ListNode>> = None;
        let result = merge_two_lists(l1, l2);
        assert!(result.is_none());
    }

    #[test]
    fn test_merge_one_empty() {
        let l1 = ListNode::from_slice(&[1, 2, 3]);
        let l2: Option<Box<ListNode>> = None;
        let result = merge_two_lists(l1, l2);
        assert_eq!(ListNode::to_vec(&result), vec![1, 2, 3]);
    }

    #[test]
    fn test_merge_other_empty() {
        let l1: Option<Box<ListNode>> = None;
        let l2 = ListNode::from_slice(&[1, 2, 3]);
        let result = merge_two_lists(l1, l2);
        assert_eq!(ListNode::to_vec(&result), vec![1, 2, 3]);
    }

    #[test]
    fn test_merge_single_each() {
        let l1 = ListNode::from_slice(&[1]);
        let l2 = ListNode::from_slice(&[2]);
        let result = merge_two_lists(l1, l2);
        assert_eq!(ListNode::to_vec(&result), vec![1, 2]);
    }

    #[test]
    fn test_merge_different_lengths() {
        let l1 = ListNode::from_slice(&[1, 3, 5, 7, 9]);
        let l2 = ListNode::from_slice(&[2, 4, 6]);
        let result = merge_two_lists(l1, l2);
        assert_eq!(ListNode::to_vec(&result), vec![1, 2, 3, 4, 5, 6, 7, 9]);
    }

    // Exercise 2: Recursive Merge Tests
    #[test]
    fn test_merge_recursive_basic() {
        let l1 = ListNode::from_slice(&[1, 2, 4]);
        let l2 = ListNode::from_slice(&[1, 3, 4]);
        let result = merge_two_lists_recursive(l1, l2);
        assert_eq!(ListNode::to_vec(&result), vec![1, 1, 2, 3, 4, 4]);
    }

    #[test]
    fn test_merge_recursive_empty() {
        let result = merge_two_lists_recursive(None, None);
        assert!(result.is_none());
    }

    // Exercise 3: Merge K Lists Tests
    #[test]
    fn test_merge_k_lists_basic() {
        let lists = [
            ListNode::from_slice(&[1, 4, 5]),
            ListNode::from_slice(&[1, 3, 4]),
            ListNode::from_slice(&[2, 6]),
        ];
        let result = merge_k_lists(&lists);
        assert_eq!(ListNode::to_vec(&result), vec![1, 1, 2, 3, 4, 4, 5, 6]);
    }

    #[test]
    fn test_merge_k_lists_empty() {
        let lists: [Option<Box<ListNode>>; 0] = [];
        let result = merge_k_lists(&lists);
        assert!(result.is_none());
    }

    // Exercise 4: Merge In Place Tests
    #[test]
    fn test_merge_in_place_basic() {
        let mut l1 = ListNode::from_slice(&[1, 2, 4]);
        let mut l2 = ListNode::from_slice(&[1, 3, 4]);
        merge_in_place(&mut l1, &mut l2);
        assert_eq!(ListNode::to_vec(&l1), vec![1, 1, 2, 3, 4, 4]);
    }

    // Exercise 5: Merge Arrays Tests
    #[test]
    fn test_merge_arrays_basic() {
        assert_eq!(merge_arrays(&[1, 2, 3], &[4, 5, 6]), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_merge_arrays_interleaved() {
        assert_eq!(merge_arrays(&[1, 3, 5], &[2, 4, 6]), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_merge_arrays_empty() {
        assert_eq!(merge_arrays(&[], &[1, 2, 3]), vec![1, 2, 3]);
    }

    // Exercise 6: Find Middle Tests
    #[test]
    fn test_find_middle_even() {
        let head = ListNode::from_slice(&[1, 2, 3, 4]);
        let result = find_middle(head);
        assert_eq!(result.unwrap().val, 3);
    }

    #[test]
    fn test_find_middle_odd() {
        let head = ListNode::from_slice(&[1, 2, 3, 4, 5]);
        let result = find_middle(head);
        assert_eq!(result.unwrap().val, 3);
    }

    #[test]
    fn test_find_middle_single() {
        let head = ListNode::from_slice(&[1]);
        let result = find_middle(head);
        assert_eq!(result.unwrap().val, 1);
    }

    // Exercise 7: Merge Limited Tests
    #[test]
    fn test_merge_limited_basic() {
        let l1 = ListNode::from_slice(&[1, 2, 3]);
        let l2 = ListNode::from_slice(&[4, 5, 6]);
        let result = merge_limited(l1, l2, 2);
        assert_eq!(ListNode::to_vec(&result), vec![1, 2, 4, 5]);
    }
}
// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("$name exercises - run tests with cargo test");
}
