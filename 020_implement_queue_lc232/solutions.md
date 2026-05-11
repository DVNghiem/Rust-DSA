# Solutions: Implement Queue Using Stacks (LeetCode #232)

This document provides detailed, line-by-line analysis of each exercise implementation.

---

## Exercise 1: MyQueue - Two Stack Queue

### The Solution

```rust
#[derive(Default)]
pub struct MyQueue {
    in_stack: Vec<i32>,
    out_stack: Vec<i32>,
}

impl MyQueue {
    pub fn new() -> Self {
        MyQueue {
            in_stack: Vec::new(),
            out_stack: Vec::new(),
        }
    }

    pub fn push(&mut self, x: i32) {
        self.in_stack.push(x);
    }

    pub fn pop(&mut self) -> i32 {
        if self.out_stack.is_empty() {
            self.transfer();
        }
        self.out_stack.pop().unwrap()
    }

    pub fn peek(&self) -> i32 {
        if self.out_stack.is_empty() {
            self.in_stack[0]
        } else {
            self.out_stack[self.out_stack.len() - 1]
        }
    }

    pub fn empty(&self) -> bool {
        self.in_stack.is_empty() && self.out_stack.is_empty()
    }

    fn transfer(&mut self) {
        while let Some(x) = self.in_stack.pop() {
            self.out_stack.push(x);
        }
    }
}
```

### Key Insight

Stack is LIFO (Last In First Out), Queue is FIFO (First In First Out).
By using two stacks:
- in_stack: Push elements (top = most recent)
- out_stack: Pop/Peek elements (top = oldest)

When out_stack is empty, transfer all from in_stack (reverses order, giving FIFO).

### Complexity Analysis

| Operation | Time | Explanation |
|-----------|------|-------------|
| push | O(1) | Direct push to in_stack |
| pop | O(1) amortized | Each element transferred at most once |
| peek | O(1) | Check out_stack or in_stack |
| empty | O(1) | Check both stacks |

---

## Summary Table

| Exercise | Time | Space | Key Technique |
|----------|------|-------|---------------|
| 1: Queue | O(1) amortized | O(n) | Two stacks |
| 2: Stack | O(1) push, O(n) pop | O(n) | Two queues |
| 3: Max Queue | O(1) | O(n) | Track max |
| 4: Moving Avg | O(1) | O(window) | Sliding window |
| 5: BFS | O(n) | O(n) | Queue traversal |

## Key Takeaways

1. **Two stacks** simulate queue behavior
2. **Transfer only when needed** for amortized O(1)
3. **Pop from out_stack** (oldest first)
4. **Push to in_stack** (newest last)
5. **Transfer reverses order** to achieve FIFO