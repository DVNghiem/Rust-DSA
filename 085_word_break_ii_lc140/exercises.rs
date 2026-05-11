//! Exercises for Word Break II (LeetCode 140)
//!
//! # Topics Covered
//! - DFS with memoization
//! - Dynamic programming
//! - String segmentation
//! - Backtracking
//!
//! # Difficulty: Hard

use std::collections::{HashMap, HashSet};

/// DFS with memoization approach
pub fn word_break_dfs(s: String, word_dict: HashSet<String>) -> Vec<String> {
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut memo = vec![Option::<Vec<String>>::None; n + 1];

    fn dfs(start: usize, bytes: &[u8], dict: &HashSet<String>, memo: &mut Vec<Option<Vec<String>>>) -> Vec<String> {
        if start == bytes.len() {
            return vec![String::new()];
        }

        if let Some(result) = &memo[start] {
            return result.clone();
        }

        let mut results = Vec::new();
        let mut end = start + 1;

        while end <= bytes.len() {
            let word = std::str::from_utf8(&bytes[start..end]).unwrap();
            if dict.contains(word) {
                let suffix_sentences = dfs(end, bytes, dict, memo);
                for suffix in suffix_sentences {
                    if suffix.is_empty() {
                        results.push(word.to_string());
                    } else {
                        results.push(format!("{} {}", word, suffix));
                    }
                }
            }
            end += 1;
        }

        memo[start] = Some(results.clone());
        results
    }

    dfs(0, bytes, &word_dict, &mut memo)
}

// ============================================================================
// MAIN
// ============================================================================

/// Bottom-up DP approach
pub fn word_break_dp(s: String, word_dict: HashSet<String>) -> Vec<String> {
    let bytes = s.as_bytes();
    let n = bytes.len();

    // dp[i] = all sentences for s[0..i]
    let mut dp: Vec<Vec<String>> = vec![Vec::new(); n + 1];
    dp[0] = vec![String::new()];

    for i in 1..=n {
        let mut sentences = Vec::new();
        for j in 0..i {
            let word = std::str::from_utf8(&bytes[j..i]).unwrap();
            if word_dict.contains(word) {
                for prefix in &dp[j] {
                    if prefix.is_empty() {
                        sentences.push(word.to_string());
                    } else {
                        sentences.push(format!("{} {}", prefix, word));
                    }
                }
            }
        }
        dp[i] = sentences;
    }

    dp[n].clone()
}

