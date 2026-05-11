# Solutions: Valid Palindrome (LeetCode #125)

This document provides detailed, line-by-line analysis of each exercise implementation.

---

## Exercise 1: Valid Palindrome - Two Pointers

### The Solution

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

### Line-by-Line Analysis

```rust
let chars: Vec<char> = s.chars()
    .filter(|c| c.is_alphanumeric())
    .map(|c| c.to_ascii_lowercase())
    .collect();
```
**Purpose:** Clean the string in one pipeline:
1. `filter(|c| c.is_alphanumeric())` - keep only letters and digits
2. `map(|c| c.to_ascii_lowercase())` - convert to lowercase
3. `collect()` - create a vector of chars

```rust
let (mut left, mut right) = (0, chars.len().saturating_sub(1));
```
**Purpose:** Initialize two pointers at opposite ends. `saturating_sub(1)` handles empty array safely.

```rust
while left < right {
```
**Purpose:** Continue while pointers haven't crossed.

```rust
if chars[left] != chars[right] {
    return false;
}
```
**Purpose:** If characters don't match, it's not a palindrome.

```rust
left += 1;
right = right.saturating_sub(1);
```
**Purpose:** Move pointers toward center.

### Complexity Analysis

| Metric | Value | Explanation |
|--------|-------|-------------|
| **Time** | O(n) | Single pass to clean + compare |
| **Space** | O(n) | Storing cleaned characters |

---

## Exercise 2: Valid Palindrome II

### The Solution

```rust
pub fn valid_palindrome_deletion(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let mut left = 0;
    let mut right = chars.len().saturating_sub(1);

    while left < right {
        if chars[left] == chars[right] {
            left += 1;
            right = right.saturating_sub(1);
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
        r = r.saturating_sub(1);
    }
    true
}
```

### Key Insight

When characters at left and right don't match, we can skip either the left character or the right character. If skipping either produces a palindrome, the string is valid.

### Why Two Checks?

If `chars[left] != chars[right]`, we try:
1. Skip left: check if `chars[left+1..=right]` is palindrome
2. Skip right: check if `chars[left..=right-1]` is palindrome

If either works, the string can become a palindrome with one deletion.

---

## Exercise 3: Palindrome Number

### The Solution

```rust
pub fn is_palindrome_number(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let original = x;
    let mut reversed = 0;
    let mut remaining = x;

    while remaining > 0 {
        let digit = remaining % 10;
        reversed = reversed * 10 + digit;
        remaining /= 10;
    }

    original == reversed
}
```

### Key Insight

Reverse the number digit by digit and compare to original. If they're equal, it's a palindrome.

### Why It Works

For 121:
- Step 1: digit = 121 % 10 = 1, reversed = 0 * 10 + 1 = 1, remaining = 12
- Step 2: digit = 12 % 10 = 2, reversed = 1 * 10 + 2 = 12, remaining = 1
- Step 3: digit = 1 % 10 = 1, reversed = 12 * 10 + 1 = 121, remaining = 0

reversed == original, so 121 is a palindrome.

---

## Exercise 4: Longest Palindromic Substring

### The Solution (Expand Around Center)

```rust
pub fn longest_palindromic_substring(s: &str) -> String {
    if s.len() < 2 {
        return s.to_string();
    }

    let chars: Vec<char> = s.chars().collect();
    let (mut start, mut end) = (0, 0);

    for i in 0..chars.len() {
        let (len1, left1, right1) = expand_around(&chars, i, i);
        let (len2, left2, right2) = expand_around(&chars, i, i + 1);

        let (best_len, best_left, _) = if len1 > len2 {
            (len1, left1, right1)
        } else {
            (len2, left2, right2)
        };

        if best_len > end - start {
            start = best_left;
            end = best_left + best_len;
        }
    }

    chars[start..end].iter().collect()
}

fn expand_around(chars: &[char], left: usize, right: usize) -> (usize, usize, usize) {
    let mut l = left;
    let mut r = right;
    while l > 0 && r < chars.len() - 1 && chars[l] == chars[r] {
        l -= 1;
        r += 1;
    }
    // Check final match
    if chars[l] == chars[r] {
        (r - l + 1, l, r)
    } else {
        (r - l, l + 1, r)
    }
}
```

### Why O(n²)?

For each center position (2n - 1 centers for odd and even length palindromes), we expand outward. In the worst case, we expand O(n) times for each center.

---

## Exercise 5: Palindrome Linked List

### The Solution (Reverse Second Half)

```rust
pub fn is_palindrome_list(head: Option<Box<ListNode>>) -> bool {
    if head.is_none() {
        return true;
    }

    // Find middle
    let mut slow = &head;
    let mut fast = &head;
    while fast.is_some() {
        slow = &slow.as_ref().unwrap().next;
        match fast.as_ref().unwrap().next.as_ref() {
            Some(next) => fast = &next.next,
            None => break,
        }
    }

    // slow now points to middle, reverse second half
    let mut prev = None;
    let mut current = slow.clone();
    while let Some(node) = current {
        let next = node.next;
        let new_node = Box::new(ListNode {
            val: node.val,
            next: prev,
        });
        prev = Some(new_node);
        current = next;
    }

    // Compare first half with reversed second half
    let mut left = &head;
    let mut right = &prev;
    while right.is_some() {
        if left.as_ref().unwrap().val != right.as_ref().unwrap().val {
            return false;
        }
        left = &left.as_ref().unwrap().next;
        right = &right.as_ref().unwrap().next;
    }
    true
}
```

### Key Insight

1. Find middle using slow/fast pointers
2. Reverse second half
3. Compare first half with reversed second half

---

## Summary Table

| Exercise | Time | Space | Key Technique |
|----------|------|-------|---------------|
| 1: Two Pointers | O(n) | O(n) | Clean + compare |
| 2: Palindrome II | O(n) | O(1) | Try both deletions |
| 3: Number | O(n) | O(1) | Reverse number |
| 4: Longest Substring | O(n²) | O(1) | Expand around center |
| 5: Linked List | O(n) | O(1) | Reverse second half |
| 6: Longest Palindrome | O(n) | O(1) | Count frequencies |
| 7: Palindrome Pairs | O(n*k²) | O(n) | Check all pairs |
| 8: After Removal | O(n) | O(1) | Count mismatches |

## Key Takeaways

1. **Two pointers** is the classic palindrome check
2. **Clean first** - filter non-alphanumeric, lowercase
3. **Expand around center** finds palindromes
4. **Fast/slow pointers** find list middle
5. **Reverse and compare** is a common pattern
