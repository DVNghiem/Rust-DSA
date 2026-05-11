/// Longest Valid Parentheses - LeetCode 32
/// Find longest valid parentheses substring.

use std::collections::VecDeque;

/// Approach 1: Stack-based O(n)
/// Push indices, pop on ')', calculate lengths
pub fn longest_valid_parentheses(s: String) -> i32 {
    let bytes = s.as_bytes();
    let mut max_len = 0;
    let mut stack = VecDeque::new();
    stack.push_back(-1); // Base offset

    for (i, &c) in bytes.iter().enumerate() {
        if c == b'(' {
            stack.push_back(i as i32);
        } else {
            stack.pop_back();
            if stack.is_empty() {
                stack.push_back(i as i32);
            } else {
                max_len = max_len.max(i as i32 - stack.back().unwrap());
            }
        }
    }

    max_len
}

/// Approach 2: Two-pass O(n) without stack
pub fn longest_valid_parentheses_two_pass(s: String) -> i32 {
    let bytes = s.as_bytes();
    let mut max_len = 0;
    let mut left = 0;
    let mut right = 0;

    // Left to right pass
    for &c in bytes.iter() {
        if c == b'(' { left += 1; } else { right += 1; }
        if left == right {
            max_len = max_len.max(2 * left);
        } else if right > left {
            left = 0;
            right = 0;
        }
    }

    // Right to left pass
    left = 0;
    right = 0;
    for &c in bytes.iter().rev() {
        if c == b'(' { left += 1; } else { right += 1; }
        if left == right {
            max_len = max_len.max(2 * left);
        } else if left > right {
            left = 0;
            right = 0;
        }
    }

    max_len
}

/// Approach 3: DP O(n)
pub fn longest_valid_parentheses_dp(s: String) -> i32 {
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut dp = vec![0; n];
    let mut max_len = 0;

    for i in 1..n {
        if bytes[i] == b')' {
            if bytes[i - 1] == b'(' {
                dp[i] = if i >= 2 { dp[i - 2] + 2 } else { 2 };
            } else if i >= dp[i - 1] + 1 && bytes[i - dp[i - 1] - 1] == b'(' {
                dp[i] = dp[i - 1] + 2 + if i >= dp[i - 1] + 2 { dp[i - dp[i - 1] - 2] } else { 0 };
            }
            max_len = max_len.max(dp[i] as i32);
        }
    }

    max_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_basic() {
        assert_eq!(longest_valid_parentheses("()".to_string()), 2);
    }

    #[test]
    fn test_longest_two_pairs() {
        assert_eq!(longest_valid_parentheses("()()".to_string()), 4);
    }

    #[test]
    fn test_longest_nested() {
        assert_eq!(longest_valid_parentheses("(()())".to_string()), 6);
    }

    #[test]
    fn test_longest_empty() {
        assert_eq!(longest_valid_parentheses("".to_string()), 0);
    }

    #[test]
    fn test_longest_no_valid() {
        assert_eq!(longest_valid_parentheses("(((".to_string()), 0);
    }

    #[test]
    fn test_longest_leading_valid() {
        assert_eq!(longest_valid_parentheses("()(()".to_string()), 2);
    }

    #[test]
    fn test_longest_trailing_valid() {
        assert_eq!(longest_valid_parentheses("(()())".to_string()), 6);
    }

    #[test]
    fn test_longest_complex() {
        assert_eq!(longest_valid_parentheses(")()())".to_string()), 4);
    }

    #[test]
    fn test_longest_single_open() {
        assert_eq!(longest_valid_parentheses("(".to_string()), 0);
    }

    #[test]
    fn test_longest_single_close() {
        assert_eq!(longest_valid_parentheses(")".to_string()), 0);
    }

    #[test]
    fn test_longest_alternating() {
        assert_eq!(longest_valid_parentheses(")(".to_string()), 0);
    }

    #[test]
    fn test_longest_two_pass_basic() {
        assert_eq!(longest_valid_parentheses_two_pass("()".to_string()), 2);
    }

    #[test]
    fn test_longest_two_pass_complex() {
        assert_eq!(longest_valid_parentheses_two_pass(")()())".to_string()), 4);
    }

    #[test]
    fn test_longest_dp_basic() {
        assert_eq!(longest_valid_parentheses_dp("()".to_string()), 2);
    }

    #[test]
    fn test_longest_dp_complex() {
        assert_eq!(longest_valid_parentheses_dp(")()())".to_string()), 4);
    }

    #[test]
    fn test_all_approaches_same() {
        let tests = vec![
            "()",
            "()()",
            "(()())",
            ")",
            "(",
            ")(",
            "())(",
            "",
        ];
        for s in tests {
            let s = s.to_string();
            let r1 = longest_valid_parentheses(s.clone());
            let r2 = longest_valid_parentheses_two_pass(s.clone());
            let r3 = longest_valid_parentheses_dp(s.clone());
            assert_eq!(r1, r2);
            assert_eq!(r2, r3);
        }
    }

    #[test]
    fn test_longest_max_length() {
        // All valid
        let s = "()".repeat(500);
        assert_eq!(longest_valid_parentheses(s), 1000);
    }

    #[test]
    fn test_longest_windows() {
        // Longest valid in the middle
        let s = "())(".to_string();
        assert_eq!(longest_valid_parentheses(s), 2);
    }

    #[test]
    fn test_longest_many_open_close() {
        let s = "((()))".to_string();
        assert_eq!(longest_valid_parentheses(s), 6);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("Longest Valid Parentheses exercises - run tests with cargo test");
}