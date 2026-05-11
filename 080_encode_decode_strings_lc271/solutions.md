# Solutions Analysis: Encode and Decode Strings (LeetCode 271)

## Solution Overview

We implement a length-prefix encoding scheme where each string is encoded as `[length]:[string]`. This ensures unambiguous decoding even when strings contain special characters.

## Encoding Strategy

### The Length-Prefix Format

```
Original: ["lint", "code", "love", "you"]
Encoded:  "4:lint4:code4:love3:you"
          │└──────┘│└──────┘│└──────┘│└──┘
          │        │        │        └── "you" (3 chars)
          │        │        └── "love" (4 chars)
          │        └── "code" (4 chars)
          └── "lint" (4 chars)
```

Each segment is self-contained: we read the length first, then read exactly that many characters.

## Encode Function Deep Dive

### Version 1: String Appending

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

**Step-by-step:**

1. `let mut encoded = String::new()` - Creates empty string
2. `for s in &strs` - Iterates over all input strings
3. `encoded.push_str(&s.len().to_string())` - Appends length as string
4. `encoded.push(':')` - Appends delimiter
5. `encoded.push_str(s)` - Appends actual string content

**Example trace:**
```
s = "hello"
encoded.len().to_string() = "5"
encoded.push(':')
encoded.push_str("hello")
encoded = "5:hello"
```

### Version 2: Functional Style

```rust
pub fn encode_v2(strs: Vec<String>) -> String {
    strs.into_iter()
        .map(|s| format!("{}:{}", s.len(), s))
        .collect::<Vec<_>>()
        .join("")
}
```

**Step-by-step:**

1. `strs.into_iter()` - Takes ownership
2. `.map(|s| format!("{}:{}", s.len(), s))` - Creates encoded segment for each
3. `.collect::<Vec<_>>()` - Collects into vector
4. `.join("")` - Concatenates all segments

**Why collect then join?**
- `format!` creates individual strings
- `join` concatenates with no separator

## Decode Function Deep Dive

### Version 1: Byte-based Parsing

```rust
pub fn decode(encoded: &str) -> Vec<String> {
    let mut result = Vec::new();
    let bytes = encoded.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        // Find colon position
        let mut colon_pos = i;
        while colon_pos < bytes.len() && bytes[colon_pos] != b':' {
            colon_pos += 1;
        }

        if colon_pos >= bytes.len() {
            break; // Invalid encoding
        }

        // Parse length
        let len_str = std::str::from_utf8(&bytes[i..colon_pos]).unwrap_or("");
        let len: usize = len_str.parse().unwrap_or(0);

        // Extract the string
        let start = colon_pos + 1;
        if start + len > bytes.len() {
            break; // Invalid encoding
        }

        let s = std::str::from_utf8(&bytes[start..start + len]).unwrap_or("");
        result.push(s.to_string());

        i = start + len;
    }
    result
}
```

**Step-by-step:**

1. `let bytes = encoded.as_bytes()` - Get raw bytes for O(1) indexing
2. `let mut i = 0` - Start index
3. First while loop finds colon by scanning bytes

**Tracing "5:hello5:world":**

```
i=0:
  colon_pos scans from 0, finds ':' at index 1
  len_str = "5" (bytes[0..1])
  len = 5
  start = 2
  s = std::str::from_utf8(&bytes[2..7]) = "hello"
  result = ["hello"]
  i = 7

i=7:
  colon_pos scans from 7, finds ':' at index 8
  len_str = "5" (bytes[7..8])
  len = 5
  start = 9
  s = std::str::from_utf8(&bytes[9..14]) = "world"
  result = ["hello", "world"]
  i = 14 (end)
```

### Version 2: Using str::find()

```rust
pub fn decode_v2(encoded: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut i = 0;

    while i < encoded.len() {
        if let Some(colon_pos) = encoded[i..].find(':') {
            let actual_colon = i + colon_pos;
            if let Ok(len) = encoded[i..actual_colon].parse::<usize>() {
                let start = actual_colon + 1;
                let end = start + len;
                if end <= encoded.len() {
                    result.push(encoded[start..end].to_string());
                    i = end;
                    continue;
                }
            }
        }
        break; // Invalid
    }
    result
}
```

**Key difference:** Uses `str::find()` instead of manual byte scanning.

## Why Use Bytes Instead of Chars?

```rust
// UTF-8 encoding examples:
"a"      → [97]           (1 byte)
"你"     → [228, 184, 189] (3 bytes)
"hello"  → [104, 101, 108, 108, 111] (5 bytes)
```

