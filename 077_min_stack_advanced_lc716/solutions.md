# Solutions Analysis: Min Stack Advanced (LeetCode 716)

## Solution Overview

This problem requires implementing a stack that supports O(1) operations for push, pop, top, and retrieving the minimum element. We implement three solutions:

1. **Two-stack approach** (standard)
2. **Single stack with pairs** (alternative)
3. **Max stack variant** (extension)

## Solution 1: Two-Stack Approach

### Data Structure Definition

```rust
pub struct MinStack {
    main: Vec<i32>,
    min: Vec<i32>,
}
```

**Line-by-line analysis:**

1. `main: Vec<i32>` - The primary stack storing all elements
2. `min: Vec<i32>` - Auxiliary stack storing minimum elements in non-decreasing order

### Constructor

```rust
pub fn new() -> Self {
    MinStack {
        main: Vec::new(),
        min: Vec::new(),
    }
}
```

Simply initializes two empty vectors. Uses `Vec::new()` which allocates zero capacity initially.

### Push Operation

```rust
pub fn push(&mut self, val: i32) {
    self.main.push(val);
    if self.min.is_empty() || val <= *self.min.last().unwrap() {
        self.min.push(val);
    }
}
```

**Step-by-step:**

1. `self.main.push(val)` - Always push to main stack
2. `self.min.is_empty()` - If min stack is empty, push the first element
3. `val <= *self.min.last().unwrap()` - If current value is less than or equal to current minimum, push to min stack
4. **Key insight**: We use `<=` (not `<`) to handle duplicate minimums correctly

**Why <= instead of <?**

When we have duplicates of the minimum value, all must be in the min stack:

```
push(3): main=[3], min=[3]
push(3): main=[3,3], min=[3,3] (must include both!)
pop(): removes one 3, if min wasn't duplicated, min=[3] would be correct
pop(): removes second 3, min should now be empty
```

If we used `<`, the second `push(3)` wouldn't add to min stack, causing incorrect behavior on pop.

### Pop Operation

```rust
pub fn pop(&mut self) -> Option<i32> {
    let val = self.main.pop();
    if let Some(v) = val {
        if !self.min.is_empty() && v == *self.min.last().unwrap() {
            self.min.pop();
        }
        return Some(v);
    }
    None
}
```

**Step-by-step:**

1. `self.main.pop()` - Remove top from main stack
2. `if let Some(v) = val` - Check if pop succeeded
3. `!self.min.is_empty() && v == *self.min.last().unwrap()` - If popped value equals current min, pop from min stack too
4. Return `Some(v)` or `None`

**Key insight**: We check equality with `==` because we only pop from min stack when the popped element matches the current minimum.

### Top Operation

```rust
pub fn top(&self) -> Option<i32> {
    self.main.last().copied()
}
```

Simple: return last element of main stack without removing. Uses `.copied()` to convert `&i32` to `i32`.

### GetMin Operation

```rust
pub fn get_min(&self) -> Option<i32> {
    self.min.last().copied()
}
```

Return top of min stack (current minimum) without removing.

## Solution 2: Single Stack with Pairs

### Data Structure

```rust
pub struct MinStackSingle {
    stack: Vec<(i32, i32)>,  // (value, min_at_this_point)
}
```

Each element stores both the value and the minimum after this element is pushed.

### Push in Single Stack

```rust
pub fn push(&mut self, val: i32) {
    let current_min = self.stack.last().map(|(_, m)| *m).unwrap_or(i32::MAX);
    let new_min = val.min(current_min);
    self.stack.push((val, new_min));
}
```

**Analysis:**

1. `self.stack.last().map(|(_, m)| *m)` - Get current minimum from top of stack (if exists)
2. `unwrap_or(i32::MAX)` - If stack empty, use MAX as current min
3. `val.min(current_min)` - New minimum is smaller of current value and previous min
4. `self.stack.push((val, new_min))` - Store both value and new minimum

**Why this works:**

When you push 5: stack = [(5, 5)]
When you push 3: current_min=5, new_min=3, stack = [(5, 5), (3, 3)]
When you push 7: current_min=3, new_min=3, stack = [(5, 5), (3, 3), (7, 3)]

The top element's second value is always the minimum for that prefix.

### Pop in Single Stack

```rust
pub fn pop(&mut self) -> Option<i32> {
    self.stack.pop().map(|(v, _)| v)
}
```

Simply pop and return the value part, ignoring the min part.

### GetMin in Single Stack

