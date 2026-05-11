# Solutions: Daily Temperatures (LeetCode #739)

This document provides detailed, line-by-line analysis of each exercise implementation.

---

## Exercise 1: Daily Temperatures - Monotonic Stack

### The Solution

```rust
pub fn daily_temperatures(temperatures: &[i32]) -> Vec<i32> {
    let n = temperatures.len();
    let mut answer = vec![0; n];
    let mut stack: Vec<usize> = Vec::new();

    for i in 0..n {
        while let Some(&j) = stack.last() {
            if temperatures[i] > temperatures[j] {
                answer[j] = (i - j) as i32;
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(i);
    }
    answer
}
```

### Line-by-Line Analysis

```rust
let n = temperatures.len();
let mut answer = vec![0; n];
```
**Purpose:** Initialize result array with 0s (default for days with no warmer temperature).

```rust
let mut stack: Vec<usize> = Vec::new();
```
**Purpose:** Create stack to store indices of days. The stack maintains a decreasing temperature sequence.

```rust
for i in 0..n {
```
**Purpose:** Iterate through each day.

```rust
while let Some(&j) = stack.last() {
    if temperatures[i] > temperatures[j] {
        answer[j] = (i - j) as i32;
        stack.pop();
    } else {
        break;
    }
}
```
**Purpose:** For each day, check if it's warmer than the days in stack:
- If warmer, calculate days difference and pop from stack
- If not warmer, stop checking (stack is decreasing)

```rust
stack.push(i);
```
**Purpose:** Push current day's index onto stack.

### Why This Works

The stack maintains indices of days in decreasing temperature order. When a warmer day is found:
- It's the first warmer day for all popped indices
- The distance to that warmer day is the answer

### Complexity Analysis

| Metric | Value | Explanation |
|--------|-------|-------------|
| **Time** | O(n) | Each index pushed/popped at most once |
| **Space** | O(n) | Stack stores at most n indices |

---

## Summary Table

| Exercise | Time | Space | Key Technique |
|----------|------|-------|---------------|
| 1: Daily Temp | O(n) | O(n) | Monotonic stack |
| 2: Brute | O(n²) | O(1) | Nested loops |
| 3: Next Greater | O(n) | O(n) | Monotonic stack |
| 4: Next Smaller | O(n) | O(n) | Monotonic stack |
| 5: Extended | O(n) | O(n) | Stack variant |
| 6: Max Distance | O(n) | O(n) | Stack |
| 7: Prices | O(n) | O(n) | Same as temp |

## Key Takeaways

1. **Monotonic stack** maintains decreasing order
2. **Pop when warmer** day found
3. **Each element** pushed/popped at most once = O(n)
4. **Remaining stack elements** have no warmer day ahead
5. **Stack of indices** tracks positions in original array
