//! Exercises for Regular Expression Matching (LeetCode 10)
//!
//! # Topics Covered
//! - Dynamic programming
//! - String matching
//! - Pattern matching with wildcards
//! - State transitions
//! - 2D DP table
//!
//! # Difficulty: Hard

/// 2D Dynamic Programming approach
pub fn is_match(text: String, pattern: String) -> bool {
    let m = text.len();
    let n = pattern.len();
    let text = text.as_bytes();
    let pattern = pattern.as_bytes();

    let mut dp = vec![vec![false; n + 1]; m + 1];
    dp[0][0] = true;

    for j in 2..=n {
        if pattern[j - 1] == b'*' {
            dp[0][j] = dp[0][j - 2];
        }
    }

    for i in 1..=m {
        for j in 1..=n {
            let curr_p = pattern[j - 1];

            if curr_p == b'*' && j >= 2 {
                dp[i][j] = dp[i][j - 2];

                if pattern[j - 2] == b'.' || pattern[j - 2] == text[i - 1] {
                    dp[i][j] = dp[i][j] || dp[i - 1][j];
                }
            } else if curr_p == b'.' || curr_p == text[i - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            }
        }
    }

    dp[m][n]
}

/// Memoized recursive approach
pub fn is_match_recursive(text: String, pattern: String) -> bool {
    let text = text.into_bytes();
    let pattern = pattern.into_bytes();
    let mut memo: Vec<Vec<Option<bool>>> = vec![vec![None; pattern.len() + 1]; text.len() + 1];
    dp_recursive(0, 0, &text, &pattern, &mut memo)
}

fn dp_recursive(
    i: usize,
    j: usize,
    text: &[u8],
    pattern: &[u8],
    memo: &mut Vec<Vec<Option<bool>>>,
) -> bool {
    if j >= pattern.len() {
        return i == text.len();
    }

    if i < text.len() && j < pattern.len() {
        if let Some(result) = memo[i][j] {
            return result;
        }
    }

    let mut result: bool;

    if j + 1 < pattern.len() && pattern[j + 1] == b'*' {
        result = dp_recursive(i, j + 2, text, pattern, memo);

        if i < text.len() && (pattern[j] == b'.' || pattern[j] == text[i]) {
            result = result || dp_recursive(i + 1, j, text, pattern, memo);
        }
    } else if i < text.len() && (pattern[j] == b'.' || pattern[j] == text[i]) {
        result = dp_recursive(i + 1, j + 1, text, pattern, memo);
    } else {
        result = false;
    }

    if i < text.len() && j < pattern.len() {
        memo[i][j] = Some(result);
    }
    result
}

/// Check if pattern has valid syntax
pub fn is_valid_pattern(pattern: &str) -> bool {
    let bytes = pattern.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i];
        if c == b'*' && i == 0 {
            return false;
        }
        if c == b'*' && i > 0 && bytes[i - 1] == b'*' {
            return false;
        }
        i += 1;
    }
    true
}

/// Count number of matches
pub fn num_matches(text: &str, pattern: &str) -> usize {
    if is_match(text.to_string(), pattern.to_string()) {
        1
    } else {
        0
    }
}

/// Find if pattern matches text
pub fn find_match(text: &str, pattern: &str) -> bool {
    is_match(text.to_string(), pattern.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_exact_match() {
        assert!(is_match("aa".to_string(), "aa".to_string()));
        assert!(!is_match("aa".to_string(), "a".to_string()));
    }

    #[test]
    fn test_dot_wildcard() {
        assert!(is_match("ab".to_string(), "..".to_string()));
        assert!(is_match("aab".to_string(), "...".to_string()));
    }

    #[test]
    fn test_star_zero_occurrences() {
        assert!(is_match("".to_string(), "a*".to_string()));
        assert!(is_match("".to_string(), ".*".to_string()));
    }

    #[test]
    fn test_star_one_occurrences() {
        assert!(is_match("a".to_string(), "a*".to_string()));
        assert!(is_match("aa".to_string(), "a*".to_string()));
        assert!(is_match("aaa".to_string(), "a*".to_string()));
    }

    #[test]
    fn test_dot_star() {
        assert!(is_match("aa".to_string(), ".*".to_string()));
        assert!(is_match("aab".to_string(), "a.*".to_string()));
    }

    #[test]
    fn test_complex_pattern() {
        assert!(is_match("mississippi".to_string(), "mis*is*ip*.".to_string()));
    }

    #[test]
    fn test_recursive_basic() {
        assert!(is_match_recursive("aa".to_string(), "aa".to_string()));
        assert!(!is_match_recursive("aa".to_string(), "a".to_string()));
    }

    #[test]
    fn test_recursive_dot_star() {
        assert!(is_match_recursive("aab".to_string(), "a.*".to_string()));
    }

    #[test]
    fn test_empty_text() {
        assert!(is_match("".to_string(), "".to_string()));
        assert!(is_match("".to_string(), "a*".to_string()));
    }

    #[test]
    fn test_empty_pattern() {
        assert!(is_match("".to_string(), "".to_string()));
        assert!(!is_match("a".to_string(), "".to_string()));
    }

    #[test]
    fn test_valid_pattern() {
        assert!(is_valid_pattern("a*"));
        assert!(is_valid_pattern(".*"));
        assert!(is_valid_pattern(""));
    }

    #[test]
    fn test_invalid_pattern() {
        assert!(!is_valid_pattern("*a"));
    }

    #[test]
    fn test_two_star_combination() {
        assert!(is_match("aaa".to_string(), "a*a".to_string()));
        assert!(is_match("aaaa".to_string(), "a*aa".to_string()));
    }

    #[test]
    fn test_three_star_combination() {
        assert!(is_match("ab".to_string(), "a*b*".to_string()));
        assert!(!is_match("ab".to_string(), ".*c".to_string()));
    }

    #[test]
    fn test_pattern_only_star() {
        assert!(!is_match("aaa".to_string(), "*".to_string()));
        assert!(!is_match("aa".to_string(), "*a".to_string()));
    }

    #[test]
    fn test_single_char_pattern() {
        assert!(is_match("a".to_string(), "a".to_string()));
        assert!(is_match("b".to_string(), ".".to_string()));
    }

    #[test]
    fn test_long_pattern() {
        assert!(is_match("aaa".to_string(), "a*a*a*".to_string()));
    }

    #[test]
    fn test_num_matches_basic() {
        assert_eq!(num_matches("aa", "a*"), 1);
        assert_eq!(num_matches("ab", "a*"), 0);
    }

    #[test]
    fn test_find_match_basic() {
        assert!(find_match("aaa", "a*"));
        assert!(!find_match("bbb", "a*"));
    }
}
// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("$name exercises - run tests with cargo test");
}
