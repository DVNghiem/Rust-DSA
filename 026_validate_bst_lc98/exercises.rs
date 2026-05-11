//! Validate BST Exercises
//!
//! This module contains exercises to practice validating Binary Search Trees.

// Definition for a binary tree node
#[derive(Debug, PartialEq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn from_vec(v: Vec<Option<i32>>) -> Option<Box<TreeNode>> {
        if v.is_empty() || v[0].is_none() {
            return None;
        }
        let mut root = Box::new(TreeNode::new(v[0].unwrap()));
        let mut queue = vec![root.as_mut()];
        let mut i = 1;

        while !queue.is_empty() && i < v.len() {
            let node = queue.remove(0);
            if let Some(left_val) = v.get(i).and_then(|x| *x) {
                node.left = Some(Box::new(TreeNode::new(left_val)));
                queue.push(node.left.as_mut().unwrap());
            }
            i += 1;
            if i < v.len() {
                if let Some(right_val) = v.get(i).and_then(|x| *x) {
                    node.right = Some(Box::new(TreeNode::new(right_val)));
                    queue.push(node.right.as_mut().unwrap());
                }
            }
            i += 1;
        }
        Some(root)
    }
}

/// Approach 1: Recursive DFS with Min/Max bounds
///
/// Each node must be within (min, max) range.
/// Left children get max = node.val
/// Right children get min = node.val
pub fn is_valid_bst_recursive(root: Option<&TreeNode>) -> bool {
    fn validate(node: Option<&TreeNode>, min: Option<i64>, max: Option<i64>) -> bool {
        match node {
            None => true,
            Some(n) => {
                let val = n.val as i64;
                if let Some(min_val) = min {
                    if val <= min_val {
                        return false;
                    }
                }
                if let Some(max_val) = max {
                    if val >= max_val {
                        return false;
                    }
                }
                validate(n.left.as_deref(), min, Some(val))
                    && validate(n.right.as_deref(), Some(val), max)
            }
        }
    }
    validate(root, None, None)
}

/// Approach 2: Inorder Traversal (valid BST produces sorted sequence)
///
/// Inorder traversal of a BST gives elements in strictly increasing order.
/// We track the previous value and ensure each new value is greater.
pub fn is_valid_bst_inorder(root: Option<&TreeNode>) -> bool {
    fn inorder(node: Option<&TreeNode>, prev: &mut Option<i64>) -> bool {
        match node {
            None => true,
            Some(n) => {
                if !inorder(n.left.as_deref(), prev) {
                    return false;
                }
                if let Some(p) = *prev {
                    if n.val as i64 <= p {
                        return false;
                    }
                }
                *prev = Some(n.val as i64);
                inorder(n.right.as_deref(), prev)
            }
        }
    }
    inorder(root, &mut None)
}

/// Approach 3: Iterative with explicit stack
pub fn is_valid_bst_iterative(root: Option<&TreeNode>) -> bool {
    let mut stack: Vec<(&TreeNode, Option<i64>, Option<i64>)> = Vec::new();
    if let Some(r) = root.as_deref() {
        stack.push((r, None, None));
    }

    while let Some((node, min, max)) = stack.pop() {
        let val = node.val as i64;
        if let Some(min_val) = min {
            if val <= min_val {
                return false;
            }
        }
        if let Some(max_val) = max {
            if val >= max_val {
                return false;
            }
        }
        if let Some(left_node) = &node.left {
            let left_val = left_node.val as i64;
            if val <= min.unwrap_or(i64::MIN) || left_val <= val {
                return false;
            }
            stack.push((left_node.as_ref(), Some(val), max));
        }
        if let Some(right_node) = &node.right {
            let right_val = right_node.val as i64;
            if val >= max.unwrap_or(i64::MAX) || right_val <= val {
                return false;
            }
            stack.push((right_node.as_ref(), min, Some(val)));
        }
    }
    true
}

/// Returns true if the BST is valid, false otherwise
pub fn is_valid_bst(root: Option<&TreeNode>) -> bool {
    is_valid_bst_recursive(root)
}

