# Solutions: Group Anagrams (LeetCode #49)

This document provides detailed, line-by-line analysis of each exercise implementation.

---

## Exercise 1: Group Anagrams - Sorted Key

### The Solution

```rust
pub fn group_anagrams(strs: &[String]) -> Vec<Vec<String>> {
    use std::collections::HashMap;
    let mut map = HashMap::new();

    for s in strs {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort();
        let key = chars.iter().collect::<String>();

        map.entry(key).or_insert_with(Vec::new).push(s.clone());
    }

    map.into_values().collect()
}
```

### Line-by-Line Analysis

```rust
use std::collections::HashMap;
```
**Purpose:** Import HashMap for key-value storage.

```rust
let mut map = HashMap::new();
```
**Purpose:** Create HashMap where key is sorted string, value is vector of anagrams.

```rust
let mut chars: Vec<char> = s.chars().collect();
```
**Purpose:** Convert string to vector of characters for sorting.

```rust
chars.sort();
```
**Purpose:** Sort characters alphabetically. Anagrams will produce identical sorted sequences.

```rust
let key = chars.iter().collect::<String>();
```
**Purpose:** Convert sorted characters back to string for use as HashMap key.

```rust
map.entry(key).or_insert_with(Vec::new).push(s.clone());
```
**Purpose:** Add string to the group identified by its sorted key.
- `entry(key)` gets or creates the entry
- `or_insert_with(Vec::new)` creates empty vector if entry is new
- `push(s.clone())` adds current string to the vector

```rust
map.into_values().collect()
```
**Purpose:** Extract all values (groups) as a vector of vectors.

### Complexity Analysis

| Metric | Value | Explanation |
|--------|-------|-------------|
| **Time** | O(n * k log k) | n strings, sorting k chars each |
| **Space** | O(n * k) | Storing all strings and keys |

---

## Exercise 2: Group Anagrams - Character Count

### The Solution

```rust
pub fn group_anagrams_count(strs: &[String]) -> Vec<Vec<String>> {
    use std::collections::HashMap;
    let mut map = HashMap::new();

    for s in strs {
        let count = count_chars(s);
        map.entry(count).or_insert_with(Vec::new).push(s.clone());
    }

    map.into_values().collect()
}

fn count_chars(s: &str) -> [i32; 26] {
    let mut count = [0; 26];
    for c in s.chars() {
        if c.is_ascii_lowercase() {
            count[(c as u8 - b'a') as usize] += 1;
        }
    }
    count
}
```

### Line-by-Line Analysis

```rust
fn count_chars(s: &str) -> [i32; 26] {
    let mut count = [0; 26];
```
**Purpose:** Create array to count occurrences of each letter.

```rust
for c in s.chars() {
    if c.is_ascii_lowercase() {
        count[(c as u8 - b'a') as usize] += 1;
    }
}
```
**Purpose:** Count each character. `is_ascii_lowercase()` ensures we only count valid letters.

```rust
map.entry(count).or_insert_with(Vec::new).push(s.clone());
```
**Purpose:** Use the count array as the key. Arrays with identical counts will hash identically.

### Complexity Analysis

| Metric | Value | Explanation |
|--------|-------|-------------|
| **Time** | O(n * k) | No sorting, just counting |
| **Space** | O(n * k) | Same as sorted approach |

### Advantage Over Sorted Key

Character counting is O(k) vs sorting which is O(k log k). For large strings, this is significantly faster.

---

## Exercise 3: Group Anagrams - Prime Product

### The Solution

```rust
pub fn group_anagrams_prime(strs: &[String]) -> Vec<Vec<String>> {
    use std::collections::HashMap;
    let primes = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101,
    ];
    let mut map: HashMap<i64, Vec<String>> = HashMap::new();

    for s in strs {
        let mut product = 1i64;
        for c in s.chars() {
            product *= primes[(c as u8 - b'a') as usize] as i64;
        }
        map.entry(product).or_insert_with(Vec::new).push(s.clone());
    }

    map.into_values().collect()
}
```

### Line-by-Line Analysis

```rust
let primes = [2, 3, 5, 7, ...];
```
**Purpose:** Array of first 26 prime numbers, one for each letter.

```rust
let mut product = 1i64;
for c in s.chars() {
    product *= primes[(c as u8 - b'a') as usize] as i64;
}
```
**Purpose:** Multiply all primes for each letter in the string. The product is the same for anagrams.

### Mathematical Basis

The Fundamental Theorem of Arithmetic states that every integer has a unique prime factorization. Since we assign a unique prime to each letter, the product of primes is unique per anagram.

### Risk: Integer Overflow

For long strings, the product can exceed i64::MAX. Use larger integer types or BigInt for production code.

---

## Exercises 4-8: Variations

### Count Groups

```rust
pub fn count_anagram_groups(strs: &[String]) -> i32 {
    let groups = group_anagrams(strs);
    groups.len() as i32
}
```

### Largest Group Size

```rust
pub fn largest_anagram_group_size(strs: &[String]) -> usize {
    let groups = group_anagrams(strs);
    groups.iter().map(|g| g.len()).max().unwrap_or(0)
}
```

### Groups with Minimum Size

```rust
pub fn find_groups_with_min_size(strs: &[String], k: usize) -> Vec<Vec<String>> {
    let groups = group_anagrams(strs);
    groups.into_iter().filter(|g| g.len() >= k).collect()
}
```

### Is Anagram of Any

```rust
pub fn is_anagram_of_any(s: &str, strs: &[String]) -> bool {
    let mut s_chars: Vec<char> = s.chars().collect();
    s_chars.sort();
    let key: String = s_chars.iter().collect();

    for t in strs {
        let mut t_chars: Vec<char> = t.chars().collect();
        t_chars.sort();
        let t_key: String = t_chars.iter().collect();
        if key == t_key {
            return true;
        }
    }
    false
}
```

### Min Group Difference

```rust
pub fn min_group_difference(strs: &[String]) -> usize {
    let groups = group_anagrams(strs);
    let sizes: Vec<usize> = groups.iter().map(|g| g.len()).collect();

    if sizes.len() < 2 {
        return 0;
    }

    let mut sizes = sizes;
    sizes.sort();
    sizes[sizes.len() - 1] - sizes[0]
}
```

---

## Summary Table

| Exercise | Time | Space | Key Technique |
|----------|------|-------|---------------|
| 1: Sorted Key | O(n*k log k) | O(n*k) | Sort and group |
| 2: Char Count | O(n*k) | O(n*k) | Frequency array |
| 3: Prime Product | O(n*k) | O(n*k) | Prime multiplication |
| 4: Count Groups | O(n*k log k) | O(n*k) | Reuse group_anagrams |
| 5: Largest Size | O(n*k log k) | O(n*k) | Reuse group_anagrams |
| 6: Min Size | O(n*k log k) | O(n*k) | Filter groups |
| 7: Is Anagram | O(n*k log k) | O(k) | Compare keys |
| 8: Min Diff | O(n*k log k) | O(n*k) | Find size range |

## Key Takeaways

1. **Sorted key** is simple but O(n log n) per string
2. **Character count** is O(n) per string - preferred for efficiency
3. **Prime product** is clever but risks overflow
4. **HashMap groups** anagrams efficiently
5. **Variations** can reuse the basic grouping logic
