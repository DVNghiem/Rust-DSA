# Solutions: Valid Parentheses (LeetCode #20)

This document provides detailed, line-by-line analysis of each exercise implementation.

---

## Exercise 1: Valid Parentheses - Stack

### The Solution

```rust
pub fn is_valid(s: &str) -> bool {
    let mut stack = Vec::new();
    let matching = |c: char| match c {
        ')' => '(',
        '}' => '{',
        ']' => '[',
        _ => unreachable!(),
    };

    for c in s.chars() {
        match c {
            '(' | '{' | '[' => stack.push(c),
            ')' | '}' | ']' => {
                if stack.pop() != Some(matching(c)) {
                    return false;
                }
            }
            _ => return false,
        }
    }
    stack.is_empty()
}
```

### Line-by-Line Analysis

```rust
let mut stack = Vec::new();
```
**Purpose:** Create a stack to store opening brackets. `Vec` with `push()` and `pop()` acts as a stack.

```rust
let matching = |c: char| match c {
    ')' => '(',
    '}' => '{',
    ']' => '[',
    _ => unreachable!(),
};
```
**Purpose:** Create a closure that returns the matching opening bracket for a closing bracket.

```rust
'(' | '{' | '[' => stack.push(c),
```
**Purpose:** If it's an opening bracket, push it onto the stack.

```rust
')' | '}' | ']' => {
    if stack.pop() != Some(matching(c)) {
        return false;
    }
}
```
**Purpose:** If it's a closing bracket, pop from stack and check if it matches. If not matching or stack was empty, return false.

```rust
stack.is_empty()
```
**Purpose:** At the end, stack should be empty (all opening brackets were matched). If not empty, there were unmatched opening brackets.

### Complexity Analysis

| Metric | Value | Explanation |
|--------|-------|-------------|
| **Time** | O(n) | Single pass through string |
| **Space** | O(n) | Stack holds at most n characters |

---

## Exercise 2: Longest Valid Parentheses

### The Solution

```rust
pub fn longest_valid_parentheses(s: &str) -> i32 {
    let mut max_len = 0;
    let mut left = 0;
    let mut right = 0;

    // Left to right pass
    for c in s.chars() {
        match c {
            '(' => left += 1,
            ')' => right += 1,
            _ => continue,
        }
        if left == right {
            max_len = max_len.max(2 * left);
        } else if right > left {
            left = 0;
            right = 0;
        }
    }

    // Right to left pass
    left = 0;
    right = 0;
    for c in s.chars().rev() {
        match c {
            '(' => left += 1,
            ')' => right += 1,
            _ => continue,
        }
        if left == right {
            max_len = max_len.max(2 * left);
        } else if left > right {
            left = 0;
            right = 0;
        }
    }
    max_len
}
```

### Key Insight

Two passes handle all cases:
- Left-to-right: Handles "(()"
- Right-to-left: Handles "())"

When counts don't match properly, we reset.

---

## Summary Table

| Exercise | Time | Space | Key Technique |
|----------|------|-------|---------------|
| 1: Is Valid | O(n) | O(n) | Stack |
| 2: Longest | O(n) | O(1) | Two passes |
| 3: Min Add | O(n) | O(1) | Count mismatch |
| 4: Generate | O(4^n) | O(n) | Backtracking |
| 5: Balanced | O(n) | O(n) | Stack |
| 6: Count | O(n) | O(n) | Stack + DP |
| 7: Reverse | O(n) | O(n) | Stack + reconstruct |

## Key Takeaways

1. **Stack** perfectly tracks unmatched brackets
2. **Push opening**, pop and verify closing
3. **Empty at end** means valid
4. **Two passes** handle all cases for longest
5. **Backtracking** generates all combinations
