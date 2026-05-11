//! Binary Tree Level Order Traversal Exercises (LeetCode #102)
//!
//! This module contains exercises for level order (BFS) traversal of binary trees.

use std::collections::{VecDeque};

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
// Exercise 1: Level Order Traversal - BFS (Primary Solution)
// ============================================================================

/// Given root of binary tree, return level order traversal (left to right, level by level).
/// Result is a Vec of Vecs, where each inner Vec contains values at that level.
///
/// Time Complexity: O(n)
/// Space Complexity: O(w) where w is max width
pub fn level_order(root: Option<Box<TreeNode>>) -> Vec<Vec<i32>> {
    todo!("Implement BFS level order traversal")
}

// ============================================================================
// Exercise 2: Level Order Bottom-Up
// ============================================================================

/// Return level order traversal from bottom-up (level by level from leaves to root).
///
/// Time Complexity: O(n)
/// Space Complexity: O(w)
pub fn level_order_bottom(root: Option<Box<TreeNode>>) -> Vec<Vec<i32>> {
    todo!("Implement bottom-up level order")
}

// ============================================================================
// Exercise 3: Zigzag Level Order Traversal (LeetCode #103)
// ============================================================================

/// Return zigzag level order (first level left-to-right, second right-to-left, etc.)
///
/// Time Complexity: O(n)
/// Space Complexity: O(w)
pub fn zigzag_level_order(root: Option<Box<TreeNode>>) -> Vec<Vec<i32>> {
    todo!("Implement zigzag level order traversal")
}

// ============================================================================
// Exercise 4: Average of Levels (LeetCode #637)
// ============================================================================

/// Return the average value at each level, rounded to the nearest integer.
///
/// Time Complexity: O(n)
/// Space Complexity: O(w)
pub fn average_of_levels(root: Option<Box<TreeNode>>) -> Vec<f64> {
    todo!("Implement level averages calculation")
}

// ============================================================================
// Exercise 5: Right Side View (LeetCode #199)
// ============================================================================

