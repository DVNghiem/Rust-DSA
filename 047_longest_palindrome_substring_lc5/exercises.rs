/// Approach 1: Expand Around Center
pub fn longest_palindrome(s: String) -> String {
    let bytes = s.as_bytes();
    let n = bytes.len();
    if n <= 1 { return s; }

    let (mut start, mut end) = (0, 0);

    fn expand(s: &[u8], mut l: usize, mut r: usize) -> (usize, usize) {
        while l > 0 && r < s.len() && s[l] == s[r] {
            l -= 1;
            r += 1;
        }
        (l + 1, r - 1)
    }

    for i in 0..n {
        // Odd length palindrome: center at i
        let (s1, e1) = expand(bytes, i, i);
        if e1 - s1 > end - start {
            start = s1;
            end = e1;
        }

        // Even length palindrome: center between i and i+1
        let (s2, e2) = expand(bytes, i, i + 1);
        if e2 - s2 > end - start {
            start = s2;
            end = e2;
        }
    }

    String::from_utf8_lossy(&bytes[start..=end]).to_string()
}

/// Approach 2: DP approach
pub fn longest_palindrome_dp(s: String) -> String {
    let bytes = s.as_bytes();
    let n = bytes.len();
    if n <= 1 { return s; }

    let mut dp = vec![vec![false; n]; n];
    let mut start = 0;
    let mut max_len = 1;

    // All single characters are palindromes
    for i in 0..n {
        dp[i][i] = true;
    }

    // Check for length 2
    for i in 0..n-1 {
        if bytes[i] == bytes[i+1] {
            dp[i][i+1] = true;
            start = i;
            max_len = 2;
        }
    }

    // Check lengths >= 3
    for len in 3..=n {
        for i in 0..=n-len {
            let j = i + len - 1;
            if dp[i+1][j-1] && bytes[i] == bytes[j] {
                dp[i][j] = true;
                if len > max_len {
                    start = i;
                    max_len = len;
                }
            }
        }
    }

    String::from_utf8_lossy(&bytes[start..start+max_len]).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_babad() {
        let result = longest_palindrome("babad".to_string());
        assert!(result == "bab" || result == "aba");
    }

    #[test]
    fn test_basic_cbbd() {
        let result = longest_palindrome("cbbd".to_string());
        assert_eq!(result, "bb");
    }

    #[test]
    fn test_single_char() {
        assert_eq!(longest_palindrome("a".to_string()), "a");
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(longest_palindrome("".to_string()), "");
    }

    #[test]
    fn test_entire_string() {
        assert_eq!(longest_palindrome("abcba".to_string()), "abcba");
    }

    #[test]
    fn test_two_chars_same() {
        assert_eq!(longest_palindrome("aa".to_string()), "aa");
    }

    #[test]
    fn test_two_chars_diff() {
        assert_eq!(longest_palindrome("ab".to_string()), "a"); // or "b"
    }

    #[test]
    fn test_three_chars_palindrome() {
        assert_eq!(longest_palindrome("aba".to_string()), "aba");
    }

    #[test]
    fn test_no_palindrome_greater_than_1() {
        // Only single chars are palindromes
        let result = longest_palindrome("abc".to_string());
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_dp_matches_expand() {
        let test_cases = vec!["babad", "cbbd", "a", "", "abcba", "aa", "ab", "aba"];

        for s in test_cases {
            let expand = longest_palindrome(s.to_string());
            let dp = longest_palindrome_dp(s.to_string());
            assert_eq!(expand.len(), dp.len(), "Failed for {}", s);
        }
    }

    #[test]
    fn test_long_string() {
        let result = longest_palindrome("a".repeat(1000));
        assert_eq!(result.len(), 1000);
    }

    #[test]
    fn test_center_even() {
        // "cbbd" - center between 1 and 2
        let result = longest_palindrome("cbbd".to_string());
        assert_eq!(result, "bb");
    }

    #[test]
    fn test_all_same() {
        assert_eq!(longest_palindrome("aaaaa".to_string()), "aaaaa");
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("047_longest_palindrome_substring_lc5 exercises - run tests with cargo test");
}
