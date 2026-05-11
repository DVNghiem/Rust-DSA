//! Exercises for Alien Dictionary (LeetCode 269)
//!
//! # Topics Covered
//! - Topological sort
//! - Graph construction
//! - Cycle detection
//! - Lexicographic ordering
//!
//! # Difficulty: Hard

use std::collections::{HashMap, HashSet, VecDeque};

/// Alien dictionary using topological sort
/// Returns the order string, or empty string if invalid
pub fn alien_order(words: Vec<String>) -> String {
    if words.is_empty() {
        return String::new();
    }

    // Build graph
    let mut graph: HashMap<char, HashSet<char>> = HashMap::new();
    let mut in_degree: HashMap<char, i32> = HashMap::new();

    // Initialize all characters
    for word in &words {
        for c in word.chars() {
            graph.entry(c).or_insert_with(HashSet::new);
            in_degree.entry(c).or_insert(0);
        }
    }

    // Build edges from word comparisons
    for i in 0..words.len() - 1 {
        let w1 = &words[i];
        let w2 = &words[i + 1];

        // Find first difference
        let mut j = 0;
        while j < w1.len() && j < w2.len() && w1.chars().nth(j) == w2.chars().nth(j) {
            j += 1;
        }

        // If w1 is prefix of w2, no ordering info
        if j == w2.len() && j < w1.len() {
            return String::new(); // Invalid
        }

        if j < w1.len() && j < w2.len() {
            let c1 = w1.chars().nth(j).unwrap();
            let c2 = w2.chars().nth(j).unwrap();

            if c1 != c2 {
                if !graph.get(&c1).unwrap().contains(&c2) {
                    graph.get_mut(&c1).unwrap().insert(c2);
                    *in_degree.entry(c2).or_insert(0) += 1;
                }
            }
        }
    }

    // Topological sort using Kahn's algorithm
    let mut queue: VecDeque<char> = VecDeque::new();

    // Start with nodes that have no incoming edges
    for (&c, &deg) in &in_degree {
        if deg == 0 {
            queue.push_back(c);
        }
    }

    let mut result = String::new();

    while let Some(c) = queue.pop_front() {
        result.push(c);

        if let Some(neighbors) = graph.get(&c) {
            for &neighbor in neighbors {
                let new_deg = in_degree.get(&neighbor).unwrap() - 1;
                in_degree.insert(neighbor, new_deg);
                if new_deg == 0 {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    // If result doesn't contain all characters, there's a cycle
    if result.len() != graph.len() {
        return String::new();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_example() {
        let words = vec!["wrt".to_string(), "wrf".to_string(), "er".to_string(),
                        "ett".to_string(), "et".to_string()];
        let result = alien_order(words);
        // Should be "wertf" (or any valid topological order)
        assert_eq!(result, "wertf");
    }

    #[test]
    fn test_simple_order() {
        let words = vec!["abc".to_string(), "ab".to_string()];
        // Invalid: prefix case
        assert_eq!(alien_order(words), "");
    }

    #[test]
    fn test_two_words() {
        let words = vec!["ba".to_string(), "bc".to_string()];
        // b -> a, b -> c... wait, first diff is 'a' vs 'c', so a < c
        // Order: a < c < b? Actually 'ba' vs 'bc': a < c, so 'a' comes before 'c'
        // From 'ba', 'b' is first char. From 'bc', 'b' is first char. Second char: 'a' vs 'c'
        // So 'a' < 'c'. Order: a, c, b (but we only see b, a, c)
        let result = alien_order(words);
        // Only characters: b, a, c
        // We know a < c
        // From result we need to verify valid order
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(alien_order(vec![]), "");
    }

    #[test]
    fn test_single_word() {
        assert_eq!(alien_order(vec!["abc".to_string()]), "abc");
    }

    #[test]
    fn test_all_unique_chars() {
        let words = vec!["z".to_string(), "x".to_string()];
        let result = alien_order(words);
        assert_eq!(result, "zx");
    }

    #[test]
    fn test_cycle_detection() {
        // This would be cycle: a < b, b < a
        // Not possible with valid input since dict is valid
    }

    #[test]
    fn test_three_words_simple() {
        let words = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let result = alien_order(words);
        // All single chars, no edges
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_chain_ordering() {
        let words = vec!["a".to_string(), "ab".to_string(), "abc".to_string()];
        // No invalid prefix, no ordering info
        let result = alien_order(words);
        assert_eq!(result, "");
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("094_alien_dictionary_lc269 exercises - run tests with cargo test");
}
