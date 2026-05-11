# Candy Solution - LeetCode 135 (Complete)

## Solution Analysis

### Two-Pass Greedy Approach

```rust
pub fn candy(ratings: Vec<i32>) -> i32 {
    let n = ratings.len();
    if n == 0 { return 0; }

    let mut candies = vec![1; n];

    // Left to right pass
    for i in 1..n {
        if ratings[i] > ratings[i - 1] {
            candies[i] = candies[i - 1] + 1;
        }
    }

    // Right to left pass
    for i in (0..n - 1).rev() {
        if ratings[i] > ratings[i + 1] {
            candies[i] = candies[i + 1] + 1;
        }
    }

    candies.iter().sum()
}
```

## Why Two Passes?

### Left-to-Right Pass
- Ensures higher rating than left neighbor gets more candies
- Only handles `>` relationship, not `<`

### Right-to-Left Pass
- Ensures higher rating than right neighbor gets more candies
- Combines with left pass to satisfy both directions

### Example Walkthrough

```
ratings: [1, 2, 3, 4, 2]
         ↓
Left pass:  [1, 2, 3, 4, 1]
            (only increases when rating increases)
         
Right pass: [1, 2, 3, 1, 1]
            (increases when rating > right neighbor)

Take max:   [1, 2, 3, 4, 1] ✓
```

## Line-by-Line Analysis

### Initialize candies array
```rust
let mut candies = vec![1; n];
```
- Every child gets at least 1 candy
- This is the base case

### Left-to-Right Pass
```rust
for i in 1..n {
    if ratings[i] > ratings[i - 1] {
        candies[i] = candies[i - 1] + 1;
    }
}
```
- If current rating is higher than left neighbor, give one more candy than left neighbor

### Right-to-Left Pass
```rust
for i in (0..n - 1).rev() {
    if ratings[i] > ratings[i + 1] {
        candies[i] = candies[i + 1] + 1;
    }
}
```
- If current rating is higher than right neighbor, give one more candy than right neighbor
- Take max with existing value (in case left pass already gave more)

## Complexity Analysis

| Approach | Time | Space |
|----------|------|-------|
| Two-Pass | O(n) | O(n) |
| Brute Force | O(n²) | O(n) |

## Edge Cases

### Empty Input
```rust
Input: ratings = []
Output: 0
```
- n == 0, returns 0

### Single Child
```rust
Input: ratings = [5]
Output: 1
```
- One candy, no comparisons needed

### All Same Rating
```rust
Input: ratings = [5, 5, 5, 5]
Output: 4
```
- Each gets 1 (minimum)

### Strictly Increasing
```rust
Input: ratings = [1, 2, 3, 4, 5]
Output: 15 (1 + 2 + 3 + 4 + 5)
```

### Strictly Decreasing
```rust
Input: ratings = [5, 4, 3, 2, 1]
Output: 15 (5 + 4 + 3 + 2 + 1)
```

## Why Max is Taken

```rust
candies[i] = candies[i + 1] + 1;  // Not candies[i].max(...)
```

We want the MAXIMUM of what both passes give because:
- If left pass gave 5 (because rating was higher than left)
- And right pass would give 3 (because rating was higher than right)
- We need to satisfy BOTH conditions
- So we take max(5, 3) = 5

## Common Mistakes

1. **Single pass**: Doesn't work - only handles one direction
2. **Not taking max**: Would incorrectly reduce candies in some cases
3. **Wrong iteration direction**: Right-to-left must use `.rev()`

## Rust Patterns Used

1. **`vec![1; n]`**: Create vector with n elements all initialized to 1
2. **`.rev()`**: Reverse iterator for right-to-left traversal
3. **`.iter().sum()`**: Sum all elements in vector