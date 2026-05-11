/// Wildcard Matching - LeetCode 44
/// Implement '?' and '*' pattern matching.

/// Approach: 2D DP
/// '?' matches any single character
/// '*' matches any sequence (including empty)
pub fn is_match(s: String, p: String) -> bool {
    let s = s.as_bytes();
    let p = p.as_bytes();
    let m = s.len();
    let n = p.len();
    let mut dp = vec![vec![false; n + 1]; m + 1];
    dp[0][0] = true;

    // Initialize first row
    for j in 1..=n {
        if p[j - 1] == b'*' {
            dp[0][j] = dp[0][j - 1];
        }
    }

    for i in 1..=m {
        for j in 1..=n {
            match p[j - 1] {
                b'?' => dp[i][j] = dp[i - 1][j - 1],
                b'*' => dp[i][j] = dp[i][j - 1] || dp[i - 1][j],
                c => dp[i][j] = dp[i - 1][j - 1] && s[i - 1] == c,
            }
        }
    }

    dp[m][n]
}

/// Greedy two-pointer approach (optimal O(n+m))
pub fn is_match_greedy(s: String, p: String) -> bool {
    let s = s.as_bytes();
    let p = p.as_bytes();
    let mut s_idx = 0;
    let mut p_idx = 0;
    let mut star_idx = None;
    let mut s_tmp_idx = None;

    while s_idx < s.len() {
        if p_idx < p.len() && (p[p_idx] == b'?' || p[p_idx] == s[s_idx]) {
            s_idx += 1;
            p_idx += 1;
        } else if p_idx < p.len() && p[p_idx] == b'*' {
            star_idx = Some(p_idx);
            s_tmp_idx = Some(s_idx);
            p_idx += 1;
        } else if let Some(star) = star_idx {
            p_idx = star + 1;
            s_idx = s_tmp_idx.unwrap() + 1;
            s_tmp_idx = Some(s_idx);
        } else {
            return false;
        }
    }

    // Check remaining pattern
    while p_idx < p.len() && p[p_idx] == b'*' {
        p_idx += 1;
    }

    p_idx == p.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_match_star() {
        assert!(is_match("aa".to_string(), "*".to_string()));
    }

    #[test]
    fn test_is_match_question() {
        assert!(!is_match("cb".to_string(), "?a".to_string()));
    }

    #[test]
    fn test_is_match_complex() {
        assert!(is_match("adceb".to_string(), "*a*b".to_string()));
    }

    #[test]
    fn test_is_match_empty() {
        assert!(is_match("".to_string(), "*".to_string()));
    }

    #[test]
    fn test_is_match_empty_question() {
        assert!(is_match("".to_string(), "?".to_string()));
    }

    #[test]
    fn test_is_match_exact() {
        assert!(is_match("abc".to_string(), "abc".to_string()));
    }

    #[test]
    fn test_is_match_question_each() {
        assert!(is_match("abc".to_string(), "???".to_string()));
    }

    #[test]
    fn test_is_match_star_question() {
        assert!(is_match("abc".to_string(), "*??".to_string()));
    }

    #[test]
    fn test_is_match_no_match() {
        assert!(!is_match("abc".to_string(), "*d".to_string()));
    }

    #[test]
    fn test_is_match_greedy_star() {
        assert!(is_match_greedy("aa".to_string(), "*".to_string()));
    }

    #[test]
    fn test_is_match_greedy_question() {
        assert!(!is_match_greedy("cb".to_string(), "?a".to_string()));
    }

    #[test]
    fn test_is_match_greedy_complex() {
        assert!(is_match_greedy("adceb".to_string(), "*a*b".to_string()));
    }

    #[test]
    fn test_all_approaches_same() {
        let tests = vec![
            ("aa", "*"),
            ("cb", "?a"),
            ("adceb", "*a*b"),
            ("", "*"),
            ("abc", "???"),
        ];
        for (s, p) in tests {
            let s = s.to_string();
            let p = p.to_string();
            let r1 = is_match(s.clone(), p.clone());
            let r2 = is_match_greedy(s.clone(), p.clone());
            assert_eq!(r1, r2, "Mismatch for s={}, p={}", s, p);
        }
    }

    #[test]
    fn test_is_match_multiple_stars() {
        assert!(is_match("abc".to_string(), "***".to_string()));
    }

    #[test]
    fn test_is_match_star_at_end() {
        assert!(is_match("abc".to_string(), "abc*".to_string()));
    }

    #[test]
    fn test_is_match_star_at_start() {
        assert!(is_match("abc".to_string(), "*abc".to_string()));
    }

    #[test]
    fn test_is_match_only_question() {
        assert!(is_match("a".to_string(), "?".to_string()));
    }

    #[test]
    fn test_is_match_mixed() {
        assert!(is_match("abc".to_string(), "?*?".to_string()));
    }

    #[test]
    fn test_is_match_long_star() {
        let s = "abc";
        let p = "*a*b*c*";
        assert!(is_match(s.to_string(), p.to_string()));
    }

    #[test]
    fn test_is_match_fail_early() {
        assert!(!is_match("abc".to_string(), "a*d".to_string()));
    }

    #[test]
    fn test_greedy_long_string() {
        let s = "a".repeat(1000);
        let p = "*".to_string();
        assert!(is_match_greedy(s, p));
    }

    #[test]
    fn test_greedy_question_sequence() {
        let s = "abc";
        let p = "???";
        assert!(is_match_greedy(s.to_string(), p.to_string()));
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("066_wildcard_matching_lc44 exercises - run tests with cargo test");
}
