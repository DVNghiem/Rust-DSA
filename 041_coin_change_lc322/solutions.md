# Coin Change - Solution Analysis

## Problem Overview

Given coin denominations and target amount, find minimum coins needed. Each coin can be used unlimited times. Return -1 if impossible.

## Solution: Bottom-up DP

### Code Implementation

```rust
pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    if amount == 0 { return 0; }

    let amount = amount as usize;
    let mut dp = vec![amount + 1; amount + 1]; // INF = amount + 1
    dp[0] = 0;

    for i in 1..=amount {
        for &coin in &coins {
            let coin = coin as usize;
            if coin <= i {
                dp[i] = dp[i].min(dp[i - coin] + 1);
            }
        }
    }

    if dp[amount] > amount as usize { -1 } else { dp[amount] as i32 }
}
```

### Line-by-Line Analysis

1. **`if amount == 0 { return 0; }`**: Zero amount needs zero coins.

2. **`let mut dp = vec![amount + 1; amount + 1];`**: Initialize dp array. Amount + 1 is safe "infinity" (greater than any possible answer).

3. **`dp[0] = 0;`**: Base case - zero amount needs zero coins.

4. **`for i in 1..=amount { ... }`**: Iterate through all amounts.

5. **`if coin <= i { dp[i] = dp[i].min(dp[i - coin] + 1); }`**: Try using this coin. If coin fits in amount, dp[i] = min(current, dp[i-coin] + 1).

6. **`if dp[amount] > amount as usize { -1 } else { dp[amount] as i32 }`**: If dp[amount] still INF, impossible. Return -1 or result.

### DP Table Visualization

```
coins = [1, 2, 5], amount = 11

dp[0] = 0
dp[1] = min(dp[1-1]+1) = dp[0]+1 = 1
dp[2] = min(dp[2-1]+1, dp[2-2]+1) = min(2, 1) = 1
dp[3] = min(dp[3-1]+1, dp[3-2]+1) = min(2, 2) = 2
dp[4] = min(dp[4-1]+1, dp[4-2]+1) = min(3, 2) = 2
dp[5] = min(dp[5-1]+1, dp[5-2]+1, dp[5-5]+1) = min(3, 3, 1) = 1
dp[6] = min(dp[6-1]+1, dp[6-2]+1, dp[6-5]+1) = min(2, 2, 2) = 2
...
dp[11] = min(dp[10]+1, dp[9]+1, dp[6]+1) = min(3, 3, 3) = 3

Result: 3 (5+5+1 or 2+2+2+2+2+1) ✓
```

## Recursive with Memoization

```rust
fn helper(amount: usize, coins: &[i32], memo: &mut Vec<i32>) -> i32 {
    if amount == 0 { return 0; }
    if memo[amount] != -1 { return memo[amount]; }

    let mut min_coins = i32::MAX;
    for &coin in coins {
        if coin as usize <= amount {
            let sub = helper(amount - coin as usize, coins, memo);
            if sub != -1 {
                min_coins = min_coins.min(sub + 1);
            }
        }
    }

    let result = if min_coins == i32::MAX { -1 } else { min_coins };
    memo[amount] = result;
    result
}
```

## Reconstruction: Which Coins Used

```rust
pub fn coin_change_with_coins(coins: Vec<i32>, amount: i32) -> (i32, Vec<i32>) {
    let mut dp = vec![amount + 1; amount + 1];
    let mut parent = vec![-1isize; amount + 1]; // Track coin index

    for i in 1..=amount {
        for (coin_idx, &coin) in coins.iter().enumerate() {
            if coin as usize <= i && dp[i - coin as usize] + 1 < dp[i] {
                dp[i] = dp[i - coin as usize] + 1;
                parent[i] = coin_idx as isize;
            }
        }
    }

    // Backtrack from amount
    let mut coins_used = Vec::new();
    let mut remaining = amount as usize;
    while remaining > 0 {
        let coin_idx = parent[remaining] as usize;
        coins_used.push(coins[coin_idx]);
        remaining -= coins[coin_idx] as usize;
    }

    (dp[amount as usize] as i32, coins_used)
}
```

## Complexity Comparison

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Bottom-up | O(n×amount) | O(amount) | Iterative, no stack overflow |
| Top-down | O(n×amount) | O(amount) | Recursive, memoized |

## Key Insights

1. **Unbounded knapsack**: Each coin can be used unlimited times.

2. **dp[i] = min(dp[i], dp[i-coin] + 1)**: Try all coins, minimize.

3. **INF = amount + 1**: Safe because max coins needed is amount (all 1s).

## Test Case Analysis

### Test: `test_greedy_fails`

```
coins = [1, 5, 9, 11], amount = 14

Greedy would pick:
11 → remaining 3 (need 3 ones) → total 4 coins
But optimal is 9 + 5 = 2 coins

DP finds optimal:
dp[14] = min(dp[14-1]+1, dp[14-5]+1, dp[14-9]+1, dp[14-11]+1)
       = min(dp[13]+1, dp[9]+1, dp[5]+1, dp[3]+1)
       = min(?, 2+1, 1+1, ?)
       = min(?, 3, 2, ?) = 2

Result: 2 coins (9 + 5) ✓
```

## Follow-up Answers

**Q: Why is greedy wrong?**
A: Because coins aren't canonical (like US coin system). With [9, 11], greedy picks 11 but optimal is 9+5.

**Q: Why INF = amount + 1?**
A: Maximum coins possible is amount (all 1s). So amount+1 is safe "never reachable" value.

**Q: Can we use BFS?**
A: BFS would find minimum steps, but DP is simpler and doesn't need visited tracking for every amount.