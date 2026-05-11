# Min Stack (LeetCode #155)

## Problem Statement

Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.

Implement the `MinStack` class:
- `MinStack()` initializes the stack object.
- `void push(int val)` pushes the element onto the stack.
- `void pop()` removes the element on the top of the stack.
- `int top()` gets the top element.
- `int get_min()` retrieves the minimum element in the stack.

All operations must have O(1) time complexity.

## Examples

```
Input: ["MinStack","push","push","push","get_min","pop","top","get_min"]
       [[],[-2],[0],[-3],[],[],[],[]]

Output: [null,null,null,null,-3,null,0,-2]

Explanation:
  MinStack minStack = new MinStack();
  minStack.push(-2);
  minStack.push(0);
  minStack.push(-3);
  minStack.get_min(); return -3
  minStack.pop();
  minStack.top();    return 0
  minStack.get_min(); return -2
```

## Two Stack Approach

### Key Insight

Use two stacks:
1. Main stack stores all elements
2. Min stack stores minimum at each level

### Visual

```
Stack state after push(-2), push(0), push(-3):

Main: -2 → 0 → -3
Min:  -2 → -2 → -3

top() returns -3 (from main)
get_min() returns -3 (from min)

After pop():
Main: -2 → 0
Min:  -2 → -2

top() returns 0
get_min() returns -2
```

## Implementation

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

## Complexity Analysis Table

| Operation | Time | Space |
|-----------|------|-------|
| push | O(1) | O(1) amortized |
| pop | O(1) | O(1) |
| top | O(1) | O(1) |
| get_min | O(1) | O(1) |

Total Space: O(n) for n elements

## Key Takeaways

1. **Two stacks** track main values and minimums
2. **Push minimum** to min_stack each time
3. **All operations O(1)** - constant time guaranteed
4. **Pop both stacks** to maintain synchronization
5. **Single element stack** works correctly

## Real-World Applications

1. **Browser history**: Back button with minimum visibility
2. **Stock prices**: Track min/max over time
3. **Temperature tracking**: Min/max thermometers
4. **Game scores**: Track current min score
5. **Resource allocation**: Track minimum available resources
