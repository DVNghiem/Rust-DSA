# Climbing Stairs - LeetCode 70

## Problem Statement

You are climbing a staircase. It takes `n` steps to reach the top.

Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?

## Visual Walkthrough

```
Example: n = 4

Ways to climb 4 steps:
1. 1 + 1 + 1 + 1 = 4
2. 1 + 1 + 2 = 4
3. 1 + 2 + 1 = 4
4. 2 + 1 + 1 = 4
5. 2 + 2 = 4

Total: 5 ways

Visual tree:
         (start)
         /     \
        1       2
       / \     / \
      1   2   1   2
     / \ |   |   / \
    ... ... ... ...

At step 4, count = 5
```

### Fibonacci Connection

```
Let ways(n) = number of ways to reach step n

To reach step n:
- From step n-1, take 1 step
- From step n-2, take 2 steps

ways(n) = ways(n-1) + ways(n-2)

Base cases:
- ways(1) = 1 (only 1)
- ways(2) = 2 (1+1 or 2)

This is Fibonacci sequence!
ways(3) = ways(2) + ways(1) = 2 + 1 = 3
ways(4) = ways(3) + ways(2) = 3 + 2 = 5
ways(5) = ways(4) + ways(3) = 5 + 3 = 8
```

## Approach Comparison

| Approach | Time | Space | Description |
|----------|------|-------|-------------|
| Recursive | O(2^n) | O(n) | Exponential, many repeated subproblems |
| Memoization | O(n) | O(n) | Store computed values |
| DP Iterative | O(n) | O(1) | Only need previous two values |
| Matrix Exp | O(log n) | O(1) | Mathematical, uses Fibonacci matrix |

## Implementation Strategy

### DP with Two Variables

```rust
pub fn climb_stairs(n: i32) -> i32 {
    if n <= 2 { return n as i32; }
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

## Edge Cases

1. **n = 1**: Return 1
2. **n = 2**: Return 2
3. **n = 0**: Return 1 (by definition, or handle separately)
4. **Large n**: Use i64 or handle overflow

## Test Cases

1. Basic n = 1, 2, 3, 4, 5
2. Large n
3. Edge cases
4. Fib sequence verification

## Solution Explanation

### Key Insight

Climbing stairs is equivalent to Fibonacci. To reach step n, you either came from step n-1 (1 step) or step n-2 (2 steps). Thus ways(n) = ways(n-1) + ways(n-2).

### Space Optimization

Only need previous two values, not full array.

## Complexity Analysis

- **Time**: O(n)
- **Space**: O(1)

## Follow-up Questions

1. How to generalize for steps of 1, 2, or 3?
2. What if you have different costs for each step type?