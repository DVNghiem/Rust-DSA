# Solutions Analysis: Design TinyURL (LeetCode 535)

## Solution Overview

We implement two URL shortening approaches:
1. **Counter-based encoding** (primary) - Uses incrementing counter converted to base-62
2. **Hash-based encoding** (alternative) - Uses hash function to generate short keys

## Key Insight: Base-62 Encoding

62 characters available:
- 0-9: digits (10 characters)
- A-Z: uppercase letters (26 characters)
- a-z: lowercase letters (26 characters)

Total: 10 + 26 + 26 = 62

This allows representing numbers in base-62, which is perfect for generating compact URL keys.

## Codec Structure Deep Dive

### Counter-Based Codec

```rust
pub struct Codec {
    counter: u64,
    short_to_long: HashMap<String, String>,
    long_to_short: HashMap<String, String>,
}
```

**Line-by-line analysis:**

1. `counter: u64` - Incrementing ID for each new URL
2. `short_to_long: HashMap<String, String>` - Maps short key to original URL
3. `long_to_short: HashMap<String, String>` - Maps original URL to short key (for duplicate detection)

**Why two maps?**
- `short_to_long`: For O(1) decode operations
- `long_to_short`: For O(1) duplicate detection during encode

### Encode Operation

```rust
pub fn encode(&mut self, long_url: &str) -> String {
    // If already encoded, return existing short URL
    if let Some(short) = self.long_to_short.get(long_url) {
        return short.clone();
    }

    // Generate new short URL
    let id = self.counter;
    self.counter += 1;

    let short_url = id_to_short_url(id);
    let full_short_url = format!("http://tinyurl.com/{}", short_url);

    // Store mappings
    self.short_to_long.insert(short_url.clone(), long_url.to_string());
    self.long_to_short.insert(long_url.to_string(), short_url.clone());

    full_short_url
}
```

**Step-by-step:**

1. **Lines 3-5**: Check if URL already exists in our system
   - If yes, return existing short URL (no duplicates)
   - This ensures same long URL always produces same short URL

2. **Lines 7-8**: Get current ID and increment counter

3. **Lines 10-11**: Convert ID to base-62 string and create full URL

4. **Lines 14-15**: Store both directions of mapping

**Time complexity**: O(1) for encode (hash map operations)

### Decode Operation

```rust
pub fn decode(&self, short_url: &str) -> Option<String> {
    // Extract the key from "http://tinyurl.com/{key}"
    let parts: Vec<&str> = short_url.split('/').collect();
    if let Some(key) = parts.last() {
        return self.short_to_long.get(key).cloned();
    }
    None
}
```

**Step-by-step:**

1. **Line 3**: Split URL by '/' separator
   - "http://tinyurl.com/4e9iBt" → ["http:", "", "tinyurl.com", "4e9iBt"]

2. **Lines 4-6**: Get last part (the key), look up in map

**Time complexity**: O(1) for decode (hash map lookup)

## Base-62 Conversion Deep Dive

### ID to Short URL

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

**Tracing id_to_short_url(1000):**

```
n = 1000
n % 62 = 1000 % 62 = 12 → 'C'
n = 1000 / 62 = 16

n % 62 = 16 % 62 = 16 → 'G'
n = 16 / 62 = 0

result = ['C', 'G']
reverse = ['G', 'C']
output = "GC"
```

**Tracing id_to_short_url(0):**

```
special case: id == 0
return "0" (first character of alphabet)
```

### Short URL to ID (Reverse)

```rust
fn short_url_to_id(short: &str) -> Option<u64> {
    const ALPHABET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

    let mut result: u64 = 0;

    for c in short.chars() {
        let idx = ALPHABET.iter().position(|&x| x as char == c)? as u64;
        result = result * 62 + idx;
    }

    Some(result)
}
```

**Tracing short_url_to_id("GC"):**

```
c = 'G'
idx = 16 (0-9=0-9, A-Z=10-35, a-z=36-61)
result = 0 * 62 + 16 = 16

c = 'C'
idx = 12
result = 16 * 62 + 12 = 992 + 12 = 1004? Wait...

Let me recalculate:
G = 16 (A=10, B=11, C=12, ... G=16)
C = 12 (A=10, B=11, C=12)

result = 0
result = 0 * 62 + 16 = 16
result = 16 * 62 + 12 = 992 + 12 = 1004

But GC should map back to 1000...
```

Wait, there's a discrepancy. Let me trace through again:

Actually for "GC": G=16, C=12

Position matters! "GC" means G is most significant.

result = 0
result = 0 * 62 + 16 = 16 (for G)
result = 16 * 62 + 12 = 1004

But we expect GC = 1000 based on forward conversion...

Let me check forward conversion again for id=1000:

```
id = 1000
1000 / 62 = 16 remainder 8
16 / 62 = 0 remainder 16

remainders: [8, 16]
reverse: [16, 8]
chars: ALPHABET[16]='G', ALPHABET[8]='8'
result: "G8"

So 1000 → "G8", not "GC"!
```

I made an error in my earlier trace. The correct encoding is:

```
id_to_short_url(1000):
  n = 1000
  1000 % 62 = 8 → '8'
  n = 1000 / 62 = 16
  16 % 62 = 16 → 'G'
  n = 16 / 62 = 0
  result = ['8', 'G']
  reverse = ['G', '8']
  output = "G8"
```

And "G8" → 16*62 + 8 = 1000. Correct!

