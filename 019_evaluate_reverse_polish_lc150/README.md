# Evaluate Reverse Polish Notation (LeetCode #150)

## Problem Statement

Evaluate the value of an arithmetic expression in Reverse Polish Notation (RPN).

Valid operators are `+`, `-`, `*`, and `/`. Each operand may be an integer or another expression.

The division always truncates toward zero.

## Examples

```
Input: tokens = ["2", "1", "+", "3", "*"]
Output: 9
Explanation: ((2 + 1) * 3) = 9

Input: tokens = ["4", "13", "5", "/", "+"]
Output: 6
Explanation: (4 + (13 / 5)) = 6

Input: tokens = ["10", "6", "9", "3", "/", "+", "*", "17", "5", "+"]
Output: 22
Explanation: ((10 * (6 / (9 + 3))) + 17) + 5 = 22
```

## Stack-Based Solution

### Key Insight

Use a stack to evaluate RPN:
1. Push numbers onto stack
2. When operator encountered, pop two operands, compute, push result
3. At end, stack contains single result

### Visual Walkthrough

```
tokens = ["4", "13", "5", "/", "+"]

Step 1: "4" is number, push onto stack
  stack: [4]

Step 2: "13" is number, push onto stack
  stack: [4, 13]

Step 3: "5" is number, push onto stack
  stack: [4, 13, 5]

Step 4: "/" is operator, pop 5 and 13
  result = 13 / 5 = 2 (truncated toward zero)
  push result
  stack: [4, 2]

Step 5: "+" is operator, pop 2 and 4
  result = 4 + 2 = 6
  push result
  stack: [6]

Result: 6
```

## Implementation

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

## Complexity Analysis Table

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Stack | O(n) | O(n) | Single pass |

## Key Takeaways

1. **Stack-based** evaluation
2. **Pop two operands** for each operator
3. **First popped is second operand** (order matters for - and /)
4. **Division truncates toward zero**
5. **Single pass** O(n) time

## Real-World Applications

1. **Calculator implementations**: Evaluate expressions
2. **Compiler design**: Code generation
3. **Postfix notation**: For printers/formatters
4. **Mathematical parsing**: Spreadsheet formulas
5. **State machines**: Operation precedence