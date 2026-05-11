# Serialize and Deserialize Tree Solution - LeetCode 297 (Complete)

## Solution Analysis

### Preorder Traversal with Null Markers

```rust
pub struct Codec;

impl Codec {
    pub fn serialize(&self, root: Option<Box<TreeNode>>) -> String {
        let mut result = String::new();
        fn encode(node: &Option<Box<TreeNode>>, result: &mut String) {
            match node {
                None => { result.push_str("null,"); }
                Some(n) => {
                    result.push_str(&format!("{},", n.val));
                    encode(&n.left, result);
                    encode(&n.right, result);
                }
            }
        }
        encode(&root, &mut result);
        result
    }

    pub fn deserialize(&self, data: String) -> Option<Box<TreeNode>> {
        let mut chars = data.split(',').collect::<Vec<_>>();
        fn decode(chars: &mut Vec<&str>) -> Option<Box<TreeNode>> {
            let val = chars.remove(0)?;
            if val == "null" { return None; }
            Some(Box::new(TreeNode {
                val: val.parse().unwrap(),
                left: decode(chars),
                right: decode(chars),
            }))
        }
        decode(&mut chars)
    }
}
```

## Why Preorder Traversal?

### Preorder = Root, Left, Right
- Easy to reconstruct by reading nodes in order
- Root comes first, so we know the tree structure immediately

### Null Markers
- Used to represent missing children
- Distinguish between "no child" and "child with value 0"

## Serialization Process

### Encode Function

```rust
fn encode(node: &Option<Box<TreeNode>>, result: &mut String) {
    match node {
        None => { result.push_str("null,"); }
        Some(n) => {
            result.push_str(&format!("{},", n.val));
            encode(&n.left, result);
            encode(&n.right, result);
        }
    }
}
```

1. If node is None: append "null,"
2. If node exists:
   - Append value + ","
   - Recursively encode left subtree
   - Recursively encode right subtree

### Example

```
       1
      / \
     2   3
        / \
       4   5

Serialized: "1,2,null,null,null,3,4,null,null,5,null,null,"
```

Breakdown:
- 1 (root)
- 2 (left child) → null, null (no children)
- 3 (right child) → 4 (left) → null, null (no children) → 5 (right) → null, null (no children)

## Deserialization Process

### Decode Function

```rust
fn decode(chars: &mut Vec<&str>) -> Option<Box<TreeNode>> {
    let val = chars.remove(0)?;
    if val == "null" { return None; }
    Some(Box::new(TreeNode {
        val: val.parse().unwrap(),
        left: decode(chars),
        right: decode(chars),
    }))
}
```

1. Take first element from chars
2. If "null": return None
3. Otherwise: parse value, recursively decode left, recursively decode right

### Example

Input: "1,2,null,null,null,3,4,null,null,5,null,null,"

```
decode(): val="1" → node with val=1
  left = decode(): val="2" → node with val=2
    left = decode(): val="null" → None
    right = decode(): val="null" → None
  right = decode(): val="3" → node with val=3
    left = decode(): val="4" → node with val=4
      left = decode(): val="null" → None
      right = decode(): val="null" → None
    right = decode(): val="5" → node with val=5
      left = decode(): val="null" → None
      right = decode(): val="null" → None
```

## Complexity Analysis

| Operation | Time | Space |
|----------|------|-------|
| Serialize | O(n) | O(n) for result string |
| Deserialize | O(n) | O(n) for tree |

Where n = number of nodes in tree.

## Edge Cases

### Empty Tree
```rust
Input: None
Output: "null,"
```

### Single Node
```rust
Input: TreeNode { val: 5, left: None, right: None }
Output: "5,null,null,"
```

### Skewed Tree (all left)
```
       1
      /
     2
    /
   3

Output: "1,2,3,null,null,null,null,"
```

## Why Comma-Separated?

Using comma as separator:
- Distinguishes between multi-digit numbers
- e.g., "10,null" vs "1,0,null"
- Easy to split and parse

## Common Mistakes

1. **Not handling null properly**: Causes deserialization to fail
2. **Using wrong traversal**: Postorder would also work but is less intuitive
3. **Forgetting trailing comma**: Makes parsing ambiguous

## Rust-Specific Patterns

1. **`Option<Box<TreeNode>>`**: Smart pointer for tree nodes, Option for nullable
2. **`Box::new(...)`**: Allocate on heap, necessary for recursive types
3. **`chars.remove(0)`**: O(n) but acceptable since we traverse all elements
4. **`format!("{},", val)`**: Convert number to string with comma