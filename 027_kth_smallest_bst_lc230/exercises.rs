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

/// Approach 1: Inorder traversal with early termination
///
/// BST inorder traversal visits nodes in ascending order.
/// We track a counter and stop when it reaches k.
pub fn kth_smallest(root: Option<&TreeNode>, k: i32) -> i32 {
    let mut count = 0;
    let mut result = 0;

    fn inorder(node: Option<&TreeNode>, k: i32, count: &mut i32, result: &mut i32) {
        if node.is_none() || *count >= k {
            return;
        }

        if let Some(n) = node {
            // Traverse left first (smaller values)
            inorder(n.left.as_deref(), k, count, result);

            // Process current node
            *count += 1;
            if *count == k {
                *result = n.val;
                return;
            }

            // Traverse right (larger values)
            inorder(n.right.as_deref(), k, count, result);
        }
    }

    inorder(root, k, &mut count, &mut result);
    result
}

/// Approach 2: Collect all values then select (simpler but O(n) space)
pub fn kth_smallest_array(root: Option<&TreeNode>, k: i32) -> i32 {
    let mut values = Vec::new();

    fn collect(node: Option<&TreeNode>, values: &mut Vec<i32>) {
        if let Some(n) = node {
            collect(n.left.as_deref(), values);
            values.push(n.val);
            collect(n.right.as_deref(), values);
        }
    }

    collect(root, &mut values);
    values[(k - 1) as usize]
}