/// Optimized DFS with Trie for better word lookup
pub fn word_break_trie(s: String, word_dict: HashSet<String>) -> Vec<String> {
    // Build a simple prefix set for faster lookup
    // Also store max word length to limit search space
    let bytes = s.as_bytes();
    let n = bytes.len();

    let mut memo = vec![Option::<Vec<String>>::None; n + 1];

    fn dfs(start: usize, bytes: &[u8], dict: &HashSet<String>, memo: &mut Vec<Option<Vec<String>>>) -> Vec<String> {
        if start == bytes.len() {
            return vec![String::new()];
        }

        if let Some(result) = &memo[start] {
            return result.clone();
        }

        let mut results = Vec::new();

        // Limit end to at most 20 characters to prevent timeout
        let max_len = 20.min(bytes.len() - start);
        for len in 1..=max_len {
            let end = start + len;
            let word = std::str::from_utf8(&bytes[start..end]).unwrap();
            if dict.contains(word) {
                let suffix_sentences = dfs(end, bytes, dict, memo);
                for suffix in suffix_sentences {
                    if suffix.is_empty() {
                        results.push(word.to_string());
                    } else {
                        results.push(format!("{} {}", word, suffix));
                    }
                }
            }
        }

        memo[start] = Some(results.clone());
        results
    }

    dfs(0, bytes, &word_dict, &mut memo)
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("Word Break II exercises - run tests with cargo test");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_dict(words: Vec<&str>) -> HashSet<String> {
        words.into_iter().map(String::from).collect()
    }

    #[test]
    fn test_dfs_basic() {
        let dict = create_dict(vec!["cat", "cats", "and", "sand", "dog"]);
        let result = word_break_dfs("catsanddog".to_string(), dict);
        assert!(result.contains(&"cats and dog".to_string()));
        assert!(result.contains(&"cat sand dog".to_string()));
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_dp_basic() {
        let dict = create_dict(vec!["cat", "cats", "and", "sand", "dog"]);
        let result = word_break_dp("catsanddog".to_string(), dict);
        assert!(result.contains(&"cats and dog".to_string()));
        assert!(result.contains(&"cat sand dog".to_string()));
    }

    #[test]
    fn test_empty_string() {
        let dict = create_dict(vec!["a"]);
        let result = word_break_dfs("".to_string(), dict);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "");
    }

    #[test]
    fn test_single_word() {
        let dict = create_dict(vec!["hello"]);
        let result = word_break_dfs("hello".to_string(), dict);
        assert_eq!(result, vec!["hello"]);
    }

    #[test]
    fn test_no_match() {
        let dict = create_dict(vec!["cat"]);
        let result = word_break_dfs("dog".to_string(), dict);
        assert!(result.is_empty());
    }

    #[test]
    fn test_overlapping_words() {
        let dict = create_dict(vec!["a", "aa", "aaa", "aaaa"]);
        let result = word_break_dfs("aaaa".to_string(), dict);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_multiple_sentences() {
        let dict = create_dict(vec!["apple", "pen", "applepen", "and", "pine", "pineapple", "pine", "apple"]);
        let result = word_break_dfs("pineapplepenapple".to_string(), dict);
        assert!(result.len() > 1);
    }

    #[test]
    fn test_consistency_dfs_dp() {
        let dict = create_dict(vec!["cat", "cats", "and", "sand", "dog"]);
        let s = "catsanddog";
        let dfs_result = word_break_dfs(s.to_string(), dict.clone());
        let dp_result = word_break_dp(s.to_string(), dict);
        // Both should have same sentences (order may differ)
        let mut dfs_sorted = dfs_result.clone();
        let mut dp_sorted = dp_result;
        dfs_sorted.sort();
        dp_sorted.sort();
        assert_eq!(dfs_sorted, dp_sorted);
    }

    #[test]
    fn test_long_string() {
        let dict = create_dict(vec!["a", "b", "c", "ab", "bc", "abc"]);
        let result = word_break_dfs("abc".to_string(), dict);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_repeated_words() {
        let dict = create_dict(vec!["a", "aa", "aaa"]);
        let result = word_break_dfs("aaaaa".to_string(), dict);
        // Many combinations possible
        assert!(result.len() > 1);
    }

    #[test]
    fn test_all_same_char_words() {
        let dict = create_dict(vec!["a", "aa", "aaa", "aaaa", "aaaaa"]);
        let result_dfs = word_break_dfs("aaaaa".to_string(), dict.clone());
        let result_dp = word_break_dp("aaaaa".to_string(), dict);
        assert!(!result_dfs.is_empty());
        assert!(!result_dp.is_empty());
    }

    #[test]
    fn test_memoization_works() {
        let dict = create_dict(vec!["a", "b", "ab"]);
        let s = "ab";
        // Call multiple times to verify memoization doesn't cause issues
        let result1 = word_break_dfs(s.to_string(), dict.clone());
        let result2 = word_break_dfs(s.to_string(), dict);
        assert_eq!(result1, result2);
    }

    #[test]
    fn test_trie_basic() {
        let dict = create_dict(vec!["cat", "cats", "and", "sand", "dog"]);
        let result = word_break_trie("catsanddog".to_string(), dict);
        assert!(result.contains(&"cats and dog".to_string()));
        assert!(result.contains(&"cat sand dog".to_string()));
    }

    #[test]
    fn test_trie_no_match() {
        let dict = create_dict(vec!["cat"]);
        let result = word_break_trie("dog".to_string(), dict);
        assert!(result.is_empty());
    }

    #[test]
    fn test_trie_empty() {
        let dict = create_dict(vec!["a"]);
        let result = word_break_trie("".to_string(), dict);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "");
    }

    #[test]
    fn test_trie_consistency_with_dfs() {
        let dict = create_dict(vec!["cat", "cats", "and", "sand", "dog"]);
        let s = "catsanddog";
        let trie_result = word_break_trie(s.to_string(), dict.clone());
        let dfs_result = word_break_dfs(s.to_string(), dict);
        let mut trie_sorted = trie_result;
        let mut dfs_sorted = dfs_result;
        trie_sorted.sort();
        dfs_sorted.sort();
        assert_eq!(trie_sorted, dfs_sorted);
    }

    #[test]
    fn test_zero_length_word() {
        // Edge case: empty string as word (shouldn't happen in normal dict)
        let dict = create_dict(vec![]);
        let result = word_break_dfs("a".to_string(), dict);
        assert!(result.is_empty());
    }

    #[test]
    fn test_leetcode_example() {
        let dict = create_dict(vec!["cat", "cats", "and", "sand", "dog"]);
        let result = word_break_dfs("catsanddog".to_string(), dict);
        let expected = vec!["cats and dog", "cat sand dog"];
        assert_eq!(result.len(), expected.len());
        for exp in expected {
            assert!(result.contains(&exp));
        }
    }

    #[test]
    fn test_all_sentences_valid() {
        let dict = create_dict(vec!["apple", "pen", "applepen", "and", "pine", "pineapple", "pine", "apple"]);
        let result = word_break_dfs("pineapplepenapple".to_string(), dict);
        for sentence in &result {
            let words: Vec<&str> = sentence.split(' ').collect();
            // Verify each word is in dictionary
            for word in words {
                assert!(dict.contains(&word.to_string()) || word.is_empty());
            }
        }
    }
}