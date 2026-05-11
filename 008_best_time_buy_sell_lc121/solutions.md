# Solutions: Best Time to Buy and Sell Stock (LeetCode #121)

This document provides detailed, line-by-line analysis of each exercise implementation.

---

## Exercise 1: Best Time to Buy and Sell Stock

### The Solution

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

### Line-by-Line Analysis

```rust
let mut min_price = i32::MAX;
```
**Purpose:** Initialize minimum price to the maximum possible value. This ensures the first price will become the new minimum.

```rust
let mut max_profit = 0;
```
**Purpose:** Initialize maximum profit to 0. A profit of 0 or greater is the minimum acceptable result (no profit case).

```rust
for &price in prices {
```
**Purpose:** Iterate through each day's price.

```rust
if price < min_price {
    min_price = price;
}
```
**Purpose:** If current price is lower than any price seen before, update the minimum price. This represents the best day to buy so far.

```rust
else if price - min_price > max_profit {
    max_profit = price - min_price;
}
```
**Purpose:** If current price minus minimum price gives a better profit, update the maximum profit. This represents selling today.

### Why O(n)?

We only iterate through the array once. All operations inside the loop are O(1). Total: O(n) time, O(1) space.

### Key Insight

We must buy before we sell. By tracking the minimum price encountered so far, we guarantee that when we calculate `price - min_price`, we're computing a valid buy-sell pair where buy happened before sell.

---

## Exercise 2: Best Time with Transaction Fee

### The Solution

```rust
pub fn max_profit_with_fee(prices: &[i32], fee: i32) -> i32 {
    let mut min_price = i32::MAX;
    let mut max_profit = 0;

    for &price in prices {
        if price < min_price {
            min_price = price;
        }

        let profit = price - min_price - fee;
        if profit > max_profit {
            max_profit = profit;
        }
    }
    max_profit
}
```

### Key Difference

The profit calculation subtracts the transaction fee. We need to track when selling becomes profitable after accounting for the fee.

Note: This is a simplified version. A more sophisticated approach would handle multiple transactions with fee deducted each time.

---

## Exercise 3: Best Time with Cooldown

### The Solution

```rust
pub fn max_profit_with_cooldown(prices: &[i32]) -> i32 {
    let mut max_profit = 0;
    let mut min_price = i32::MAX;
    let mut prev_price = i32::MAX;
    let mut cooldown = false;

    for &price in prices {
        if cooldown {
            cooldown = false;
            // After cooldown, we can buy but skip the previous price as buy point
            if price < min_price {
                min_price = price;
            }
            continue;
        }

        if price < min_price {
            min_price = price;
        }

        let profit = price - min_price;
        if profit > max_profit {
            max_profit = profit;
            cooldown = true;
            prev_price = price;
        }
    }
    max_profit
}
```

### Key Insight

After selling, we can't buy the next day. The cooldown mechanic means we need to track whether we just sold and should skip the next potential buy.

---

## Exercise 4: Find Buy and Sell Days

### The Solution

```rust
pub fn find_buy_sell_days(prices: &[i32]) -> Option<(usize, usize)> {
    let mut min_price = i32::MAX;
    let mut max_profit = 0;
    let mut buy_day = 0;
    let mut sell_day = 0;

    for (i, &price) in prices.iter().enumerate() {
        if price < min_price {
            min_price = price;
            buy_day = i;
        } else if price - min_price > max_profit {
            max_profit = price - min_price;
            sell_day = i;
        }
    }

    if max_profit > 0 {
        Some((buy_day, sell_day))
    } else {
        None
    }
}
```

### Key Difference

We track the actual indices along with prices. When we find a better profit, we record both the buy_day and sell_day.

---

## Exercise 5: Maximum Profit with k Transactions

### The Solution

```rust
pub fn max_profit_k_transactions(prices: &[i32], k: i32) -> i32 {
    if prices.is_empty() || k == 0 {
        return 0;
    }

    let n = prices.len();
    if k >= n / 2 {
        // With unlimited transactions equivalent
        let mut profit = 0;
        for i in 1..n {
            profit += (prices[i] - prices[i - 1]).max(0);
        }
        return profit;
    }

    let mut dp = vec![vec![0; n]; (k as usize) + 1];
    for t in 1..=k as usize {
        let mut max_diff = i32::MIN;
        for d in 1..n {
            max_diff = max_diff.max(dp[t - 1][d - 1] - prices[d - 1]);
            dp[t][d] = dp[t][d - 1].max(prices[d] + max_diff);
        }
    }
    dp[k as usize][n - 1]
}
```

### Dynamic Programming Approach

- `dp[t][d]` = maximum profit with at most t transactions on day d
- For each transaction, we either don't do a transaction on day d, or we sell on day d after buying on some earlier day

---

## Summary Table

| Exercise | Time | Space | Key Technique |
|----------|------|-------|---------------|
| 1: Basic | O(n) | O(1) | Track min price |
| 2: With Fee | O(n) | O(1) | Subtract fee |
| 3: Cooldown | O(n) | O(1) | State tracking |
| 4: Find Days | O(n) | O(1) | Track indices |
| 5: K Trans | O(n*k) | O(k) | DP |
| 6: Unlimited | O(n) | O(1) | Greedy |
| 7: Difference | O(n) | O(1) | Track min |
| 8: Two Stocks | O(n+m) | O(1) | Sum profits |

## Key Takeaways

1. **Track minimum price** to know the best buy day
2. **Calculate profit** as current price minus minimum price
3. **Update maximum profit** when a better profit is found
4. **Single pass** is sufficient for one transaction
5. **Multiple transactions** require dynamic programming
