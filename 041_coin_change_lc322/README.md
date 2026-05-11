# Coin Change - LeetCode 322

## Problem Statement

You are given an integer array `coins` representing coins of different denominations and an integer `amount` representing a total amount of money.

Return the fewest number of coins that you need to make up that amount. If that amount of money cannot be made up, return -1.

You have an infinite number of each coin denomination.

## Visual Walkthrough

```
Example:
coins = [1, 2, 5], amount = 11

Minimum coins: 3 (5 + 5 + 1 = 11)

DP table for amount 0 to 11:
amount:  0  1  2  3  4  5  6  7  8  9 10 11
coins:   0  1  2  3  4  1  2  3  4  5  2  3
```

### DP State

```
dp[i] = minimum coins to make amount i

For each coin c:
  if c <= i:
    dp[i] = min(dp[i], dp[i - c] + 1)

dp[0] = 0 (0 coins for amount 0)
dp[i] = INF for i > 0 initially
```

## Approach Comparison

| Approach | Time | Space | Description |
|----------|------|-------|-------------|
| Bottom-up DP | O(n × amount) | O(amount) | Iterate amounts |
| Top-down Memo | O(n × amount) | O(amount) | Recursive with cache |
| BFS (coin change 2) | O(amount) | O(amount) | For count problems |

## Implementation Strategy

```rust
pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let amount = amount as usize;
    let mut dp = vec![amount + 1; amount + 1];
    dp[0] = 0;

    for i in 1..=amount {
        for &coin in &coins {
            if coin as usize <= i {
                dp[i] = dp[i].min(dp[i - coin as usize] + 1);
            }
        }
    }

    if dp[amount] > amount as i32 { -1 } else { dp[amount] as i32 }
}
```

## Edge Cases

1. **amount = 0**: Return 0
2. **No coins**: Return -1
3. **Cannot make amount**: Return -1
4. **Large amount**: Efficient DP

## Test Cases

1. Basic case
2. amount = 0
3. No solution
4. Single coin type
5. Large amount

## Solution Explanation

### Key Insight

Unbounded knapsack - each coin can be used infinite times. dp[i] = min(dp[i], dp[i-coin] + 1).

## Complexity Analysis

- **Time**: O(n × amount)
- **Space**: O(amount)

## Follow-up Questions

1. How to return which coins used?
2. What if we need all combinations?