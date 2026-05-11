use std::collections::{HashSet, VecDeque};

/// Approach 1: BFS with naive neighbor generation
///
/// For each word, try changing each position to every letter a-z.
/// If result is in wordList and not visited, it's a neighbor.
/// BFS ensures shortest path.
pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
    let mut word_set: HashSet<String> = word_list.into_iter().collect();

    if !word_set.contains(&end_word) {
        return 0;
    }

    let mut queue = VecDeque::new();

    let mut visited = HashSet::new();
    visited.insert(begin_word.clone());
    queue.push_back((begin_word, 1)); // (word, level)

    while let Some((word, level)) = queue.pop_front() {
        if word == end_word {
            return level;
        }

        let chars: Vec<char> = word.chars().collect();
        let word_len = chars.len();

        for i in 0..word_len {
            let original_char = chars[i];

            for c in b'a'..=b'z' {
                let c = c as char;
                if c == original_char {
                    continue;
                }

                let mut new_chars = chars.clone();
                new_chars[i] = c;
                let new_word: String = new_chars.into_iter().collect();

                if word_set.contains(&new_word) && !visited.contains(&new_word) {
                    visited.insert(new_word.clone());
                    queue.push_back((new_word, level + 1));
                }
            }
        }
    }

    0
}

/// Approach 2: BFS with pattern-based neighbors
///
/// Instead of generating all 25 variants for each position,
/// generate a pattern like "_og" and find all matching words.
/// More efficient when word list is large.
pub fn ladder_length_pattern(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
    let mut word_set: HashSet<String> = word_list.into_iter().collect();

    if !word_set.contains(&end_word) {
        return 0;
    }

    let mut queue = VecDeque::new();
    queue.push_back((begin_word.clone(), 1));

    let mut visited = HashSet::new();
    visited.insert(begin_word);

    // Build pattern -> [words] map
    let mut pattern_map: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();

    for word in &word_set {
        let chars: Vec<char> = word.chars().collect();
        for i in 0..chars.len() {
            let mut pattern_chars = chars.clone();
            pattern_chars[i] = '_';
            let pattern: String = pattern_chars.into_iter().collect();
            pattern_map
                .entry(pattern)
                .or_insert_with(Vec::new)
                .push(word.clone());
        }
    }

    while let Some((word, level)) = queue.pop_front() {
        if word == end_word {
            return level;
        }

        let chars: Vec<char> = word.chars().collect();
        for i in 0..chars.len() {
            let mut pattern_chars = chars.clone();
            pattern_chars[i] = '_';
            let pattern: String = pattern_chars.into_iter().collect();

            if let Some(neighbors) = pattern_map.get(&pattern) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        visited.insert(neighbor.clone());
                        queue.push_back((neighbor.clone(), level + 1));
                    }
                }
            }
        }
    }

    0
}

/// Approach 3: Bidirectional BFS
///
/// Start BFS from both beginWord and endWord.
/// Meet in the middle for faster convergence.
pub fn ladder_length_bidirectional(
    begin_word: String,
    end_word: String,
    word_list: Vec<String>,
) -> i32 {
    let mut word_set: HashSet<String> = word_list.into_iter().collect();

    if !word_set.contains(&end_word) {
        return 0;
    }

    let mut begin_set = HashSet::new();
    let mut end_set = HashSet::new();
    let mut visited = HashSet::new();
    visited.insert(begin_word.clone());
    visited.insert(end_word.clone());
    begin_set.insert(begin_word);
    end_set.insert(end_word);

    let mut level = 2;

    while !begin_set.is_empty() && !end_set.is_empty() {
        // Always expand smaller set
        if begin_set.len() > end_set.len() {
            std::mem::swap(&mut begin_set, &mut end_set);
        }

        let mut next_set = HashSet::new();

        for word in begin_set {
            let chars: Vec<char> = word.chars().collect();

            for i in 0..chars.len() {
                let original = chars[i];

                for c in b'a'..=b'z' {
                    let c = c as char;
                    if c == original {
                        continue;
                    }

                    let mut new_chars = chars.clone();
                    new_chars[i] = c;
                    let new_word: String = new_chars.into_iter().collect();

                    if end_set.contains(&new_word) {
                        return level;
                    }

                    if word_set.contains(&new_word) && !visited.contains(&new_word) {
                        visited.insert(new_word.clone());
                        next_set.insert(new_word);
                    }
                }
            }
        }

        begin_set = next_set;
        level += 1;

        if level as usize > word_set.len() + 1 {
            break; // No path
        }
    }

    0
}

