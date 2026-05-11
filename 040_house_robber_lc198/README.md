# House Robber - LeetCode 198

## Problem Statement

You are a professional robber planning to rob houses along a street. Each house has a certain amount of money stashed, and adjacent houses have security systems that will contact the police if two adjacent houses were broken into on the same night.

Given an integer array `nums` representing the amount of money of each house, return the maximum amount you can rob tonight without alerting the police.

## Visual Walkthrough

```
Example:
nums = [2, 7, 9, 3, 1]

Maximum without robbing adjacent:
Pick indices 0, 2, 4 → 2 + 9 + 1 = 12

Alternative: Pick 1, 3 → 7 + 3 = 10 (less)

Houses:   [2] [7] [9] [3] [1]
Indices:    0   1   2   3   4

Robbed:    X   -   X   -   X
Money:     2       9       1 = 12 ✓
```

### DP State Machine

```
dp[i] = maximum money we can rob from houses 0 to i

At house i:
- Don't rob: dp[i] = dp[i-1]
- Rob: dp[i] = nums[i] + dp[i-2] (can't rob i-1)

dp[i] = max(dp[i-1], nums[i] + dp[i-2])

Base cases:
dp[-1] = 0 (no houses)
dp[0] = nums[0]
```

## Approach Comparison

| Approach | Time | Space | Description |
|----------|------|-------|-------------|
| DP Array | O(n) | O(n) | Full dp array |
| DP Two Variables | O(n) | O(1) | Only previous two needed |
| Recursive + Memo | O(n) | O(n) | Top-down |

## Implementation Strategy

```rust
pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.is_empty() { return 0; }
    let n = nums.len();

    let mut prev2 = 0;  // dp[-1] = 0
    let mut prev1 = nums[0];  // dp[0] = nums[0]

    for i in 1..n {
        let current = std::cmp::max(prev1, prev2 + nums[i]);
        prev2 = prev1;
        prev1 = current;
    }

    prev1
}
```

## Edge Cases

1. **Empty array**: Return 0
2. **Single house**: Return that house's money
3. **Two houses**: Return max of the two
4. **All same values**: Pick every other house
5. **Alternating high/low**: Rob all high values

## Test Cases

1. Basic robbery
2. Single house
3. Two houses
4. All same values
5. Alternating pattern

## Solution Explanation

### Key Insight

DP[i] = max(DP[I-1] (don't rob), DP[i-2] + nums[i] (rob))

Only need previous two values, not full array.

## Complexity Analysis

- **Time**: O(n)
- **Space**: O(1)

## Follow-up Questions

1. How to return which houses were robbed?
2. What if houses are in a circle (LeetCode 213)?