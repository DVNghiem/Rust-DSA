# Valid Parentheses (LeetCode #20)

## Problem Statement

Given a string `s` containing just the characters `'('`, `')'`, `'{'`, `'}'`, `'['` and `']'`, determine if the input string is valid.

An input string is valid if:
1. Open brackets must be closed by the same type of brackets.
2. Open brackets must be closed in the correct order.
3. Every close bracket has a corresponding open bracket of the same type.

## Examples

```
Input: s = "()"
Output: true

Input: s = "()[]{}"
Output: true

Input: s = "(]"
Output: false

Input: s = "([)]"
Output: false

Input: s = "{[]}"
Output: true
```

## Stack-Based Solution

### Key Insight

Use a stack to track opening brackets:
1. Push opening brackets onto stack
2. When closing bracket found, check if it matches the top of stack
3. At end, stack should be empty

### Visual Walkthrough

```
s = "{[]}"

Step 1: '{' is opening bracket, push onto stack
  stack: ['{']

Step 2: '[' is opening bracket, push onto stack
  stack: ['{', '[']

Step 3: ']' is closing bracket, check top of stack
  Top is '[', which matches ']' → pop
  stack: ['{']

Step 4: '}' is closing bracket, check top of stack
  Top is '{', which matches '}' → pop
  stack: []

End: Stack is empty → Valid!
```

## Implementation

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

## Complexity Analysis Table

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Stack | O(n) | O(n) | Single pass |

## Edge Cases to Consider

1. **Empty string**: Return true
2. **Single character**: Return false
3. **Only closing brackets**: Return false (can't pop)
4. **Only opening brackets**: Return false (stack not empty)
5. **Mismatched types**: "(]" returns false
6. **Wrong order**: "([)]" returns false

## Related Problems

### LeetCode 22: Generate Parentheses
Generate all valid parenthesis combinations.

### LeetCode 32: Longest Valid Parentheses
Find longest valid substring.

### LeetCode 921: Minimum Add to Make Parentheses Valid
Minimum additions to make valid.

## Exercises

### Exercise 1: Basic Valid Parentheses
Implement using stack.

### Exercise 2: Longest Valid Substring
Find longest valid parentheses substring.

### Exercise 3: Minimum Additions
Minimum brackets to add for validity.

### Exercise 4: Generate Parentheses
Generate all valid combinations.

### Exercise 5: Check Bracket Pairs
Validate various bracket types including < >.

## Key Takeaways

1. **Stack tracks** unmatched opening brackets
2. **Matching closing** bracket must match top of stack
3. **Stack empty** at end means valid
4. **O(n) time** - single pass through string
5. **O(n) space** - worst case all opening brackets
