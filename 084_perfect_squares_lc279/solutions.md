# Solutions Analysis: Perfect Squares (LeetCode 279)

## Problem Overview

Find the minimum number of perfect square numbers that sum to n.

## Approach 1: DP Bottom-Up

### Algorithm

```rust
pub fn num_squares_dp(n: i32) -> i32 {
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

### DP State Definition

```rust
dp[i] = minimum number of perfect squares that sum to i
```

### DP Transition

```rust
dp[i] = min(dp[i - j²] + 1) for all j where j² ≤ i
```

For each number i, try subtracting each perfect square j² and add 1 (for j² itself).

### Tracing for n = 12

```
dp[0] = 0
dp[1] = min(dp[0] + 1) = 1        (1 = 1²)
dp[2] = min(dp[1] + 1) = 2        (2 = 1 + 1)
dp[3] = min(dp[2] + 1) = 3        (3 = 1 + 1 + 1)
dp[4] = min(dp[3] + 1, dp[0] + 1) = 1   (4 = 2²)
dp[5] = min(dp[4] + 1, dp[1] + 1) = 2   (5 = 4 + 1)
dp[6] = min(dp[5] + 1, dp[2] + 1) = 3   (6 = 4 + 1 + 1)
dp[7] = min(dp[6] + 1, dp[3] + 1) = 4   (7 = 4 + 1 + 1 + 1)
dp[8] = min(dp[7] + 1, dp[4] + 1) = 2   (8 = 4 + 4)
dp[9] = min(dp[8] + 1, dp[5] + 1, dp[0] + 1) = 1  (9 = 3²)
dp[10] = min(dp[9] + 1, dp[6] + 1, dp[1] + 1) = 2 (10 = 9 + 1)
dp[11] = min(dp[10] + 1, dp[7] + 1, dp[2] + 1) = 3 (11 = 9 + 1 + 1)
dp[12] = min(dp[11] + 1, dp[8] + 1, dp[3] + 1, dp[0] + 1)
       = min(3 + 1, 2 + 1, 3 + 1, 0 + 1)
       = 3 (12 = 4 + 4 + 4)
```

## Approach 2: BFS

### Graph Model

Think of numbers as nodes, edges connect n to n - j² for all valid j.

```
       12
      / | \
    11   8   3
   / \  |   |
  ... ...  ...
```

Shortest path from 12 to 0 = minimum squares.

### BFS Implementation

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

### BFS Trace for n = 12

```
Level 0: [12]
Level 1: [11, 8, 3]     (12 - 1, 12 - 4, 12 - 9)
Level 2: [10, 7, 2, 7, 4, 0]
                      ↑
              Found 0! depth = 2? Wait...

Actually trace more carefully:
n = 12
queue starts: [12]
depth = 0

Pop 12, generate children:
  12 - 1 = 11
  12 - 4 = 8
  12 - 9 = 3
queue = [11, 8, 3], depth = 1

Pop 11, generate children:
  11 - 1 = 10
  11 - 4 = 7
  11 - 9 = 2
queue = [8, 3, 10, 7, 2]

Pop 8, generate children:
  8 - 1 = 7
  8 - 4 = 4
queue = [3, 10, 7, 2, 7, 4]

Pop 3, generate children:
  3 - 1 = 2
  3 - 4 = (not valid)
queue = [10, 7, 2, 7, 4, 2]

Pop 10, generate children:
  10 - 1 = 9
  10 - 4 = 6
  10 - 9 = 1
queue = [7, 2, 7, 4, 2, 9, 6, 1]

Continue until we find 0...
Actually we saw from 12, 12 - 9 = 3, then 3 - 1 = 2... this is getting complex.

Actually from 12, depth 2 means 2 squares. But 12 = 4 + 4 + 4 is 3 squares.

The path 12 -> 3 -> 0 is:
  12 - 9 = 3 (first square)
  3 - 1 = 2 (second square? No, 3 is not square)
  3 - 1 - 1 - 1 = 0 (4 squares total)

Let me trace better:
depth = 0: queue = [12]
depth = 1: expand 12 → generate 11, 8, 3
depth = 2: expand 11 → 10, 7, 2
         expand 8 → 7, 4
         expand 3 → 2, ...
         
We need to find shortest path to 0.

12 → 8 → 4 → 0: 12 - 4 - 4 = 4? 8 - 4 = 4, 4 - 4 = 0
  That's 12 → 8 (using 4), 8 → 4 (using 4), 4 → 0 (using 4)
  3 squares: 4 + 4 + 4 = 12 ✓

So BFS finds depth 3 (three edges).
```

Wait, I need to clarify. BFS depth is number of levels, which equals number of squares in this context because each step subtracts one perfect square.

Actually: each level represents one subtraction of a perfect square.
- Level 0: start at n
- Level 1: n - square
- Level 2: (n - square) - square
- etc.

If we reach 0 at level k, we used k squares.

For n = 12, we reach 0 at level 3 with path 12 → 8 → 4 → 0 (using 3 squares).

## Approach 3: Math-Based (Optimal)

### Lagrange's Four Square Theorem

Every natural number can be represented as the sum of at most four perfect squares:

```rust
n = a² + b² + c² + d²
```

### Legendre's Three Square Theorem

n cannot be expressed as sum of 3 squares if and only if:

```rust
n = 4^a × (8b + 7)
```

### Math Algorithm

```rust
pub fn num_squares_math(n: i32) -> i32 {
    // 1. Check if perfect square (1 square)
    if is_perfect_square(n) { return 1; }

    // 2. Check sum of 2 squares (2 squares)
    // n = a² + b² iff exists a where n - a² is perfect square
    let mut a = 0;
    while a * a <= n {
        if is_perfect_square(n - a * a) {
            return 2;
        }
        a += 1;
    }

    // 3. Check 4-square condition: n = 4^a * (8b + 7)
    // If true, answer is 4
    let mut m = n;
    while m % 4 == 0 { m /= 4; }
    if m % 8 == 7 { return 4; }

    // 4. Otherwise, answer is 3
    3
}
```

### Math Algorithm Trace for n = 12

```
n = 12

