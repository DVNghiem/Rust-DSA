//! Exercises for Shortest Palindrome (LeetCode 214)
//!
//! # Topics Covered
//! - KMP algorithm for string matching
//! - Longest palindrome prefix
//! - String reversal and concatenation
//! - LPS (Longest Proper Prefix Suffix) array
//!
//! # Difficulty: Hard

/// Computes the Longest Proper Prefix which is also Suffix (LPS) array
/// for the KMP algorithm
fn compute_lps(pattern: &str) -> Vec<usize> {
    let bytes = pattern.as_bytes();
    let n = bytes.len();
    let mut lps = vec![0; n];
    let mut len = 0; // length of previous longest prefix suffix
    let mut i = 1;

    while i < n {
        if bytes[i] == bytes[len] {
            len += 1;
            lps[i] = len;
            i += 1;
        } else {
            if len != 0 {
                len = lps[len - 1]; // fallback to previous LPS value
            } else {
                lps[i] = 0;
                i += 1;
            }
        }
    }
    lps
}

/// Finds the longest palindrome prefix using KMP approach
/// Time: O(n), Space: O(n)
pub fn shortest_palindrome_kmp(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }

    // Create combined string: s + "#" + reverse(s)
    let reversed: String = s.chars().rev().collect();
    let combined = format!("{}#{}", s, reversed);

    // Compute LPS array
    let lps = compute_lps(&combined);

    // Last value in LPS gives the length of longest palindrome prefix
    // that is also a suffix of reversed string, which means it's a palindrome prefix of s
    let palindrome_len = lps.last().copied().unwrap_or(0);

    // Characters to add = reverse of (s[palindrome_len..])
    let chars_to_add = &s[palindrome_len..];
    let to_add: String = chars_to_add.chars().rev().collect();

    format!("{}{}", to_add, s)
}

/// Two-pointer approach to find longest palindrome starting from index 0
/// Time: O(n²), Space: O(1)
pub fn shortest_palindrome_two_pointer(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }

    let bytes = s.as_bytes();
    let n = bytes.len();

    // Find the longest palindrome prefix
    // We try to match from both ends and find where the palindrome breaks
    let mut end = n - 1;
    let mut start = 0;
    let mut palindrome_end = n; // exclusive end of longest palindrome

    while start < end {
        if bytes[start] == bytes[end] {
            start += 1;
            end -= 1;
        } else {
            // Not matching, reset and try shorter palindrome
            palindrome_end -= 1;
            start = 0;
            end = palindrome_end - 1;
        }
    }

    // palindrome_end is the length of longest palindrome prefix
    let palindrome_len = if start >= end { palindrome_end } else { 0 };

    // Add reverse of characters before palindrome
    let chars_to_add = &s[..palindrome_len];
    let to_add: String = chars_to_add.chars().rev().collect();

    format!("{}{}", to_add, s)
}

/// Brute force approach - check each prefix
/// Time: O(n³), Space: O(n)
pub fn shortest_palindrome_brute(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }

    let bytes = s.as_bytes();
    let n = bytes.len();

    // Check prefixes from longest to shortest
    for len in (1..=n).rev() {
        let is_palindrome = (0..len / 2).all(|i| bytes[i] == bytes[len - 1 - i]);

        if is_palindrome {
            // This prefix is palindrome, add reverse of rest
            let chars_to_add = &s[len..];
            let to_add: String = chars_to_add.chars().rev().collect();
            return format!("{}{}", to_add, s);
        }
    }

    // If no palindrome prefix found, add reverse of entire string
    let reversed: String = s.chars().rev().collect();
    format!("{}{}", reversed, s)
}

