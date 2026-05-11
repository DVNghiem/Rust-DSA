# Implement Trie Solution - LeetCode 208 (Complete)

## Solution Analysis

### HashMap-Based Trie

```rust
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: bool,
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie { root: TrieNode::new() }
    }

    pub fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for c in word.chars() {
            node = node.children.entry(c).or_insert_with(TrieNode::new);
        }
        node.is_end = true;
    }

    pub fn search(&self, word: String) -> bool {
        let mut node = &self.root;
        for c in word.chars() {
            match node.children.get(&c) {
                Some(n) => node = n,
                None => return false,
            }
        }
        node.is_end
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        let mut node = &self.root;
        for c in prefix.chars() {
            match node.children.get(&c) {
                Some(n) => node = n,
                None => return false,
            }
        }
        true
    }
}
```

## Trie Structure

### What is a Trie?

A tree where:
- Each node represents a prefix
- Edges represent characters
- Nodes have `is_end` flag to mark word endings

```
root
├── 'a'
│   └── 'p'
│       └── 'p'
│           └── 'l'
│               └── 'e' (is_end = true)
└── 'b'
    └── ...
```

### Why HashMap for Children?

```rust
children: HashMap<char, TrieNode>
```
- Each node has a map from character to child node
- Allows O(1) child lookup

## Line-by-Line Analysis

### Insert

```rust
pub fn insert(&mut self, word: String) {
    let mut node = &mut self.root;
    for c in word.chars() {
        node = node.children.entry(c).or_insert_with(TrieNode::new);
    }
    node.is_end = true;
}
```
- Start at root
- For each character, go to or create child
- After all characters, mark node as word end

### Search

```rust
pub fn search(&self, word: String) -> bool {
    let mut node = &self.root;
    for c in word.chars() {
        match node.children.get(&c) {
            Some(n) => node = n,
            None => return false,
        }
    }
    node.is_end  // Must be end of a word
}
```
- Traverse characters
- If any character missing, word doesn't exist
- At end, must be marked as word end (not just a prefix)

### Starts With

```rust
pub fn starts_with(&self, prefix: String) -> bool {
    let mut node = &self.root;
    for c in prefix.chars() {
        match node.children.get(&c) {
            Some(n) => node = n,
            None => return false,
        }
    }
    true  // Just need to reach end of prefix
}
```
- Same as search, but don't check `is_end`
- Just need to traverse all characters

## Visual Example

### Insert "apple", "apply", "app"

```
Insert "apple":
root → 'a' → 'p' → 'p' → 'l' → 'e' (is_end)

Insert "apply":
root → 'a' → 'p' → 'p' → 'l' → 'e' → 'y' (is_end)
(share path with "apple")

Insert "app":
root → 'a' → 'p' → 'p' (is_end)
```

### Trie Structure

```
root
└── 'a'
    └── 'p'
        └── 'p'
            ├── 'l'
            │   ├── 'e' (is_end for "apple")
            │   └── 'y' (is_end for "apply")
            └── (is_end for "app")
```

## Search vs Starts With

### Search "app"
- Traverse: root → 'a' → 'p' → 'p'
- Check: is_end at final node? YES
- Return true

### Search "appl"
- Traverse: root → 'a' → 'p' → 'p' → 'l'
- Check: is_end? NO
- Return false

### Starts With "appl"
- Traverse: root → 'a' → 'p' → 'p' → 'l'
- Found all characters
- Return true

## Complexity Analysis

| Operation | Time | Space |
|-----------|------|-------|
| insert | O(L) | O(L) |
| search | O(L) | O(1) |
| starts_with | O(L) | O(1) |

Where L = length of word/prefix.

## Edge Cases

### Empty String
```rust
insert("") → marks root as end (or not)
search("") → checks root.is_end
```
Depending on implementation, empty string may or may not be valid.

### Prefix of Word
```rust
insert("apple")
starts_with("app") → true
search("app") → false (not a complete word)
```

### Word That Is Prefix
```rust
insert("app")
insert("apple")
search("app") → true
search("apple") → true
```

## Why Two Separate Methods?

1. **search**: For checking if exact word exists
2. **starts_with**: For checking if any words have given prefix

Both are O(L) but serve different purposes in different problems.

## Common Mistakes

1. **Not marking is_end**: After insert, must mark final node
2. **Checking wrong flag**: search checks is_end, starts_with doesn't
3. **Confusing insert vs search**: Insert creates path, search traverses it