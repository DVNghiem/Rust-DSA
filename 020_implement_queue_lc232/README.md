# Implement Queue Using Stacks (LeetCode #232)

## Problem Statement

Implement a first in first out (FIFO) queue using two stacks. The implemented queue should support all the functions of a normal queue (`push`, `peek`, `pop`, and `empty`).

## Examples

```
Input: ["MyQueue", "push", "push", "peek", "pop", "empty"]
       [[], [1], [2], [], [], []]
Output: [null, null, null, 1, 1, false]

Explanation:
  MyQueue myQueue = new MyQueue();
  myQueue.push(1); // queue is: [1]
  myQueue.push(2); // queue is: [1, 2]
  myQueue.peek(); // return 1
  myQueue.pop(); // return 1, queue is [2]
  myQueue.empty(); // return false
```

## Approach

### Two-Stack Design

Use two stacks:
- **in_stack**: For pushing new elements
- **out_stack**: For popping/peeking elements

When popping, if out_stack is empty, transfer all elements from in_stack to out_stack (reversing order).

## Implementation

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

## Complexity Analysis Table

| Operation | Amortized Time | Space |
|-----------|----------------|-------|
| push | O(1) | O(n) total |
| pop | O(1) amortized | |
| peek | O(1) | |
| empty | O(1) | |

## Key Takeaways

1. **Two stacks** simulate a queue
2. **Transfer only when needed** - amortized O(1)
3. **Reverse order** when transferring
4. **All operations O(1)** amortized
5. **peek/pop use out_stack** first

## Real-World Applications

1. **Task scheduling**: FIFO task processing
2. **Print queue**: Print jobs in order
3. **Message queues**: Process messages in order
4. **Breadth-first search**: Queue for BFS implementation
5. **Undo functionality**: Transaction order