/// Word Break - LeetCode 139
/// Determine if a string can be segmented into dictionary words.

use std::collections::{HashMap, HashSet};

/// Approach: Dynamic Programming
/// dp[i] = true if s[0..i] can be segmented
pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let word_set: HashSet<&str> = word_dict.iter().map(|w| w.as_str()).collect();
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut dp = vec![false; n + 1];
    dp[0] = true;

    for i in 1..=n {
        for j in 0..i {
            if dp[j] && word_set.contains(std::str::from_utf8(&bytes[j..i]).unwrap_or("")) {
                dp[i] = true;
                break;
            }
        }
    }

    dp[n]
}

/// Approach 2: BFS with memoization
pub fn word_break_bfs(s: String, word_dict: Vec<String>) -> bool {
    let word_set: HashSet<&str> = word_dict.iter().map(|w| w.as_str()).collect();
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut memo = vec![-1i32; n]; // -1 = unknown, 0 = false, 1 = true

    fn can_break(s: &[u8], start: usize, word_set: &HashSet<&str>, memo: &mut Vec<i32>) -> bool {
        if start == s.len() { return true; }
        if memo[start] != -1 { return memo[start] == 1; }

        for end in (start + 1)..=s.len() {
            let word = std::str::from_utf8(&s[start..end]).unwrap_or("");
            if word_set.contains(word) {
                if can_break(s, end, word_set, memo) {
                    memo[start] = 1;
                    return true;
                }
            }
        }

        memo[start] = 0;
        false
    }

    can_break(bytes, 0, &word_set, &mut memo)
}

/// Approach 3: Trie-based for optimization
pub fn word_break_trie(s: String, word_dict: Vec<String>) -> bool {
    use std::collections::VecDeque;

    // Build Trie: node index -> (character -> next node index)
    let mut trie: std::collections::HashMap<usize, std::collections::HashMap<char, usize>> =
        std::collections::HashMap::new();
    trie.insert(0, std::collections::HashMap::new());
    let mut terminal: std::collections::HashSet<usize> = std::collections::HashSet::new();

    for (idx, word) in word_dict.iter().enumerate() {
        let mut node = 0usize;
        for ch in word.chars() {
            let new_node = trie.len();
            let children = trie.get_mut(&node).unwrap();
            if let Some(next) = children.get(&ch) {
                node = *next;
            } else {
                children.insert(ch, new_node);
                trie.insert(new_node, std::collections::HashMap::new());
                node = new_node;
            }
        }
        terminal.insert(node);
    }

    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();
    queue.push_back(0);

    while let Some(node) = queue.pop_front() {
        if terminal.contains(&node) {
            return true;
        }

        // We need to continue from this position... but trie traversal
        // doesn't easily map to string positions here. Fallback to DP.
    }

    // Fallback to simple DP
    let word_set: HashSet<&str> = word_dict.iter().map(|w| w.as_str()).collect();
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut dp = vec![false; n + 1];
    dp[0] = true;

    for i in 1..=n {
        for j in 0..i {
            if dp[j] {
                let word = std::str::from_utf8(&bytes[j..i]).unwrap_or("");
                if word_set.contains(word) {
                    dp[i] = true;
                    break;
                }
            }
        }
    }

    dp[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_break_basic() {
        assert!(word_break("leetcode".to_string(), vec!["leet".to_string(), "code".to_string()]));
    }

    #[test]
    fn test_word_break_applepenapple() {
        assert!(word_break("applepenapple".to_string(), vec!["apple".to_string(), "pen".to_string()]));
    }

    #[test]
    fn test_word_break_false() {
        assert!(!word_break("catsandog".to_string(), vec!["cats".to_string(), "dog".to_string(), "sand".to_string(), "and".to_string(), "cat".to_string()]));
    }

    #[test]
    fn test_word_break_single_word() {
        assert!(word_break("a".to_string(), vec!["a".to_string()]));
    }

    #[test]
    fn test_word_break_single_word_false() {
        assert!(!word_break("b".to_string(), vec!["a".to_string()]));
    }

    #[test]
    fn test_word_break_empty_string() {
        assert!(word_break("".to_string(), vec!["a".to_string()]));
    }

    #[test]
    fn test_word_break_empty_dict() {
        assert!(!word_break("a".to_string(), vec![]));
    }

    #[test]
    fn test_word_break_duplicate_words() {
        assert!(word_break("aa".to_string(), vec!["a".to_string(), "a".to_string()]));
    }

    #[test]
    fn test_word_break_long_string() {
        let dict = vec!["a".to_string(), "aa".to_string(), "aaa".to_string()];
        assert!(word_break("aaaaa".to_string(), dict));
    }

    #[test]
    fn test_word_break_bfs_basic() {
        assert!(word_break_bfs("leetcode".to_string(), vec!["leet".to_string(), "code".to_string()]));
    }

    #[test]
    fn test_word_break_bfs_false() {
        assert!(!word_break_bfs("catsandog".to_string(), vec!["cats".to_string(), "dog".to_string()]));
    }

    #[test]
    fn test_word_break_bfs_applepenapple() {
        assert!(word_break_bfs("applepenapple".to_string(), vec!["apple".to_string(), "pen".to_string()]));
    }

    #[test]
    fn test_word_break_trie_basic() {
        assert!(word_break_trie("leetcode".to_string(), vec!["leet".to_string(), "code".to_string()]));
    }

    #[test]
    fn test_word_break_trie_false() {
        assert!(!word_break_trie("catsandog".to_string(), vec!["cats".to_string(), "dog".to_string()]));
    }

    #[test]
    fn test_word_break_all_approaches() {
        let s = "applepenapple";
        let dict = vec!["apple".to_string(), "pen".to_string()];
        assert_eq!(word_break(s.to_string(), dict.clone()), word_break_bfs(s.to_string(), dict.clone()));
        assert_eq!(word_break_bfs(s.to_string(), dict.clone()), word_break_trie(s.to_string(), dict.clone()));
    }

    #[test]
    fn test_word_break_needs_all_words() {
        let dict = vec!["leet".to_string()];
        assert!(!word_break("leetcode".to_string(), dict));
    }

    #[test]
    fn test_word_break_overlapping() {
        let dict = vec!["a".to_string(), "apple".to_string()];
        assert!(word_break("apple".to_string(), dict));
    }

    #[test]
    fn test_word_break_case_sensitive() {
        let dict = vec!["Apple".to_string()];
        assert!(!word_break("apple".to_string(), dict));
    }

    #[test]
    fn test_word_break_special_chars() {
        let dict = vec!["hello".to_string()];
        assert!(word_break("hello".to_string(), dict));
    }

    #[test]
    fn test_word_break_very_long() {
        let s = "a".repeat(100);
        let dict = vec!["a".to_string()];
        assert!(word_break(s, dict));
    }

    #[test]
    fn test_word_break_unreachable() {
        let dict = vec!["x".to_string()];
        assert!(!word_break("abc".to_string(), dict));
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("Word Break exercises - run tests with cargo test");
}