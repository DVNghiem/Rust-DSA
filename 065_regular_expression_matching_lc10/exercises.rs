/// Regular Expression Matching - LeetCode 10
/// Implement '.' and '*' pattern matching with dynamic programming.

/// Approach: 2D DP
/// dp[i][j] = does s[0..i] match p[0..j]?
pub fn is_match(s: String, p: String) -> bool {
    let s = s.as_bytes();
    let p = p.as_bytes();
    let m = s.len();
    let n = p.len();
    let mut dp = vec![vec![false; n + 1]; m + 1];
    dp[0][0] = true;

    // Initialize first row (empty string)
    for j in 2..=n {
        if p[j - 1] == b'*' {
            dp[0][j] = dp[0][j - 2];
        }
    }

    fn matches(sc: u8, pc: u8) -> bool {
        pc == b'.' || sc == pc
    }

    for i in 1..=m {
        for j in 1..=n {
            match p[j - 1] {
                b'*' => {
                    // Zero occurrences of preceding element
                    dp[i][j] = dp[i][j - 2];
                    // One or more occurrences
                    if matches(s[i - 1], p[j - 2]) {
                        dp[i][j] = dp[i][j] || dp[i - 1][j];
                    }
                }
                b'.' => {
                    dp[i][j] = dp[i - 1][j - 1];
                }
                c => {
                    dp[i][j] = dp[i - 1][j - 1] && s[i - 1] == c;
                }
            }
        }
    }

    dp[m][n]
}

/// Space-optimized version using two rows
pub fn is_match_optimized(s: String, p: String) -> bool {
    let s = s.as_bytes();
    let p = p.as_bytes();
    let m = s.len();
    let n = p.len();
    let mut prev = vec![false; n + 1];
    prev[0] = true;

    for j in 2..=n {
        if p[j - 1] == b'*' {
            prev[j] = prev[j - 2];
        }
    }

    for i in 1..=m {
        let mut curr = vec![false; n + 1];
        for j in 1..=n {
            match p[j - 1] {
                b'*' => {
                    curr[j] = curr[j - 2];
                    if p[j - 2] == b'.' || s[i - 1] == p[j - 2] {
                        curr[j] = curr[j] || prev[j];
                    }
                }
                b'.' => {
                    curr[j] = prev[j - 1];
                }
                c => {
                    curr[j] = prev[j - 1] && s[i - 1] == c;
                }
            }
        }
        prev = curr;
    }

    prev[n]
}

/// Recursive with memoization
pub fn is_match_memo(s: String, p: String) -> bool {
    let s = s.as_bytes();
    let p = p.as_bytes();
    let m = s.len();
    let n = p.len();
    let mut memo = vec![vec![-1i32; n + 1]; m + 1];

    fn dp(i: usize, j: usize, s: &[u8], p: &[u8], memo: &mut Vec<Vec<i32>>) -> bool {
        if j >= p.len() { return i >= s.len(); }
        if memo[i][j] != -1 { return memo[i][j] == 1; }

        let first_match = i < s.len() && (p[j] == b'.' || s[i] == p[j]);
        let mut result = false;

        if j + 1 < p.len() && p[j + 1] == b'*' {
            // Zero or more
            result = dp(i, j + 2, s, p, memo) ||
                     (first_match && dp(i + 1, j, s, p, memo));
        } else {
            result = first_match && dp(i + 1, j + 1, s, p, memo);
        }

        memo[i][j] = if result { 1 } else { 0 };
        result
    }

    dp(0, 0, s, p, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_match_false() {
        assert!(!is_match("aa".to_string(), "a".to_string()));
    }

    #[test]
    fn test_is_match_star() {
        assert!(is_match("aa".to_string(), "a*".to_string()));
    }

    #[test]
    fn test_is_match_dot_star() {
        assert!(is_match("ab".to_string(), ".*".to_string()));
    }

    #[test]
    fn test_is_match_empty() {
        assert!(is_match("".to_string(), "".to_string()));
    }

    #[test]
    fn test_is_match_empty_pattern_a() {
        assert!(!is_match("a".to_string(), "".to_string()));
    }

    #[test]
    fn test_is_match_dot() {
        assert!(is_match("a".to_string(), ".".to_string()));
    }

    #[test]
    fn test_is_match_exact() {
        assert!(is_match("abc".to_string(), "abc".to_string()));
    }

    #[test]
    fn test_is_match_star_aa() {
        assert!(is_match("aa".to_string(), "a*".to_string()));
    }

    #[test]
    fn test_is_match_star_zero() {
        assert!(is_match("".to_string(), "a*".to_string()));
    }

    #[test]
    fn test_is_match_complex() {
        assert!(is_match("aab".to_string(), "c*a*b".to_string()));
    }

    #[test]
    fn test_is_match_no_match() {
        assert!(!is_match("mississippi".to_string(), "mis*is*p*.".to_string()));
    }

    #[test]
    fn test_is_match_optimized_basic() {
        assert!(!is_match_optimized("aa".to_string(), "a".to_string()));
        assert!(is_match_optimized("aa".to_string(), "a*".to_string()));
    }

    #[test]
    fn test_is_match_optimized_dot_star() {
        assert!(is_match_optimized("ab".to_string(), ".*".to_string()));
    }

    #[test]
    fn test_is_match_memo_basic() {
        assert!(!is_match_memo("aa".to_string(), "a".to_string()));
        assert!(is_match_memo("aa".to_string(), "a*".to_string()));
    }

    #[test]
    fn test_is_match_memo_empty() {
        assert!(is_match_memo("".to_string(), "".to_string()));
    }

    #[test]
    fn test_all_approaches_same() {
        let tests = vec![
            ("aa", "a"),
            ("aa", "a*"),
            ("ab", ".*"),
            ("aab", "c*a*b"),
            ("", ""),
            ("a", "."),
        ];
        for (s, p) in tests {
            let s = s.to_string();
            let p = p.to_string();
            let r1 = is_match(s.clone(), p.clone());
            let r2 = is_match_optimized(s.clone(), p.clone());
            let r3 = is_match_memo(s.clone(), p.clone());
            assert_eq!(r1, r2);
            assert_eq!(r2, r3);
        }
    }

    #[test]
    fn test_is_match_longer_pattern() {
        assert!(is_match("aaa".to_string(), "a*a".to_string()));
    }

    #[test]
    fn test_is_match_bbbb() {
        assert!(is_match("bbbb".to_string(), "b*".to_string()));
    }

    #[test]
    fn test_is_match_acb() {
        // "a" matches ".*c*" but not "a*cb*"
        assert!(is_match("acb".to_string(), ".*c".to_string()));
    }

    #[test]
    fn test_is_match_empty_with_star() {
        assert!(is_match("".to_string(), ".*".to_string()));
    }

    #[test]
    fn test_is_match_many_stars() {
        assert!(is_match("a".to_string(), ".*.*.*".to_string()));
    }

    #[test]
    fn test_is_match_complex_2() {
        assert!(!is_match("abc".to_string(), "a*c".to_string()));
    }

    #[test]
    fn test_is_match_multiple_stars() {
        // "aaa" matches "a*a"
        assert!(is_match("aaa".to_string(), "a*a".to_string()));
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("065_regular_expression_matching_lc10 exercises - run tests with cargo test");
}
