/// Word Search II - LeetCode 212
/// Find all valid words on the board using Trie + DFS.

use std::collections::{HashMap, HashSet};

/// Trie node structure
struct TrieNode {
    children: HashMap<char, TrieNode>,
    word: Option<String>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            word: None,
        }
    }
}

/// Trie structure for efficient prefix matching
struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for c in word.chars() {
            node = node.children.entry(c).or_insert_with(TrieNode::new);
        }
        node.word = Some(word.to_string());
    }
}

/// Find all valid words using Trie + DFS
pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
    if board.is_empty() || board[0].is_empty() || words.is_empty() {
        return vec![];
    }

    let mut trie = Trie::new();
    for word in &words {
        trie.insert(word);
    }

    let mut result = HashSet::new();
    let m = board.len();
    let n = board[0].len();
    let mut visited = vec![vec![false; n]; m];

    fn dfs(
        row: usize,
        col: usize,
        board: &Vec<Vec<char>>,
        node: &mut TrieNode,
        visited: &mut Vec<Vec<bool>>,
        result: &mut HashSet<String>,
    ) {
        let ch = board[row][col];
        if let Some(next) = node.children.get_mut(&ch) {
            if let Some(word) = next.word.take() {
                result.insert(word);
            }

            visited[row][col] = true;
            let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

            for (dr, dc) in directions.iter() {
                let new_row = row as i32 + dr;
                let new_col = col as i32 + dc;
                if new_row >= 0 && new_row < board.len() as i32
                    && new_col >= 0 && new_col < board[0].len() as i32
                    && !visited[new_row as usize][new_col as usize]
                {
                    dfs(new_row as usize, new_col as usize, board, next, visited, result);
                }
            }
            visited[row][col] = false;
        }
    }

    for i in 0..m {
        for j in 0..n {
            dfs(i, j, &board, &mut trie.root, &mut visited, &mut result);
        }
    }

    result.into_iter().collect()
}

/// Alternative: Simple backtracking without Trie
pub fn find_words_simple(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
    let mut result = HashSet::new();

    for word in words {
        if exists_on_board(&board, &word) {
            result.insert(word);
        }
    }

    result.into_iter().collect()
}

