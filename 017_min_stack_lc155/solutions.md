# Solutions: Min Stack (LeetCode #155)

This document provides detailed, line-by-line analysis of each exercise implementation.

---

## Exercise 1: Min Stack - Two Stack Approach

### The Solution

```rust
#[derive(Default)]
pub struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {
    pub fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    pub fn push(&mut self, val: i32) {
        self.stack.push(val);
        let min = self.min_stack.last().copied().unwrap_or(i32::MAX).min(val);
        self.min_stack.push(min);
    }

    pub fn pop(&mut self) {
        self.stack.pop();
        self.min_stack.pop();
    }

    pub fn top(&self) -> i32 {
        self.stack.last().copied().unwrap_or(0)
    }

    pub fn get_min(&self) -> i32 {
        self.min_stack.last().copied().unwrap_or(0)
    }
}
```

### Key Insight

Track the minimum at each "level" of the stack:
- `stack` holds all values
- `min_stack[i]` holds the minimum of all values from `stack[0]` to `stack[i]`

### Complexity Analysis

| Metric | Value |
|--------|-------|
| **Time** | O(1) for all operations |
| **Space** | O(2n) = O(n) |

---

## Exercise 2: Max Stack

Uses the same pattern as MinStack but with max tracking.

---

## Exercise 4: Single Stack Trick

### The Trick

Instead of using two stacks, encode both the value and the minimum in one value:

```rust
pub fn push(&mut self, val: i32) {
    if self.stack.is_empty() {
        self.stack.push(val as i64);
    } else {
        let current_min = self.get_min();
        // Encode: actual_value + current_min * RANGE
        let encoded = ((val as i64) << 32) | (current_min as i64);
        self.stack.push(encoded);
    }
}
```

This is a space optimization but makes the code more complex.

---

## Summary Table

| Exercise | Time | Space | Key Technique |
|----------|------|-------|---------------|
| 1: Min Stack | O(1) | O(n) | Two stacks |
| 2: Max Stack | O(1) | O(n) | Two stacks |
| 3: Advanced | O(1) | O(n) | Multiple stacks |
| 4: Single Stack | O(1) | O(1)* | Value encoding |
| 5: Product | O(1) | O(n) | Product tracking |

## Key Takeaways

1. **Two stacks** is the clearest solution
2. **Track minimum** at each stack level
3. **Pop from both** stacks simultaneously
4. **Single stack trick** uses encoding
5. **All operations O(1)** is the key constraint