```rust
pub fn get_min(&self) -> Option<i32> {
    self.stack.last().map(|(_, m)| m).copied()
}
```

Look at the top element's min value.

## Max Stack Solution

### Key Difference

Uses `>=` instead of `<=` because we track maximum instead of minimum:

```rust
if self.max.is_empty() || val >= *self.max.last().unwrap() {
    self.max.push(val);
}
```

## Complexity Comparison

| Implementation | Push | Pop | Top | GetMin | Extra Space |
|----------------|------|-----|-----|--------|-------------|
| Two Stack | O(1) | O(1) | O(1) | O(1) | O(n) worst |
| Single Stack | O(1) | O(1) | O(1) | O(1) | O(n) |
| MinMaxStack | O(1) | O(1) | O(1) | O(1) | O(2n) worst |

## Space Analysis

### Two-Stack Space

Worst case: Elements in decreasing order

```
push(5): main=[5], min=[5]
push(4): main=[5,4], min=[5,4]
push(3): main=[5,4,3], min=[5,4,3]
push(2): main=[5,4,3,2], min=[5,4,3,2]
push(1): main=[5,4,3,2,1], min=[5,4,3,2,1]
```

min stack has n elements when all elements decreasing.

Best case: Elements increasing

```
push(1): main=[1], min=[1]
push(2): main=[1,2], min=[1]
push(3): main=[1,2,3], min=[1]
push(4): main=[1,2,3,4], min=[1]
push(5): main=[1,2,3,4,5], min=[1]
```

min stack has only 1 element.

### Single Stack Space

Always O(n) - each element stores a tuple.

## Trade-offs Between Approaches

| Aspect | Two Stack | Single Stack |
|--------|-----------|--------------|
| Memory | Variable O(d) to O(n) | Fixed O(n) |
| Simplicity | Slightly more complex | Cleaner concept |
| Cache behavior | May be worse (2 vectors) | Better (1 vector) |
| Min retrieval | O(1) | O(1) |

## Edge Cases in Detail

### Empty Stack Operations

All methods handle empty gracefully:
- `pop()` returns `None` if main empty
- `top()` returns `None` via `last().copied()`
- `get_min()` returns `None` via `min.last().copied()`

### Duplicate Minimums

```
Scenario: push(3), push(3), pop()

Two-stack:
push(3): main=[3], min=[3]
push(3): main=[3,3], min=[3,3] (val<=min, so push)
pop(): v=3, matches min top, min.pop() → min=[3]
pop(): v=3, matches min top, min.pop() → min=[]

Single-stack:
push(3): stack=[(3,3)]
push(3): stack=[(3,3),(3,3)]
pop(): removes (3,3) → returns 3
pop(): removes (3,3) → returns 3
```

Both handle correctly.

### Negative Numbers

Works identically because comparisons work for negative values.

## Test Case Coverage

### Basic Operations (4 tests)
- push, pop, top, get_min individually

### Edge Cases (6 tests)
- empty stack operations
- duplicate minimums
- alternating push/pop
- negative numbers

### Edge Values (5 tests)
- all same values
- strictly decreasing
- strictly increasing
- large number of operations

### Variants (5+ tests)
- MaxStack implementation
- MinMaxStack implementation
- Single stack comparison

## Common Mistakes

### Mistake 1: Using < instead of <=

```rust
// WRONG
if val < *self.min.last().unwrap() {
    self.min.push(val);
}

// CORRECT
if val <= *self.min.last().unwrap() {
    self.min.push(val);
}
```

Without `<=`, duplicate minimums won't be properly handled.

### Mistake 2: Not handling empty min stack

```rust
// WRONG - might panic on empty
if v == *self.min.last().unwrap() {

// CORRECT - check empty first
if !self.min.is_empty() && v == *self.min.last().unwrap() {
```

### Mistake 3: Forgetting to pop from min stack

When popping, must check if popped value equals current min before popping min stack.

## Why Rust's Ownership Model Helps

Rust's borrow checker prevents:
- Using after free (main.pop() returns value, can't use after)
- Data races (all operations take &mut self)
- Iterator invalidation (Vec operations are bounds-checked)

## Conclusion

The two-stack approach is the most commonly used solution:

1. **Intuitive**: Clear separation between data and metadata
2. **Efficient**: O(1) amortized for all operations
3. **Correct**: Handles all edge cases properly
4. **Space efficient**: Can use less memory than single-stack approach in best case

The single-stack approach is a valid alternative with slightly different trade-offs.