## Hash-Based Alternative

### Hash Function

```rust
fn hash_url(url: &str) -> String {
    let mut hasher = DefaultHasher::new();
    url.hash(&mut hasher);
    let hash = hasher.finish();

    const ALPHABET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

    let mut result = Vec::new();
    let mut h = hash;

    for _ in 0..6 {
        result.push(ALPHABET[(h % 62) as usize] as char);
        h /= 62;
    }

    result.reverse();
    result.into_iter().collect()
}
```

**Step-by-step:**

1. `url.hash(&mut hasher)` - Uses Rust's built-in hash function
2. `hasher.finish()` - Gets 64-bit hash value
3. Extract 6 base-62 characters using modulo and division

**Key difference from counter-based:**
- Same URL always produces same hash (deterministic)
- Different URLs produce different hashes (with very high probability)
- No counter needed

### Collision Handling

```rust
while self.short_to_long.contains_key(&short) {
    if self.short_to_long.get(&short) == Some(&long_url.to_string()) {
        break;
    }
    // Collision - try alternative
    short = format!("{}{}", original, counter);
    counter += 1;
}
```

If hash collision occurs (extremely rare with 62^6 ≈ 56 billion possibilities):
1. Check if existing entry matches our URL (same URL, same hash)
2. If not, append counter to make unique

## Complexity Analysis

### Counter-Based

| Operation | Time | Space |
|-----------|------|-------|
| encode | O(1) | O(1) |
| decode | O(1) | O(1) |
| storage | - | O(n) |

### Hash-Based

| Operation | Time | Space |
|-----------|------|-------|
| encode | O(1) | O(1) |
| decode | O(1) | O(1) |
| storage | - | O(n) |

Both achieve O(1) for encode/decode operations.

## Trade-offs

| Aspect | Counter-Based | Hash-Based |
|--------|---------------|------------|
| Collision risk | None (guaranteed unique) | Very rare (62^6 combos) |
| Predictability | Sequential IDs (security concern) | Hash-based (not predictable) |
| URL stability | Same long URL → Same short URL | Same long URL → Same hash |
| Large-scale | Counter may overflow (u64 max) | Hash always works |

## Edge Cases Analysis

### Empty URL Path

```rust
encode("https://example.com") → "http://tinyurl.com/0"
decode("http://tinyurl.com/0") → "https://example.com"
```

Works correctly.

### Duplicate Encoding

```rust
encode("https://example.com") → "http://tinyurl.com/0"
encode("https://example.com") → "http://tinyurl.com/0" (same!)
```

`long_to_short` map ensures same URL returns same short URL.

### Very Long URLs

```rust
let long_url = "https://example.com/".repeat(100);
```

Our implementation accepts any length. Real TinyURL has a limit of ~2048 chars.

### Unicode URLs

```rust
encode("https://example.com/你好")
```

Works because we store the exact URL string, not processing it.

## Why Use Both HashMaps?

```
long_to_short: Long URL → Short Key (for duplicate detection)
short_to_long: Short Key → Long URL (for decode)
```

Having both enables O(1) for both operations:
- Encode: Check if already exists → O(1) lookup
- Decode: Find original URL → O(1) lookup

## Test Coverage Analysis

### Basic Tests
- Single URL encode/decode
- Multiple different URLs
- Decode non-existent

### Edge Cases
- Duplicate URL returns same short
- Long URLs
- URLs with special characters
- URLs with unicode

### Complexity Tests
- Many encodes (1000 URLs)
- Sequential IDs work correctly

### Round-Trip Tests
- ID ↔ short URL conversion
- Hash stability

## Common Mistakes

### Mistake 1: Not handling duplicate URLs

```rust
// WRONG - always creates new short URL
pub fn encode(&mut self, long_url: &str) -> String {
    let id = self.counter;
    self.counter += 1;
    // ...
}

// CORRECT - check for existing
pub fn encode(&mut self, long_url: &str) -> String {
    if let Some(short) = self.long_to_short.get(long_url) {
        return short.clone();
    }
    // ...
}
```

### Mistake 2: Forgetting to increment counter

```rust
// WRONG
pub fn encode(&mut self, long_url: &str) -> String {
    let id = self.counter;
    // forgot: self.counter += 1
    // ...
}
```

### Mistake 3: Not handling collisions in hash approach

```rust
// Need while loop to handle rare collisions
while self.short_to_long.contains_key(&short) {
    // handle collision
}
```

## Alternative Approaches Considered

### 1. Direct Base-36 (lowercase only)

```
0123456789abcdefghijklmnopqrstuvwxyz (36 chars)
```

**Disadvantage:** Longer strings for same ID space.

### 2. UUID-based

```
generate UUID → take first 6 chars
```

**Disadvantage:** Not deterministic, collisions possible.

### 3. JSON storage (external)

```rust
serde_json::to_string(&urls)
```

**Disadvantage:** External dependency, slower.

## Conclusion

The counter-based approach is preferred for this problem:

1. **Guaranteed unique keys** - No collisions possible
2. **O(1) operations** - Hash maps provide constant time
3. **Simple to implement** - Base-62 conversion is straightforward
4. **Complete encode/decode cycle** - Can convert back and forth

The hash-based approach is an alternative with:
1. **Deterministic output** - Same URL always produces same hash
2. **Rare collisions** - 62^6 key space is very large
3. **No counter management** - Uses hash instead of incrementing ID

Both achieve O(1) encode/decode as required by the problem.