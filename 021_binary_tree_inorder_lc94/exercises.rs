//! Binary Tree Inorder Traversal Exercises (LeetCode #94)
//!
//! This module contains exercises for Binary Tree Inorder Traversal.

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
}

// ============================================================================
// Exercise 1: Inorder Traversal - Recursive (Primary Solution)
// ============================================================================

/// Given binary tree root, return inorder traversal (Left, Root, Right).
///
/// Time Complexity: O(n)
/// Space Complexity: O(h) where h is tree height
pub fn inorder_traversal(root: Option<Box<TreeNode>>) -> Vec<i32> {
    todo!("Implement recursive inorder traversal")
}

// ============================================================================
// Exercise 2: Inorder Traversal - Iterative
// ============================================================================

/// Iterative implementation using explicit stack.
///
/// Time Complexity: O(n)
/// Space Complexity: O(h)
pub fn inorder_iterative(root: Option<Box<TreeNode>>) -> Vec<i32> {
    todo!("Implement iterative inorder traversal")
}

// ============================================================================
// Exercise 3: Preorder Traversal
// ============================================================================

/// Given binary tree root, return preorder traversal (Root, Left, Right).
///
/// Time Complexity: O(n)
/// Space Complexity: O(h)
pub fn preorder_traversal(root: Option<Box<TreeNode>>) -> Vec<i32> {
    todo!("Implement preorder traversal")
}

// ============================================================================
// Exercise 4: Postorder Traversal
// ============================================================================

/// Given binary tree root, return postorder traversal (Left, Right, Root).
///
/// Time Complexity: O(n)
/// Space Complexity: O(h)
pub fn postorder_traversal(root: Option<Box<TreeNode>>) -> Vec<i32> {
    todo!("Implement postorder traversal")
}

// ============================================================================
// Exercise 5: Level Order Traversal (BFS)
// ============================================================================

/// Given binary tree root, return level order traversal (level by level).
///
/// Time Complexity: O(n)
/// Space Complexity: O(w) where w is max width
pub fn level_order_traversal(root: Option<Box<TreeNode>>) -> Vec<Vec<i32>> {
    todo!("Implement level order traversal using queue")
}

// ============================================================================
// Exercise 6: Zigzag Level Order
// ============================================================================

/// Given binary tree root, return zigzag level order traversal.
///
/// Time Complexity: O(n)
/// Space Complexity: O(w)
pub fn zigzag_level_order(root: Option<Box<TreeNode>>) -> Vec<Vec<i32>> {
    todo!("Zigzag pattern: left-to-right, right-to-left")
}

// ============================================================================
// Exercise 7: Morris Inorder (Threaded Tree)
// ============================================================================

/// Inorder traversal without stack or recursion.
///
/// Time Complexity: O(n)
/// Space Complexity: O(1)
pub fn morris_inorder(root: Option<Box<TreeNode>>) -> Vec<i32> {
    todo!("Implement Morris inorder traversal")
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Exercise 1: Basic Inorder Tests
    #[test]
    fn test_inorder_basic() {
        let root = TreeNode::from_vec(&[Some(1), None, Some(2), Some(3)]);
        assert_eq!(inorder_traversal(root), vec![1, 3, 2]);
    }

    #[test]
    fn test_inorder_empty() {
        assert_eq!(inorder_traversal(None), Vec::<i32>::new());
    }

    #[test]
    fn test_inorder_single() {
        let root = TreeNode::from_vec(&[Some(1)]);
        assert_eq!(inorder_traversal(root), vec![1]);
    }

    #[test]
    fn test_inorder_complete() {
        //     1
        //    / \
        //   2   3
        //  / \   \
        // 4   5   6
        let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(6)]);
        assert_eq!(inorder_traversal(root), vec![4, 2, 5, 1, 6, 3]);
    }

    // Exercise 2: Iterative Tests
    #[test]
    fn test_inorder_iterative_basic() {
        let root = TreeNode::from_vec(&[Some(1), None, Some(2), Some(3)]);
        assert_eq!(inorder_iterative(root), vec![1, 3, 2]);
    }

    // Exercise 3: Preorder Tests
    #[test]
    fn test_preorder_basic() {
        let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3)]);
        assert_eq!(preorder_traversal(root), vec![1, 2, 3]);
    }

    #[test]
    fn test_preorder_complex() {
        let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(6)]);
        assert_eq!(preorder_traversal(root), vec![1, 2, 4, 5, 3, 6]);
    }

    // Exercise 4: Postorder Tests
    #[test]
    fn test_postorder_basic() {
        let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3)]);
        assert_eq!(postorder_traversal(root), vec![2, 3, 1]);
    }

    #[test]
    fn test_postorder_complete() {
        let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(6)]);
        assert_eq!(postorder_traversal(root), vec![4, 5, 2, 6, 3, 1]);
    }

    // Exercise 5: Level Order Tests
    #[test]
    fn test_level_order_basic() {
        let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3)]);
        assert_eq!(level_order_traversal(root), vec![vec![1], vec![2, 3]]);
    }

    #[test]
    fn test_level_order_complete() {
        let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)]);
        assert_eq!(level_order_traversal(root), vec![vec![1], vec![2, 3], vec![4, 5, 6, 7]]);
    }

    // Exercise 6: Zigzag Tests
    #[test]
    fn test_zigzag_basic() {
        let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3)]);
        assert_eq!(zigzag_level_order(root), vec![vec![1], vec![3, 2]]);
    }

    // Exercise 7: Morris Inorder Tests
    #[test]
    fn test_morris_basic() {
        let root = TreeNode::from_vec(&[Some(1), None, Some(2), Some(3)]);
        assert_eq!(morris_inorder(root), vec![1, 3, 2]);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("Binary Tree Inorder exercises - run tests with cargo test");
}