/// Approach 1: Iterative DP with two variables (optimal)
///
/// ways(n) = ways(n-1) + ways(n-2)
/// Only need previous two values, not full array.
pub fn climb_stairs(n: i32) -> i32 {
    if n <= 2 {
        return n;
    }

    let mut prev2 = 1;  // ways(1)
    let mut prev1 = 2;  // ways(2)

    for i in 3..=n {
        let current = prev1 + prev2;
        prev2 = prev1;
        prev1 = current;
    }

    prev1
}

/// Approach 2: Recursive with memoization
pub fn climb_stairs_recursive(n: i32) -> i32 {
    fn helper(n: i32, memo: &mut Vec<i64>) -> i64 {
        if n <= 2 {
            return n as i64;
        }
        if memo[n as usize] != 0 {
            return memo[n as usize];
        }
        memo[n as usize] = helper(n - 1, memo) + helper(n - 2, memo);
        memo[n as usize]
    }

    let mut memo = vec![0i64; (n + 1) as usize];
    helper(n, &mut memo) as i32
}

/// Approach 3: Using i64 to prevent overflow
pub fn climb_stairs_i64(n: i32) -> i64 {
    if n <= 2 {
        return n as i64;
    }

    let mut prev2: i64 = 1;
    let mut prev1: i64 = 2;

    for _ in 3..=n {
        let current = prev1 + prev2;
        prev2 = prev1;
        prev1 = current;
    }

    prev1
}

/// Approach 4: Generalized for k steps
pub fn climb_stairs_k(n: i32, k: i32) -> i64 {
    if n <= 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let n = n as usize;
    let k = k as usize;

    let mut dp = vec![0i64; n + 1];
    dp[0] = 1;  // 1 way to stay at ground
    dp[1] = 1;  // 1 way to reach step 1

    for i in 2..=n {
        for j in 1..=k {
            if i >= j {
                dp[i] += dp[i - j];
            }
        }
    }

    dp[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n_1() {
        assert_eq!(climb_stairs(1), 1);
    }

    #[test]
    fn test_n_2() {
        assert_eq!(climb_stairs(2), 2);
    }

    #[test]
    fn test_n_3() {
        // ways(3) = 3: (1+1+1), (1+2), (2+1)
        assert_eq!(climb_stairs(3), 3);
    }

    #[test]
    fn test_n_4() {
        // ways(4) = 5: (1+1+1+1), (1+1+2), (1+2+1), (2+1+1), (2+2)
        assert_eq!(climb_stairs(4), 5);
    }

    #[test]
    fn test_n_5() {
        // ways(5) = 8
        assert_eq!(climb_stairs(5), 8);
    }

    #[test]
    fn test_n_6() {
        // ways(6) = 13
        assert_eq!(climb_stairs(6), 13);
    }

    #[test]
    fn test_n_10() {
        // ways(10) = 89
        assert_eq!(climb_stairs(10), 89);
    }

    #[test]
    fn test_n_20() {
        // ways(20) = 10946
        assert_eq!(climb_stairs(20), 10946);
    }

    #[test]
    fn test_n_44() {
        // ways(44) fits in i32
        assert_eq!(climb_stairs(44), 1134903170);
    }

    #[test]
    fn test_n_45() {
        // ways(45) exceeds i32 but fits in i64
        assert_eq!(climb_stairs_i64(45), 1836311903);
    }

    #[test]
    fn test_recursive_same_as_iterative() {
        for n in 1..=20 {
            let iter = climb_stairs(n);
            let rec = climb_stairs_recursive(n);
            assert_eq!(iter, rec, "Failed for n={}", n);
        }
    }

    #[test]
    fn test_k_1() {
        // Only 1-step allowed
        assert_eq!(climb_stairs_k(5, 1), 1);
    }

    #[test]
    fn test_k_2() {
        // 1 or 2 steps allowed (original problem)
        assert_eq!(climb_stairs_k(4, 2), 5);
    }

    #[test]
    fn test_k_3() {
        // 1, 2, or 3 steps allowed
        // ways(4) with k=3: (1+1+1+1), (1+1+2), (1+2+1), (2+1+1), (1+3), (3+1), (2+2) = 7
        assert_eq!(climb_stairs_k(4, 3), 7);
    }

    #[test]
    fn test_k_4() {
        // 1, 2, 3, or 4 steps allowed
        // ways(4) with k=4: all combos including 4 = 8
        assert_eq!(climb_stairs_k(4, 4), 8);
    }

    #[test]
    fn test_fibonacci_sequence() {
        // Verify we produce Fibonacci numbers
        assert_eq!(climb_stairs(1), 1);    // F1
        assert_eq!(climb_stairs(2), 2);    // F2
        assert_eq!(climb_stairs(3), 3);    // F3
        assert_eq!(climb_stairs(4), 5);    // F4
        assert_eq!(climb_stairs(5), 8);    // F5
        assert_eq!(climb_stairs(6), 13);   // F6
        assert_eq!(climb_stairs(7), 21);   // F7
        assert_eq!(climb_stairs(8), 34);   // F8
    }

    #[test]
    fn test_large_n() {
        // Verify i64 version works for larger values
        assert_eq!(climb_stairs_i64(50), 20365011074);
    }

    #[test]
    fn test_zero_steps() {
        // 0 steps - 1 way (do nothing)
        assert_eq!(climb_stairs_k(0, 2), 1);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("039_climbing_stairs_lc70 exercises - run tests with cargo test");
}
