/// Implement Trie (Prefix Tree) - LeetCode 208
/// Build a Trie with insert, search, and startsWith operations.

use std::collections::HashMap;

/// TrieNode in the prefix tree
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

/// Trie (Prefix Tree) implementation
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie { root: TrieNode::new() }
    }

    /// Insert a word into the trie
    pub fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for c in word.chars() {
            node = node.children.entry(c).or_insert_with(TrieNode::new);
        }
        node.is_end = true;
    }

    /// Search for a complete word in the trie
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

    /// Check if any word in the trie starts with the given prefix
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

/// Alternative implementation with arrays for children (faster)
pub struct TrieArray {
    children: Vec<Option<Box<TrieArray>>>,
    is_end: bool,
}

impl TrieArray {
    fn new() -> Self {
        TrieArray {
            children: (0..26).map(|_| None).collect(),
            is_end: false,
        }
    }

    pub fn insert(&mut self, word: String) {
        let mut node = self;
        for c in word.chars() {
            let idx = (c as u8 - b'a') as usize;
            if node.children[idx].is_none() {
                node.children[idx] = Some(Box::new(TrieArray::new()));
            }
            node = node.children[idx].as_mut().unwrap();
        }
        node.is_end = true;
    }

    pub fn search(&self, word: String) -> bool {
        let mut node = self;
        for c in word.chars() {
            let idx = (c as u8 - b'a') as usize;
            match node.children[idx].as_ref() {
                Some(n) => node = n,
                None => return false,
            }
        }
        node.is_end
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        let mut node = self;
        for c in prefix.chars() {
            let idx = (c as u8 - b'a') as usize;
            match node.children[idx].as_ref() {
                Some(n) => node = n,
                None => return false,
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie_basic() {
        let mut t = Trie::new();
        t.insert("apple".to_string());
        assert!(t.search("apple".to_string()));
        assert!(!t.search("app".to_string()));
        assert!(t.starts_with("app".to_string()));
    }

    #[test]
    fn test_trie_insert() {
        let mut t = Trie::new();
        t.insert("hello".to_string());
        assert!(t.search("hello".to_string()));
        assert!(t.starts_with("hel".to_string()));
    }

    #[test]
    fn test_trie_empty_search() {
        let t = Trie::new();
        assert!(!t.search("a".to_string()));
    }

    #[test]
    fn test_trie_empty_prefix() {
        let mut t = Trie::new();
        t.insert("abc".to_string());
        assert!(t.starts_with("".to_string()));
    }

    #[test]
    fn test_trie_partial_word() {
        let mut t = Trie::new();
        t.insert("cat".to_string());
        assert!(!t.search("ca".to_string()));
    }

    #[test]
    fn test_trie_multiple_words() {
        let mut t = Trie::new();
        t.insert("apple".to_string());
        t.insert("apply".to_string());
        t.insert("app".to_string());
        assert!(t.search("apple".to_string()));
        assert!(t.search("apply".to_string()));
        assert!(t.search("app".to_string()));
        assert!(t.starts_with("ap".to_string()));
    }

    #[test]
    fn test_trie_common_prefix() {
        let mut t = Trie::new();
        t.insert("dog".to_string());
        t.insert("dodge".to_string());
        t.insert("done".to_string());
        assert!(t.starts_with("do".to_string()));
        assert!(t.starts_with("dog".to_string()));
    }

    #[test]
    fn test_trie_no_prefix() {
        let mut t = Trie::new();
        t.insert("apple".to_string());
        assert!(!t.starts_with("bad".to_string()));
    }

    #[test]
    fn test_trie_array_basic() {
        let mut t = TrieArray::new();
        t.insert("apple".to_string());
        assert!(t.search("apple".to_string()));
        assert!(!t.search("app".to_string()));
        assert!(t.starts_with("app".to_string()));
    }

    #[test]
    fn test_trie_array_insert() {
        let mut t = TrieArray::new();
        t.insert("hello".to_string());
        assert!(t.search("hello".to_string()));
    }

    #[test]
    fn test_trie_array_multiple() {
        let mut t = TrieArray::new();
        t.insert("apple".to_string());
        t.insert("apply".to_string());
        assert!(t.search("apple".to_string()));
        assert!(t.search("apply".to_string()));
    }

    #[test]
    fn test_both_same_result() {
        let mut t1 = Trie::new();
        let mut t2 = TrieArray::new();

        let words = vec!["apple", "apply", "app", "dog", "dodge"];
        for w in words {
            t1.insert(w.to_string());
            t2.insert(w.to_string());
        }

        let tests = vec!["apple", "app", "apply", "dog", "dodge", "appl", "do", "cat"];
        for word in tests {
            assert_eq!(t1.search(word.to_string()), t2.search(word.to_string()));
            assert_eq!(t1.starts_with(word.to_string()), t2.starts_with(word.to_string()));
        }
    }

    #[test]
    fn test_trie_lowercase_only() {
        let mut t = Trie::new();
        t.insert("abc".to_string());
        assert!(t.search("abc".to_string()));
    }

    #[test]
    fn test_trie_single_char() {
        let mut t = Trie::new();
        t.insert("a".to_string());
        assert!(t.search("a".to_string()));
        assert!(!t.search("b".to_string()));
    }

    #[test]
    fn test_trie_long_word() {
        let mut t = Trie::new();
        let long = "a".repeat(1000);
        t.insert(long.clone());
        assert!(t.search(long));
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("Implement Trie exercises - run tests with cargo test");
}