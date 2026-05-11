# Encode and Decode Strings - LeetCode 271

## Problem Statement

Design an algorithm to encode a list of strings to a single string, and decode the single string back to the list of strings.

```
Example:
Input: ["lint", "code", "love", "you"]
Output: ["lint", "code", "love", "you"]

Encoded string must be self-contained (no external delimiter knowledge)
```

## The Challenge: Variable-Length Strings

How do we encode strings of different lengths so we can decode them?

```
["ab", "cde", "f"] → ???

If we just concatenate with commas: "ab,cde,f"
But what if string contains comma? "a,b" → "a,b" → ambiguous!

We need a self-delimiting encoding scheme.
```

## Approach: Length Prefix

Store each string as: `[length]:[string]`

```
["lint", "code", "love", "you"]
     4     4       4       3
"4:lint4:code4:love3:you"
```

### Why This Works

- Each segment is self-contained
- We read length first, then that many characters
- No ambiguity even with special characters

### Encoding Algorithm

```rust
fn encode(strs: &[String]) -> String {
    let mut result = String::new();
    for s in strs {
        result.push_str(&s.len().to_string());
        result.push(':');
        result.push_str(s);
    }
    result
}
```

Example: "hello" → "5:hello"

### Decoding Algorithm

```rust
fn decode(s: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut i = 0;
    while i < s.len() {
        // Find colon
        let colon_pos = s[i..].find(':').unwrap();
        let len: usize = s[i..i + colon_pos].parse().unwrap();
        i += colon_pos + 1;
        let word = &s[i..i + len];
        result.push(word.to_string());
        i += len;
    }
    result
}
```

## Visual Walkthrough

```
Encode: ["hello", "world", "!"]
────────────────────────────
Step 1: "hello" → "5:hello"
Step 2: "world" → "5:world"
Step 3: "!"     → "1:!"

Result: "5:hello5:world1:!"

Decode: "5:hello5:world1:!"
────────────────────────────
i=0: Read "5", read ":", read 5 chars "hello" → ["hello"]
i=6: Read "5", read ":", read 5 chars "world" → ["hello", "world"]
i=12: Read "1", read ":", read 1 char "!" → ["hello", "world", "!"]
Done!
```

## Edge Cases and Challenges

### Empty Strings

```
["", ""] → "0:0:" → ["", ""]
```

### Strings with Colons

```
["a:b"] → "3:a:b" → ["a:b"] ✓
```

### Strings with Numbers

```
["123:abc"] → "7:123:abc" → ["123:abc"] ✓
```

The length prefix ensures we never misinterpret.

### Unicode Characters

```
["你好"] → "6:你好" (3 bytes per char in UTF-8 × 2 = 6)
```

We use byte length, not character count, which handles UTF-8 correctly.

## Four Approaches Compared

| Approach | Encode | Decode | Special Chars | Empty Strings |
|----------|--------|--------|---------------|---------------|
| Length prefix | O(n) | O(n) | ✓ Safe | ✓ Works |
| Char count prefix | O(n) | O(n) | ✓ Safe | ✓ Works |
| Null delimiter | O(n) | O(n) | ✗ Unsafe | ✗ Breaks |
| JSON wrapper | O(n) | O(n) | ✓ Safe | ✓ Works |

### Length Prefix (Chosen)

**Pros:**
- Handles all characters
- Simple and efficient
- Fixed overhead per string

**Cons:**
- Max length limited by usize (usually 2^64-1)

### Null Delimiter (Unsafe!)

```rust
// UNSAFE - breaks on strings containing '\0'
fn encode(strs: &[String]) -> String {
    strs.join("\0")
}
```

Don't use this for general strings.

## Implementation Details

### Encode Function

```rust
pub fn encode(strs: Vec<String>) -> String {
    let mut encoded = String::new();
    for s in &strs {
        encoded.push_str(&s.len().to_string());
        encoded.push(':');
        encoded.push_str(s);
    }
    encoded
}
```

Time: O(total length)
Space: O(total length)

### Decode Function

```rust
pub fn decode(encoded: &str) -> Vec<String> {
    let mut result = Vec::new();
    let bytes = encoded.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        // Find colon to get length
        let mut colon_idx = i;
        while colon_idx < bytes.len() && bytes[colon_idx] != b':' {
            colon_idx += 1;
        }
        let len: usize = std::str::from_utf8(&bytes[i..colon_idx])
            .unwrap()
            .parse()
            .unwrap();

        // Extract string
        let start = colon_idx + 1;
        let end = start + len;
        let s = std::str::from_utf8(&bytes[start..end]).unwrap();
        result.push(s.to_string());

        i = end;
    }
    result
}
```

Key insight: We work with bytes to correctly handle UTF-8 multi-byte characters.

## Handling Edge Cases

### Empty List

```rust
encode([]) → ""
decode("") → []
```

### Single Empty String

```rust
encode([""]) → "0:"
decode("0:") → [""]
```

### Mixed Empty and Non-Empty

```rust
encode(["", "a", ""]) → "0:1:a0:"
decode("0:1:a0:") → ["", "a", ""]
```

## Complexity Analysis

| Operation | Time | Space |
|-----------|------|-------|
| encode | O(n) | O(n) |
| decode | O(n) | O(n) |

Where n = total length of all strings combined.

## Test Cases Design

### Basic Cases
1. Encode/decode with normal strings
2. Single string
3. Multiple strings

### Edge Cases
4. Empty list
5. Single empty string
6. Multiple empty strings
7. Strings with special characters

### Complex Cases
8. Very long strings
9. Unicode strings
10. Strings containing ':'
11. Strings containing numbers

## Implementation with String Methods

```rust
fn encode(strs: Vec<String>) -> String {
    strs.into_iter()
        .map(|s| format!("{}:{}", s.len(), s))
        .collect::<Vec<_>>()
        .join("")
}

fn decode(s: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut i = 0;
    let bytes = s.as_bytes();

    while i < bytes.len() {
        // Find colon
        let colon_pos = s[i..].find(':').unwrap();
        let len: usize = s[i..i + colon_pos].parse().unwrap();
        i += colon_pos + 1;
        let end = i + len;
        result.push(s[i..end].to_string());
        i = end;
    }
    result
}
```

## Why Not Just Use serde_json?

```rust
// Alternative using serde
serde_json::to_string(&strs).unwrap()
// "\"lint\",\"code\""
```

**Pros:** Handles escaping automatically

**Cons:** Adds overhead, not pure std::collections

The length-prefix approach is preferred for interview because:
1. Shows understanding of encoding schemes
2. Pure Rust, no external dependencies
3. Simple and efficient

## Related Problems

1. **LeetCode 271**: Encode and Decode Strings (this problem)
2. **LeetCode 394**: Decode String (nested decoding)
3. **Serialize and Deserialize BST**

## Time to Complete

**Target**: 30 minutes
**Optimal**: 20 minutes