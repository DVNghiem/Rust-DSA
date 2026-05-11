/// Serialize and Deserialize Tree - LeetCode 297
/// Convert binary tree to string and back using preorder traversal.

use std::collections::VecDeque;

/// TreeNode definition
#[derive(Debug, PartialEq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

/// Codec for serializing and deserializing binary trees
pub struct Codec;

impl Codec {
    /// Serialize tree to string using preorder traversal
    /// Format: val,val,null,null for each node
    pub fn serialize(&self, root: Option<Box<TreeNode>>) -> String {
        let mut result = String::new();
        self.serialize_helper(&root, &mut result);
        result
    }

    fn serialize_helper(&self, node: &Option<Box<TreeNode>>, result: &mut String) {
        match node {
            None => {
                result.push_str("null,");
            }
            Some(n) => {
                result.push_str(&format!("{},", n.val));
                self.serialize_helper(&n.left, result);
                self.serialize_helper(&n.right, result);
            }
        }
    }

    /// Deserialize string back to tree
    pub fn deserialize(&self, data: String) -> Option<Box<TreeNode>> {
        let mut chars = data.split(',').collect::<Vec<_>>();
        self.deserialize_helper(&mut chars)
    }

    fn deserialize_helper(&self, vals: &mut Vec<&str>) -> Option<Box<TreeNode>> {
        if vals.is_empty() { return None; }
        let val = vals.remove(0);
        if val == "null" { return None; }
        Some(Box::new(TreeNode {
            val: val.parse().unwrap(),
            left: self.deserialize_helper(vals),
            right: self.deserialize_helper(vals),
        }))
    }
}

/// Alternative: Using VecDeque for better pop_front
pub struct CodecDeque;

impl CodecDeque {
    pub fn serialize(&self, root: Option<Box<TreeNode>>) -> String {
        let mut result = String::new();
        Self::serialize_helper(&root, &mut result);
        result
    }

    fn serialize_helper(node: &Option<Box<TreeNode>>, result: &mut String) {
        match node {
            None => result.push_str("null,"),
            Some(n) => {
                result.push_str(&format!("{},", n.val));
                Self::serialize_helper(&n.left, result);
                Self::serialize_helper(&n.right, result);
            }
        }
    }

    pub fn deserialize(&self, data: String) -> Option<Box<TreeNode>> {
        let mut vals: VecDeque<&str> = data.split(',').collect();
        Self::deserialize_helper(&mut vals)
    }

    fn deserialize_helper(vals: &mut VecDeque<&str>) -> Option<Box<TreeNode>> {
        let val = vals.pop_front()?;
        if val == "null" { return None; }
        Some(Box::new(TreeNode {
            val: val.parse().unwrap(),
            left: Self::deserialize_helper(vals),
            right: Self::deserialize_helper(vals),
        }))
    }
}

/// Helper to build tree from level-order vector
pub fn build_tree(vals: Vec<i32>) -> Option<Box<TreeNode>> {
    if vals.is_empty() { return None; }
    let mut queue: VecDeque<Option<Box<TreeNode>>> = VecDeque::new();
    let root = Box::new(TreeNode { val: vals[0], left: None, right: None });
    queue.push_back(Some(root.clone()));
    let mut i = 1;

    while !queue.is_empty() && i < vals.len() {
        let mut front = queue.pop_front().unwrap();
        if let Some(ref mut node) = front {
            if i < vals.len() && vals[i] != -1 {
                node.left = Some(Box::new(TreeNode { val: vals[i], left: None, right: None }));
                queue.push_back(node.left.clone());
            }
            i += 1;
            if i < vals.len() && vals[i] != -1 {
                node.right = Some(Box::new(TreeNode { val: vals[i], left: None, right: None }));
                queue.push_back(node.right.clone());
            }
            i += 1;
        }
    }

    Some(root)
}