/// Check if two words differ by exactly one letter
fn is_one_letter_diff(word1: &str, word2: &str) -> bool {
    let chars1: Vec<char> = word1.chars().collect();
    let chars2: Vec<char> = word2.chars().collect();

    if chars1.len() != chars2.len() {
        return false;
    }

    let mut diff_count = 0;
    for i in 0..chars1.len() {
        if chars1[i] != chars2[i] {
            diff_count += 1;
            if diff_count > 1 {
                return false;
            }
        }
    }

    diff_count == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_path() {
        let result = ladder_length(
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
        assert_eq!(result, 5);
    }

    #[test]
    fn test_no_path() {
        let result = ladder_length(
            "hit".to_string(),
            "cog".to_string(),
            vec![
                "hot".to_string(),
                "dot".to_string(),
                "dog".to_string(),
                "lot".to_string(),
                "log".to_string(),
            ],
        );
        assert_eq!(result, 0);
    }

    #[test]
    fn test_direct_transform() {
        // "abc" -> "abd" differs by 1 letter
        let result = ladder_length(
            "abc".to_string(),
            "abd".to_string(),
            vec!["abc".to_string(), "abd".to_string()],
        );
        assert_eq!(result, 2);
    }

    #[test]
    fn test_begin_in_list() {
        let result = ladder_length(
            "a".to_string(),
            "c".to_string(),
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
        );
        assert_eq!(result, 2);
    }

    #[test]
    fn test_begin_equals_end() {
        let result = ladder_length(
            "abc".to_string(),
            "abc".to_string(),
            vec!["abc".to_string()],
        );
        assert_eq!(result, 1);
    }

    #[test]
    fn test_empty_word_list() {
        let result = ladder_length(
            "hit".to_string(),
            "cog".to_string(),
            vec![],
        );
        assert_eq!(result, 0);
    }

    #[test]
    fn test_no_intermediate_words() {
        // "hit" -> "cog" with no intermediate in list
        let result = ladder_length(
            "hit".to_string(),
            "cog".to_string(),
            vec!["hit".to_string(), "cog".to_string()],
        );
        assert_eq!(result, 0);
    }

    #[test]
    fn test_all_same_word() {
        let result = ladder_length(
            "a".to_string(),
            "a".to_string(),
            vec!["a".to_string()],
        );
        assert_eq!(result, 1);
    }

    #[test]
    fn test_long_words() {
        let result = ladder_length(
            "abcdefgh".to_string(),
            "abcdefgi".to_string(),
            vec![
                "abcdefgh".to_string(),
                "abcdefgi".to_string(),
                "abcdefgj".to_string(),
            ],
        );
        assert_eq!(result, 2);
    }

    #[test]
    fn test_single_letter() {
        let result = ladder_length(
            "a".to_string(),
            "b".to_string(),
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
        );
        assert_eq!(result, 2);
    }

    #[test]
    fn test_pattern_same_as_naive() {
        let words = vec![
            "hot".to_string(),
            "dot".to_string(),
            "dog".to_string(),
            "lot".to_string(),
            "log".to_string(),
            "cog".to_string(),
        ];
        let naive = ladder_length(
            "hit".to_string(),
            "cog".to_string(),
            words.clone(),
        );
        let pattern = ladder_length_pattern(
            "hit".to_string(),
            "cog".to_string(),
            words,
        );
        assert_eq!(naive, pattern);
    }

    #[test]
    fn test_bidirectional_same_as_naive() {
        let words = vec![
            "hot".to_string(),
            "dot".to_string(),
            "dog".to_string(),
            "lot".to_string(),
            "log".to_string(),
            "cog".to_string(),
        ];
        let naive = ladder_length(
            "hit".to_string(),
            "cog".to_string(),
            words.clone(),
        );
        let bi = ladder_length_bidirectional(
            "hit".to_string(),
            "cog".to_string(),
            words,
        );
        assert_eq!(naive, bi);
    }

    #[test]
    fn test_is_one_letter_diff() {
        assert!(is_one_letter_diff("abc", "abd"));
        assert!(is_one_letter_diff("abc", "bbc"));
        assert!(is_one_letter_diff("abc", "dbc"));
        assert!(!is_one_letter_diff("abc", "abc"));
        assert!(!is_one_letter_diff("abc", "abd"));
        assert!(!is_one_letter_diff("abc", "xyz"));
    }

    #[test]
    fn test_ladder_length_with_duplicates() {
        let result = ladder_length(
            "hit".to_string(),
            "cog".to_string(),
            vec![
                "hot".to_string(),
                "hot".to_string(), // duplicate
                "dot".to_string(),
                "dog".to_string(),
                "lot".to_string(),
                "log".to_string(),
                "cog".to_string(),
            ],
        );
        assert_eq!(result, 5);
    }

    #[test]
    fn test_many_words() {
        // Test with larger word list
        let mut word_list = vec![
            "hot".to_string(),
            "dot".to_string(),
            "dog".to_string(),
            "lot".to_string(),
            "log".to_string(),
            "cog".to_string(),
        ];
        for i in 0..100 {
            word_list.push(format!("word{}", i));
        }

        let result = ladder_length(
            "hit".to_string(),
            "cog".to_string(),
            word_list,
        );
        assert_eq!(result, 5);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("038_word_ladder_lc127 exercises - run tests with cargo test");
}
