# Design TinyURL - LeetCode 535

## Problem Statement

Design a URL shortener service similar to TinyURL.

```
Features required:
1. encode(longUrl): Convert long URL to short URL
2. decode(shortUrl): Convert short URL back to long URL

Example:
Input: "https://leetcode.com/problems/design-tinyurl"
Output: "http://tinyurl.com/4e9iBt"

Both encode and decode must be O(1) time complexity.
```

## Understanding URL Encoding

URL shortening maps a long string to a short string:

```
Long URL (50+ chars) → Short URL (7 chars)
https://example.com/very/long/path?param=value&other=123
                              ↓
                          4e9iBt
```

The short URL must uniquely identify the long URL, and we must be able to decode it back.

## Approach: Base-62 Encoding

Use a character set of 62 characters:
- A-Z (uppercase): 26 chars
- a-z (lowercase): 26 chars
- 0-9 (digits): 10 chars

Total: 26 + 26 + 10 = 62 characters

Each character represents a digit in base-62.

### Encoding URL to Number

```rust
const CHARS: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

fn to_base62(num: u64) -> String {
    let mut result = Vec::new();
    let mut n = num;
    loop {
        result.push(CHARS[(n % 62) as usize] as char);
        n /= 62;
        if n == 0 { break; }
    }
    result.reverse();
    String::from_utf8(result).unwrap()
}
```

## Visual Walkthrough

```
Long URL: "https://leetcode.com/problems/design-tinyurl"
              ↓
         Hash to number
              ↓
         5181073248321 (example)
              ↓
         Convert to base-62
              ↓
         "4e9iBt" (6 chars)

Short URL: "http://tinyurl.com/4e9iBt"
```

### Reverse Process

```
Short URL: "http://tinyurl.com/4e9iBt"
                ↓
           Extract "4e9iBt"
                ↓
           Convert from base-62
                ↓
           5181073248321
                ↓
           Lookup in map
                ↓
           Original long URL
```

## Data Structure Design

### Two Main Approaches

1. **Hash-based**: Use a hash function (like MD5) and take first 6 chars
2. **Counter-based**: Use incrementing counter, convert to base-62

### Counter-based Approach

```rust
struct Codec {
    counter: u64,
    map: HashMap<u64, String>,
}

impl Codec {
    fn encode(&mut self, longUrl: &str) -> String {
        let id = self.counter;
        self.counter += 1;
        self.map.insert(id, longUrl.to_string());
        format!("http://tinyurl.com/{}", to_base62(id))
    }
}
```

**Problem**: Predictable URLs (security issue), counter eventually overflows.

### Hash-based Approach

```rust
struct Codec {
    map: HashMap<String, String>,  // short -> long
}

impl Codec {
    fn encode(&mut self, longUrl: &str) -> String {
        let hash = md5(longUrl);
        let short = &hash[..6];
        self.map.insert(short.to_string(), longUrl.to_string());
        format!("http://tinyurl.com/{}", short)
    }
}
```

**Problem**: Hash collisions possible, though unlikely with 6 chars.

## Collision Handling

With base-62 and 6 characters:
- Possible values: 62^6 = ~56 billion combinations
- Very low collision probability with good hash function

Even if collision occurs:

```rust
if self.map.contains_key(&short) {
    // Try alternative: append more chars or use different hash
}
```

## Four Approaches Compared

| Approach | Encode | Decode | Collisions | Predictability |
|----------|--------|--------|------------|----------------|
| Counter (sequential) | O(1) | O(1) | None | Predictable |
| Counter (random offset) | O(1) | O(1) | None | Less predictable |
| MD5 hash (6 chars) | O(1) | O(1) | Very rare | Not predictable |
| SHA-256 (first 6) | O(1) | O(1) | Very rare | Not predictable |

## Implementation Details

### Base Conversion

```rust
fn id_to_short_url(id: u64) -> String {
    const ALPHABET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

    if id == 0 {
        return String::from_utf8(vec![ALPHABET[0]]).unwrap();
    }

    let mut result = Vec::new();
    let mut n = id;

    while n > 0 {
        result.push(ALPHABET[(n % 62) as usize] as char);
        n /= 62;
    }

    result.reverse();
    result.into_iter().collect()
}
```

### Hash Function

```rust
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn hash_url(url: &str) -> String {
    let mut hasher = DefaultHasher::new();
    url.hash(&mut hasher);
    let hash = hasher.finish();

    // Convert hash to base62 string
    let mut result = Vec::new();
    let mut h = hash;
    for _ in 0..6 {
        result.push(ALPHABET[(h % 62) as usize] as char);
        h /= 62;
    }
    result.into_iter().rev().collect()
}
```

## Edge Cases and Considerations

### Long URLs

TinyURL has a limit of ~2048 characters for long URLs. We should handle this:

```rust
if longUrl.len() > 2048 {
    // Handle error
}
```

### Special Characters in URLs

URLs often contain special characters (`?`, `&`, `=`, `#`, etc.). Our base-62 encoding handles these via hashing, not direct encoding.

### Duplicate URLs

```
encode("https://example.com") → "4e9iBt"
encode("https://example.com") → "4e9iBt" (same, correct!)
```

We store in map with short URL as key, so duplicates are handled naturally.

### Case Sensitivity

TinyURL is case-sensitive: `4e9iBt` ≠ `4E9iBt`

Our base-62 uses uppercase for values 10-35, lowercase for 36-61.

## Complexity Analysis

| Operation | Time | Space |
|-----------|------|-------|
| encode | O(1) | O(n) for storage |
| decode | O(1) | O(1) |

Storage grows with number of unique URLs.

## Test Cases Design

### Basic Cases
1. Encode single URL
2. Decode short URL back to original
3. Multiple different URLs

### Edge Cases
4. Very long URL (near limit)
5. URL with special characters
6. Duplicate URLs (same encode result)

### Complex Cases
7. Many URLs (test counter/hash)
8. URL with unicode characters
9. Empty-like URL

## Related Problems

1. **LeetCode 535**: Design TinyURL (this problem)
2. **LeetCode 706**: Design HashMap
3. **LeetCode 271**: Encode/Decode Strings

## Time to Complete

**Target**: 30 minutes
**Optimal**: 20 minutes

## Implementation Note

The problem doesn't specify hash function, so any O(1) encoding is valid. Counter-based is simpler but predictable; hash-based is more realistic.