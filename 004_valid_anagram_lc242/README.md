# Valid Anagram (LeetCode #242)

## Problem Statement

Given two strings `s` and `t`, return `true` if `t` is an anagram of `s`, and `false` otherwise.

An **anagram** is a word or phrase formed by rearranging the letters of a different word or phrase, using all the original letters exactly once.

## Examples

```
Input: s = "anagram", t = "nagaram"
Output: true
Explanation: "nagaram" is an anagram of "anagram"

Input: s = "rat", t = "car"
Output: false
Explanation: "car" is not an anagram of "rat"
```

## Approaches Overview

### Approach 1: Sort Both Strings O(n log n)
Sort both strings and compare character by character.

```rust
pub fn is_anagram_sort(s: &str, t: &str) -> bool {
    let mut s_chars: Vec<char> = s.chars().collect();
    let mut t_chars: Vec<char> = t.chars().collect();
    s_chars.sort();
    t_chars.sort();
    s_chars == t_chars
}
```

### Approach 2: Character Count with HashMap O(n) - Preferred
Count characters in s, then subtract from t.

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

### Approach 3: Fixed Size Array O(n)
Since strings contain lowercase English letters, use a fixed-size array.

```rust
pub fn is_anagram_array(s: &str, t: &str) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut count = [0; 26];

    for (c1, c2) in s.chars().zip(t.chars()) {
        count[(c1 as u8 - b'a') as usize] += 1;
        count[(c2 as u8 - b'a') as usize] -= 1;
    }

    count.iter().all(|&c| c == 0)
}
```

## HashMap Deep Dive for Character Counting

### How Character Counting Works

The HashMap stores characters as keys and counts as values:
- When we see a character in s, we increment its count
- When we see a character in t, we decrement its count
- At the end, all counts should be zero if the strings are anagrams

### Visual Walkthrough

```
s = "anagram", t = "nagaram"

Step 1: Count characters in s
  'a': +1
  'n': +1
  'a': +1
  'g': +1
  'r': +1
  'a': +1
  'm': +1
  count = {a:3, n:1, g:1, r:1, m:1}

Step 2: Decrement counts for t
  'n': -1 → n:0
  'a': -1 → a:2
  'g': -1 → g:0
  'a': -1 → a:1
  'r': -1 → r:0
  'a': -1 → a:0
  'm': -1 → m:0
  count = {a:0, n:0, g:0, r:0, m:0}

Step 3: Verify all counts are zero
  All counts are zero!
  Return true

SUCCESS - ANAGRAM!
```

## Fixed Size Array Approach

### Why 26?

The problem states lowercase English letters, which gives us exactly 26 possible characters (a-z).

### How It Works

```rust
let mut count = [0; 26];  // Array of 26 zeros

// For 'a': index = 0
// For 'b': index = 1
// ...
// For 'z': index = 25

count[(c as u8 - b'a') as usize]
// 'a' (97) - 'a' (97) = 0 → index 0
// 'b' (98) - 'a' (97) = 1 → index 1
// 'z' (122) - 'a' (97) = 25 → index 25
```

### Advantages of Array Approach

1. **Cache friendly** - Sequential memory access
2. **No hash collisions** - Direct indexing
3. **Predictable performance** - No hash function overhead
4. **Lower memory** - Fixed 26 integers vs HashMap overhead

## Complexity Analysis Table

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Sort | O(n log n) | O(n) | Sorting strings |
| HashMap | O(n) | O(k) | k = unique characters |
| Fixed Array | O(n) | O(1) | 26 integers |

## Edge Cases to Consider

1. **Different lengths**: "anagram" vs "ana" - return false immediately
2. **Empty strings**: Both empty → true; one empty → false
3. **Case sensitivity**: "Anagram" vs "nagaram" - depends on problem (usually lowercase)
4. **Unicode**: "日本語" vs "日日本語" - may need proper Unicode handling
5. **Special characters**: Spaces, numbers in strings

## Related Problems

### LeetCode 49: Group Anagrams
Group strings that are anagrams of each other.

```rust
pub fn group_anagrams(strs: &[&str]) -> HashMap<String, Vec<String>> {
    use std::collections::HashMap;
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for &s in strs {
        let mut key: Vec<char> = s.chars().collect();
        key.sort();
        map.entry(key.iter().collect()).or_default().push(s.to_string());
    }
    map.into_values().collect()
}
```

### LeetCode 438: Find All Anagrams in a String
Find all start indices of t's anagrams in s.

```rust
pub fn find_anagrams(s: &str, p: &str) -> Vec<i32> {
    use std::collections::HashMap;

    let p_chars: Vec<char> = p.chars().collect();
    let s_chars: Vec<char> = s.chars().collect();

    if p.len() > s.len() {
        return vec![];
    }

    let mut result = vec![];
    let mut target_count = HashMap::new();

    for c in p_chars {
        *target_count.entry(c).or_insert(0) += 1;
    }

    let mut window_count = HashMap::new();

    for i in 0..s.len() {
        let c = s_chars[i];
        *window_count.entry(c).or_insert(0) += 1;

        if i >= p.len() {
            let left_char = s_chars[i - p.len()];
            *window_count.entry(left_char).or_insert(0) -= 1;
        }

        if window_count == target_count {
            result.push((i + 1 - p.len()) as i32);
        }
    }
    result
}
```

## Exercises

### Exercise 1: Basic Valid Anagram
Implement using HashMap character counting.

### Exercise 2: Valid Anagram with Array
Implement using fixed-size array for lowercase letters.

### Exercise 3: Valid Anagram with Sort
Implement by sorting and comparing.

### Exercise 4: Check if Two Strings are Equivalent
Given two strings A and B, check if they are "equivalent" - meaning they can form anagrams.

### Exercise 5: Find Minimum Number of Steps
Given strings s and t, find minimum steps to transform s into t (insert, delete, replace).

### Exercise 6: Group Anagrams
Group all anagram strings from an array.

### Exercise 7: Find Anagram Mappings
Given two arrays A and B, find index mapping such that A becomes anagram of B.

## Key Takeaways

1. **Character counting** is the fundamental technique
2. **Array approach is fastest** for bounded character sets
3. **Early exit on length mismatch** saves time
4. **HashMap approach is most flexible** for Unicode/general characters
5. **Sort approach is simplest** but not most efficient

## Real-World Applications

1. **Spelling correction**: Check if misspelled word could be an anagram of a valid word
2. **Scrabble cheat**: Find valid words from letter tiles
3. **Cryptography**: Simple letter substitution analysis
4. **Password validation**: Check if password uses anagram of username
5. **Crossword solving**: Find words that can be formed from given letters
