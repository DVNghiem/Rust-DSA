//! Exercises for Word Ladder II (LeetCode 126)
//!
//! # Topics Covered
//! - BFS graph traversal
//! - Word transformation
//! - Path reconstruction
//! - Backtracking
//! - Pattern matching
//!
//! # Difficulty: Hard

use std::collections::{HashSet, VecDeque};

/// Find all shortest transformation sequences from beginWord to endWord
pub fn find_ladders(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<Vec<String>> {
    let word_list: HashSet<String> = word_list.into_iter().collect();
    if !word_list.contains(&end_word) {
        return vec![];
    }

    let mut result: Vec<Vec<String>> = vec![];
    let mut queue: VecDeque<Vec<String>> = VecDeque::new();
    queue.push_back(vec![begin_word.clone()]);

    let mut visited: HashSet<String> = HashSet::new();
    visited.insert(begin_word.clone());

    let mut found_depth: Option<usize> = None;

    while let Some(path) = queue.pop_front() {
        let word = path.last().unwrap().clone();
        let current_depth = path.len();

        if let Some(depth) = found_depth {
            if current_depth > depth {
                break;
            }
        }

        if word == end_word {
            if found_depth.is_none() {
                found_depth = Some(current_depth);
            }
            result.push(path);
            continue;
        }

        if let Some(depth) = found_depth {
            if current_depth >= depth {
                continue;
            }
        }

        let bytes = word.as_bytes();
        for i in 0..bytes.len() {
            let mut pattern: Vec<u8> = bytes.to_vec();
            pattern[i] = b'_';

            for c in b'a'..=b'z' {
                if c == bytes[i] {
                    continue;
                }
                pattern[i] = c;
                let next_word = String::from_utf8(pattern.clone()).unwrap();

                if word_list.contains(&next_word) && !visited.contains(&next_word) {
                    let mut new_path = path.clone();
                    new_path.push(next_word.clone());
                    queue.push_back(new_path);
                }
            }
        }
    }

    if result.is_empty() && found_depth.is_none() {
        return vec![];
    }

    result
}

/// Calculate minimum transformation length
pub fn min_length(begin_word: String, end_word: String, word_list: Vec<String>) -> usize {
    let word_list: HashSet<String> = word_list.into_iter().collect();
    if !word_list.contains(&end_word) {
        return 0;
    }

    let mut queue: VecDeque<(String, usize)> = VecDeque::new();
    queue.push_back((begin_word.clone(), 1));

    let mut visited: HashSet<String> = HashSet::new();
    visited.insert(begin_word);

    while let Some((word, depth)) = queue.pop_front() {
        if word == end_word {
            return depth;
        }

        let bytes = word.as_bytes();
        for i in 0..bytes.len() {
            let mut pattern: Vec<u8> = bytes.to_vec();
            pattern[i] = b'_';

            for c in b'a'..=b'z' {
                if c == bytes[i] {
                    continue;
                }
                pattern[i] = c;
                let next_word = String::from_utf8(pattern.clone()).unwrap();

                if word_list.contains(&next_word) && !visited.contains(&next_word) {
                    visited.insert(next_word.clone());
                    queue.push_back((next_word, depth + 1));
                }
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_example() {
        let result = find_ladders(
            "hit".to_string(),
            "cog".to_string(),
            vec![
                "hot".to_string(),
                "dot".to_string(),
                "dog".to_string(),
                "lot".to_string(),
                "log".to_string(),
                "cog".to_string(),
            ],
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn test_no_transformation() {
        let result = find_ladders(
            "hit".to_string(),
            "cog".to_string(),
            vec!["hot".to_string(), "dot".to_string()],
        );
        assert!(result.is_empty());
    }

    #[test]
    fn test_single_step() {
        let result = find_ladders(
            "a".to_string(),
            "b".to_string(),
            vec!["a".to_string(), "b".to_string()],
        );
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_same_word() {
        let result = find_ladders(
            "a".to_string(),
            "a".to_string(),
            vec!["a".to_string()],
        );
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_two_possible_paths() {
        let result = find_ladders(
            "hit".to_string(),
            "cog".to_string(),
            vec![
                "hot".to_string(),
                "dot".to_string(),
                "dog".to_string(),
                "lot".to_string(),
                "log".to_string(),
                "cog".to_string(),
            ],
        );
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_min_length_basic() {
        let len = min_length(
            "hit".to_string(),
            "cog".to_string(),
            vec![
                "hot".to_string(),
                "dot".to_string(),
                "dog".to_string(),
                "lot".to_string(),
                "log".to_string(),
                "cog".to_string(),
            ],
        );
        assert_eq!(len, 5);
    }

    #[test]
    fn test_min_length_no_path() {
        let len = min_length(
            "hit".to_string(),
            "cog".to_string(),
            vec!["hot".to_string(), "dot".to_string()],
        );
        assert_eq!(len, 0);
    }

    #[test]
    fn test_chain_transformation() {
        let result = find_ladders(
            "a".to_string(),
            "d".to_string(),
            vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string(),
            ],
        );
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_two_step_transformation() {
        let result = find_ladders(
            "a".to_string(),
            "c".to_string(),
            vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
            ],
        );
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_no_end_in_list() {
        let result = find_ladders(
            "a".to_string(),
            "b".to_string(),
            vec!["x".to_string(), "y".to_string()],
        );
        assert!(result.is_empty());
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("098_word_ladder_ii_lc126 exercises - run tests with cargo test");
}
