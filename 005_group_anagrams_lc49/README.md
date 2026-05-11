# Group Anagrams (LeetCode #49)

## Problem Statement

Given an array of strings, group the anagrams together. You can return the answer in any order.

## Examples

```
Input: strs = ["eat", "tea", "tan", "ate", "nat", "bat"]
Output: [["bat"], ["nat", "tan"], ["ate", "eat", "tea"]]

Input: strs = [""]
Output: [[""]]

Input: strs = ["a"]
Output: [["a"]]
```

## Approaches Overview

### Approach 1: Sorted Key HashMap O(n * k log k)
Sort each string to create a key, group by key.

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

### Approach 2: Character Count Key O(n * k)
Use character frequency as key instead of sorting.

```rust
pub fn group_anagrams(strs: &[String]) -> Vec<Vec<String>> {
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
        count[(c as u8 - b'a') as usize] += 1;
    }
    count
}
```

### Approach 3: Prime Product Key O(n * k)
Assign each letter a prime, use product as key (potential overflow).

```rust
pub fn group_anagrams_prime(strs: &[String]) -> Vec<Vec<String>> {
    use std::collections::HashMap;
    let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47,
                  53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101];
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

## Key Insight

**Two strings are anagrams if and only if they have the same character frequency.**

This means:
- Sorting anagrams produces identical sorted strings
- Anagrams have identical character counts
- The product of primes (one per letter) is identical for anagrams

## Visual Walkthrough: Sorted Key Approach

```
strs = ["eat", "tea", "tan", "ate", "nat", "bat"]

Process "eat":
  sorted = ['a', 'e', 't'] → "aet"
  map["aet"] = ["eat"]

Process "tea":
  sorted = ['a', 'e', 't'] → "aet"
  map["aet"] = ["eat", "tea"]

Process "tan":
  sorted = ['a', 'n', 't'] → "ant"
  map["ant"] = ["tan"]

Process "ate":
  sorted = ['a', 'e', 't'] → "aet"
  map["aet"] = ["eat", "tea", "ate"]

Process "nat":
  sorted = ['a', 'n', 't'] → "ant"
  map["ant"] = ["tan", "nat"]

Process "bat":
  sorted = ['a', 'b', 't'] → "abt"
  map["abt"] = ["bat"]

Final result:
  [["eat", "tea", "ate"], ["tan", "nat"], ["bat"]]
```

## Complexity Analysis Table

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Sorted Key | O(n * k log k) | O(n * k) | Sorting each string |
| Character Count | O(n * k) | O(n * k) | Count per string |
| Prime Product | O(n * k) | O(n) | Potential overflow |

n = number of strings, k = max string length

## Edge Cases to Consider

1. **Empty string**: "" should group with other ""
2. **Single character**: "a" groups with "a"
3. **Strings with same letters different order**: "eat", "ate", "tea"
4. **Large input**: Handle 10^4+ strings efficiently
5. **Unicode**: Handle non-ASCII characters properly
6. **Mixed case**: "Eat" vs "eat" (usually lowercase assumed)

## Related Problems

### LeetCode 242: Valid Anagram
Check if two strings are anagrams.

### LeetCode 438: Find All Anagrams in a String
Find starting indices of anagram substrings.

### LeetCode 567: Permutation in String
Check if one string contains an anagram of another.

## Exercises

### Exercise 1: Basic Group Anagrams
Implement using sorted key approach.

### Exercise 2: Group Anagrams with Count
Implement using character frequency.

### Exercise 3: Group Anagrams using Prime Product
Implement using prime number multiplication.

### Exercise 4: Count Number of Groups
Count how many anagram groups exist.

### Exercise 5: Find Largest Anagram Group
Find the largest group and return its size.

### Exercise 6: Find Groups with Minimum Size
Find all groups with at least k members.

## Key Takeaways

1. **Sorted key** is the simplest approach - sort and group
2. **Character count** is most efficient for fixed alphabet
3. **Prime product** is clever but risks integer overflow
4. **HashMap values** should use Vec to store multiple strings
5. **Clone strings** when inserting into grouped results

## Real-World Applications

1. **Spelling correction**: Group similar misspellings
2. **Cryptography**: Letter frequency analysis
3. **Crossword tools**: Find anagram words
4. **Word games**: Group valid word combinations
5. **Document similarity**: Find documents with similar letter patterns
