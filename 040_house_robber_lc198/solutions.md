# House Robber - Solution Analysis

## Problem Overview

Given array of house values, rob maximum without robbing two adjacent houses.

## Solution: DP with O(1) Space

### Code Implementation

```rust
pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.is_empty() { return 0; }
    let n = nums.len();
    if n == 1 { return nums[0]; }

    let mut prev2 = 0;  // dp[i-2]
    let mut prev1 = nums[0];  // dp[i-1] = dp[0]

    for i in 1..n {
        let current = std::cmp::max(prev1, prev2 + nums[i]);
        prev2 = prev1;
        prev1 = current;
    }

    prev1
}
```

### Line-by-Line Analysis

1. **`if nums.is_empty() { return 0; }`**: Edge case.
2. **`if n == 1 { return nums[0]; }`**: Only one house, must rob it.
3. **`let mut prev2 = 0; let mut prev1 = nums[0];`**: Initialize dp[-1]=0, dp[0]=nums[0].
4. **`for i in 1..n { ... }`**: Start from house 1 (already processed house 0).
5. **`let current = std::cmp::max(prev1, prev2 + nums[i]);`**: Either don't rob (dp[i-1]=prev1) or rob (dp[i-2]+nums[i]=prev2+nums[i]).
6. **`prev2 = prev1; prev1 = current;`**: Slide window for next iteration.

### DP Visualization

```
nums = [2, 7, 9, 3, 1]

i=0: prev1 = 2 (rob house 0)
i=1: current = max(2, 0+7) = 7
     prev2=2, prev1=7 (rob house 1 instead of 0)
i=2: current = max(7, 2+9) = 11
     prev2=7, prev1=11 (rob house 2 and 0)
i=3: current = max(11, 7+3) = 11
     prev2=11, prev1=11 (don't rob house 3)
i=4: current = max(11, 11+1) = 12
     prev2=11, prev1=12

Result: 12 ✓ (rob houses 0, 2, 4)
```

## Alternative: Full DP Array

```rust
pub fn rob_with_array(nums: Vec<i32>) -> i32 {
    if nums.is_empty() { return 0; }
    let n = nums.len();

    let mut dp = vec![0i32; n];
    dp[0] = nums[0];
    dp[1] = std::cmp::max(nums[0], nums[1]);

    for i in 2..n {
        dp[i] = std::cmp::max(dp[i - 1], dp[i - 2] + nums[i]);
    }

    dp[n - 1]
}
```

## Complexity Comparison

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| DP O(1) | O(n) | O(1) | Optimal |
| DP O(n) | O(n) | O(n) | Full array |
| Recursive + Memo | O(n) | O(n) | Top-down |

## Key Insights

1. **DP recurrence**: dp[i] = max(dp[i-1], dp[i-2] + nums[i])
2. **Two states only**: Previous two dp values needed
3. **Slide window**: Update prev2, prev1 each iteration

## Test Case Analysis

### Test: `test_basic`

```
nums = [2, 7, 9, 3, 1]

dp[0] = 2
dp[1] = max(2, 7) = 7
dp[2] = max(7, 2+9=11) = 11
dp[3] = max(11, 7+3=10) = 11
dp[4] = max(11, 11+1=12) = 12

Result: 12 ✓
Robbed houses: 0, 2, 4
```

### Test: `test_alternating_high_low`

```
nums = [1, 2, 3, 1]

dp[0] = 1
dp[1] = max(1, 2) = 2
dp[2] = max(2, 1+3=4) = 4
dp[3] = max(4, 2+1=3) = 4

Result: 4 ✓
Robbed: houses 0 and 2 (or 1 and 3)
```

## Follow-up Answers

**Q: Why two variables suffice?**
A: dp[i] only depends on dp[i-1] and dp[i-2]. Older values not needed.

**Q: How to reconstruct which houses?**
A: Track decision at each step (skip or take), then backtrack from dp[n-1].

**Q: What about circular houses (LeetCode 213)?**
A: Run twice - once excluding last house, once excluding first house, return max.