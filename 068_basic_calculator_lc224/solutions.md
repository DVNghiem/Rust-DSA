# Basic Calculator Solution - LeetCode 224 (Complete)

## Solution Analysis

### Stack-Based Evaluation

```rust
pub fn calculate(s: String) -> i32 {
    let bytes = s.as_bytes();
    let mut result = 0;
    let mut sign = 1;
    let mut stack: Vec<i32> = Vec::new();

    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'+' => sign = 1,
            b'-' => sign = -1,
            b'(' => {
                // Save current state
                stack.push(result);
                stack.push(sign);
                result = 0;
                sign = 1;
            }
            b')' => {
                // Apply current result with its sign
                result = result * stack.pop().unwrap() + stack.pop().unwrap();
            }
            b' ' => {}
            c => {
                // Parse multi-digit number
                let mut num = (c - b'0') as i32;
                while i + 1 < bytes.len() && bytes[i + 1].is_ascii_digit() {
                    i += 1;
                    num = num * 10 + (bytes[i] - b'0') as i32;
                }
                result += num * sign;
            }
        }
        i += 1;
    }

    result
}
```

## Key Concepts

### Why Stack?

When we encounter '(', we need to:
1. Save the current result (calculation so far)
2. Save the current sign (positive or negative)
3. Start fresh for the sub-expression

When we encounter ')', we need to:
1. Apply the sub-expression result with its sign
2. Add to the saved result from before '('

## Line-by-Line Analysis

### Initialize
```rust
let mut result = 0;  // Running total
let mut sign = 1;    // Current sign (+1 or -1)
let mut stack: Vec<i32> = Vec::new();  // To save state
```

### When '+' or '-'
```rust
b'+' => sign = 1,
b'-' => sign = -1,
```
- Just update the sign for the next number

### When '('
```rust
b'(' => {
    stack.push(result);  // Save current result
    stack.push(sign);    // Save current sign
    result = 0;          // Reset for new sub-expression
    sign = 1;            // Reset sign (sub-expressions start positive)
}
```
- Push result and sign onto stack
- Reset for computing sub-expression

### When ')'
```rust
b')' => {
    result = result * stack.pop().unwrap() + stack.pop().unwrap();
}
```
- Pop sign (multiplier) and previous result
- Apply: result = result * sign + previous_result

### When Digit
```rust
c => {
    let mut num = (c - b'0') as i32;
    while i + 1 < bytes.len() && bytes[i + 1].is_ascii_digit() {
        i += 1;
        num = num * 10 + (bytes[i] - b'0') as i32;
    }
    result += num * sign;
}
```
- Parse multi-digit number
- Add to result with current sign

## Visual Example

### Input: s = "(1+(4+5+2)-3)+(6+8)"

```
i=0, '(':
  push result(0), push sign(1)
  result=0, sign=1

i=1, '1':
  num=1, result += 1*1 = 1

i=2, '+':
  sign = 1

i=3, '4':
  num=4, result += 4*1 = 5

i=4, '+':
  sign = 1

i=5, '5':
  num=5, result += 5*1 = 10

i=6, '+':
  sign = 1

i=7, '2':
  num=2, result += 2*1 = 12

i=8, ')':
  result = 12 * 1 + 0 = 12

i=9, '-':
  sign = -1

i=10, '3':
  num=3, result += 3*(-1) = 9

i=11, ')':
  result = 9 * 1 + 0 = 9

i=12, '+':
  sign = 1

i=13, '(':
  push result(9), push sign(1)
  result=0, sign=1

i=14, '6':
  num=6, result += 6*1 = 6

i=15, '+':
  sign = 1

i=16, '8':
  num=8, result += 8*1 = 14

i=17, ')':
  result = 14 * 1 + 9 = 23

Final: 23
```

## Complexity Analysis

| Approach | Time | Space |
|----------|------|-------|
| Stack | O(n) | O(n) |

## Edge Cases

### Empty String
```rust
Input: ""
Output: 0
```

### Simple Expression
```rust
Input: "1 + 1"
Output: 2
```

### Nested Parentheses
```rust
Input: "((1))"
Output: 1
```

## Why Save Both result and sign?

When we encounter '(':
- result = calculation up to now
- sign = sign that will apply to sub-expression result

Example: "2-(1+2)"
- After processing "2-(", result = 2, sign = -1
- Inside "(1+2)", result starts at 0, becomes 3
- At ')', result = 3 * (-1) + 2 = 1

The sign stored is the sign that was active when '(' was encountered.

## Common Mistakes

1. **Not resetting sign after '('**: Each sub-expression should start fresh
2. **Wrong order when popping**: Sign is pushed first, so popped second
3. **Not parsing multi-digit numbers**: Single digit parsing is wrong for "12"