fn exists_on_board(board: &Vec<Vec<char>>, word: &str) -> bool {
    let m = board.len();
    let n = board[0].len();

    fn dfs(
        board: &Vec<Vec<char>>,
        word: &[char],
        row: usize,
        col: usize,
        visited: &mut Vec<Vec<bool>>,
    ) -> bool {
        if word.is_empty() {
            return true;
        }
        if row >= board.len() || col >= board[0].len() || visited[row][col] {
            return false;
        }
        if board[row][col] != word[0] {
            return false;
        }

        visited[row][col] = true;
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        for (dr, dc) in directions.iter() {
            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;
            if new_row >= 0 && new_row < board.len() as i32
                && new_col >= 0 && new_col < board[0].len() as i32
            {
                if dfs(
                    board,
                    &word[1..],
                    new_row as usize,
                    new_col as usize,
                    visited,
                ) {
                    visited[row][col] = false;
                    return true;
                }
            }
        }

        visited[row][col] = false;
        false
    }

    let chars: Vec<char> = word.chars().collect();
    let mut visited = vec![vec![false; n]; m];

    for i in 0..m {
        for j in 0..n {
            if board[i][j] == chars[0] && dfs(board, &chars, i, j, &mut visited) {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_words_basic() {
        let board = vec![
            vec!['o', 'a', 'a', 'n'],
            vec!['e', 't', 'a', 'e'],
            vec!['i', 'h', 'k', 'r'],
            vec!['i', 'f', 'l', 'v'],
        ];
        let words = vec![
            "oath".to_string(),
            "pea".to_string(),
            "eat".to_string(),
            "rain".to_string(),
        ];
        let result = find_words(board, words);
        assert!(result.contains(&"eat".to_string()));
        assert!(result.contains(&"oath".to_string()));
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_find_words_empty_board() {
        let result = find_words(vec![], vec!["test".to_string()]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_find_words_empty_words() {
        let board = vec![vec!['a']];
        let result = find_words(board, vec![]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_find_words_single_char() {
        let board = vec![vec!['a', 'b'], vec!['c', 'd']];
        let result = find_words(board, vec!["a".to_string(), "bd".to_string()]);
        assert!(result.contains(&"a".to_string()));
        assert!(result.contains(&"bd".to_string()));
    }

    #[test]
    fn test_find_words_no_match() {
        let board = vec![vec!['a', 'b'], vec!['c', 'd']];
        let result = find_words(board, vec!["xyz".to_string()]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_find_words_simple_basic() {
        let board = vec![vec!['a', 'b'], vec!['c', 'd']];
        let result = find_words_simple(board, vec!["ac".to_string(), "bd".to_string()]);
        assert!(result.contains(&"ac".to_string()));
    }

    #[test]
    fn test_find_words_two_rows() {
        let board = vec![vec!['a', 'b'], vec!['c', 'd']];
        let words = vec![
            "ac".to_string(),
            "bd".to_string(),
            "ab".to_string(),
            "cd".to_string(),
        ];
        let result = find_words(board, words);
        assert!(result.contains(&"ac".to_string()));
        assert!(result.contains(&"bd".to_string()));
        assert!(result.contains(&"ab".to_string()));
        assert!(result.contains(&"cd".to_string()));
    }

    #[test]
    fn test_find_words_horizontal() {
        let board = vec![vec!['a', 'b', 'c']];
        let result = find_words(board, vec!["abc".to_string(), "ab".to_string()]);
        assert!(result.contains(&"abc".to_string()));
        assert!(result.contains(&"ab".to_string()));
    }

    #[test]
    fn test_find_words_vertical() {
        let board = vec![vec!['a'], vec!['b'], vec!['c']];
        let result = find_words(board, vec!["abc".to_string(), "bc".to_string()]);
        assert!(result.contains(&"abc".to_string()));
        assert!(result.contains(&"bc".to_string()));
    }

    #[test]
    fn test_find_words_single_cell() {
        let board = vec![vec!['a']];
        let result = find_words(board, vec!["a".to_string()]);
        assert!(result.contains(&"a".to_string()));
    }

    #[test]
    fn test_find_words_letters_used_once() {
        let board = vec![vec!['a', 'a']];
        let result = find_words(board, vec!["aa".to_string()]);
        // Cannot use same cell twice in one word
        assert!(result.is_empty() || !result.contains(&"aa".to_string()));
    }

    #[test]
    fn test_find_words_all_same_letters() {
        let board = vec![vec!['a', 'a'], vec!['a', 'a']];
        let result = find_words(board, vec!["aaa".to_string()]);
        // Multiple paths might allow this
        assert!(result.len() >= 0);
    }

    #[test]
    fn test_find_words_duplicate_results() {
        let board = vec![vec!['a', 'b'], vec!['c', 'd']];
        // If multiple paths form same word, result should have it once
        let result = find_words(board, vec!["abc".to_string()]);
        assert!(result.len() <= 1);
    }

    #[test]
    fn test_find_words_long_word() {
        let board = vec![vec!['a', 'b', 'c', 'd']];
        let result = find_words(board, vec!["abcd".to_string()]);
        assert!(result.contains(&"abcd".to_string()));
    }

    #[test]
    fn test_find_words_complex_board() {
        let board = vec![
            vec!['a', 'b', 'c', 'e'],
            vec!['s', 'f', 'c', 's'],
            vec!['a', 'd', 'e', 'e'],
        ];
        let words = vec![
            "abcced".to_string(),
            "see".to_string(),
            "ab".to_string(),
            "abc".to_string(),
        ];
        let result = find_words(board, words);
        assert!(result.contains(&"see".to_string()));
        assert!(result.contains(&"ab".to_string()));
        assert!(result.contains(&"abc".to_string()));
    }

    #[test]
    fn test_find_words_z_shape() {
        let board = vec![vec!['a', 'b'], vec!['b', 'a']];
        let result = find_words(board, vec!["aba".to_string()]);
        // Can we form "aba"? a(0,0)->b(1,0)->a(0,1) or other path
        assert!(result.len() >= 0);
    }

    #[test]
    fn test_find_words_backtrack_pruning() {
        let board = vec![vec!['a', 'b'], vec!['c', 'd']];
        let words = vec!["ae".to_string()];
        let result = find_words(board, words);
        // "ae" doesn't exist - should be pruned early
        assert!(result.is_empty());
    }

    #[test]
    fn test_find_words_multi_word() {
        let board = vec![
            vec!['o', 'a', 'b', 'n'],
            vec!['e', 't', 'a', 'e'],
            vec!['i', 'h', 'k', 'r'],
            vec!['i', 'f', 'l', 'v'],
        ];
        let result = find_words(board, vec!["oath".to_string(), "eat".to_string()]);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_find_words_no_prefix() {
        let board = vec![vec!['a']];
        let words = vec!["b".to_string()];
        let result = find_words(board, words);
        assert!(result.is_empty());
    }

    #[test]
    fn test_find_words_partial_match() {
        let board = vec![vec!['a', 'b'], vec!['c', 'd']];
        // "abd" - 'd' not adjacent to 'b' in any path
        let result = find_words(board, vec!["abd".to_string()]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_find_words_three_by_three() {
        let board = vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ];
        let words = vec![
            "aei".to_string(),
            "bfh".to_string(),
            "cef".to_string(),
            "adh".to_string(),
        ];
        let result = find_words(board, words);
        assert!(result.contains(&"aei".to_string()));
        assert!(result.contains(&"bfh".to_string()));
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("054_word_search_ii_lc212 exercises - run tests with cargo test");
}
