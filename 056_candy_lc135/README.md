# Candy - LeetCode 135

## Problem Overview

There are `n` children standing in a line, each with a rating. Give each child at least one candy. Children with a higher rating than their neighbor must have more candies than that neighbor. Return minimum candies needed.

**Examples:**
```
Input: ratings = [1,0,2]
Output: 5 (candies: [2,1,2])

Input: ratings = [1,2,2]
Output: 4 (candies: [1,2,1])
```

## Theory

### Two-Pass Greedy Solution

Key insight: Consider left-to-right pass, then right-to-left pass.

```
ratings: [1, 2, 3, 4, 2]
         ↓
Left pass:  [1, 2, 3, 4, 1] (only increase when rating increases)
Right pass: [1, 2, 3, 1, 1] (only increase when rating increases from right)

Take max:   [1, 2, 3, 4, 1] ✓
```

## Implementation

```rust
pub fn candy(ratings: Vec<i32>) -> i32 {
    let n = ratings.len();
    let mut candies = vec![1; n];

    // Left to right pass
    for i in 1..n {
        if ratings[i] > ratings[i-1] {
            candies[i] = candies[i-1] + 1;
        }
    }

    // Right to left pass
    for i in (0..n-1).rev() {
        if ratings[i] > ratings[i+1] {
            candies[i] = candies[i+1] + 1;
        }
    }

    candies.iter().sum()
}
```

## Complexity Analysis

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Two-Pass | O(n) | O(n) | Optimal |
| Brute Force | O(n²) | O(n) | Try all |

## Edge Cases

1. **Empty**: 0
2. **Single child**: 1
3. **All equal ratings**: n
4. **Increasing sequence**: n*(n+1)/2
5. **Decreasing sequence**: n

## Test Cases

```rust
#[test]
fn test_candy_basic() {
    assert_eq!(candy(vec![1,0,2]), 5);
}
```