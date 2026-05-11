/// Decode Ways II - LeetCode 639
/// Count decode ways where '*' can be any digit 1-9.

/// Approach: Dynamic Programming with modulo arithmetic
/// dp[i] = ways to decode s[0..i]
pub fn num_decodings(s: String) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut dp = vec![0i64; n + 1];
    dp[0] = 1;

    // Handle first character
    match bytes[0] {
        b'*' => dp[1] = 9,
        b'0' => return 0,
        _ => dp[1] = 1,
    }

    for i in 2..=n {
        // Single character decode
        match bytes[i - 1] {
            b'*' => dp[i] = (dp[i] + dp[i - 1] * 9) % MOD,
            b'0' => {}
            _ => dp[i] = (dp[i] + dp[i - 1]) % MOD,
        }

        // Two character decode
        match (bytes[i - 2], bytes[i - 1]) {
            (b'1', b'*') => dp[i] = (dp[i] + dp[i - 2] * 9) % MOD,
            (b'2', b'*') => dp[i] = (dp[i] + dp[i - 2] * 6) % MOD,
            (b'*', b'*') => dp[i] = (dp[i] + dp[i - 2] * 15) % MOD,
            (b'1', _) => dp[i] = (dp[i] + dp[i - 2]) % MOD,
            (b'2', b'0'..=b'6') => dp[i] = (dp[i] + dp[i - 2]) % MOD,
            _ => {}
        }
    }

    dp[n] as i32
}

/// Space-optimized version
pub fn num_decodings_optimized(s: String) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut prev2: i64 = 1; // dp[0]
    let mut prev1: i64 = match bytes[0] {
        b'*' => 9,
        b'0' => return 0,
        _ => 1,
    };

    if n == 1 { return prev1 as i32; }

    let mut curr: i64 = 0;
    for i in 2..=n {
        curr = 0;

        // Single
        match bytes[i - 1] {
            b'*' => curr = (curr + prev1 * 9) % MOD,
            b'0' => {}
            _ => curr = (curr + prev1) % MOD,
        }

        // Double
        match (bytes[i - 2], bytes[i - 1]) {
            (b'1', b'*') => curr = (curr + prev2 * 9) % MOD,
            (b'2', b'*') => curr = (curr + prev2 * 6) % MOD,
            (b'*', b'*') => curr = (curr + prev2 * 15) % MOD,
            (b'1', _) => curr = (curr + prev2) % MOD,
            (b'2', b'0'..=b'6') => curr = (curr + prev2) % MOD,
            _ => {}
        }

        prev2 = prev1;
        prev1 = curr;
    }

    prev1 as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_decodings_basic() {
        assert_eq!(num_decodings("12".to_string()), 2);
    }

    #[test]
    fn test_num_decodings_star() {
        assert_eq!(num_decodings("*".to_string()), 9);
    }

    #[test]
    fn test_num_decodings_one_star() {
        assert_eq!(num_decodings("1*".to_string()), 18);
    }

    #[test]
    fn test_num_decodings_two_stars() {
        assert_eq!(num_decodings("**".to_string()), 96);
    }

    #[test]
    fn test_num_decodings_zero_invalid() {
        assert_eq!(num_decodings("0".to_string()), 0);
    }

    #[test]
    fn test_num_decodings_leading_zero() {
        assert_eq!(num_decodings("10".to_string()), 1);
    }

    #[test]
    fn test_num_decodings_20() {
        assert_eq!(num_decodings("20".to_string()), 1);
    }

    #[test]
    fn test_num_decodings_30() {
        assert_eq!(num_decodings("30".to_string()), 0);
    }

    #[test]
    fn test_num_decodings_single_1() {
        assert_eq!(num_decodings("1".to_string()), 1);
    }

    #[test]
    fn test_num_decodings_single_9() {
        assert_eq!(num_decodings("9".to_string()), 1);
    }

    #[test]
    fn test_num_decodings_complex() {
        assert_eq!(num_decodings("*1".to_string()), 18);
    }

    #[test]
    fn test_num_decodings_11() {
        assert_eq!(num_decodings("11".to_string()), 2);
    }

    #[test]
    fn test_num_decodings_27() {
        assert_eq!(num_decodings("27".to_string()), 1);
    }

    #[test]
    fn test_num_decodings_30_star() {
        // "30" -> 0 ways, but "3*" -> 9 ways
        assert_eq!(num_decodings("3*".to_string()), 9);
    }

    #[test]
    fn test_num_decodings_10_star() {
        // "10*" -> 10 -> valid, * -> 9 options = 9
        assert_eq!(num_decodings("10*".to_string()), 9);
    }

    #[test]
    fn test_num_decodings_empty() {
        assert_eq!(num_decodings("".to_string()), 1);
    }

    #[test]
    fn test_num_decodings_long_string() {
        let s = "11111111111111111111".to_string();
        let result = num_decodings(s);
        assert!(result > 0);
    }

    #[test]
    fn test_num_decodings_optimized_basic() {
        assert_eq!(num_decodings_optimized("*".to_string()), 9);
    }

    #[test]
    fn test_num_decodings_optimized_1star() {
        assert_eq!(num_decodings_optimized("1*".to_string()), 18);
    }

    #[test]
    fn test_num_decodings_optimized_12() {
        assert_eq!(num_decodings_optimized("12".to_string()), 2);
    }

    #[test]
    fn test_num_decodings_optimized_20() {
        assert_eq!(num_decodings_optimized("20".to_string()), 1);
    }

    #[test]
    fn test_num_decodings_optimized_zero() {
        assert_eq!(num_decodings_optimized("0".to_string()), 0);
    }

    #[test]
    fn test_num_decodings_same_result() {
        let s = "1*2*3*".to_string();
        assert_eq!(num_decodings(s.clone()), num_decodings_optimized(s));
    }

    #[test]
    fn test_num_decodings_all_stars() {
        // 9 * 9 * 9 * 9 * 9 = 59049
        assert_eq!(num_decodings("*****".to_string()), 59049);
    }

    #[test]
    fn test_num_decodings_no_zeros() {
        assert_eq!(num_decodings("123".to_string()), 3);
    }

    #[test]
    fn test_num_decodings_with_27() {
        // 2 can be part of 27, but also alone
        assert_eq!(num_decodings("27".to_string()), 1);
    }

    #[test]
    fn test_num_decodings_large_result() {
        let s = "*".repeat(100);
        let result = num_decodings(s);
        assert!(result > 0);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("064_decode_ways_ii_lc639 exercises - run tests with cargo test");
}