By using `as_bytes()`, we get the byte length (what matters for storage and indexing), not character count. This correctly handles multi-byte Unicode characters.

**Incorrect approach using chars:**
```rust
// WRONG - char count vs byte count
let len = s.chars().count(); // Will give wrong result for UTF-8
```

## Edge Cases Analysis

### Empty String

```
Input: [""]
Encode: "0:"
Decode:
  i=0: colon_pos=1, len_str="0", len=0
       start=2, end=2, s="" (empty slice)
       result=[""]
```

### Empty List

```
Input: []
Encode: "" (empty string)
Decode: while i < 0 (false), returns []
```

### Mixed Empty and Non-Empty

```
Input: ["", "a", ""]
Encode: "0:1:a0:"
Decode:
  i=0: colon_pos=1, len=0, s="", result=[""]
  i=2: colon_pos=3, len=1, s="a", result=["", "a"]
  i=5: colon_pos=6, len=0, s="", result=["", "a", ""]
```

### Strings Containing Colons

```
Input: ["a:b"]
Encode: "3:a:b"
Decode:
  i=0: colon_pos=1, len_str="3", len=3
       start=2, end=5
       s = bytes[2..5] = "a:b"
  result=["a:b"] ✓
```

The length prefix is the key: we know to read exactly 3 characters after the colon, so "a:b" is read correctly even though it contains a colon.

### Unicode Strings

```
Input: ["你好"]
Encode: "6:你好" (2 chars × 3 bytes each = 6 bytes)
Decode:
  i=0: colon_pos=1, len_str="6", len=6
       start=2, end=8
       s = bytes[2..8] = "你好" (6 bytes)
  result=["你好"] ✓
```

## Complexity Analysis

### Encode

| Metric | Value |
|--------|-------|
| Time | O(n) where n = total input length |
| Space | O(n) for the encoded string |

### Decode

| Metric | Value |
|--------|-------|
| Time | O(n) |
| Space | O(n) for result + O(1) extra |

## Why Not JSON Encoding?

```rust
// Using serde_json
serde_json::to_string(&strs).unwrap()
// Returns: "[\"lint\",\"code\",\"love\"]"
```

**Pros:**
- Handles all edge cases
- Standard format

**Cons:**
- External dependency
- Added overhead ("[\"]" characters)
- Not "pure" interview solution

The length-prefix approach is preferred for interviews because:
1. Demonstrates understanding of encoding
2. No external dependencies
3. Simple and efficient

## Common Pitfalls

### Pitfall 1: Using char length instead of byte length

```rust
// WRONG
encoded.push_str(&s.chars().count().to_string());

// CORRECT
encoded.push_str(&s.len().to_string());  // bytes, not chars
```

### Pitfall 2: Off-by-one in decode

```rust
// WRONG - incorrect index calculation
let end = start + len - 1;  // Off by one!

// CORRECT
let end = start + len;  // Exclusive end index
```

### Pitfall 3: Not handling invalid encoding

```rust
// Need to check bounds before slicing
if start + len > bytes.len() {
    break; // Invalid, don't panic
}
```

## Test Coverage Analysis

### Basic Tests
- Normal strings
- Single string
- Multiple strings

### Empty Tests
- Empty list
- Single empty string
- Multiple empty strings

### Special Character Tests
- Colons in strings
- Numbers in strings
- Unicode characters
- Whitespace characters

### Edge Cases
- Long strings
- Many small strings
- Mixed empty and non-empty

## Alternative Approaches

### 1. Count Prefix (Character-based)

```rust
// Encode: "count#string" for each
// e.g., "4#lint" instead of "4:lint"
```

Similar to our approach but different delimiter.

### 2. Null Delimiter (UNSAFE!)

```rust
fn encode(strs: Vec<String>) -> String {
    strs.join("\0")
}
```

**PROBLEMS:**
- Breaks on strings containing '\0'
- Not self-delimiting in a general sense

### 3. JSON/Wrapper

```rust
serde_json::to_string(&strs).unwrap()
```

Safe but requires external crate.

## Conclusion

The length-prefix encoding scheme is:

1. **Simple**: Easy to understand and implement
2. **Efficient**: O(n) time and space
3. **Safe**: Handles all edge cases including special characters
4. **Self-contained**: No external dependencies needed

Key insights:
- Use byte length (not char count) for correct UTF-8 handling
- Scan for colon to get length, then read exact number of bytes
- Always validate bounds before slicing to avoid panics