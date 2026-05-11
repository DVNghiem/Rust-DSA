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

/// Approach 1: Recursive BST-based solution
///
/// BST property: left < root < right
/// If both p and q are on same side, go that direction
/// Otherwise, current node is LCA
pub fn lowest_common_ancestor(
    root: Option<&TreeNode>,
    p: i32,
    q: i32,
) -> Option<i32> {
    fn find_lca(node: Option<&TreeNode>, p: i32, q: i32) -> Option<i32> {
        match node {
            None => None,
            Some(n) => {
                // Both in left subtree
                if p < n.val && q < n.val {
                    return find_lca(n.left.as_deref(), p, q);
                }
                // Both in right subtree
                if p > n.val && q > n.val {
                    return find_lca(n.right.as_deref(), p, q);
                }
                // On different sides or one is current → current is LCA
                Some(n.val)
            }
        }
    }
    find_lca(root, p, q)
}

/// Approach 2: Iterative solution
pub fn lowest_common_ancestor_iterative(
    root: Option<&TreeNode>,
    p: i32,
    q: i32,
) -> Option<i32> {
    let mut current = root.as_deref();

    while let Some(node) = current {
        if p < node.val && q < node.val {
            current = node.left.as_deref();
        } else if p > node.val && q > node.val {
            current = node.right.as_deref();
        } else {
            // Found LCA
            return Some(node.val);
        }
    }

    None
}

/// Helper to find node by value and return its path
fn find_path(root: Option<&TreeNode>, target: i32, path: &mut Vec<i32>) -> bool {
    match root {
        None => false,
        Some(n) => {
            path.push(n.val);
            if n.val == target {
                return true;
            }
            if target < n.val {
                if find_path(n.left.as_deref(), target, path) {
                    return true;
                }
            } else {
                if find_path(n.right.as_deref(), target, path) {
                    return true;
                }
            }
            path.pop();
            false
        }
    }
}