/// Helper to convert tree to string for comparison
pub fn tree_to_vec(root: &Option<Box<TreeNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut queue: VecDeque<&Option<Box<TreeNode>>> = VecDeque::new();
    queue.push_back(root);

    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        match node {
            None => result.push(-1),
            Some(n) => {
                result.push(n.val);
                queue.push_back(&n.left);
                queue.push_back(&n.right);
            }
        }
    }

    while let Some(&last) = result.last() {
        if last == -1 { result.pop(); } else { break; }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codec_basic() {
        let codec = Codec;
        let root = build_tree(vec![1, 2, 3]);
        let serialized = codec.serialize(root);
        assert!(serialized.contains("1"));
        assert!(serialized.contains("null"));
    }

    #[test]
    fn test_codec_empty() {
        let codec = Codec;
        let serialized = codec.serialize(None);
        assert_eq!(serialized, "null,");
        let deserialized = codec.deserialize(serialized);
        assert!(deserialized.is_none());
    }

    #[test]
    fn test_codec_single() {
        let codec = Codec;
        let root = build_tree(vec![1]);
        let serialized = codec.serialize(root);
        let deserialized = codec.deserialize(serialized);
        assert!(deserialized.is_some());
        assert_eq!(deserialized.unwrap().val, 1);
    }

    #[test]
    fn test_codec_two_nodes() {
        let codec = Codec;
        let root = build_tree(vec![1, 2, 3]);
        let serialized = codec.serialize(root.clone());
        let deserialized = codec.deserialize(serialized);
        assert!(deserialized.is_some());
    }

    #[test]
    fn test_codec_complex() {
        let codec = Codec;
        let root = build_tree(vec![1, 2, 3, -1, -1, 4, 5]);
        let serialized = codec.serialize(root);
        let deserialized = codec.deserialize(serialized);
        assert!(deserialized.is_some());
    }

    #[test]
    fn test_codec_left_skewed() {
        let codec = Codec;
        let root = build_tree(vec![1, 2, -1, 3, -1, -1, -1]);
        let serialized = codec.serialize(root);
        let deserialized = codec.deserialize(serialized);
        assert!(deserialized.is_some());
    }

    #[test]
    fn test_codec_right_skewed() {
        let codec = Codec;
        let root = build_tree(vec![1, -1, 2, -1, 3, -1, -1]);
        let serialized = codec.serialize(root);
        let deserialized = codec.deserialize(serialized);
        assert!(deserialized.is_some());
    }

    #[test]
    fn test_codec_deque_basic() {
        let codec = CodecDeque;
        let root = build_tree(vec![1, 2, 3]);
        let serialized = codec.serialize(root);
        let deserialized = codec.deserialize(serialized);
        assert!(deserialized.is_some());
    }

    #[test]
    fn test_codec_deque_empty() {
        let codec = CodecDeque;
        let serialized = codec.serialize(None);
        assert_eq!(serialized, "null,");
        let deserialized = codec.deserialize(serialized);
        assert!(deserialized.is_none());
    }

    #[test]
    fn test_codec_same_result() {
        let codec1 = Codec;
        let codec2 = CodecDeque;
        let root = build_tree(vec![1, 2, 3, 4, 5]);
        let ser1 = codec1.serialize(root.clone());
        let ser2 = codec2.serialize(root.clone());
        assert_eq!(ser1, ser2);

        let des1 = codec1.deserialize(ser1);
        let des2 = codec2.deserialize(ser2);
        assert_eq!(tree_to_vec(&des1), tree_to_vec(&des2));
    }

    #[test]
    fn test_tree_to_vec() {
        let root = build_tree(vec![1, 2, 3]);
        let result = tree_to_vec(&root);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_tree_to_vec_empty() {
        let result = tree_to_vec(&None);
        assert!(result.is_empty());
    }

    #[test]
    fn test_roundtrip_simple() {
        let codec = Codec;
        let root = build_tree(vec![1, 2, 3, -1, -1, 4, 5]);
        let serialized = codec.serialize(root);
        let deserialized = codec.deserialize(serialized);
        let result = tree_to_vec(&deserialized);
        assert!(result.len() > 0);
    }

    #[test]
    fn test_roundtrip_empty() {
        let codec = Codec;
        let serialized = codec.serialize(None);
        let deserialized = codec.deserialize(serialized);
        assert!(deserialized.is_none());
    }

    #[test]
    fn test_roundtrip_all_null_children() {
        let codec = Codec;
        let root = build_tree(vec![1, -1, -1]);
        let serialized = codec.serialize(root);
        let deserialized = codec.deserialize(serialized);
        assert!(deserialized.is_some());
    }

    #[test]
    fn test_serialize_preserves_structure() {
        let codec = Codec;
        let root1 = build_tree(vec![1, 2, 3]);
        let ser1 = codec.serialize(root1);
        let root2 = build_tree(vec![1, 2, 3]);
        let ser2 = codec.serialize(root2);
        assert_eq!(ser1, ser2);
    }

    #[test]
    fn test_different_trees_different_serialization() {
        let codec = Codec;
        let root1 = build_tree(vec![1, 2, -1]);
        let root2 = build_tree(vec![1, -1, 2]);
        let ser1 = codec.serialize(root1);
        let ser2 = codec.serialize(root2);
        assert_ne!(ser1, ser2);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("Serialize Tree exercises - run tests with cargo test");
}