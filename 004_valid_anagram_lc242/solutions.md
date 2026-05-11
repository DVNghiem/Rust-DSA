# Solutions: Valid Anagram (LeetCode #242)

This document provides detailed, line-by-line analysis of each exercise implementation.

---

## Exercise 1: Valid Anagram - HashMap

### The Solution

```rust
pub fn is_anagram(s: &str, t: &str) -> bool {
    use std::collections::HashMap;
    let mut count = HashMap::new();

    for c in s.chars() {
        *count.entry(c).or_insert(0) += 1;
    }

    for c in t.chars() {
        *count.entry(c).or_insert(0) -= 1;
    }

    count.values().all(|&v| v == 0)
}
```

### Line-by-Line Analysis

```rust
let mut count = HashMap::new();
```
**Purpose:** Create a new empty HashMap to store character counts. Key is char, value is i32.

```rust
for c in s.chars() {
    *count.entry(c).or_insert(0) += 1;
}
```
**Purpose:** Increment count for each character in s.
- `count.entry(c)` returns a VacantEntry or OccupiedEntry
- `or_insert(0)` inserts 0 if the entry is vacant
- `+= 1` increments the value

```rust
for c in t.chars() {
    *count.entry(c).or_insert(0) -= 1;
}
```
**Purpose:** Decrement count for each character in t.
- Same pattern as incrementing s

```rust
count.values().all(|&v| v == 0)
```
**Purpose:** Check if all counts are zero.
- `count.values()` returns an iterator over all values
- `all(|&v| v == 0)` returns true if all values equal 0

### Complexity Analysis

| Metric | Value | Explanation |
|--------|-------|-------------|
| **Time** | O(n) | Two passes through strings |
| **Space** | O(k) | k = unique characters in s and t |

### Early Exit Optimization

```rust
pub fn is_anagram_optimized(s: &str, t: &str) -> bool {
    if s.len() != t.len() {
        return false;
    }
    // ... rest of implementation
}
```

Adding a length check first makes the algorithm O(n) best case for mismatched lengths.

---

## Exercise 2: Valid Anagram - Fixed Size Array

### The Solution

```rust
pub fn is_anagram_array(s: &str, t: &str) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut count = [0; 26];

    for c in s.chars() {
        count[(c as u8 - b'a') as usize] += 1;
    }

    for c in t.chars() {
        count[(c as u8 - b'a') as usize] -= 1;
    }

    count.iter().all(|&c| c == 0)
}
```

### Line-by-Line Analysis

```rust
if s.len() != t.len() {
    return false;
}
```
**Purpose:** Early exit for different length strings. Anagrams must have the same length.

```rust
let mut count = [0; 26];
```
**Purpose:** Create an array of 26 zeros. One for each letter a-z.

```rust
count[(c as u8 - b'a') as usize] += 1;
```
**Purpose:** Convert char to array index.
- `c as u8` converts char to byte (assuming ASCII)
- `b'a'` is the byte value of 'a' (97)
- Subtracting gives 0 for 'a', 1 for 'b', etc.
- `as usize` converts to array index type

```rust
count.iter().all(|&c| c == 0)
```
**Purpose:** Check that all counts are zero.

### Why This Works for Lowercase Letters

The problem of valid anagram with lowercase English letters has exactly 26 possible characters. We can use direct indexing instead of hashing because:
- Index 0 represents 'a'
- Index 1 represents 'b'
- ...
- Index 25 represents 'z'

### Complexity Analysis

| Metric | Value | Explanation |
|--------|-------|-------------|
| **Time** | O(n) | Two passes, constant work per character |
| **Space** | O(1) | Fixed 26 integers, doesn't scale |

### Advantages Over HashMap

1. **No hashing overhead** - Direct array indexing
2. **Better cache locality** - Sequential memory access
3. **Predictable timing** - No hash collisions
4. **Lower memory** - No HashMap overhead

---

## Exercise 3: Valid Anagram - Sort and Compare

### The Solution

```rust
pub fn is_anagram_sort(s: &str, t: &str) -> bool {
    let mut s_chars: Vec<char> = s.chars().collect();
    let mut t_chars: Vec<char> = t.chars().collect();
    s_chars.sort();
    t_chars.sort();
    s_chars == t_chars
}
```

### Line-by-Line Analysis

```rust
let mut s_chars: Vec<char> = s.chars().collect();
```
**Purpose:** Convert string to vector of characters.

```rust
s_chars.sort();
t_chars.sort();
```
**Purpose:** Sort both character vectors. After sorting, identical strings will have identical character sequences.

```rust
s_chars == t_chars
```
**Purpose:** Compare sorted vectors for equality.

### Complexity Analysis

| Metric | Value | Explanation |
|--------|-------|-------------|
| **Time** | O(n log n) | Sorting dominates |
| **Space** | O(n) | Storing character vectors |

### Why Not Optimal

While simple, sorting is O(n log n) when O(n) is achievable with HashMap or array approach. Sorting is useful when:
- You need the sorted output
- Memory is extremely constrained (no extra HashMap/array)

---

## Exercise 4: String Equivalence

### The Solution

