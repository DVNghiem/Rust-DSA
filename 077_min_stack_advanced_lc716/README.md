# Min Stack Advanced - LeetCode 716

## Problem Statement

Design a data structure that supports push, pop, top, and retrieving the minimum element in constant time.

```
Operations:
- push(x): Push element x onto stack
- pop(): Remove and return top element
- top(): Return top element without removing
- get_min(): Return minimum element in stack

All operations must be O(1) time complexity.
```

## Visual Walkthrough

Consider a MinStack with the following operations:

```
Step 1: push(5)    Stack: [5]          Min: 5
Step 2: push(3)    Stack: [5, 3]       Min: 3
Step 3: push(7)    Stack: [5, 3, 7]    Min: 3
Step 4: push(3)    Stack: [5, 3, 7, 3] Min: 3
Step 5: pop()      Returns 3, Stack: [5, 3, 7]  Min: 3
Step 6: pop()      Returns 7, Stack: [5, 3]    Min: 3
Step 7: push(2)    Stack: [5, 3, 2]    Min: 2
Step 8: get_min()  Returns 2
```

## Naive Approach Problem

A naive approach using a single stack would require O(n) time to find the minimum because you'd need to scan the entire stack.

```
Naive: Store (value, min_so_far) pairs
Stack: [(5,5), (3,3), (7,3), (3,3)]
                           ↑
                     Current min is 3
```

## The Two-Stack Solution

We use two stacks:
1. **Main stack**: Stores all elements
2. **Min stack**: Stores elements in non-decreasing order (ascending)

```
Operation    Main Stack      Min Stack        Explanation
push(5)      [5]            [5]              First element is min
push(3)      [5,3]          [5,3]            3 < 5, push to min stack
push(7)      [5,3,7]        [5,3]            7 > 3, don't push (duplicate min)
push(3)      [5,3,7,3]      [5,3,3]          3 == 3, push (handles duplicates)
pop()        [5,3,7]        [5,3]            Top of min stack matches, pop it
get_min()    -               [5,3]           Return top of min stack: 3
```

## Why Two Stacks Work

The key insight is that the min stack only stores elements that could potentially be the minimum:

1. **When pushing**: Only push to min stack if new element <= current min
2. **When popping**: Pop from min stack only if popped element == current min

This ensures the min stack's top is always the minimum element.

## Edge Cases

### Duplicate Minimums

```
push(5): min_stack = [5]
push(3): min_stack = [5, 3]
push(3): min_stack = [5, 3, 3]  <- Push even though equal (needed for pop)
pop()   : min_stack = [5, 3]    <- Pop because top == popped value
```

We push duplicates because when we pop the real minimum, we need a backup.

### Empty Stack Handling

All operations must handle empty stack gracefully:
- `pop()` on empty: return None or handle error
- `top()` on empty: return None
- `get_min()` on empty: return None

## Implementation Details

### Data Structure

```rust
struct MinStack {
    main: Vec<i32>,
    min: Vec<i32>,
}
```

### Push Operation

```rust
fn push(&mut self, val: i32) {
    self.main.push(val);
    // Push if min stack is empty OR new value <= current minimum
    if self.min.is_empty() || val <= *self.min.last().unwrap() {
        self.min.push(val);
    }
}
```

### Pop Operation

```rust
fn pop(&mut self) -> Option<i32> {
    let val = self.main.pop();
    // If pop matches min stack top, pop from min stack too
    if let Some(v) = val {
        if !self.min.is_empty() && v == *self.min.last().unwrap() {
            self.min.pop();
        }
        return Some(v);
    }
    None
}
```

## Four Approaches Compared

| Approach | Push | Pop | Top | GetMin | Space |
|----------|------|-----|-----|--------|-------|
| Single stack with pairs | O(1) | O(1) | O(1) | O(1) | O(n) |
| Two stacks | O(1) | O(1) | O(1) | O(1) | O(n) |
| Single stack (store deltas) | O(1) | O(1) | O(1) | O(1) | O(n) |
| Linked list approach | O(1) | O(1) | O(1) | O(1) | O(n) |

## Single Stack with Pairs Approach

Instead of two stacks, store `(value, min_at_this_point)` tuples:

```
push(5):  stack = [(5, 5)]
push(3):  stack = [(5, 5), (3, 3)]
push(7):  stack = [(5, 5), (3, 3), (7, 3)]
push(3):  stack = [(5, 5), (3, 3), (7, 3), (3, 3)]

get_min:  look at last element's min = 3 (top of stack)
pop():    return last element's value = 3
```

```rust
struct MinStack {
    stack: Vec<(i32, i32)>,  // (value, min_at_this_point)
}
```

## Complexity Analysis

| Metric | Value |
|--------|-------|
| Time Complexity | O(1) for all operations |
| Space Complexity | O(n) worst case |
| Push Space | 2n in worst case (all decreasing) |

Worst case occurs when elements are pushed in decreasing order (e.g., `[5, 4, 3, 2, 1]`), causing min_stack to contain all elements.

## Test Cases Design

### Basic Operations
1. Push single element, verify min
2. Push multiple elements, verify min changes
3. Pop and verify min updates correctly
4. Multiple pushes and pops

### Edge Cases
5. Pop all elements, verify min stack is empty
6. Push duplicate minimums
7. Alternate push/pop operations
8. Empty stack operations

### Complex Scenarios
9. Large number of operations
10. Negative numbers
11. All same values
12. Strictly decreasing sequence
13. Strictly increasing sequence

## Implementation with Option Handling

For production code, we use `Option<i32>` to handle empty stack gracefully:

```rust
fn pop(&mut self) -> Option<i32> {
    let val = self.main.pop()?;
    if !self.min.is_empty() && val == *self.min.last().unwrap() {
        self.min.pop();
    }
    Some(val)
}
```

## Why Not Use RefCell or Cell?

We don't need interior mutability here because:
- All operations take `&mut self` (exclusive access)
- No shared references across multiple threads
- Rust's borrow checker ensures safety at compile time

## Related Problems

1. **LeetCode 155**: Min Stack (original, same problem)
2. **LeetCode 225**: Implement Stack using Queues
3. **LeetCode 232**: Implement Queue using Stacks
4. **LeetCode 716**: Max Stack (this variant)

## Extension: Max Stack

Similar to Min Stack but tracking maximum instead:

```rust
struct MaxStack {
    main: Vec<i32>,
    max: Vec<i32>,
}

impl MaxStack {
    fn push(&mut self, val: i32) {
        self.main.push(val);
        if self.max.is_empty() || val >= *self.max.last().unwrap() {
            self.max.push(val);
        }
    }
}
```

## Time to Complete

**Target**: 30 minutes for first implementation
**Optimal**: 20 minutes with two-stack approach

## Next Steps

After completing this problem:
1. Move to HashMap/HashSet design problems
2. Explore more O(1) operation data structures
3. Practice with stack-based algorithms