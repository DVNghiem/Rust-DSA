/// Approach 1: DP with 2D array (standard)
pub fn min_distance(word1: String, word2: String) -> i32 {
    let w1 = word1.as_bytes();
    let w2 = word2.as_bytes();
    let m = w1.len();
    let n = w2.len();

    let mut dp = vec![vec![0i32; n + 1]; m + 1];

    // Base cases: converting to/from empty string
    for i in 0..=m { dp[i][0] = i as i32; }
    for j in 0..=n { dp[0][j] = j as i32; }

    for i in 1..=m {
        for j in 1..=n {
            if w1[i-1] == w2[j-1] {
                dp[i][j] = dp[i-1][j-1];
            } else {
                dp[i][j] = 1 + dp[i-1][j].min(dp[i][j-1]).min(dp[i-1][j-1]);
            }
        }
    }

    dp[m][n]
}

/// Approach 2: DP with 1D array (space optimized)
pub fn min_distance_1d(word1: String, word2: String) -> i32 {
    let w1 = word1.as_bytes();
    let w2 = word2.as_bytes();
    let m = w1.len();
    let n = w2.len();

    // Make sure w2 is the shorter one for 1D optimization
    if m < n {
        // Already optimal
    }

    let mut dp = vec![0i32; n + 1];

    // Base case: empty word1
    for j in 0..=n {
        dp[j] = j as i32;
    }

    for i in 1..=m {
        let mut prev = dp[0]; // dp[i-1][0] before update
        dp[0] = i as i32;

        for j in 1..=n {
            let temp = dp[j]; // This is dp[i-1][j] before update
            if w1[i-1] == w2[j-1] {
                dp[j] = prev; // dp[i-1][j-1]
            } else {
                dp[j] = 1 + prev.min(dp[j]).min(dp[j-1]);
            }
            prev = temp;
        }
    }

    dp[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horse_to_ros() {
        // horse -> ros = 3
        let result = min_distance("horse".to_string(), "ros".to_string());
        assert_eq!(result, 3);
    }

    #[test]
    fn test_empty_word1() {
        // "" -> "ros" = 3 insertions
        let result = min_distance("".to_string(), "ros".to_string());
        assert_eq!(result, 3);
    }

    #[test]
    fn test_empty_word2() {
        // "horse" -> "" = 5 deletions
        let result = min_distance("horse".to_string(), "".to_string());
        assert_eq!(result, 5);
    }

    #[test]
    fn test_intention() {
        // "intention" -> "execution" = 5
        let result = min_distance("intention".to_string(), "execution".to_string());
        assert_eq!(result, 5);
    }

    #[test]
    fn test_identical() {
        let result = min_distance("abc".to_string(), "abc".to_string());
        assert_eq!(result, 0);
    }

    #[test]
    fn test_single_char_diff() {
        // "a" -> "b" = 1 replace
        let result = min_distance("a".to_string(), "b".to_string());
        assert_eq!(result, 1);
    }

    #[test]
    fn test_insert() {
        // "ab" -> "abc" = 1 insert
        let result = min_distance("ab".to_string(), "abc".to_string());
        assert_eq!(result, 1);
    }

    #[test]
    fn test_delete() {
        // "abc" -> "ab" = 1 delete
        let result = min_distance("abc".to_string(), "ab".to_string());
        assert_eq!(result, 1);
    }

    #[test]
    fn test_1d_same_as_2d() {
        let test_cases = vec![
            ("horse", "ros"),
            ("", "ros"),
            ("intention", "execution"),
            ("abc", "abc"),
            ("a", "b"),
        ];

        for (w1, w2) in test_cases {
            let dp = min_distance(w1.to_string(), w2.to_string());
            let one_d = min_distance_1d(w1.to_string(), w2.to_string());
            assert_eq!(dp, one_d, "Failed for {} -> {}", w1, w2);
        }
    }

    #[test]
    fn test_all_same_prefix() {
        // "abc" -> "abx" = 1 replace
        let result = min_distance("abc".to_string(), "abx".to_string());
        assert_eq!(result, 1);
    }

    #[test]
    fn test_longer_both() {
        let result = min_distance("abcdef".to_string(), "azced".to_string());
        assert!(result > 0);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("046_edit_distance_lc72 exercises - run tests with cargo test");
}
