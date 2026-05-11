# Serialize and Deserialize Tree - LeetCode 297

## Problem Overview

Design an algorithm to serialize a binary tree into a string and deserialize the string back to the tree.

**Examples:**
```
Input: [1,2,3,null,null,4,5]
Output: "1,2,3,null,null,4,5,null,null,null,null"
```

## Theory

### Preorder Traversal with Null Markers

Using preorder (root, left, right) with null markers:
- Easy to reconstruct
- Handles any tree shape
- Null markers distinguish missing children

```
       1
      / \
     2   3
        / \
       4   5

Serialized: 1,2,null,null,3,4,null,null,5,null,null
```

## Implementation

```rust
pub struct Codec;

impl Codec {
    pub fn serialize(&self, root: Option<Box<TreeNode>>) -> String {
        fn encode(node: &Option<Box<TreeNode>>, sb: &mut String) {
            match node {
                None => { sb.push_str("null,"); }
                Some(n) => {
                    sb.push_str(&format!("{},", n.val));
                    encode(&n.left, sb);
                    encode(&n.right, sb);
                }
            }
        }
        let mut result = String::new();
        encode(&root, &mut result);
        result
    }

    pub fn deserialize(&self, data: String) -> Option<Box<TreeNode>> {
        let mut vals: VecDeque<&str> = data.split(',').collect();
        fn decode(vals: &mut VecDeque<&str>) -> Option<Box<TreeNode>> {
            let val = vals.pop_front()?;
            if val == "null" { return None; }
            let node = Box::new(TreeNode {
                val: val.parse().unwrap(),
                left: decode(vals),
                right: decode(vals),
            });
            Some(node)
        }
        decode(&mut vals)
    }
}
```

## Test Cases

```rust
#[test]
fn test_codec_basic() {
    let codec = Codec;
    let tree = build_tree(vec![1,2,3]);
    let serialized = codec.serialize(tree);
    let deserialized = codec.deserialize(serialized);
    assert!(deserialized.is_some());
}
```