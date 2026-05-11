# Longest Valid Parentheses - LeetCode 32

## Problem Overview

Given a string containing '(' and ')', find the length of the longest valid (well-formed) parentheses substring.

**Examples:**
```
Input: s = ")()())"
Output: 4 (valid: "()()")

Input: s = "(()"
Output: 2 (valid: "()")

Input: s = ""
Output: 0
```

## Theory

### Approach 1: Stack-Based

Use a stack storing indices. Push -1 as base. When ')' is found, pop and calculate length.

```
s = ")()())"

i=0: ')' → pop -1, len=1 → stack empty, push 0
i=1: '(' → push 1
i=2: ')' → pop 1, len=2-1=1
i=3: ')' → pop 0, len=3-(-1)=4
```

### Approach 2: Two-Pass Scan

Left-to-right and right-to-left passes to track valid lengths.

## Implementation

```rust
pub fn longest_valid_parentheses(s: String) -> i32 {
    use std::collections::VecDeque;

    let bytes = s.as_bytes();
    let mut max_len = 0;
    let mut stack = VecDeque::new();
    stack.push_back(-1);

    for (i, &c) in bytes.iter().enumerate() {
        if c == b'(' {
            stack.push_back(i as i32);
        } else {
            stack.pop_back();
            if stack.is_empty() {
                stack.push_back(i as i32);
            } else {
                max_len = max_len.max(i as i32 - stack.back().unwrap());
            }
        }
    }

    max_len
}
```

## Test Cases

```rust
#[test]
fn test_longest_basic() {
    assert_eq!(longest_valid_parentheses("()".to_string()), 2);
}
```