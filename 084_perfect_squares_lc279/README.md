# Perfect Squares - LeetCode 279

## Problem Statement

Given an integer n, find the minimum number of perfect square numbers that sum to n.

```
Example 1:
Input: n = 12
Output: 3
Explanation: 12 = 4 + 4 + 4 (3 squares)

Example 2:
Input: n = 13
Output: 2
Explanation: 13 = 4 + 9 (2 squares)
```

## Understanding the Problem

A perfect square is an integer that is the square of an integer (1, 4, 9, 16, 25, ...).

We need to represent n as a sum of perfect squares with minimum count.

### Key Insight

This is a classic DP problem:
- Subproblem: minimum squares for any number from 0 to n
- We can build up from smaller numbers to larger ones

## Visual Walkthrough

```
n = 12

Approach 1: Greedy (doesn't work for all cases)
12 = 9 + 1 + 1 + 1 → 4 squares (NOT optimal!)

Approach 2: DP
dp[0] = 0 (0 squares sum to 0)
dp[1] = 1 (1 = 1)
dp[2] = 2 (1 + 1)
dp[3] = 3 (1 + 1 + 1)
dp[4] = 1 (4)
dp[5] = 2 (4 + 1)
dp[6] = 3 (4 + 1 + 1)
dp[7] = 4 (4 + 1 + 1 + 1)
dp[8] = 2 (4 + 4)
dp[9] = 1 (9)
dp[10] = 2 (9 + 1)
dp[11] = 3 (9 + 1 + 1)
dp[12] = 3 (4 + 4 + 4)

Result: 3
```

## Four Approaches

| Approach | Time | Space | Description |
|----------|------|-------|-------------|
| DP Bottom-up | O(n√n) | O(n) | Iterate, try all squares |
| BFS | O(n × √n) | O(n) | Shortest path in graph |
| Greedy + Math | O(√n) | O(1) | Lagrange's theorem |
| Math (Legendre) | O(√n) | O(1) | Three-square theorem |

## Approach 1: DP Bottom-Up

```rust
pub fn num_squares(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![usize::MAX; n + 1];
    dp[0] = 0;

    for i in 1..=n {
        let mut j = 1;
        let mut min_count = usize::MAX;
        while j * j <= i {
            min_count = min_count.min(dp[i - j * j] + 1);
            j += 1;
        }
        dp[i] = min_count;
    }

    dp[n] as i32
}
```

### DP Transition

```
dp[i] = min(dp[i - j²] + 1) for all j where j² ≤ i

For each number i:
  - Try subtracting each perfect square j²
  - Take minimum of (1 + dp[i - j²])
```

## Approach 2: BFS

Think of numbers as nodes in a graph:

```
     12
    / | \
   11  8  3
  / \  |   \
 ... ... ...
```

Each edge connects a number to (number - perfect square).
Shortest path from n to 0 = minimum squares.

```rust
pub fn num_squares_bfs(n: i32) -> i32 {
    let n = n as usize;
    let mut visited = vec![false; n + 1];
    let mut queue = VecDeque::new();
    queue.push_back(n);
    visited[n] = true;
    let mut depth = 0;

    while !queue.is_empty() {
        let level_size = queue.len();
        depth += 1;
        for _ in 0..level_size {
            let num = queue.pop_front().unwrap();
            let mut i = 1;
            while i * i <= num {
                let next = num - i * i;
                if next == 0 {
                    return depth;
                }
                if !visited[next] {
                    visited[next] = true;
                    queue.push_back(next);
                }
                i += 1;
            }
        }
    }
    0
}
```

## Approach 3: Lagrange's Four Square Theorem

**Key theorem**: Every natural number can be represented as the sum of at most four perfect squares.

```
n = a² + b² + c² + d²  (for some a, b, c, d ≥ 0)
```

So answer is 1, 2, 3, or 4.

We can check in order:
1. Is n a perfect square? → 1
2. Can n be expressed as sum of 2 squares? → 2
3. Can n be expressed as sum of 3 squares? → 3
4. Otherwise → 4

## Approach 4: Math-based 3-square check

For the 3-square check, we use the theorem:
- n is NOT a sum of 3 squares if and only if n = 4^a × (8b + 7)

```rust
pub fn num_squares_math(n: i32) -> i32 {
    let mut n = n;

    // Check 1: perfect square
    if is_perfect_square(n) {
        return 1;
    }

    // Check 2: sum of two squares
    // n = a² + b²  iff there exists a where n - a² is perfect square
    let mut a = 0;
    while a * a <= n {
        let b_squared = n - a * a;
        if is_perfect_square(b_squared) {
            return 2;
        }
        a += 1;
    }

    // Check 3: n = 4^a × (8b + 7) → 4 squares needed
    while n % 4 == 0 {
        n /= 4;
    }
    if n % 8 == 7 {
        return 4;
    }

    // Otherwise 3 squares
    3
}
```

## Complexity Analysis

| Approach | Time | Space |
|----------|------|-------|
| DP Bottom-up | O(n√n) | O(n) |
| BFS | O(n√n) | O(n) |
| Math-based | O(√n) | O(1) |

## Edge Cases

### n = 0

```rust
dp[0] = 0  // 0 squares sum to 0
```

### n = 1

```rust
dp[1] = 1  // 1 = 1²
```

### n = 2

```rust
dp[2] = 2  // 2 = 1 + 1
```

## Test Cases Design

### Basic Cases
1. n = 1 → 1
2. n = 2 → 2
3. n = 3 → 3

### Medium Cases
4. n = 4 → 1
5. n = 12 → 3
6. n = 13 → 2

### Edge Cases
7. n = 0 → 0
8. n = 9999 → varies
9. Large perfect squares

## Related Problems

1. **LeetCode 279**: Perfect Squares (this problem)
2. **LeetCode 279**: Similar to coin change
3. **LeetCode 279**: Sum of Square Numbers

## Time to Complete

**Target**: 30 minutes
**Optimal**: 20 minutes