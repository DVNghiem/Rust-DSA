# Solutions: Evaluate Reverse Polish Notation (LeetCode #150)

This document provides detailed, line-by-line analysis of each exercise implementation.

---

## Exercise 1: Eval RPN - Stack

### The Solution

```rust
pub fn eval_rpn(tokens: &[&str]) -> i32 {
    let mut stack: Vec<i32> = Vec::new();

    for token in tokens {
        match *token {
            "+" | "-" | "*" | "/" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                let result = match *token {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => a / b,
                    _ => unreachable!(),
                };
                stack.push(result);
            }
            _ => {
                stack.push(token.parse().unwrap());
            }
        }
    }
    stack.pop().unwrap()
}
```

### Line-by-Line Analysis

```rust
let mut stack: Vec<i32> = Vec::new();
```
**Purpose:** Create a stack to hold intermediate values and operands.

```rust
for token in tokens {
```
**Purpose:** Iterate through each token in the RPN expression.

```rust
"+" | "-" | "*" | "/" => {
    let b = stack.pop().unwrap();
    let a = stack.pop().unwrap();
```
**Purpose:** When an operator is encountered, pop two operands. Important: `a` is popped second (was pushed earlier), so `a` is the first operand.

```rust
let result = match *token {
    "+" => a + b,
    "-" => a - b,
    "*" => a * b,
    "/" => a / b,
    _ => unreachable!(),
};
```
**Purpose:** Compute the operation. Note that for subtraction and division, `a` is the first operand, `b` is the second.

```rust
_ => {
    stack.push(token.parse().unwrap());
}
```
**Purpose:** If token is not an operator, it's a number. Parse and push onto stack.

```rust
stack.pop().unwrap()
```
**Purpose:** At the end, stack should contain exactly one value - the result.

### Complexity Analysis

| Metric | Value | Explanation |
|--------|-------|-------------|
| **Time** | O(n) | Each token processed once |
| **Space** | O(n) | Stack holds at most n/2 + 1 values |

### Division Truncation

Rust's integer division truncates toward zero, which is the behavior required by the problem:
- 7 / 3 = 2
- -7 / 3 = -2
- 7 / -3 = -2
- -7 / -3 = 2

---

## Summary Table

| Exercise | Time | Space | Key Technique |
|----------|------|-------|---------------|
| 1: Eval RPN | O(n) | O(n) | Stack |
| 2: Calculator | O(n) | O(n) | Stack + parens |
| 3: Tree | O(n) | O(n) | Recursion |
| 4: Valid RPN | O(n) | O(n) | Count check |
| 5: Infix to RPN | O(n) | O(n) | Shunting-yard |
| 6: Min Value | O(n) | O(n) | DP |

## Key Takeaways

1. **Stack processes** RPN efficiently
2. **Order matters** for - and / operations
3. **Push numbers, pop and compute** for operators
4. **Final result** is the only item left on stack
5. **Rust division** already truncates toward zero