/// Approach 3: Find paths and get last common node (works for any binary tree)
pub fn lowest_common_ancestor_path(
    root: Option<&TreeNode>,
    p: i32,
    q: i32,
) -> Option<i32> {
    let mut path_p = Vec::new();
    let mut path_q = Vec::new();

    if !find_path(root, p, &mut path_p) || !find_path(root, q, &mut path_q) {
        return None;
    }

    let mut lca = path_p[0];
    for i in 0..path_p.len().min(path_q.len()) {
        if path_p[i] == path_q[i] {
            lca = path_p[i];
        } else {
            break;
        }
    }

    Some(lca)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tree_node_with_val(root: Option<&TreeNode>, val: i32) -> Option<&TreeNode> {
        fn find(node: Option<&TreeNode>, val: i32) -> Option<&TreeNode> {
            match node {
                None => None,
                Some(n) => {
                    if n.val == val {
                        return Some(n);
                    }
                    find(n.left.as_deref(), val)
                        .or_else(|| find(n.right.as_deref(), val))
                }
            }
        }
        find(root, val)
    }

    #[test]
    fn test_p_q_on_different_sides() {
        //       6
        //      / \
        //     2   8
        //    / \ / \
        //   0  4 7  9
        //      / \
        //     3   5
        let root = TreeNode::from_vec(vec![
            Some(6), Some(2), Some(8),
            Some(0), Some(4), Some(7), Some(9),
            None, None, Some(3), Some(5), None, None, None, None
        ]);
        // LCA of 2 (val=2) and 8 (val=8) is 6
        assert_eq!(lowest_common_ancestor(root.as_deref(), 2, 8), Some(6));
    }

    #[test]
    fn test_p_ancestor_of_q() {
        //       6
        //      / \
        //     2   8
        let root = TreeNode::from_vec(vec![Some(6), Some(2), Some(8)]);
        // LCA of 2 and 4 is 2 (2 is ancestor of 4 in this tree)
        let root2 = TreeNode::from_vec(vec![Some(6), Some(2), Some(8)]);
        assert_eq!(lowest_common_ancestor(root2.as_deref(), 2, 4), Some(2));
    }

    #[test]
    fn test_q_ancestor_of_p() {
        //       6
        //      / \
        //     2   8
        let root = TreeNode::from_vec(vec![Some(6), Some(2), Some(8)]);
        // LCA of 4 and 2 is 2 (swap of previous)
        assert_eq!(lowest_common_ancestor(root.as_deref(), 4, 2), Some(2));
    }

    #[test]
    fn test_root_is_lca() {
        //     2
        //    / \
        //   0   4
        let root = TreeNode::from_vec(vec![Some(2), Some(0), Some(4)]);
        // LCA of 0 and 4 is root 2
        assert_eq!(lowest_common_ancestor(root.as_deref(), 0, 4), Some(2));
    }

    #[test]
    fn test_both_in_left_subtree() {
        //       6
        //      / \
        //     2   8
        //    / \
        //   0   4
        let root = TreeNode::from_vec(vec![Some(6), Some(2), Some(8), Some(0), Some(4)]);
        // LCA of 0 and 4 is 2
        assert_eq!(lowest_common_ancestor(root.as_deref(), 0, 4), Some(2));
    }

    #[test]
    fn test_both_in_right_subtree() {
        //       6
        //      / \
        //     2   8
        //        / \
        //       7   9
        let root = TreeNode::from_vec(vec![Some(6), Some(2), Some(8), None, None, Some(7), Some(9)]);
        // LCA of 7 and 9 is 8
        assert_eq!(lowest_common_ancestor(root.as_deref(), 7, 9), Some(8));
    }

    #[test]
    fn test_deep_lca() {
        //       20
        //      /  \
        //     10   30
        //    /  \
        //   5   15
        //      /
        //     12
        let root = TreeNode::from_vec(vec![Some(20), Some(10), Some(30), Some(5), Some(15), None, None, None, Some(12)]);
        // LCA of 5 and 12 is 10
        assert_eq!(lowest_common_ancestor(root.as_deref(), 5, 12), Some(10));
    }

    #[test]
    fn test_adjacent_nodes() {
        //     5
        //    / \
        //   3   7
        let root = TreeNode::from_vec(vec![Some(5), Some(3), Some(7)]);
        // LCA of 3 and 5 is 5 (parent-child)
        assert_eq!(lowest_common_ancestor(root.as_deref(), 3, 5), Some(5));
    }

    #[test]
    fn test_iterative_same_as_recursive() {
        let root = TreeNode::from_vec(vec![
            Some(6), Some(2), Some(8),
            Some(0), Some(4), Some(7), Some(9)
        ]);
        let recursive = lowest_common_ancestor(root.as_deref(), 2, 8);
        let root2 = TreeNode::from_vec(vec![
            Some(6), Some(2), Some(8),
            Some(0), Some(4), Some(7), Some(9)
        ]);
        let iterative = lowest_common_ancestor_iterative(root2.as_deref(), 2, 8);
        assert_eq!(recursive, iterative);
    }

    #[test]
    fn test_path_method_same_result() {
        let root = TreeNode::from_vec(vec![Some(6), Some(2), Some(8), Some(0), Some(4), Some(7), Some(9)]);
        let bst_result = lowest_common_ancestor(root.as_deref(), 0, 7);
        let root2 = TreeNode::from_vec(vec![Some(6), Some(2), Some(8), Some(0), Some(4), Some(7), Some(9)]);
        let path_result = lowest_common_ancestor_path(root2.as_deref(), 0, 7);
        assert_eq!(bst_result, path_result);
    }

    #[test]
    fn test_single_node_tree() {
        let root = TreeNode::new(5);
        // LCA of 5 and 5 is 5
        assert_eq!(lowest_common_ancestor(Some(&root), 5, 5), Some(5));
    }

    #[test]
    fn test_left_child_lca() {
        //     5
        //    /
        //   3
        let root = TreeNode::from_vec(vec![Some(5), Some(3), None]);
        // LCA of 3 and 5 is 5
        assert_eq!(lowest_common_ancestor(root.as_deref(), 3, 5), Some(5));
    }

    #[test]
    fn test_right_child_lca() {
        //   5
        //    \
        //     7
        let root = TreeNode::from_vec(vec![Some(5), None, Some(7)]);
        // LCA of 5 and 7 is 5
        assert_eq!(lowest_common_ancestor(root.as_deref(), 5, 7), Some(5));
    }

    #[test]
    fn test_large_balanced_tree() {
        //           50
        //       /       \
        //      25       75
        //     /  \     /  \
        //   10   30  60   90
        let root = TreeNode::from_vec(vec![
            Some(50), Some(25), Some(75),
            Some(10), Some(30), Some(60), Some(90)
        ]);
        assert_eq!(lowest_common_ancestor(root.as_deref(), 10, 30), Some(25));
        assert_eq!(lowest_common_ancestor(root.as_deref(), 60, 90), Some(75));
        assert_eq!(lowest_common_ancestor(root.as_deref(), 10, 90), Some(50));
    }

    #[test]
    fn test_skewed_left_tree() {
        //     10
        //    /
        //   5
        //  /
        // 3
        let root = TreeNode::from_vec(vec![Some(10), Some(5), None, Some(3), None, None, None]);
        assert_eq!(lowest_common_ancestor(root.as_deref(), 5, 3), Some(5));
    }

    #[test]
    fn test_skewed_right_tree() {
        //   3
        //    \
        //     5
        //      \
        //       10
        let root = TreeNode::from_vec(vec![Some(3), None, Some(5), None, None, None, Some(10)]);
        assert_eq!(lowest_common_ancestor(root.as_deref(), 5, 10), Some(5));
    }

    #[test]
    fn test_negative_values() {
        //       -5
        //      /  \
        //   -10   -3
        let root = TreeNode::from_vec(vec![Some(-5), Some(-10), Some(-3)]);
        assert_eq!(lowest_common_ancestor(root.as_deref(), -10, -3), Some(-5));
        assert_eq!(lowest_common_ancestor(root.as_deref(), -10, -5), Some(-5));
    }

    #[test]
    fn test_iterative_deep_tree() {
        //       100
        //       /
        //      50
        //     /
        //   25
        //  /
        // 10
        let root = TreeNode::from_vec(vec![Some(100), Some(50), None, Some(25), None, Some(10), None]);
        assert_eq!(lowest_common_ancestor_iterative(root.as_deref(), 25, 10), Some(25));
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("028_lowest_common_ancestor_lc235 exercises - run tests with cargo test");
}
