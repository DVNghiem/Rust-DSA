# Best Time to Buy and Sell Stock (LeetCode #121)

## Problem Statement

You are given an array `prices` where `prices[i]` is the price of a given stock on the ith day.

You want to maximize your profit by choosing a single day to buy one stock and choosing a different day in the future to sell that stock.

Return the maximum profit. If you cannot achieve any profit, return 0.

## Examples

```
Input: prices = [7, 1, 5, 3, 6, 4]
Output: 5
Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6 - 1 = 5.

Input: prices = [7, 6, 4, 3, 1]
Output: 0
Explanation: No profitable transaction is possible.
```

## Approaches Overview

### Approach 1: Brute Force O(n²)
Check all buy-sell pairs.

```rust
pub fn max_profit_brute(prices: &[i32]) -> i32 {
    let mut max_profit = 0;
    for i in 0..prices.len() {
        for j in (i + 1)..prices.len() {
            let profit = prices[j] - prices[i];
            if profit > max_profit {
                max_profit = profit;
            }
        }
    }
    max_profit
}
```

### Approach 2: One Pass O(n) - Preferred
Track minimum price seen so far and maximum profit.

```rust
pub fn max_profit(prices: &[i32]) -> i32 {
    let mut min_price = i32::MAX;
    let mut max_profit = 0;

    for &price in prices {
        if price < min_price {
            min_price = price;
        } else if price - min_price > max_profit {
            max_profit = price - min_price;
        }
    }
    max_profit
}
```

## The Key Insight

For each day, we need to know:
1. The minimum price on any day BEFORE this day
2. The maximum profit if we sell on THIS day

We can track both in a single pass.

## Visual Walkthrough: One Pass

```
prices = [7, 1, 5, 3, 6, 4]

Step 1: price = 7
  min_price = 7 (new minimum)
  max_profit = 0 (no profit yet)

Step 2: price = 1
  min_price = 1 (new minimum!)
  max_profit = 0

Step 3: price = 5
  profit = 5 - 1 = 4 > max_profit
  max_profit = 4 (best profit if sell here)

Step 4: price = 3
  profit = 3 - 1 = 2 < max_profit
  max_profit stays 4

Step 5: price = 6
  profit = 6 - 1 = 5 > max_profit
  max_profit = 5 (best profit so far!)

Step 6: price = 4
  profit = 4 - 1 = 3 < max_profit
  max_profit stays 5

Return max_profit = 5

BUY on day 2 (price = 1), SELL on day 5 (price = 6)
```

## Complexity Analysis Table

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Brute Force | O(n²) | O(1) | Check all pairs |
| One Pass | O(n) | O(1) | Track min and max |

## Edge Cases to Consider

1. **Empty array**: Return 0
2. **Single element**: Return 0 (can't sell after buy)
3. **Decreasing prices**: Return 0 (no profitable transaction)
4. **Same price all days**: Return 0
5. **Large profit possible**: Handle integer overflow with i32

## Related Problems

### LeetCode 122: Best Time to Buy and Sell Stock II
Can make unlimited transactions.

### LeetCode 123: Best Time to Buy and Sell Stock III
Can make at most 2 transactions.

### LeetCode 188: Best Time to Buy and Sell Stock IV
Can make at most k transactions.

### LeetCode 714: Best Time to Buy and Sell Stock with Transaction Fee
Infinite transactions with fees.

## Exercises

### Exercise 1: Basic Best Time to Buy Sell
Implement one-pass solution.

### Exercise 2: Best Time with Transaction Fee
Maximum profit with transaction fee.

### Exercise 3: Best Time with Cooldown
Maximum profit with 1-day cooldown after selling.

### Exercise 4: Find Buy and Sell Days
Return the actual buy and sell days, not just profit.

### Exercise 5: Maximum Profit with k Transactions
Find maximum profit with at most k transactions.

## Key Takeaways

1. **Track minimum price** seen so far
2. **Calculate potential profit** at each step
3. **Update maximum profit** when better is found
4. **Single pass** achieves O(n) time
5. **Handle edge cases** for invalid transactions

## Real-World Applications

1. **Trading algorithms**: Find optimal buy/sell timing
2. **Financial planning**: Maximize investment returns
3. **Resource allocation**: Buy low, sell high strategies
4. **Inventory management**: Optimize purchase timing
5. **Cryptocurrency**: Trading bot strategies