Step 1: is_perfect_square(12)? No

Step 2: check for two squares
  a=0: 12 - 0 = 12, is_square? No
  a=1: 12 - 1 = 11, is_square? No
  a=2: 12 - 4 = 8, is_square? No
  a=3: 12 - 9 = 3, is_square? No
No match → not 2 squares

Step 3: check 4-square condition
  12 % 4 == 0? Yes → m = 12/4 = 3
  3 % 4 == 0? No
  m % 8 == 7? 3 % 8 = 3 ≠ 7 → not 4 squares

Step 4: answer is 3
```

### Math Algorithm Trace for n = 13

```
n = 13

Step 1: is_perfect_square(13)? No

Step 2: check for two squares
  a=0: 13 - 0 = 13, no
  a=1: 13 - 1 = 12, no
  a=2: 13 - 4 = 9, yes! 9 is perfect square (3²)
  → return 2 (13 = 4 + 9 = 2² + 3²)
```

### Math Algorithm Trace for n = 7

```
n = 7

Step 1: is_perfect_square(7)? No

Step 2: check for two squares
  a=0: 13 - 0 = 13, no
  (only need to check until a² < 7)
  a=1: 7 - 1 = 6, no
  a=2: 7 - 4 = 3, no
No match

Step 3: check 4-square condition
  7 % 4 == 0? No
  7 % 8 == 7? Yes! → return 4
```

## Complexity Comparison

| Approach | Time | Space | Description |
|----------|------|-------|-------------|
| DP Bottom-up | O(n√n) | O(n) | Check all j for each i |
| BFS | O(n√n) | O(n) | Explore level by level |
| Math-based | O(√n) | O(1) | Direct mathematical check |

## Why Math Works

### Key Theorems

1. **Lagrange's Four Square Theorem**: Guarantees answer is 1, 2, 3, or 4
2. **Legendre's Three Square Theorem**: Tells us when answer is 4

### Why Legendre's Theorem Works

If `n = 4^a × (8b + 7)`, then we need at least 4 squares because:
- Any number of form `8b + 7` cannot be expressed as sum of 3 squares
- The factor `4^a` doesn't help (it's just 2^a squared factor)

### Why DP Works

DP finds the optimal by building up from smaller subproblems. The recurrence `dp[i] = min(dp[i - j²] + 1)` is correct because any optimal representation of i either:
- Includes j² as one component (then remaining is dp[i - j²])
- Doesn't include j² (then we check other j)

## Edge Cases

### n = 0
```rust
dp[0] = 0 // 0 squares to make 0
```

### n = 1
```rust
is_perfect_square(1) = true → return 1
```

### n = 2
```rust
not perfect square
not sum of 2 squares (13 - 1 = 12, not square)
7 % 8 = 7? No → return 3
```

But 2 = 1 + 1 = 2 squares, not 3!

Wait, let me check my math for n=2:
```
n = 2

Step 1: is_perfect_square(2)? No

Step 2: check for two squares
  a=0: 2 - 0 = 2, not square
  a=1: 2 - 1 = 1, IS SQUARE! → return 2

So 2 = 1 + 1 = 2 squares ✓
```

### n = 3
```
Step 1: no
Step 2: a=0: 3-0=3 no, a=1: 3-1=2 no, a=2: 4 > 3, stop
Step 3: 3 % 4 != 0, 3 % 8 = 3 != 7
→ return 3

3 = 1 + 1 + 1 = 3 squares ✓
```

## Test Coverage Analysis

### Basic Cases
- n = 0 → 0
- n = 1 → 1 (perfect square)
- n = 2 → 2 (1 + 1)
- n = 3 → 3 (1 + 1 + 1)
- n = 4 → 1 (2²)

### Medium Cases
- n = 12 → 3 (4 + 4 + 4)
- n = 13 → 2 (4 + 9)
- n = 99 → 3 (9 + 9 + 9 + 9 + 9 + 9 + 9 + 9 + 9 + 9 + 9... no wait)
  Actually 99 = 9 + 9 + 9 + 9 + 9 + 9 + 9 + 9 + 9 + 9 + 9 = 11? No optimal is 3: 36 + 36 + 25 + 1 + 1 = 5...

### Edge Cases
- All approaches should give same answer
- All numbers should have answer 1-4 per Lagrange's theorem

## Conclusion

The mathematical approach is optimal with O(√n) time and O(1) space. The key insights are:

1. Answer is always 1, 2, 3, or 4 (Lagrange's theorem)
2. 1: check if n is perfect square
3. 2: check if n - a² is perfect square for any a
4. 4: check if n = 4^a × (8b + 7)
5. 3: otherwise

The DP approach is easier to understand and implement but slower.