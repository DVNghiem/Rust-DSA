# Implement Trie (Prefix Tree) - LeetCode 208

## Problem Overview

Implement a Trie (prefix tree) with:
- `insert(word)` - insert a word
- `search(word)` - check if word exists
- `starts_with(prefix)` - check if any word starts with prefix

**Examples:**
```
Trie t = new Trie();
t.insert("apple");
t.search("apple") → true
t.search("app") → false
t.starts_with("app") → true
```

## Theory

### Trie Node Structure

```
root
├── 'a'
│   └── 'p'
│       └── 'p'
│           └── 'l'
│               └── 'e' (word end)
└── 'b'
    └── ...
```

Each node has:
- `children`: HashMap<Char, TrieNode>
- `is_end`: bool (marks word end)

## Implementation

```rust
use std::collections::HashMap;

struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_end: false,
        }
    }
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

## Test Cases

```rust
#[test]
fn test_trie_basic() {
    let mut t = Trie::new();
    t.insert("apple");
    assert!(t.search("apple"));
    assert!(!t.search("app"));
    assert!(t.starts_with("app"));
}
```