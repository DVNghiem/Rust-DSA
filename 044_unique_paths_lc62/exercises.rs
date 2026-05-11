/// Approach 1: Combinatorial (optimal)
pub fn unique_paths(m: i32, n: i32) -> i32 {
    let m = m as usize;
    let n = n as usize;

    // C(m+n-2, m-1) = (m+n-2)! / ((m-1)! * (n-1)!)
    let total = m + n - 2;
    let k = (m - 1).min(n - 1);

    let mut result: f64 = 1.0;
    for i in 0..k {
        result *= (total - i) as f64 / (i + 1) as f64;
    }

    result as i32
}

/// Approach 2: DP with 2D array
pub fn unique_paths_dp(m: i32, n: i32) -> i32 {
    let m = m as usize;
    let n = n as usize;

    let mut dp = vec![vec![0i64; n]; m];
    dp[0][0] = 1;

    for i in 0..m {
        for j in 0..n {
            if i > 0 {
                dp[i][j] += dp[i - 1][j];
            }
            if j > 0 {
                dp[i][j] += dp[i][j - 1];
            }
        }
    }

    dp[m - 1][n - 1] as i32
}

/// Approach 3: DP with 1D array (space optimized)
pub fn unique_paths_dp_1d(m: i32, n: i32) -> i32 {
    let m = m as usize;
    let n = n as usize;

    let mut dp = vec![0i64; n];

    for i in 0..m {
        for j in 0..n {
            if i == 0 && j == 0 {
                dp[j] = 1;
            } else {
                let from_top = if i > 0 { dp[j] } else { 0 };
                let from_left = if j > 0 { dp[j - 1] } else { 0 };
                dp[j] = from_top + from_left;
            }
        }
    }

    dp[n - 1] as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3x7() {
        // C(8,2) = 28
        assert_eq!(unique_paths(3, 7), 28);
    }

    #[test]
    fn test_1x1() {
        assert_eq!(unique_paths(1, 1), 1);
    }

    #[test]
    fn test_1_row() {
        assert_eq!(unique_paths(1, 10), 1);
    }

    #[test]
    fn test_1_column() {
        assert_eq!(unique_paths(10, 1), 1);
    }

    #[test]
    fn test_2x2() {
        // Only 2 paths: RD or DR
        assert_eq!(unique_paths(2, 2), 2);
    }

    #[test]
    fn test_3x3() {
        // C(4,2) = 6
        assert_eq!(unique_paths(3, 3), 6);
    }

    #[test]
    fn test_comb_vs_dp() {
        for m in 1..=10 {
            for n in 1..=10 {
                let comb = unique_paths(m, n);
                let dp = unique_paths_dp(m, n);
                assert_eq!(comb, dp, "Failed for {}x{}", m, n);
            }
        }
    }

    #[test]
    fn test_dp_vs_dp_1d() {
        for m in 1..=10 {
            for n in 1..=10 {
                let dp = unique_paths_dp(m, n);
                let dp1d = unique_paths_dp_1d(m, n);
                assert_eq!(dp, dp1d, "Failed for {}x{}", m, n);
            }
        }
    }

    #[test]
    fn test_large_m_n() {
        // 20x20 = C(38,19) = 35345263800
        let result = unique_paths(20, 20);
        assert_eq!(result, 35345263800);
    }

    #[test]
    fn test_small_m_large_n() {
        // m=3, n=10 → C(11,2) = 55
        assert_eq!(unique_paths(3, 10), 55);
    }

    #[test]
    fn test_large_m_small_n() {
        // m=10, n=3 → C(11,2) = 55
        assert_eq!(unique_paths(10, 3), 55);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("Unique Paths exercises - run tests with cargo test");
}