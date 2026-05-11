//! Invert Binary Tree Exercises (LeetCode #226)
//!
//! This module contains exercises for Inverting Binary Tree.

use std::collections::{BinaryHeap, VecDeque};

// ============================================================================
// TreeNode definition
// ============================================================================

#[derive(Debug, PartialEq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }

    pub fn from_vec(vals: &[Option<i32>]) -> Option<Box<TreeNode>> {
        if vals.is_empty() || vals[0].is_none() {
            return None;
        }
        let mut queue = VecDeque::new();
        let mut root = Box::new(TreeNode::new(vals[0].unwrap()));
        queue.push_back(&mut *root);

        let mut i = 1;
        while i < vals.len() {
            let node = queue.pop_front().unwrap();
            if i < vals.len() && vals[i].is_some() {
                node.left = Some(Box::new(TreeNode::new(vals[i].unwrap())));
                queue.push_back(&mut *node.left.as_mut().unwrap());
            }
            i += 1;
            if i < vals.len() && vals[i].is_some() {
                node.right = Some(Box::new(TreeNode::new(vals[i].unwrap())));
                queue.push_back(&mut *node.right.as_mut().unwrap());
            }
            i += 1;
        }
        Some(root)
    }

    pub fn to_vec(root: &Option<Box<TreeNode>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut queue = VecDeque::new();
        if let Some(r) = root {
            queue.push_back(r);
        }
        while let Some(node) = queue.pop_front() {
            result.push(node.val);
            if let Some(left) = &node.left {
                queue.push_back(left);
            }
            if let Some(right) = &node.right {
                queue.push_back(right);
            }
        }
        result
    }
}

// ============================================================================
// Exercise 1: Invert Binary Tree - Recursive (Primary Solution)
// ============================================================================

/// Invert a binary tree (swap left and right children recursively).
///
/// Time Complexity: O(n)
/// Space Complexity: O(h)
pub fn invert_tree(root: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    todo!("Implement recursive invert")
}

// ============================================================================
// Exercise 2: Invert Binary Tree - Iterative
// ============================================================================

/// Invert using iterative approach with queue.
///
/// Time Complexity: O(n)
/// Space Complexity: O(w) where w is max width
pub fn invert_tree_iterative(root: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    todo!("Implement iterative invert using BFS")
}

// ============================================================================
// Exercise 3: Invert Tree in Place
// ============================================================================

/// Invert tree without creating new nodes.
///
/// Time Complexity: O(n)
/// Space Complexity: O(1) extra if recursion ignored
pub fn invert_in_place(root: &mut Option<Box<TreeNode>>) {
    todo!("Invert without allocation")
}

// ============================================================================
// Exercise 4: Mirror Tree Check
// ============================================================================

/// Check if a tree is a mirror of itself (symmetric).
///
/// Time Complexity: O(n)
/// Space Complexity: O(h)
pub fn is_symmetric(root: Option<Box<TreeNode>>) -> bool {
    todo!("Check if tree is symmetric")
}

// ============================================================================
// Exercise 5: Same Tree Check
// ============================================================================

/// Check if two trees are identical.
///
/// Time Complexity: O(n)
/// Space Complexity: O(h)
pub fn is_same_tree(p: Option<Box<TreeNode>>, q: Option<Box<TreeNode>>) -> bool {
    todo!("Check if two trees are identical")
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Exercise 1: Basic Invert Tests
    #[test]
    fn test_invert_basic() {
        let root = TreeNode::from_vec(&[Some(4), Some(2), Some(7), Some(1), Some(3), Some(6), Some(9)]);
        let result = invert_tree(root);
        assert_eq!(TreeNode::to_vec(&result), vec![4, 7, 2, 9, 6, 3, 1]);
    }

    #[test]
    fn test_invert_empty() {
        assert!(invert_tree(None).is_none());
    }

    #[test]
    fn test_invert_single() {
        let root = TreeNode::from_vec(&[Some(1)]);
        let result = invert_tree(root);
        assert_eq!(TreeNode::to_vec(&result), vec![1]);
    }

    #[test]
    fn test_invert_two_nodes() {
        let root = TreeNode::from_vec(&[Some(1), Some(2), None]);
        let result = invert_tree(root);
        // After invert, left becomes right
        assert_eq!(TreeNode::to_vec(&result), vec![1, 2]);
    }

    // Exercise 2: Iterative Tests
    #[test]
    fn test_invert_iterative_basic() {
        let root = TreeNode::from_vec(&[Some(4), Some(2), Some(7)]);
        let result = invert_tree_iterative(root);
        assert_eq!(TreeNode::to_vec(&result), vec![4, 7, 2]);
    }

    // Exercise 3: In Place Tests
    #[test]
    fn test_invert_in_place_basic() {
        let mut root = TreeNode::from_vec(&[Some(4), Some(2), Some(7)]);
        invert_in_place(&mut root);
        assert_eq!(TreeNode::to_vec(&root), vec![4, 7, 2]);
    }

    // Exercise 4: Symmetric Tests
    #[test]
    fn test_is_symmetric_true() {
        //     1
        //    / \
        //   2   2
        //    \ /
        //     3
        let root = TreeNode::from_vec(&[Some(1), Some(2), Some(2), None, Some(3), None, Some(3)]);
        assert!(is_symmetric(root));
    }

    #[test]
    fn test_is_symmetric_false() {
        let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3)]);
        assert!(!is_symmetric(root));
    }

    #[test]
    fn test_is_symmetric_empty() {
        assert!(is_symmetric(None));
    }

    // Exercise 5: Same Tree Tests
    #[test]
    fn test_is_same_tree_true() {
        let t1 = TreeNode::from_vec(&[Some(1), Some(2), Some(3)]);
        let t2 = TreeNode::from_vec(&[Some(1), Some(2), Some(3)]);
        assert!(is_same_tree(t1, t2));
    }

    #[test]
    fn test_is_same_tree_false() {
        let t1 = TreeNode::from_vec(&[Some(1), Some(2)]);
        let t2 = TreeNode::from_vec(&[Some(1), None, Some(3)]);
        assert!(!is_same_tree(t1, t2));
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("Invert Binary Tree exercises - run tests with cargo test");
}