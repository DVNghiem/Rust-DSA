# Climbing Stairs - Solution Analysis

## Problem Overview

Given n steps, you can climb 1 or 2 steps at a time. Find the number of distinct ways to reach the top.

## Solution: Iterative DP with O(1) Space

### Code Implementation

```rust
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
```

### Line-by-Line Analysis

1. **`if n <= 2 { return n; }`**: Base cases - 1 step has 1 way, 2 steps have 2 ways.

2. **`let mut prev2 = 1; let mut prev1 = 2;`**: Initialize for Fibonacci calculation. prev2 = ways(n-2), prev1 = ways(n-1).

3. **`for i in 3..=n { ... }`**: Start from step 3, compute up to n.

4. **`let current = prev1 + prev2;`**: ways(i) = ways(i-1) + ways(i-2).

5. **`prev2 = prev1; prev1 = current;`**: Slide window forward for next iteration.

6. **`prev1`**: After loop, holds ways(n).

### DP Table Visualization

```
n:     1   2   3   4   5   6   7   8
ways:  1   2   3   5   8  13  21  34

Climb from 4 steps:
- From step 3: take 1 step (ways to reach 3 = 3)
- From step 2: take 2 steps (ways to reach 2 = 2)
Total = 3 + 2 = 5 ✓
```

## Why Fibonacci?

```
To reach step n:
- Last move was either 1 step from (n-1)
- OR last move was 2 steps from (n-2)

ways(n) = ways(n-1) + ways(n-2)

This IS the Fibonacci recurrence.
Starting with ways(1)=1, ways(2)=2:
ways(3) = 2 + 1 = 3
ways(4) = 3 + 2 = 5
ways(5) = 5 + 3 = 8
...
```

## Recursive with Memoization

```rust
fn helper(n: i32, memo: &mut Vec<i64>) -> i64 {
    if n <= 2 { return n as i64; }
    if memo[n as usize] != 0 { return memo[n as usize]; }
    memo[n as usize] = helper(n - 1, memo) + helper(n - 2, memo);
    memo[n as usize]
}
```

- Avoids repeated computation of same subproblems
- Time: O(n), Space: O(n) for memo array + recursion stack

## Generalized for k Steps

```rust
pub fn climb_stairs_k(n: i32, k: i32) -> i64 {
    let n = n as usize;
    let k = k as usize;

    let mut dp = vec![0i64; n + 1];
    dp[0] = 1;
    dp[1] = 1;

    for i in 2..=n {
        for j in 1..=k {
            if i >= j {
                dp[i] += dp[i - j];
            }
        }
    }

    dp[n]
}
```

dp[i] = sum of dp[i-j] for j=1 to k (where i >= j)

## Complexity Comparison

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Naive recursive | O(2^n) | O(n) | Exponential! |
| Memoization | O(n) | O(n) | Stores computed values |
| Iterative DP | O(n) | O(1) | Optimal |
| Matrix exp | O(log n) | O(1) | For large n |

## Key Insights

1. **Fibonacci relationship**: ways(n) = ways(n-1) + ways(n-2)
2. **Only previous two needed**: Can discard older values
3. **Initial conditions matter**: ways(1)=1, ways(2)=2 not 1,1

## Overflow Considerations

- For n=44, ways = 1134903170 (fits in i32)
- For n=45, ways = 1836311903 (still fits, but close to max)
- For n=46+, ways exceeds i32::MAX (2147483647)
- Solution uses i64 for larger n

## Test Case Analysis

### Test: `test_n_4`

```
n=4

Tree of possibilities:
        start
        /    \
       1      2
      / \    / \
     1   2  1   2
    /|   |  |   |
   1 2   1 2   1

Ways:
1. 1+1+1+1
2. 1+1+2
3. 1+2+1
4. 2+1+1
5. 2+2

Total: 5 ✓
```

## Follow-up Answers

**Q: Why base cases are 1 and 2?**
A: ways(1)=1 (only one way: take 1 step), ways(2)=2 (1+1 or just 2). This makes Fibonacci work correctly.

**Q: How to handle step of 3 as well?**
A: Change recurrence to ways(n) = ways(n-1) + ways(n-2) + ways(n-3).

**Q: Can we solve with matrix exponentiation?**
A: Yes, using [[1,1],[1,0]]^n matrix gives Fibonacci in O(log n).

**Q: Relation to coin change?**
A: Similar DP pattern - count ways to reach sum with given denominations (1 and 2 coins here).

**Q: What if we can also climb 3 steps?**
A: ways(n) = ways(n-1) + ways(n-2) + ways(n-3)