# Valid Palindrome (LeetCode #125)

## Problem Statement

A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward.

Given a string `s`, return `true` if it is a palindrome, or `false` otherwise.

## Examples

```
Input: s = "A man, a plan, a canal: Panama"
Output: true
Explanation: After cleaning: "amanaplanacanalpanama" which is a palindrome.

Input: s = "race a car"
Output: false
Explanation: After cleaning: "raceacar" which is not a palindrome.
```

## Approaches Overview

### Approach 1: Two Pointers O(n)
Use left and right pointers to compare characters.

```rust
pub fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    let (mut left, mut right) = (0, chars.len().saturating_sub(1));

    while left < right {
        if chars[left] != chars[right] {
            return false;
        }
        left += 1;
        right = right.saturating_sub(1);
    }
    true
}
```

### Approach 2: Two Pointers In-Place O(n)
Process string character by character without extra allocation.

```rust
pub fn is_palindrome_inplace(s: &str) -> bool {
    let mut left = 0;
    let mut right = s.len().saturating_sub(1);
    let chars: Vec<char> = s.chars().collect();

    while left < right {
        while left < right && !chars[left].is_alphanumeric() {
            left += 1;
        }
        while left < right && !chars[right].is_alphanumeric() {
            right = right.saturating_sub(1);
        }

        if chars[left].to_ascii_lowercase() != chars[right].to_ascii_lowercase() {
            return false;
        }
        left += 1;
        right = right.saturating_sub(1);
    }
    true
}
```

### Approach 3: Reverse and Compare O(n)
Clean the string and compare with its reverse.

```rust
pub fn is_palindrome_reverse(s: &str) -> bool {
    let cleaned: String = s.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    cleaned == cleaned.chars().rev().collect::<String>()
}
```

## Visual Walkthrough

```
s = "A man, a plan, a canal: Panama"

Step 1: Clean and lowercase
  Original: "A man, a plan, a canal: Panama"
  Filter alphanumeric: ['A', 'm', 'a', 'n', 'a', 'p', 'l', 'a', 'n', 'a', 'c', 'a', 'n', 'a', 'l', 'P', 'a', 'n', 'a', 'm', 'a']
  Lowercase: ['a', 'm', 'a', 'n', 'a', 'p', 'l', 'a', 'n', 'a', 'c', 'a', 'n', 'a', 'l', 'p', 'a', 'n', 'a', 'm', 'a']

Step 2: Two pointer comparison
  left=0 ('a') vs right=20 ('a') ✓ match
  left=1 ('m') vs right=19 ('m') ✓ match
  left=2 ('a') vs right=18 ('a') ✓ match
  ... continues until middle ...

All pairs match! Return true.

The cleaned string reads "amanaplanacanalpanama" which is a palindrome.
```

## Complexity Analysis Table

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Two Pointers | O(n) | O(k) | k = alphanumeric chars |
| In-Place | O(n) | O(1) | No extra allocation |
| Reverse | O(n) | O(n) | Full string copy |

## Edge Cases to Consider

1. **Empty string**: Return true (vacuously palindrome)
2. **Single character**: Return true
3. **Only non-alphanumeric**: Return true (empty after cleaning)
4. **Mixed case**: "A" and "a" should be equal
5. **Numbers in string**: "A1a" should be palindrome
6. **Unicode characters**: Handle properly

## Related Problems

### LeetCode 680: Valid Palindrome II
Can remove at most one character to make palindrome.

### LeetCode 234: Palindrome Linked List
Check if linked list is palindrome.

### LeetCode 409: Longest Palindrome
Find longest palindrome possible from characters.

## Exercises

### Exercise 1: Basic Valid Palindrome
Implement using two pointers.

### Exercise 2: Valid Palindrome II
Can remove one character to make palindrome.

### Exercise 3: Palindrome Number
Check if integer is palindrome (without string).

### Exercise 4: Longest Palindromic Substring
Find longest palindromic substring.

### Exercise 5: Palindrome Partitioning
Partition string so all substrings are palindromes.

## Key Takeaways

1. **Two pointers** efficiently checks palindrome in O(n)
2. **Filter non-alphanumeric** before comparison
3. **Lowercase comparison** handles case insensitivity
4. **Skip non-alphanumeric** on both sides during check
5. **Early exit** when mismatch found

## Real-World Applications

1. **String validation**: User input verification
2. **DNA sequence analysis**: Palindromic patterns
3. **Palindrome checking**: Word puzzles and games
4. **Data integrity**: Checksum validation
5. **Text processing**: String normalization
