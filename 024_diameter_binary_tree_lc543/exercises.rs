//! Diameter of Binary Tree Exercises (LeetCode #543)
//!
//! This module contains exercises for finding the diameter of a binary tree.

use std::collections::VecDeque;

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
// Exercise 1: Diameter of Binary Tree - Recursive (Primary Solution)
// ============================================================================

/// Return the length of the diameter of the binary tree.
/// The diameter is the longest path between any two nodes in the tree.
///
/// Time Complexity: O(n)
/// Space Complexity: O(h) where h is tree height
pub fn diameter_of_binary_tree(root: Option<Box<TreeNode>>) -> i32 {
    todo!("Implement recursive diameter using post-order traversal")
}

// ============================================================================
// Exercise 2: Diameter - Two Pass Approach
// ============================================================================

/// First find the two endpoints of the longest path, then compute distance.
///
/// Time Complexity: O(n)
/// Space Complexity: O(h)
pub fn diameter_of_binary_tree_two_pass(root: Option<Box<TreeNode>>) -> i32 {
    todo!("Implement two-pass diameter finding")
}

// ============================================================================
// Exercise 3: Diameter - Iterative (Post-order with Stack)
// ============================================================================

/// Use explicit stack to simulate post-order traversal.
///
/// Time Complexity: O(n)
/// Space Complexity: O(h)
pub fn diameter_of_binary_tree_iterative(root: Option<Box<TreeNode>>) -> i32 {
    todo!("Implement iterative diameter using stack")
}

// ============================================================================
// Exercise 4: Height of Binary Tree
// ============================================================================

/// Helper function: compute height of a binary tree.
///
/// Time Complexity: O(n)
/// Space Complexity: O(h)
pub fn height(root: Option<Box<TreeNode>>) -> i32 {
    todo!("Implement tree height calculation")
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Exercise 1: Basic Diameter Tests
    #[test]
    fn test_diameter_basic() {
        //     1
        //    / \
        //   2   3
        //  / \
        // 4   5
        // Longest path: 4 → 2 → 1 → 3 = 4 nodes = 3 edges
        let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3), Some(4), Some(5)]);
        assert_eq!(diameter_of_binary_tree(root), 3);
    }

    #[test]
    fn test_diameter_empty() {
        assert_eq!(diameter_of_binary_tree(None), 0);
    }

    #[test]
    fn test_diameter_single() {
        let root = TreeNode::from_vec(&[Some(1)]);
        assert_eq!(diameter_of_binary_tree(root), 0);
    }

    #[test]
    fn test_diameter_root_only() {
        // Only root, diameter is 0
        let root = TreeNode::from_vec(&[Some(1)]);
        assert_eq!(diameter_of_binary_tree(root), 0);
    }

    #[test]
    fn test_diameter_line() {
        // 1 → 2 → 3 → 4 → 5 (diameter = 4 edges)
        let root = TreeNode::from_vec(&[Some(1), Some(2), None, Some(3), None, Some(4), None, Some(5)]);
        assert_eq!(diameter_of_binary_tree(root), 4);
    }

    #[test]
    fn test_diameter_through_root() {
        //     1
        //    / \
        //   2   3
        // Diameter = 2 edges (2 → 1 → 3 or 2 → 1 → 3)
        let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3)]);
        assert_eq!(diameter_of_binary_tree(root), 2);
    }

    // Exercise 2: Two Pass Tests
    #[test]
    fn test_diameter_two_pass_basic() {
        let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3), Some(4), Some(5)]);
        assert_eq!(diameter_of_binary_tree_two_pass(root), 3);
    }

    #[test]
    fn test_diameter_two_pass_empty() {
        assert_eq!(diameter_of_binary_tree_two_pass(None), 0);
    }

    #[test]
    fn test_diameter_two_pass_line() {
        let root = TreeNode::from_vec(&[Some(1), Some(2), None, Some(3), None, Some(4), None, Some(5)]);
        assert_eq!(diameter_of_binary_tree_two_pass(root), 4);
    }

    // Exercise 3: Iterative Tests
    #[test]
    fn test_diameter_iterative_basic() {
        let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3), Some(4), Some(5)]);
        assert_eq!(diameter_of_binary_tree_iterative(root), 3);
    }

    #[test]
    fn test_diameter_iterative_empty() {
        assert_eq!(diameter_of_binary_tree_iterative(None), 0);
    }

    #[test]
    fn test_diameter_iterative_single() {
        let root = TreeNode::from_vec(&[Some(1)]);
        assert_eq!(diameter_of_binary_tree_iterative(root), 0);
    }

    // Exercise 4: Height Tests
    #[test]
    fn test_height_basic() {
        //     1
        //    / \
        //   2   3  (height = 2)
        let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3)]);
        assert_eq!(height(root), 3); // height includes root node
    }

    #[test]
    fn test_height_empty() {
        assert_eq!(height(None), 0);
    }

    #[test]
    fn test_height_single() {
        let root = TreeNode::from_vec(&[Some(1)]);
        assert_eq!(height(root), 1);
    }

    #[test]
    fn test_height_line() {
        // 1 → 2 → 3 → 4 (height = 4)
        let root = TreeNode::from_vec(&[Some(1), Some(2), None, Some(3), None, None, Some(4)]);
        assert_eq!(height(root), 4);
    }

    // Additional edge cases
    #[test]
    fn test_diameter_left_skewed() {
        //   1
        //  /
        // 2
        //  \
        //   3
        //  /
        // 4  (diameter = 3 edges: 4 → 3 → 2 → 1)
        let root = TreeNode::from_vec(&[Some(1), Some(2), None, None, Some(3), None, Some(4)]);
        assert_eq!(diameter_of_binary_tree(root), 3);
    }

    #[test]
    fn test_diameter_right_skewed() {
        // 1
        //  \
        //   2
        //  /
        // 3
        //  \
        //   4  (diameter = 3 edges)
        let root = TreeNode::from_vec(&[Some(1), None, Some(2), Some(3), None, None, Some(4)]);
        assert_eq!(diameter_of_binary_tree(root), 3);
    }

    #[test]
    fn test_diameter_balanced() {
        //       1
        //      / \
        //     2   3
        //    / \   \
        //   4   5   6
        // diameter through root: 2 + 1 = 3
        let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)]);
        assert_eq!(diameter_of_binary_tree(root), 3);
    }

    #[test]
    fn test_diameter_not_through_root() {
        //     1
        //    /
        //   2
        //  / \
        // 3   4
        // diameter = 2 (3 → 2 → 4) not through root
        let root = TreeNode::from_vec(&[Some(1), Some(2), None, Some(3), Some(4)]);
        assert_eq!(diameter_of_binary_tree(root), 2);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("Diameter Binary Tree exercises - run tests with cargo test");
}