/// Return values of rightmost node at each level.
///
/// Time Complexity: O(n)
/// Space Complexity: O(h)
pub fn right_side_view(root: Option<Box<TreeNode>>) -> Vec<i32> {
    todo!("Implement right side view of binary tree")
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Exercise 1: Basic Level Order Tests
    #[test]
    fn test_level_order_basic() {
        //     3
        //    / \
        //   9   20
        //      /  \
        //     15   7
        let root = TreeNode::from_vec(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
        let result = level_order(root);
        assert_eq!(result, vec![vec![3], vec![9, 20], vec![15, 7]]);
    }

    #[test]
    fn test_level_order_empty() {
        let result = level_order(None);
        assert_eq!(result, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_level_order_single() {
        let root = TreeNode::from_vec(&[Some(1)]);
        let result = level_order(root);
        assert_eq!(result, vec![vec![1]]);
    }

    #[test]
    fn test_level_order_two_levels() {
        //   1
        //  / \
        // 2   3
        let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3)]);
        let result = level_order(root);
        assert_eq!(result, vec![vec![1], vec![2, 3]]);
    }

    #[test]
    fn test_level_order_three_levels() {
        //     1
        //    / \
        //   2   3
        //  / \
        // 4   5
        let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3), Some(4), Some(5)]);
        let result = level_order(root);
        assert_eq!(result, vec![vec![1], vec![2, 3], vec![4, 5]]);
    }

    // Exercise 2: Bottom-Up Tests
    #[test]
    fn test_level_order_bottom_basic() {
        let root = TreeNode::from_vec(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
        let result = level_order_bottom(root);
        assert_eq!(result, vec![vec![15, 7], vec![9, 20], vec![3]]);
    }

    #[test]
    fn test_level_order_bottom_empty() {
        let result = level_order_bottom(None);
        assert_eq!(result, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_level_order_bottom_single() {
        let root = TreeNode::from_vec(&[Some(1)]);
        let result = level_order_bottom(root);
        assert_eq!(result, vec![vec![1]]);
    }

    // Exercise 3: Zigzag Tests
    #[test]
    fn test_zigzag_basic() {
        //     3
        //    / \
        //   9   20
        //      /  \
        //     15   7
        // Level 0: [3] (L2R)
        // Level 1: [20, 9] (R2L)
        // Level 2: [15, 7] (L2R)
        let root = TreeNode::from_vec(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
        let result = zigzag_level_order(root);
        assert_eq!(result, vec![vec![3], vec![20, 9], vec![15, 7]]);
    }

    #[test]
    fn test_zigzag_empty() {
        let result = zigzag_level_order(None);
        assert_eq!(result, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_zigzag_single() {
        let root = TreeNode::from_vec(&[Some(1)]);
        let result = zigzag_level_order(root);
        assert_eq!(result, vec![vec![1]]);
    }

    #[test]
    fn test_zigzag_two_levels() {
        //   1
        //  / \
        // 2   3
        let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3)]);
        let result = zigzag_level_order(root);
        assert_eq!(result, vec![vec![1], vec![3, 2]]);
    }

    // Exercise 4: Average Tests
    #[test]
    fn test_average_basic() {
        //     3
        //    / \
        //   9   20
        // Level 0 avg: 3
        // Level 1 avg: (9 + 20) / 2 = 14.5
        let root = TreeNode::from_vec(&[Some(3), Some(9), Some(20)]);
        let result = average_of_levels(root);
        assert_eq!(result, vec![3.0, 14.5]);
    }

    #[test]
    fn test_average_empty() {
        let result = average_of_levels(None);
        assert_eq!(result, Vec::<f64>::new());
    }

    #[test]
    fn test_average_single() {
        let root = TreeNode::from_vec(&[Some(1)]);
        let result = average_of_levels(root);
        assert_eq!(result, vec![1.0]);
    }

    #[test]
    fn test_average_three_levels() {
        //     3
        //    / \
        //   9   20
        //  / \    \
        // 8   15   7
        // Level 0: 3
        // Level 1: (9 + 20) / 2 = 14.5
        // Level 2: (8 + 15 + 7) / 3 = 10.0
        let root = TreeNode::from_vec(&[Some(3), Some(9), Some(20), Some(8), Some(15), None, Some(7)]);
        let result = average_of_levels(root);
        assert_eq!(result, vec![3.0, 14.5, 10.0]);
    }

    // Exercise 5: Right Side View Tests
    #[test]
    fn test_right_side_basic() {
        //     1
        //    / \
        //   2   3
        //  /
        // 4
        // Right view: [1, 3, 4]
        let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3), Some(4)]);
        let result = right_side_view(root);
        assert_eq!(result, vec![1, 3, 4]);
    }

    #[test]
    fn test_right_side_empty() {
        let result = right_side_view(None);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_right_side_single() {
        let root = TreeNode::from_vec(&[Some(1)]);
        let result = right_side_view(root);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_right_side_complete() {
        //   1
        //  / \
        // 2   3
        // Right view: [1, 3]
        let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3)]);
        let result = right_side_view(root);
        assert_eq!(result, vec![1, 3]);
    }

    #[test]
    fn test_right_side_skewed_left() {
        //   1
        //  /
        // 2
        //  \
        //   3
        // Right view: [1, 3]
        let root = TreeNode::from_vec(&[Some(1), Some(2), None, None, Some(3)]);
        let result = right_side_view(root);
        assert_eq!(result, vec![1, 3]);
    }

    // Additional edge cases
    #[test]
    fn test_level_order_all_left() {
        // 1
        //  \
        //   2
        //    \
        //     3
        //      \
        //       4
        let root = TreeNode::from_vec(&[Some(1), None, Some(2), None, Some(3), None, Some(4)]);
        let result = level_order(root);
        assert_eq!(result, vec![vec![1], vec![2], vec![3], vec![4]]);
    }

    #[test]
    fn test_level_order_all_right() {
        // 1
        //  \
        //   2
        //  /
        // 3
        //  \
        //   4
        let root = TreeNode::from_vec(&[Some(1), None, Some(2), Some(3), None, None, Some(4)]);
        let result = level_order(root);
        assert_eq!(result, vec![vec![1], vec![2], vec![3], vec![4]]);
    }

    #[test]
    fn test_zigzag_single_node() {
        let root = TreeNode::from_vec(&[Some(42)]);
        let result = zigzag_level_order(root);
        assert_eq!(result, vec![vec![42]]);
    }

    #[test]
    fn test_average_single_level() {
        //     1
        //    / \
        //   2   3
        // Level 0 avg: 1
        // Level 1 avg: (2 + 3) / 2 = 2.5
        let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3)]);
        let result = average_of_levels(root);
        assert_eq!(result, vec![1.0, 2.5]);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("Level Order Traversal exercises - run tests with cargo test");
}