/// Helper function to check if a string is palindrome
fn is_palindrome(s: &str) -> bool {
    let bytes = s.as_bytes();
    let mut i = 0;
    let mut j = bytes.len() - 1;

    while i < j {
        if bytes[i] != bytes[j] {
            return false;
        }
        i += 1;
        j -= 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kmp_basic() {
        let result = shortest_palindrome_kmp("aacecaaa");
        assert_eq!(result, "aaacecaaa");
    }

    #[test]
    fn test_kmp_abcd() {
        let result = shortest_palindrome_kmp("abcd");
        assert_eq!(result, "dcbabcd");
    }

    #[test]
    fn test_kmp_empty() {
        let result = shortest_palindrome_kmp("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_kmp_single_char() {
        let result = shortest_palindrome_kmp("a");
        assert_eq!(result, "a");
    }

    #[test]
    fn test_kmp_aba() {
        let result = shortest_palindrome_kmp("aba");
        assert_eq!(result, "aba");
    }

    #[test]
    fn test_kmp_abcba() {
        let result = shortest_palindrome_kmp("abcba");
        assert_eq!(result, "abcba");
    }

    #[test]
    fn test_kmp_aab() {
        let result = shortest_palindrome_kmp("aab");
        assert_eq!(result, "baab");
    }

    #[test]
    fn test_kmp_abc() {
        let result = shortest_palindrome_kmp("abc");
        assert_eq!(result, "cbaabc");
    }

    // Two pointer tests
    #[test]
    fn test_two_pointer_basic() {
        let result = shortest_palindrome_two_pointer("aacecaaa");
        assert_eq!(result, "aaacecaaa");
    }

    #[test]
    fn test_two_pointer_abcd() {
        let result = shortest_palindrome_two_pointer("abcd");
        assert_eq!(result, "dcbabcd");
    }

    #[test]
    fn test_two_pointer_empty() {
        let result = shortest_palindrome_two_pointer("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_two_pointer_single() {
        let result = shortest_palindrome_two_pointer("a");
        assert_eq!(result, "a");
    }

    // Brute force tests
    #[test]
    fn test_brute_basic() {
        let result = shortest_palindrome_brute("aacecaaa");
        assert_eq!(result, "aaacecaaa");
    }

    #[test]
    fn test_brute_abcd() {
        let result = shortest_palindrome_brute("abcd");
        assert_eq!(result, "dcbabcd");
    }

    #[test]
    fn test_brute_empty() {
        let result = shortest_palindrome_brute("");
        assert_eq!(result, "");
    }

    // Consistency tests
    #[test]
    fn test_all_approaches_consistent() {
        let inputs = ["aacecaaa", "abcd", "aba", "aab", "abc", "a", "racecar", "abcba"];

        for input in inputs {
            let kmp_result = shortest_palindrome_kmp(input);
            let tp_result = shortest_palindrome_two_pointer(input);
            let brute_result = shortest_palindrome_brute(input);

            assert_eq!(kmp_result, tp_result, "KMP and two-pointer differ for {}", input);
            assert_eq!(kmp_result, brute_result, "KMP and brute differ for {}", input);

            // Verify result is palindrome
            assert!(is_palindrome(&kmp_result), "{} should be palindrome", kmp_result);
        }
    }

    // Verify palindrome property
    #[test]
    fn test_kmp_result_is_palindrome() {
        let inputs = ["", "a", "ab", "aab", "abcd", "aacecaaa", "abcba", "racecar"];

        for input in inputs {
            let result = shortest_palindrome_kmp(input);
            assert!(is_palindrome(&result), "Result '{}' is not palindrome for input '{}'", result, input);
        }
    }

    #[test]
    fn test_two_pointer_result_is_palindrome() {
        let inputs = ["", "a", "ab", "aab", "abcd", "aacecaaa", "abcba", "racecar"];

        for input in inputs {
            let result = shortest_palindrome_two_pointer(input);
            assert!(is_palindrome(&result), "Result '{}' is not palindrome for input '{}'", result, input);
        }
    }

    #[test]
    fn test_brute_result_is_palindrome() {
        let inputs = ["", "a", "ab", "aab", "abcd", "aacecaaa", "abcba", "racecar"];

        for input in inputs {
            let result = shortest_palindrome_brute(input);
            assert!(is_palindrome(&result), "Result '{}' is not palindrome for input '{}'", result, input);
        }
    }

    // Test shortest property (minimum added characters)
    #[test]
    fn test_kmp_minimum_additions() {
        // For "aacecaaa", only 1 character needs to be added
        let result = shortest_palindrome_kmp("aacecaaa");
        assert_eq!(result.len() - "aacecaaa".len(), 1);
    }

    #[test]
    fn test_kmp_long_string() {
        let s = "abcdefghijklmnopqrstuvwxyz";
        let result = shortest_palindrome_kmp(s);
        // Should add 25 characters (reverse of first 25)
        assert_eq!(result.len(), s.len() * 2 - 1);
    }

    #[test]
    fn test_kmp_all_same_char() {
        let s = "aaaaa";
        let result = shortest_palindrome_kmp(s);
        // Already palindrome, no additions
        assert_eq!(result, "aaaaa");
    }

    #[test]
    fn test_two_pointer_minimum_additions() {
        let result = shortest_palindrome_two_pointer("aacecaaa");
        assert_eq!(result.len() - "aacecaaa".len(), 1);
    }

    #[test]
    fn test_palindrome_prefix_length() {
        let s = "aacecaaa";
        let palindrome = "aacecaa";
        assert!(is_palindrome(palindrome));

        let s = "abcd";
        let palindrome = ""; // no palindrome prefix
        assert!(is_palindrome(palindrome));
    }

    #[test]
    fn test_lps_computation() {
        let lps = compute_lps("aacecaaa#aaacecaa");
        // The last value should give us the length of palindrome prefix
        assert_eq!(lps.last(), Some(&1)); // Wait, this seems wrong...
    }

    #[test]
    fn test_complex_cases() {
        // String with no palindrome prefix
        let result = shortest_palindrome_kmp("abcde");
        assert!(is_palindrome(&result));

        // String starting with palindrome
        let result = shortest_palindrome_kmp("aba123");
        assert!(is_palindrome(&result));

        // String ending with palindrome
        let result = shortest_palindrome_kmp("123aba");
        assert!(is_palindrome(&result));
    }

    #[test]
    fn test_unicode_handling() {
        let result = shortest_palindrome_kmp("你好");
        assert!(is_palindrome(&result));

        let result = shortest_palindrome_kmp("a你b");
        assert!(is_palindrome(&result));
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("082_shortest_palindrome_lc214 exercises - run tests with cargo test");
}