fn main() {
    println!("Run `cargo test` to test your solutions!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_valid_bst() {
        //     2
        //    / \
        //   1   3
        let root = TreeNode::from_vec(vec![Some(2), Some(1), Some(3)]);
        assert!(is_valid_bst(root.as_deref()));
    }

    #[test]
    fn test_invalid_bst() {
        //     5
        //    / \
        //   1   4
        //      / \
        //     3   6
        // 4 is in right subtree but 4 < 5, invalid
        let root = TreeNode::from_vec(vec![Some(5), Some(1), Some(4), None, None, Some(3), Some(6)]);
        assert!(!is_valid_bst(root.as_deref()));
    }

    #[test]
    fn test_empty_tree() {
        assert!(is_valid_bst(None));
    }

    #[test]
    fn test_single_node() {
        let root = TreeNode::new(1);
        assert!(is_valid_bst(Some(&root)));
    }

    #[test]
    fn test_strictly_increasing() {
        //       1
        //         \
        //          2
        //           \
        //            3
        //             \
        //              4
        let root = TreeNode::from_vec(vec![Some(1), None, Some(2), None, Some(3), None, Some(4)]);
        assert!(is_valid_bst(root.as_deref()));
    }

    #[test]
    fn test_left_skewed() {
        //        4
        //       /
        //      3
        //     /
        //    2
        //   /
        //  1
        let root = TreeNode::from_vec(vec![Some(4), Some(3), None, Some(2), None, Some(1), None]);
        assert!(is_valid_bst(root.as_deref()));
    }

    #[test]
    fn test_negative_numbers() {
        //     -1
        //    / \
        //  -5   3
        let root = TreeNode::from_vec(vec![Some(-1), Some(-5), Some(3)]);
        assert!(is_valid_bst(root.as_deref()));
    }

    #[test]
    fn test_large_values() {
        let root = TreeNode::from_vec(vec![Some(i32::MAX), Some(i32::MIN), None]);
        // i32::MIN < i32::MAX, so this is valid
        assert!(is_valid_bst(root.as_deref()));
    }

    #[test]
    fn test_duplicates_invalid() {
        //     2
        //    / \
        //   2   3
        // Duplicate 2 at root and left child - invalid
        let root = TreeNode::from_vec(vec![Some(2), Some(2), Some(3)]);
        assert!(!is_valid_bst(root.as_deref()));
    }

    #[test]
    fn test_complex_valid_bst() {
        //       10
        //      /  \
        //     5    15
        //    / \   / \
        //   3   7 12  20
        let root = TreeNode::from_vec(vec![Some(10), Some(5), Some(15), Some(3), Some(7), Some(12), Some(20)]);
        assert!(is_valid_bst(root.as_deref()));
    }

    #[test]
    fn test_invalid_deep_left() {
        //        5
        //       /
        //      3
        //       \
        //        4   <- 4 > 3 but 4 < 5, should be in right subtree of 3
        let root = TreeNode::from_vec(vec![Some(5), Some(3), None, None, Some(4)]);
        assert!(!is_valid_bst(root.as_deref()));
    }

    #[test]
    fn test_all_left_subtrees() {
        //        5
        //       /
        //      4
        //     /
        //    3
        //   /
        //  2
        let root = TreeNode::from_vec(vec![Some(5), Some(4), None, Some(3), None, Some(2), None]);
        assert!(is_valid_bst(root.as_deref()));
    }

    #[test]
    fn test_all_right_subtrees() {
        //  1
        //   \
        //    2
        //     \
        //      3
        //       \
        //        4
        let root = TreeNode::from_vec(vec![Some(1), None, Some(2), None, Some(3), None, Some(4)]);
        assert!(is_valid_bst(root.as_deref()));
    }

    #[test]
    fn test_balanced_bst() {
        //       8
        //      / \
        //     3   10
        //    / \    \
        //   1   6    14
        //      / \   /
        //     4   7 13
        let root = TreeNode::from_vec(vec![
            Some(8), Some(3), Some(10),
            Some(1), Some(6), None, Some(14),
            None, Some(4), Some(7), None, None, Some(13), None, None
        ]);
        assert!(is_valid_bst(root.as_deref()));
    }

    #[test]
    fn test_balanced_bst_invalid() {
        //       8
        //      / \
        //     3   10
        //    / \    \
        //   1   7    14  <- 7 should be > 8 but 7 < 8, invalid
        let root = TreeNode::from_vec(vec![
            Some(8), Some(3), Some(10),
            Some(1), Some(7), None, Some(14)
        ]);
        assert!(!is_valid_bst(root.as_deref()));
    }

    #[test]
    fn test_inorder_consistency() {
        // Both methods should give same result
        let root = TreeNode::from_vec(vec![Some(5), Some(3), Some(7), Some(2), Some(4), Some(6), Some(8)]);
        let recursive = is_valid_bst_recursive(root.as_deref());
        // Recreate tree for second test since first consumes it
        let root2 = TreeNode::from_vec(vec![Some(5), Some(3), Some(7), Some(2), Some(4), Some(6), Some(8)]);
        let inorder = is_valid_bst_inorder(root2.as_deref());
        assert_eq!(recursive, inorder);
    }

    #[test]
    fn test_iterative_consistency() {
        let root = TreeNode::from_vec(vec![Some(2), Some(1), Some(3)]);
        let recursive = is_valid_bst_recursive(root.as_deref());
        let root2 = TreeNode::from_vec(vec![Some(2), Some(1), Some(3)]);
        let iterative = is_valid_bst_iterative(root2.as_deref());
        assert_eq!(recursive, iterative);
    }

    #[test]
    fn test_height_one_tree() {
        //     0
        //    / \
        //  -1   0
        let root = TreeNode::from_vec(vec![Some(0), Some(-1), Some(0)]);
        assert!(!is_valid_bst(root.as_deref())); // 0 appears twice
    }

    #[test]
    fn test_wide_tree_valid() {
        //       5
        //   / / \ \
        //  1 2  3  4
        let root = TreeNode::from_vec(vec![Some(5), Some(1), Some(4), Some(2), Some(3)]);
        assert!(is_valid_bst(root.as_deref()));
    }

    #[test]
    fn test_wide_tree_invalid() {
        //       5
        //   / / \ \
        //  1 6  3  4  <- 6 > 5 but in left subtree, invalid
        let root = TreeNode::from_vec(vec![Some(5), Some(1), Some(4), Some(6), Some(3)]);
        assert!(!is_valid_bst(root.as_deref()));
    }
}