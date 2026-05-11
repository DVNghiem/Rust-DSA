/// Approach 1: DP with O(n) space
pub fn num_decodings(s: String) -> i32 {
    if s.is_empty() || s.as_bytes()[0] == b'0' {
        return 0;
    }

    let bytes = s.as_bytes();
    let n = bytes.len();

    let mut dp = vec![0i32; n + 1];
    dp[0] = 1;  // empty string has 1 way
    dp[1] = 1;  // first character is always valid (1-9)

    for i in 2..=n {
        // Single digit: check if s[i-1] is not '0'
        if bytes[i - 1] != b'0' {
            dp[i] += dp[i - 1];
        }

        // Two digits: check s[i-2] and s[i-1]
        let two = (bytes[i - 2] as i32 - b'0' as i32) * 10 + (bytes[i - 1] as i32 - b'0' as i32);
        if two >= 10 && two <= 26 {
            dp[i] += dp[i - 2];
        }
    }

    dp[n]
}

/// Approach 2: DP with O(1) space
pub fn num_decodings_optimized(s: String) -> i32 {
    if s.is_empty() || s.as_bytes()[0] == b'0' {
        return 0;
    }

    let bytes = s.as_bytes();
    let n = bytes.len();

    let mut prev2 = 1;  // dp[0]
    let mut prev1 = 1;  // dp[1]

    for i in 2..=n {
        let mut curr = 0;

        // Single digit
        if bytes[i - 1] != b'0' {
            curr += prev1;
        }

        // Two digits
        let two = (bytes[i - 2] as i32 - b'0' as i32) * 10 + (bytes[i - 1] as i32 - b'0' as i32);
        if two >= 10 && two <= 26 {
            curr += prev2;
        }

        prev2 = prev1;
        prev1 = curr;
    }

    prev1
}

/// Approach 3: Recursive with memo
pub fn num_decodings_recursive(s: String) -> i32 {
    fn helper(bytes: &[u8], i: usize, memo: &mut Vec<i32>) -> i32 {
        if i == bytes.len() {
            return 1;  // valid decoding
        }
        if i > bytes.len() {
            return 0;  // invalid
        }
        if bytes[i] == b'0' {
            return 0;
        }
        if memo[i] != -1 {
            return memo[i];
        }

        let mut result = helper(bytes, i + 1, memo);

        if i + 1 < bytes.len() {
            let two = (bytes[i] as i32 - b'0' as i32) * 10 + (bytes[i + 1] as i32 - b'0' as i32);
            if two >= 10 && two <= 26 {
                result += helper(bytes, i + 2, memo);
            }
        }

        memo[i] = result;
        result
    }

    let bytes = s.as_bytes();
    if bytes.is_empty() || bytes[0] == b'0' {
        return 0;
    }

    let mut memo = vec![-1i32; bytes.len()];
    helper(bytes, 0, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_12() {
        // "12" -> "AB" or "L"
        let result = num_decodings("12".to_string());
        assert_eq!(result, 2);
    }

    #[test]
    fn test_basic_226() {
        // "226" -> "BBF", "BZ", "VF"
        let result = num_decodings("226".to_string());
        assert_eq!(result, 3);
    }

    #[test]
    fn test_single_zero() {
        // "0" has no valid decoding
        assert_eq!(num_decodings("0".to_string()), 0);
    }

    #[test]
    fn test_single_digit() {
        assert_eq!(num_decodings("1".to_string()), 1);
        assert_eq!(num_decodings("9".to_string()), 1);
    }

    #[test]
    fn test_10() {
        // "10" -> only "J"
        assert_eq!(num_decodings("10".to_string()), 1);
    }

    #[test]
    fn test_100() {
        // "100" -> no valid decoding
        assert_eq!(num_decodings("100".to_string()), 0);
    }

    #[test]
    fn test_1010() {
        // "1010" -> "JJ"
        assert_eq!(num_decodings("1010".to_string()), 1);
    }

    #[test]
    fn test_110() {
        // "110" -> "KK"
        assert_eq!(num_decodings("110".to_string()), 1);
    }

    #[test]
    fn test_all_single_digits() {
        // "1111111111" -> many ways
        let result = num_decodings("1111111111".to_string());
        assert_eq!(result, 89);
    }

    #[test]
    fn test_all_valid_two_digits() {
        // "1111111111" with 11 pattern has specific count
        let result = num_decodings("11111".to_string());
        assert_eq!(result, 8);
    }

    #[test]
    fn test_two_digits_max() {
        // "26" -> "BF"
        assert_eq!(num_decodings("26".to_string()), 1);
    }

    #[test]
    fn test_thirty_invalid() {
        // "30" -> no valid decoding
        assert_eq!(num_decodings("30".to_string()), 0);
    }

    #[test]
    fn test_optimized_same_as_dp() {
        let test_cases = vec![
            "12", "226", "0", "1", "10", "100", "1010", "110",
            "27", "1111111111", "2222222222"
        ];

        for s in test_cases {
            let dp = num_decodings(s.to_string());
            let opt = num_decodings_optimized(s.to_string());
            assert_eq!(dp, opt, "Failed for {}", s);
        }
    }

    #[test]
    fn test_recursive_same_as_dp() {
        let test_cases = vec![
            "12", "226", "1", "10", "11111"
        ];

        for s in test_cases {
            let dp = num_decodings(s.to_string());
            let rec = num_decodings_recursive(s.to_string());
            assert_eq!(dp, rec, "Failed for {}", s);
        }
    }

    #[test]
    fn test_empty_string() {
        // Non-empty per problem, but handle gracefully
        assert_eq!(num_decodings("".to_string()), 0);
    }

    #[test]
    fn test_large_string() {
        let s = "11111111111111111111111111";
        let result = num_decodings(s.to_string());
        assert!(result > 0);
    }

    #[test]
    fn test_invalid_leading_zero() {
        assert_eq!(num_decodings("01".to_string()), 0);
        assert_eq!(num_decodings("012".to_string()), 0);
    }

    #[test]
    fn test_27() {
        // "27" -> only "BG" (2, 7)
        assert_eq!(num_decodings("27".to_string()), 1);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("043_decode_ways_lc91 exercises - run tests with cargo test");
}
