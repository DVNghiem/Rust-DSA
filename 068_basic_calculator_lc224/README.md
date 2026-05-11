# Basic Calculator - LeetCode 224

## Problem Overview

Given a string `s` containing '(', ')', '+', '-', and spaces, evaluate the expression.

**Examples:**
```
Input: s = "1 + 1"
Output: 2

Input: s = "2-1 + 2"
Output: 3

Input: s = "(1+(4+5+2)-3)+(6+8)"
Output: 23
```

## Theory

### Stack-Based Evaluation

- '+' followed by 'number' → push number
- '-' followed by 'number' → push -number
- '(' → push current result, reset sign
- ')' → pop and add to result

```
Expression: "2-(1+2)"
- '2' → push 2, result = 2
- '-' → sign = -1
- '(' → push result(2), sign(1), reset
- '1' → push 1, result = 1
- '+' → sign = +1
- '2' → push 2, result = 3
- ')' → pop 3, add to prev result(2) = 5

Result: 5 - 2 = 3
```

## Implementation

```rust
pub fn calculate(s: String) -> i32 {
    let mut result = 0;
    let mut sign = 1;
    let mut stack: Vec<i32> = Vec::new();
    let bytes = s.as_bytes();

    for i in 0..bytes.len() {
        match bytes[i] {
            b'+' => sign = 1,
            b'-' => sign = -1,
            b'(' => {
                stack.push(result);
                stack.push(sign);
                result = 0;
                sign = 1;
            }
            b')' => {
                result = result * stack.pop().unwrap() + stack.pop().unwrap();
            }
            b' ' => continue,
            c => {
                let mut num = (c - b'0') as i32;
                while i + 1 < bytes.len() && bytes[i + 1].is_ascii_digit() {
                    num = num * 10 + (bytes[i + 1] - b'0') as i32;
                }
                result += num * sign;
            }
        }
    }

    result
}
```

## Test Cases

```rust
#[test]
fn test_calculate_basic() {
    assert_eq!(calculate("1 + 1".to_string()), 2);
}
```