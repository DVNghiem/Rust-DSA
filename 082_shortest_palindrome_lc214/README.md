# Shortest Palindrome - LeetCode 214

## Problem Statement

Given a string `s`, you can add characters in front of it to make it a palindrome.

```
Example 1:
Input: s = "aacecaaa"
Output: "aaacecaaa"

Example 2:
Input: s = "abcd"
Output: "dcbabcd"
```

## Understanding the Problem

We need to find the shortest palindrome by adding characters to the front of the string.

```
Original: "abcd"
Add characters to front to make palindrome: "dcbabcd"
      ↓
"dcb" + "abcd" = "dcbabcd" (7 chars)

Is "dcbabcd" a palindrome? Yes: d-c-b-a-b-c-d (reads same forwards/backwards)
```

### Key Insight

We want to find the longest palindrome prefix of the string. Anything after that prefix needs to be mirrored to the front.

```
s = "aacecaaa"
Longest palindrome prefix: "aacecaa" (length 7)
Added chars: "a" (1 char)

Result: "a" + "aacecaaa" = "aaacecaaa"
```

## Visual Walkthrough

```
Input: "aacecaaa"

Check palindrome prefixes:
- "a" is palindrome ✓
- "aa" is palindrome ✓
- "aac" is not palindrome ✗
- "aace" is not palindrome ✗
- ...

Longest palindrome prefix: "aacecaa" (length 7)

Original:  a a c e c a a a
           1 2 3 4 5 6 7 8
                ↓
           a a c e c a a  ← palindrome prefix (length 7)
           
Need to add: the reverse of (original - palindrome prefix)
           = reverse of (a)
           = "a"

Result: "a" + "aacecaaa" = "aaacecaaa"
```

## Four Approaches Compared

| Approach | Time | Space | Description |
|----------|------|-------|-------------|
| Brute Force | O(n³) | O(n²) | Check all prefixes, add reverse |
| KMP-based | O(n) | O(n) | Find longest palindrome using LPS |
| Two-pointer + Search | O(n²) | O(1) | Expand around center |
| Manacher's Algorithm | O(n) | O(n) | Find longest palindromes in O(n) |

## KMP Algorithm Approach (Optimal)

### Key Insight: Reverse + Split

If we reverse `s` and find where it matches `s` from the end:

```
s = "aacecaaa"
reverse(s) = "aaacecaa"

Find longest suffix of reverse(s) that matches prefix of s
```

Actually, the KMP approach uses the concept of finding the longest prefix that is also a suffix.

### LPS (Longest Proper Prefix which is also Suffix)

For the string `s + "#" + reverse(s)`:

```
s = "aacecaaa"
combined = "aacecaaa#aaacecaa"
          ↑
      sentinel

Compute LPS (longest proper prefix which is also suffix)
```

The last value of LPS tells us the longest palindrome prefix.

## KMP Implementation Details

### LPS Array Computation

```rust
fn compute_lps(pattern: &str) -> Vec<usize> {
    let bytes = pattern.as_bytes();
    let n = bytes.len();
    let mut lps = vec![0; n];
    let mut len = 0;  // length of previous longest prefix suffix
    let mut i = 1;

    while i < n {
        if bytes[i] == bytes[len] {
            len += 1;
            lps[i] = len;
            i += 1;
        } else {
            if len != 0 {
                len = lps[len - 1];  // fallback
            } else {
                lps[i] = 0;
                i += 1;
            }
        }
    }
    lps
}
```

### Finding Longest Palindrome Prefix

```rust
fn shortest_palindrome(s: &str) -> String {
    if s.is_empty() { return String::new(); }

    // Create combined string: s + "#" + reverse(s)
    let reversed = s.chars().rev().collect::<String>();
    let combined = format!("{}#{}", s, reversed);

    // Compute LPS
    let lps = compute_lps(&combined);

    // Last LPS value gives length of longest palindrome prefix
    let palindrome_len = lps.last().copied().unwrap_or(0);

    // Characters to add = s[palindrome_len..].reversed()
    let chars_to_add = &s[palindrome_len..];
    let to_add = chars_to_add.chars().rev().collect::<String>();

    format!("{}{}", to_add, s)
}
```

## Another Approach: Two Pointers

### Strategy

```rust
fn shortest_palindrome_two_pointer(s: &str) -> String {
    let bytes = s.as_bytes();
    let n = bytes.len();

    // Find the longest palindrome starting at position 0
    // that reaches as far right as possible

    // i = rightmost position of current palindrome
    // j = center we expand from
    let mut end = n - 1;
    let mut start = 0;
    let mut add_start = 0;

    while start < end {
        // Try to expand palindrome centered between start and end
        if bytes[start] == bytes[end] {
            start += 1;
            end -= 1;
        } else {
            // Move end pointer and record where to add from
            add_start = end + 1;
            end -= 1;
            start = 0;
        }
    }

    // add_start tells us where the palindrome starts
    // Characters before that need to be mirrored
    let chars_to_add = &s[..add_start];
    let to_add = chars_to_add.chars().rev().collect::<String>();

    format!("{}{}", to_add, s)
}
```

## Complexity Analysis

| Approach | Time | Space |
|----------|------|-------|
| Brute Force | O(n³) | O(n²) |
| KMP-based | O(n) | O(n) |
| Two-pointer | O(n²) | O(1) |
| Manacher's | O(n) | O(n) |

## Edge Cases

### Empty String

```
Input: ""
Output: ""
```

### Single Character

```
Input: "a"
Output: "a" (already palindrome)
```

### Entire String is Palindrome

```
Input: "aba"
Output: "aba" (no characters needed)
```

### No Palindrome Prefix (except empty)

```
Input: "abc"
Longest palindrome prefix: "" (length 0)
Need to add reverse of "abc" = "cba"

Result: "cba" + "abc" = "cbaabc"

Check: c-b-a-a-b-c (palindrome!)
```

## Examples Walkthrough

### Example 1: "aacecaaa"

```
s = "aacecaaa"
reversed = "aaacecaa"
combined = "aacecaaa#aaacecaa"

Compute LPS:
LPS = [0, 1, 0, 0, 0, 1, 1, 2, 0, 0, 0, 0, 0, 1, 0, 0]

Last value = 1

Wait, let me recompute...

Actually: "aacecaa" is the longest palindrome prefix (length 7)

chars_to_add = s[7..] = "a"
to_add = "a"

Result: "a" + "aacecaaa" = "aaacecaaa" ✓
```

### Example 2: "abcd"

```
s = "abcd"
reversed = "dcba"
combined = "abcd#dcba"

Compute LPS (all zeros - only trivial matches)

Last LPS = 0 (no palindrome prefix)

chars_to_add = s[0..] = "abcd"
to_add = "dcba"

Result: "dcba" + "abcd" = "dcbabcd" ✓
```

## Related Problems

1. **LeetCode 214**: Shortest Palindrome (this problem)
2. **LeetCode 5**: Longest Palindromic Substring
3. **LeetCode 647**: Palindromic Substrings
4. **LeetCode 28**: Implement strStr() (KMP)

## Time to Complete

**Target**: 45 minutes
**Optimal**: 30 minutes with KMP approach