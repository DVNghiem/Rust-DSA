/// Approach 1: Bottom-up DP (optimal)
///
/// dp[i] = minimum coins to make amount i
/// dp[0] = 0, others = INF initially
/// For each coin, try to use it and minimize
pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    if amount == 0 {
        return 0;
    }

    let amount = amount as usize;
    let mut dp = vec![amount + 1; amount + 1];
    dp[0] = 0;

    for i in 1..=amount {
        for &coin in &coins {
            let coin = coin as usize;
            if coin <= i {
                dp[i] = dp[i].min(dp[i - coin] + 1);
            }
        }
    }

    if dp[amount] > amount as usize {
        -1
    } else {
        dp[amount] as i32
    }
}

/// Approach 2: Top-down recursive with memoization
pub fn coin_change_recursive(coins: Vec<i32>, amount: i32) -> i32 {
    fn helper(amount: usize, coins: &[i32], memo: &mut Vec<i32>) -> i32 {
        if amount == 0 {
            return 0;
        }
        if amount < memo.len() && memo[amount] != -1 {
            return memo[amount];
        }

        let mut min_coins = i32::MAX;
        for &coin in coins {
            let coin = coin as usize;
            if coin <= amount {
                let sub_result = helper(amount - coin, coins, memo);
                if sub_result != -1 {
                    min_coins = min_coins.min(sub_result + 1);
                }
            }
        }

        let result = if min_coins == i32::MAX { -1 } else { min_coins };
        if amount < memo.len() {
            memo[amount] = result;
        }
        result
    }

    if amount == 0 {
        return 0;
    }
    let mut memo = vec![-1; (amount + 1) as usize];
    helper(amount as usize, &coins, &mut memo)
}

/// Approach 3: BFS (find minimum number of coins)
/// Actually BFS is for "can we make amount" not "minimum coins"
/// For minimum coins, DP is better.

/// Approach 4: Return which coins used (reconstruction)
pub fn coin_change_with_coins(coins: Vec<i32>, amount: i32) -> (i32, Vec<i32>) {
    if amount == 0 {
        return (0, vec![]);
    }

    let amount = amount as usize;
    let mut dp = vec![amount + 1; amount + 1];
    let mut parent = vec![-1isize; amount + 1]; // Track which coin was used
    dp[0] = 0;

    for i in 1..=amount {
        for (coin_idx, &coin) in coins.iter().enumerate() {
            let coin = coin as usize;
            if coin <= i && dp[i - coin] + 1 < dp[i] {
                dp[i] = dp[i - coin] + 1;
                parent[i] = coin_idx as isize;
            }
        }
    }

    if dp[amount] > amount {
        return (-1, vec![]);
    }

    // Reconstruct coins used
    let mut coins_used = Vec::new();
    let mut remaining = amount;
    while remaining > 0 {
        let coin_idx = parent[remaining] as usize;
        coins_used.push(coins[coin_idx]);
        remaining -= coins[coin_idx] as usize;
    }

    (dp[amount] as i32, coins_used)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        // coins = [1, 2, 5], amount = 11
        // 5 + 5 + 1 = 11 → 3 coins
        let result = coin_change(vec![1, 2, 5], 11);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_amount_zero() {
        assert_eq!(coin_change(vec![1, 2, 5], 0), 0);
    }

    #[test]
    fn test_no_solution() {
        // amount = 3, coins = [2]
        // Can only make 2, 4, 6... not 3
        let result = coin_change(vec![2], 3);
        assert_eq!(result, -1);
    }

    #[test]
    fn test_single_coin_type() {
        // amount = 10, coins = [5]
        // Need 2 coins of 5
        let result = coin_change(vec![5], 10);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_large_amount() {
        // amount = 10000, test efficiency
        let result = coin_change(vec![1, 2, 5], 10000);
        assert_eq!(result, 10000 / 5 + 10000 % 5 / 2 + 10000 % 5 % 2);
    }

    #[test]
    fn test_coin_1() {
        // Only coin is 1, always works
        let result = coin_change(vec![1], 123);
        assert_eq!(result, 123);
    }

    #[test]
    fn test_greedy_fails() {
        // coins = [1, 5, 9, 11], amount = 14
        // Greedy (11 + 1 + 1 + 1 = 4) is wrong
        // Optimal is 5 + 5 + 1 + 1 + 1 + 1 = 6? Wait
        // Actually 9 + 5 = 2 coins. Greedy gives 11 + 1 + 1 + 1 = 4.
        // But 5 + 5 + 1 + 1 + 1 + 1 = 6 coins
        // So 9 + 5 = 2 coins is optimal
        let result = coin_change(vec![1, 5, 9, 11], 14);
        assert_eq!(result, 2); // 9 + 5
    }

    #[test]
    fn test_same_result_recursive() {
        for amount in 0..50 {
            let dp = coin_change(vec![1, 2, 5], amount);
            let rec = coin_change_recursive(vec![1, 2, 5], amount);
            assert_eq!(dp, rec, "Failed for amount {}", amount);
        }
    }

    #[test]
    fn test_with_coins_basic() {
        let (count, coins) = coin_change_with_coins(vec![1, 2, 5], 11);
        assert_eq!(count, 3);
        // Sum should be 11
        let sum: i32 = coins.iter().sum();
        assert_eq!(sum, 11);
    }

    #[test]
    fn test_with_coins_zero() {
        let (count, coins) = coin_change_with_coins(vec![1, 2, 5], 0);
        assert_eq!(count, 0);
        assert!(coins.is_empty());
    }

    #[test]
    fn test_no_solution_with_coins() {
        let (count, _) = coin_change_with_coins(vec![2], 3);
        assert_eq!(count, -1);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(coin_change(vec![], 1), -1);
        assert_eq!(coin_change(vec![1], 0), 0);
    }

    #[test]
    fn test_various_coins() {
        let test_cases = vec![
            (vec![1], 0, 0),
            (vec![1], 1, 1),
            (vec![1], 100, 100),
            (vec![2], 1, -1),
            (vec![2], 2, 1),
            (vec![2], 4, 2),
            (vec![1, 2, 5], 11, 3),
            (vec![186, 419, 83, 408], 6249, 20),
        ];

        for (coins, amount, expected) in test_cases {
            let result = coin_change(coins, amount);
            assert_eq!(result, expected, "Failed: coins={:?}, amount={}", coins, amount);
        }
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("Coin Change exercises - run tests with cargo test");
}