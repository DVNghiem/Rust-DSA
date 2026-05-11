# Burst Balloons - LeetCode 312

## Problem Statement

Given `n` balloons, indexed from 0 to n-1. Each balloon is assigned a minimum coin reward based on its neighboring balloons at burst time.

```
Example:
nums = [3, 1, 5, 8]

If we burst balloon i, we get:
coins = nums[left] * nums[i] * nums[right]

where left and right are adjacent balloons (or boundaries at index -1 and n).
```

Find the maximum coins you can collect by bursting all balloons optimally.

## Understanding the Problem

When you burst a balloon, you earn coins based on its neighbors at that moment. The key insight is that bursting order matters!

```
Burst order example:
nums = [3, 1, 5, 8]

Option 1: Burst in order 1, 2, 3, 0
Option 2: Burst in order 0, 2, 1, 3
...many other orders
```

The best order maximizes coins earned.

## Visual Walkthrough

```
nums = [3, 1, 5, 8]
Assume boundaries: nums[-1] = 1, nums[n] = 1

Bursting balloon 2 (value 5) when neighbors are 1 and 8:
coins = 1 * 5 * 8 = 40

Then bursting balloon 1 (value 1) with neighbors 1 and 8:
coins = 1 * 1 * 8 = 8

But wait, balloon 2 is gone, so neighbors change!
```

### Key Insight: Intervals

We can think of this as interval DP:
- First burst the balloons at the edges of an interval
- Then burst the last balloon in the center

## Four Approaches

| Approach | Time | Space | Description |
|----------|------|-------|-------------|
| Interval DP | O(n³) | O(n²) | Divide and conquer on intervals |
| DFS + Memoization | O(n³) | O(n²) | Recursive with cache |
| Divide and Conquer | O(n³) | O(n²) | Bottom-up interval DP |
| Optimized DP | O(n²) | O(n²) | Space optimization |

## Interval DP Approach

### The Core Idea

For any interval (i, j), we want to find the maximum coins when:
- Balloons between i and j are already burst
- Only balloons at i and j remain (as boundaries)

We choose which balloon k to burst LAST in (i, j).

If we burst k last, then (i, k) and (k, j) are already optimized!

```
Interval (i, j):
├── (i, k): already burst optimally
├── k: last to burst
└── (k, j): already burst optimally

coins = dp[i][k] + dp[k][j] + nums[i] * nums[k] * nums[j]
```

### Base Case

When i + 1 = j (adjacent or same), interval has no balloon to burst:
```
dp[i][j] = 0  // no balloons in this interval
```

### Transition

```rust
for len in 2..=n+1 {  // len = interval length + 1 (for boundaries)
    for i in 0..n - len + 1 {
        let j = i + len;
        for k in i + 1..j {
            dp[i][j] = max(dp[i][j], dp[i][k] + dp[k][j] + nums[i] * nums[k] * nums[j]);
        }
    }
}
```

## Example Walkthrough

```
nums = [3, 1, 5, 8], with virtual boundaries 1 at both ends
nums_with_bounds = [1, 3, 1, 5, 8, 1]

Intervals (i, j):
- (0, 2): between indices 0 and 2, balloon at index 1 (value 3)
- (0, 3): between 0 and 3, balloons at 1, 2
- etc.

Let dp[i][j] = max coins from interval (i, j)

Base: dp[i][i+1] = 0 (no balloons between adjacent boundaries)

Compute for increasing interval lengths:

len = 2: (0,2), (1,3), (2,4), (3,5)
  dp[0][2] = nums[0] * nums[1] * nums[2] = 1*3*1 = 3
  dp[1][3] = nums[1] * nums[2] * nums[3] = 3*1*5 = 15
  dp[2][4] = nums[2] * nums[3] * nums[4] = 1*5*8 = 40
  dp[3][5] = nums[3] * nums[4] * nums[5] = 5*8*1 = 40

len = 3: (0,3), (1,4), (2,5)
  dp[0][3]: try k=1,2
    k=1: 0 + dp[1][3] + nums[0]*nums[1]*nums[3] = 0 + 15 + 1*3*5 = 15 + 15 = 30
    k=2: dp[0][2] + 0 + nums[0]*nums[2]*nums[3] = 3 + 0 + 1*1*5 = 8
    max = 30
  dp[1][4]: try k=2,3
    k=2: 0 + dp[2][4] + nums[1]*nums[2]*nums[4] = 0 + 40 + 3*1*8 = 64
    k=3: dp[1][3] + 0 + nums[1]*nums[3]*nums[4] = 15 + 0 + 3*5*8 = 135
    max = 135
  ...

len = 4: (0,4)
  dp[0][4]: try k=1,2,3
    k=1: dp[0][1] + dp[1][4] + nums[0]*nums[1]*nums[4] = 0 + 135 + 1*3*8 = 159
    k=2: dp[0][2] + dp[2][4] + nums[0]*nums[2]*nums[4] = 3 + 40 + 1*1*8 = 51
    k=3: dp[0][3] + dp[3][4] + nums[0]*nums[3]*nums[4] = 30 + 0 + 1*5*8 = 70
    max = 159

Final result: dp[0][n+1] = dp[0][5] = 167
```

Answer: 167 coins maximum!

## Complexity Analysis

| Metric | Value |
|--------|-------|
| Time | O(n³) - three nested loops |
| Space | O(n²) - dp table |

## Why O(n³) is Acceptable

- n ≤ 500 (balloon count is limited)
- O(n³) = 125 million operations max, acceptable

## Edge Cases

### Empty or Single Balloon

```rust
nums = []
// No balloons to burst, result = 0

nums = [3]
// One balloon with neighbors 1 and 1
// coins = 1 * 3 * 1 = 3
```

### All Same Values

```rust
nums = [1, 1, 1, 1]
// Still complex - order doesn't matter much
// But DP still works
```

## Implementation Details

```rust
pub fn max_coins(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n == 0 { return 0; }

    // Add virtual boundaries
    let mut extended = vec![1; n + 2];
    for i in 0..n {
        extended[i + 1] = nums[i];
    }

    let n = extended.len();
    let mut dp = vec![vec![0; n]; n];

    // len = interval length (number of balloons considered)
    for len in 2..n {
        for i in 0..n - len {
            let j = i + len;
            for k in i + 1..j {
                dp[i][j] = dp[i][j].max(
                    dp[i][k] + dp[k][j] + extended[i] * extended[k] * extended[j]
                );
            }
        }
    }

    dp[0][n - 1]
}
```

## Related Problems

1. **LeetCode 312**: Burst Balloons (this problem)
2. **LeetCode 1130**: Minimum Score Triangulation of Polygon
3. **LeetCode 1039**: Minimum Score Triangulation of Polygon

## Time to Complete

**Target**: 60 minutes
**Optimal**: 45 minutes