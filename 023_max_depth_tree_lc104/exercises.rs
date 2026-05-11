//! Maximum Depth of Binary Tree Exercises (LeetCode #104)
//!
//! This module contains exercises for Maximum Depth of Binary Tree.

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
// Exercise 1: Maximum Depth - Recursive (Primary Solution)
// ============================================================================

/// Given binary tree root, return the maximum depth (number of nodes on longest path).
///
/// Time Complexity: O(n)
/// Space Complexity: O(h) where h is tree height
pub fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
    todo!("Implement recursive max depth")
}

// ============================================================================
// Exercise 2: Maximum Depth - Iterative (BFS)
// ============================================================================

/// Use level-order traversal to find max depth.
///
/// Time Complexity: O(n)
/// Space Complexity: O(w) where w is max width
pub fn max_depth_iterative(root: Option<Box<TreeNode>>) -> i32 {
    todo!("Implement iterative max depth using BFS")
}

// ============================================================================
// Exercise 3: Minimum Depth of Binary Tree
// ============================================================================

/// Find minimum depth (shortest path to leaf).
///
/// Time Complexity: O(n)
/// Space Complexity: O(h)
pub fn min_depth(root: Option<Box<TreeNode>>) -> i32 {
    todo!("Find minimum depth of binary tree")
}

// ============================================================================
// Exercise 4: Balanced Binary Tree Check
// ============================================================================

/// Check if tree is balanced (left and right subtrees differ by at most 1).
///
/// Time Complexity: O(n)
/// Space Complexity: O(h)
pub fn is_balanced(root: Option<Box<TreeNode>>) -> bool {
    todo!("Check if binary tree is balanced")
}

// ============================================================================
// Exercise 5: Diameter of Binary Tree (LC 543)
// ============================================================================

/// Find the diameter (longest path between any two nodes, not necessarily through root).
///
/// Time Complexity: O(n)
/// Space Complexity: O(h)
pub fn diameter_of_binary_tree(root: Option<Box<TreeNode>>) -> i32 {
    todo!("Find diameter of binary tree")
}

// ============================================================================
// Exercise 6: Maximum Width of Binary Tree
// ============================================================================

/// Find maximum width (number of nodes between leftmost and rightmost in a level).
///
/// Time Complexity: O(n)
/// Space Complexity: O(w)
pub fn max_width(root: Option<Box<TreeNode>>) -> i32 {
    todo!("Find maximum width of binary tree")
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Exercise 1: Basic Max Depth Tests
    #[test]
    fn test_max_depth_basic() {
        //     3
        //    / \
        //   9   20
        //      /  \
        //     15   7
        let root = TreeNode::from_vec(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
        assert_eq!(max_depth(root), 3);
    }

    #[test]
    fn test_max_depth_empty() {
        assert_eq!(max_depth(None), 0);
    }

    #[test]
    fn test_max_depth_single() {
        let root = TreeNode::from_vec(&[Some(1)]);
        assert_eq!(max_depth(root), 1);
    }

    #[test]
    fn test_max_depth_stick() {
        // 1 -> 2 -> 3 -> 4 -> 5 (depth 5)
        let root = TreeNode::from_vec(&[Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5)]);
        assert_eq!(max_depth(root), 5);
    }

    // Exercise 2: Iterative Tests
    #[test]
    fn test_max_depth_iterative_basic() {
        let root = TreeNode::from_vec(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
        assert_eq!(max_depth_iterative(root), 3);
    }

    #[test]
    fn test_max_depth_iterative_empty() {
        assert_eq!(max_depth_iterative(None), 0);
    }

    // Exercise 3: Min Depth Tests
    #[test]
    fn test_min_depth_basic() {
        //     3
        //    /
        //   9
        let root = TreeNode::from_vec(&[Some(3), Some(9), None]);
        assert_eq!(min_depth(root), 2);
    }

    #[test]
    fn test_min_depth_empty() {
        assert_eq!(min_depth(None), 0);
    }

    #[test]
    fn test_min_depth_complete() {
        let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3), Some(4), Some(5)]);
        // Depth: 1 -> 2 -> 4 = 3 (left side is min)
        assert_eq!(min_depth(root), 3);
    }

    // Exercise 4: Balanced Check Tests
    #[test]
    fn test_is_balanced_true() {
        let root = TreeNode::from_vec(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
        assert!(is_balanced(root));
    }

    #[test]
    fn test_is_balanced_false() {
        //     1
        //      \
        //       2
        //        \
        //         3 (unbalanced)
        let root = TreeNode::from_vec(&[Some(1), None, Some(2), None, Some(3)]);
        assert!(!is_balanced(root));
    }

    #[test]
    fn test_is_balanced_empty() {
        assert!(is_balanced(None));
    }

    // Exercise 5: Diameter Tests
    #[test]
    fn test_diameter_basic() {
        //     1
        //    / \
        //   2   3
        //  / \
        // 4   5
        // Longest path is 4 -> 2 -> 1 -> 3 = 4 nodes = 3 edges
        let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3), Some(4), Some(5)]);
        assert_eq!(diameter_of_binary_tree(root), 3);
    }

    #[test]
    fn test_diameter_root() {
        let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3)]);
        assert_eq!(diameter_of_binary_tree(root), 2);
    }

    // Exercise 6: Max Width Tests
    #[test]
    fn test_max_width_basic() {
        //     1
        //    / \
        //   3   2
        //  /
        // 5
        // Max width is 3 (nodes 1, 3, 2 at level with node 5)
        let root = TreeNode::from_vec(&[Some(1), Some(3), Some(2), Some(5)]);
        assert_eq!(max_width(root), 2);
    }

    #[test]
    fn test_max_width_single() {
        let root = TreeNode::from_vec(&[Some(1)]);
        assert_eq!(max_width(root), 1);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("Max Depth Tree exercises - run tests with cargo test");
}