```rust
pub fn are_strings_equivalent(s1: &str, s2: &str) -> bool {
    if s1.len() > s2.len() {
        return are_strings_equivalent(s2, s1);
    }

    let mut count = HashMap::new();

    for c in s1.chars() {
        *count.entry(c).or_insert(0) += 1;
    }

    for c in s2.chars() {
        *count.entry(c).or_insert(0) -= 1;
    }

    count.values().all(|&v| v <= 0)
}
```

### Key Insight

For strings to be equivalent where we can only append to one string:
- s2 can have extra characters (positive counts after processing)
- s2 cannot be missing any characters (negative counts not allowed)

Processing s1 adds, s2 subtracts. All counts must be <= 0 (s2 has all of s1's characters).

---

## Exercise 5: Minimum Steps to Make Anagram

### The Solution

```rust
pub fn min_steps_to_make_anagram(s: &str, t: &str) -> i32 {
    let mut count = [0; 26];

    for c in s.chars() {
        count[(c as u8 - b'a') as usize] += 1;
    }

    for c in t.chars() {
        count[(c as u8 - b'a') as usize] -= 1;
    }

    count.iter().map(|&c| c.abs()).filter(|&c| c > 0).sum();
    0  // placeholder - real implementation below
}

pub fn min_steps_to_make_anagram(s: &str, t: &str) -> i32 {
    let mut count = [0; 26];

    for c in s.chars() {
        count[(c as u8 - b'a') as usize] += 1;
    }

    for c in t.chars() {
        count[(c as u8 - b'a') as usize] -= 1;
    }

    count.iter()
        .filter(|&&c| c > 0)
        .map(|&c| c)
        .sum()
}
```

### Logic

1. Count characters in s (positive counts)
2. Subtract characters in t (negative means t has excess)
3. Sum all positive counts (these are characters s needs that t doesn't have)

---

## Exercise 6: Group Anagrams

### The Solution

```rust
pub fn group_anagrams(strs: &[&str]) -> Vec<Vec<String>> {
    use std::collections::HashMap;
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for &s in strs {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort();
        let key = chars.iter().collect::<String>();

        map.entry(key).or_default().push(s.to_string());
    }

    map.into_values().collect()
}
```

### Key Insight

Two strings are anagrams if they sort to the same sequence of characters. Use the sorted string as the key in a HashMap.

### Complexity Analysis

| Metric | Value | Explanation |
|--------|-------|-------------|
| **Time** | O(n * k log k) | n strings, each sorting k chars |
| **Space** | O(n * k) | Storing all strings |

---

## Exercise 7: Anagram Mappings

### The Solution

```rust
pub fn anagram_mappings(a: &[i32], b: &[i32]) -> Vec<i32> {
    use std::collections::HashMap;
    let mut index_map: HashMap<i32, Vec<i32>> = HashMap::new();

    for (i, &val) in b.iter().enumerate() {
        index_map.entry(val).or_default().push(i as i32);
    }

    a.iter()
        .map(|&val| {
            index_map.get(&val).unwrap().pop().unwrap()
        })
        .collect()
}
```

### Logic

1. Build a map from value to all indices where that value appears in B
2. For each value in A, pop an index from B's list
3. This guarantees A[i] is an anagram of B[index]

---

## Exercise 8: Valid Palindrome II

### The Solution

```rust
pub fn valid_palindrome_deletion(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let mut left = 0;
    let mut right = chars.len() - 1;

    while left < right {
        if chars[left] == chars[right] {
            left += 1;
            right -= 1;
        } else {
            // Try skipping left or right character
            return is_palindrome_range(&chars, left + 1, right) ||
                   is_palindrome_range(&chars, left, right - 1);
        }
    }
    true
}

fn is_palindrome_range(chars: &[char], left: usize, right: usize) -> bool {
    let mut l = left;
    let mut r = right;
    while l < r {
        if chars[l] != chars[r] {
            return false;
        }
        l += 1;
        r -= 1;
    }
    true
}
```

### Key Insight

Two-pointer approach with ability to skip one character. When chars don't match, try both possibilities.

---

## Exercises 9-10

These follow the same patterns as Exercises 1-5 with slight modifications:

**count_anagrams_in_string**: Sliding window with character counting
**min_deletions_to_anagram**: Similar to min_steps but for deletions

---

## Summary Table

| Exercise | Time | Space | Key Technique |
|----------|------|-------|---------------|
| 1: HashMap | O(n) | O(k) | Character counting |
| 2: Array | O(n) | O(1) | Direct indexing |
| 3: Sort | O(n log n) | O(n) | Sorted comparison |
| 4: Equivalent | O(n) | O(k) | Character counting |
| 5: Min Steps | O(n) | O(1) | Count difference |
| 6: Group | O(n*k log k) | O(n*k) | Sorted key grouping |
| 7: Mapping | O(n) | O(n) | Index tracking |
| 8: Palindrome | O(n) | O(1) | Two pointers |
| 9: Count Anagrams | O(n) | O(k) | Sliding window |
| 10: Min Deletions | O(n) | O(1) | Count difference |

## Key Takeaways

1. **Character counting** is the fundamental technique for anagram problems
2. **Array approach** is optimal for bounded character sets (26 lowercase letters)
3. **HashMap approach** is most flexible for Unicode/general characters
4. **Sorted key** works well for group anagrams
5. **Two pointers** can solve palindrome variants