/// Approach 3: Iterative inorder with stack
pub fn kth_smallest_iterative(root: Option<&TreeNode>, k: i32) -> i32 {
    let mut stack = Vec::new();
    let mut current = root.as_deref();
    let mut count = 0;

    while current.is_some() || !stack.is_empty() {
        // Go to leftmost node
        while let Some(node) = current {
            stack.push(node);
            current = node.left.as_deref();
        }

        // Process node
        if let Some(node) = stack.pop() {
            count += 1;
            if count == k {
                return node.val;
            }
            current = node.right.as_deref();
        }
    }

    -1 // Should never reach here per problem constraints
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_kth_smallest() {
        //     3
        //    / \
        //   1   4
        //    \
        //     2
        // Inorder: 1, 2, 3, 4
        let root = TreeNode::from_vec(vec![Some(3), Some(1), Some(4), None, Some(2)]);
        assert_eq!(kth_smallest(root.as_deref(), 1), 1); // 1st smallest
        assert_eq!(kth_smallest(root.as_deref(), 2), 2); // 2nd smallest
        assert_eq!(kth_smallest(root.as_deref(), 3), 3); // 3rd smallest
        assert_eq!(kth_smallest(root.as_deref(), 4), 4); // 4th smallest
    }

    #[test]
    fn test_k_equals_one() {
        //     5
        //    / \
        //   3   7
        let root = TreeNode::from_vec(vec![Some(5), Some(3), Some(7)]);
        assert_eq!(kth_smallest(root.as_deref(), 1), 3); // Minimum
    }

    #[test]
    fn test_k_equals_n() {
        //     5
        //    / \
        //   3   7
        let root = TreeNode::from_vec(vec![Some(5), Some(3), Some(7)]);
        assert_eq!(kth_smallest(root.as_deref(), 3), 7); // Maximum
    }

    #[test]
    fn test_left_skewed() {
        //       4
        //      /
        //     3
        //    /
        //   2
        //  /
        // 1
        let root = TreeNode::from_vec(vec![Some(4), Some(3), None, Some(2), None, Some(1), None]);
        assert_eq!(kth_smallest(root.as_deref(), 1), 1);
        assert_eq!(kth_smallest(root.as_deref(), 2), 2);
        assert_eq!(kth_smallest(root.as_deref(), 3), 3);
        assert_eq!(kth_smallest(root.as_deref(), 4), 4);
    }

    #[test]
    fn test_right_skewed() {
        // 1
        //  \
        //   2
        //    \
        //     3
        //      \
        //       4
        let root = TreeNode::from_vec(vec![Some(1), None, Some(2), None, Some(3), None, Some(4)]);
        assert_eq!(kth_smallest(root.as_deref(), 1), 1);
        assert_eq!(kth_smallest(root.as_deref(), 4), 4);
    }

    #[test]
    fn test_single_node() {
        let root = TreeNode::new(5);
        assert_eq!(kth_smallest(Some(&root), 1), 5);
    }

    #[test]
    fn test_balanced_tree() {
        //       6
        //      / \
        //     3   8
        //    / \   \
        //   2   4   10
        // Inorder: 2, 3, 4, 6, 8, 10
        let root = TreeNode::from_vec(vec![Some(6), Some(3), Some(8), Some(2), Some(4), None, Some(10)]);
        assert_eq!(kth_smallest(root.as_deref(), 1), 2);
        assert_eq!(kth_smallest(root.as_deref(), 3), 4);
        assert_eq!(kth_smallest(root.as_deref(), 5), 8);
    }

    #[test]
    fn test_negative_values() {
        //     -2
        //    / \
        //  -5   -1
        let root = TreeNode::from_vec(vec![Some(-2), Some(-5), Some(-1)]);
        assert_eq!(kth_smallest(root.as_deref(), 1), -5);
        assert_eq!(kth_smallest(root.as_deref(), 2), -2);
        assert_eq!(kth_smallest(root.as_deref(), 3), -1);
    }

    #[test]
    fn test_large_k() {
        //       10
        //      /  \
        //     5    15
        //    / \   / \
        //   3   7 12  20
        // Inorder: 3, 5, 7, 10, 12, 15, 20
        let root = TreeNode::from_vec(vec![Some(10), Some(5), Some(15), Some(3), Some(7), Some(12), Some(20)]);
        assert_eq!(kth_smallest(root.as_deref(), 7), 20); // Last element
    }

    #[test]
    fn test_complex_tree() {
        //         15
        //       /    \
        //      10     20
        //     /  \   /  \
        //    8   12 17  25
        // Inorder: 8, 10, 12, 15, 17, 20, 25
        let root = TreeNode::from_vec(vec![
            Some(15), Some(10), Some(20),
            Some(8), Some(12), Some(17), Some(25)
        ]);
        assert_eq!(kth_smallest(root.as_deref(), 1), 8);
        assert_eq!(kth_smallest(root.as_deref(), 4), 15);
        assert_eq!(kth_smallest(root.as_deref(), 7), 25);
    }

    #[test]
    fn test_iterative_same_as_recursive() {
        let root = TreeNode::from_vec(vec![Some(5), Some(3), Some(7), Some(2), Some(4), Some(6), Some(8)]);
        let recursive = kth_smallest(root.as_deref(), 4);
        let root2 = TreeNode::from_vec(vec![Some(5), Some(3), Some(7), Some(2), Some(4), Some(6), Some(8)]);
        let iterative = kth_smallest_iterative(root2.as_deref(), 4);
        assert_eq!(recursive, iterative);
    }

    #[test]
    fn test_array_same_as_counter() {
        let root = TreeNode::from_vec(vec![Some(3), Some(1), Some(4), None, Some(2)]);
        for k in 1..=4 {
            let counter = kth_smallest(root.as_deref(), k);
            let root2 = TreeNode::from_vec(vec![Some(3), Some(1), Some(4), None, Some(2)]);
            let array = kth_smallest_array(root2.as_deref(), k);
            assert_eq!(counter, array, "k={}", k);
        }
    }

    #[test]
    fn test_alternative_tree_structure() {
        //     2
        //    / \
        //   1   3
        let root = TreeNode::from_vec(vec![Some(2), Some(1), Some(3)]);
        assert_eq!(kth_smallest(root.as_deref(), 1), 1);
        assert_eq!(kth_smallest(root.as_deref(), 2), 2);
        assert_eq!(kth_smallest(root.as_deref(), 3), 3);
    }

    #[test]
    fn test_deep_left_tree() {
        //        10
        //       /
        //      8
        //     /
        //    6
        //   /
        //  4
        // Inorder: 4, 6, 8, 10
        let root = TreeNode::from_vec(vec![Some(10), Some(8), None, Some(6), None, Some(4), None]);
        assert_eq!(kth_smallest(root.as_deref(), 1), 4);
        assert_eq!(kth_smallest(root.as_deref(), 4), 10);
    }

    #[test]
    fn test_deep_right_tree() {
        // 1
        //  \
        //   3
        //    \
        //     5
        //      \
        //       7
        // Inorder: 1, 3, 5, 7
        let root = TreeNode::from_vec(vec![Some(1), None, Some(3), None, Some(5), None, Some(7)]);
        assert_eq!(kth_smallest(root.as_deref(), 2), 3);
    }

    #[test]
    fn test_middle_k_balanced() {
        //       8
        //      / \
        //     4   12
        // Inorder: 4, 8, 12
        let root = TreeNode::from_vec(vec![Some(8), Some(4), Some(12)]);
        assert_eq!(kth_smallest(root.as_deref(), 2), 8);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("027_kth_smallest_bst_lc230 exercises - run tests with cargo test");
}
