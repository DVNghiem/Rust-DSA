# Iterator for Nested List

## Problem Statement (LeetCode 341)

Design an iterator to flatten a nested list of integers. Each element can be an integer or a list (which may contain more integers or lists).

```
Example:
Input: [[1,2], [3, [4,5]], 6]
Output: [1, 2, 3, 4, 5, 6]
```

## Understanding Nested Lists

A nested list is a recursive structure where each element can be:
- A scalar value (integer in our case)
- A collection of elements (another nested list)

This recursive nature makes flattening challenging because we need to track the current position in each level of nesting.

## Visual Walkthrough

Consider the nested structure: `[[1,2], [3,[4,5]], 6]`

```
Level 0:    [ [1,2], [3,[4,5]], 6 ]
                 |      |         |
Level 1:      [1,2]   [3,[4,5]]  6
                   |     |   |
Level 2:            1,2   3  [4,5]
                               |
Level 3:                              4,5

Flattened: 1 -> 2 -> 3 -> 4 -> 5 -> 6
```

## Approaches

### Approach 1: Recursive with Flatten Helper
Use recursion to fully flatten the list before iteration.

```rust
fn flatten(nested: &NestedInteger) -> Vec<i32>
```

**Time Complexity**: O(n) where n = total integers
**Space Complexity**: O(n) for storing all integers

### Approach 2: Stack-Based Iteration
Use an explicit stack to simulate recursion without recursive calls.

```rust
struct NestedIterator {
    stack: Vec<NestedInteger>
}
```

**Time Complexity**: O(n) amortized
**Space Complexity**: O(d) where d = maximum nesting depth

### Approach 3: Generator Pattern with Lazy Evaluation
Only flatten what's needed, when it's needed (most memory efficient).

**Time Complexity**: O(1) per element (amortized)
**Space Complexity**: O(d) for stack

### Approach 4: Index-Based Traversal
Maintain indices at each nesting level for random access capability.

**Time Complexity**: O(1) per element
**Space Complexity**: O(d)

## Comparison Table

| Approach | Time (next) | Space | Lazy | Reset |
|----------|-------------|-------|------|-------|
| Recursive Flatten | O(n) | O(n) | No | O(n) |
| Stack-Based | O(1) amortized | O(d) | Yes | O(d) |
| Generator | O(1) amortized | O(d) | Yes | O(d) |
| Index-Based | O(1) | O(d) | Yes | O(1) |

## Implementation Strategy

We implement the **stack-based approach** because:
1. Handles arbitrarily deep nesting
2. Lazy evaluation (doesn't pre-flatten)
3. O(1) amortized time per `next()` call
4. Memory efficient

## Algorithm Details

### Core Data Structure

```rust
pub struct NestedIterator {
    // Stack of iterators over nested levels
    // We store &mut Iterator so we can peek without consuming
    stack: Vec<vec::IntoIter<NestedInteger>>,
}
```

### Key Operations

1. **Initialization**: Push the top-level list's iterator onto stack
2. **hasNext()**: 
   - While stack not empty, try to get next element
   - If next is integer, put it back (remember position) and return true
   - If next is list, push its iterator and continue
3. **next()**: Returns the integer found by hasNext()

### Handling NestedIntegers

```rust
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}
```

## Edge Cases

1. **Empty list**: `[]` - should return false for hasNext()
2. **Deeply nested**: `[[[[[1]]]]]` - must handle arbitrary depth
3. **Mixed nesting**: `[1, [2, [3, [4]]]]` - various depths
4. **Single element**: `[42]` - edge case
5. **Null elements**: Not applicable (Rust has Option, but our structure doesn't)

## Test Cases Design

### Basic Cases
- Simple flat list: `[1, 2, 3]`
- Single nested list: `[[1, 2, 3]]`
- Single integer: `[[42]]`

### Edge Cases  
- Empty list: `[]`
- Deep nesting: `[[[[[1]]]]]`
- Wide but shallow: `[[1], [2], [3], [4], [5]]`
- Mixed: `[1, [2, 3], [4, [5, 6]], 7]`

### Complex Cases
- Alternating pattern: `[[1], 2, [[3], 4], 5]`
- Empty sublists: `[[], [1], [], [2, []]]`
- All empty: `[[], [], []]`

## Implementation Notes

### Memory Management
- Use `VecDeque` for efficient front operations
- Store references to avoid cloning nested structures
- Iterator consumes elements as we traverse

### Rust-Specific Patterns
- Use `vec.into_iter()` to convert Vec to iterator
- Use `Option::take()` to avoid ownership issues
- Use `while let` for clean iterator consumption

## Problem Constraints

- `1 <= nestedList.length <= 500`
- Values in nested integers range from `-(2^31)` to `(2^31 - 1)`
- Recursive depth should not exceed system limits

## Related Problems

- Flatten Nested List Iterator (LeetCode 341) - This problem
- Binary Search Tree Iterator (LeetCode 173)
- Zigzag Iterator (LeetCode 281)
- Flatten 2D Vector (LeetCode 251)

## Time to Complete

**Target**: 45 minutes for first implementation
**Optimal**: 30-35 minutes with clean stack-based approach

## Next Steps

After completing this problem:
1. Move to MinStack for stack-based problems
2. Explore more complex iterator patterns
3. Practice with nested data structure problems