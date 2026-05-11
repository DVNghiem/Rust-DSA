# Contains Duplicate (LeetCode #26)

## Problem Statement

Given an integer array `nums`, return `true` if any value appears **at least twice** in the array, and return `false` if every element is distinct.

## Examples

```
Input: nums = [1, 2, 3, 1]
Output: true
Explanation: 1 appears twice (at index 0 and index 3).

Input: nums = [1, 2, 3, 4]
Output: false
Explanation: All elements are distinct.

Input: nums = [1, 1, 1, 3, 3, 4, 3, 2, 4, 2]
Output: true
```

## Approaches Overview

### Approach 1: Brute Force O(n²)
Check every pair of elements for equality.

```rust
pub fn contains_duplicate_brute_force(nums: &[i32]) -> bool {
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] == nums[j] {
                return true;
            }
        }
    }
    false
}
```

### Approach 2: Sort First O(n log n)
Sort the array and check adjacent elements.

```rust
pub fn contains_duplicate_sort(nums: &[i32]) -> bool {
    let mut sorted = nums.to_vec();
    sorted.sort();
    for i in 1..sorted.len() {
        if sorted[i] == sorted[i - 1] {
            return true;
        }
    }
    false
}
```

### Approach 3: HashSet O(n) - Preferred
Use a HashSet to track seen elements.

```rust
pub fn contains_duplicate(nums: &[i32]) -> bool {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    for num in nums {
        if set.contains(num) {
            return true;
        }
        set.insert(num);
    }
    false
}
```

## HashSet Deep Dive

### How HashSet Works in Rust

HashSet is essentially a HashMap where we only care about keys (the values are unit type `()`). It provides:
- `insert()`: Add an element
- `contains()`: Check if an element exists
- `len()`: Get number of elements

### Why HashSet Lookup is O(1) Average

1. **Hash function** computes an index from the element
2. **Bucket access** is O(1) array indexing
3. **Collision handling** uses chaining or open addressing

### Collision Resolution

Rust's stdlib uses **quadratic probing** for open addressing. When a collision occurs, it probes positions using a quadratic sequence until an empty slot is found.

## Visual Walkthrough: HashSet Approach

```
nums = [1, 2, 3, 1]

Step 1: num = 1
  Is 1 in set? NO
  Insert 1 into set
  set = {1}

Step 2: num = 2
  Is 2 in set? NO
  Insert 2 into set
  set = {1, 2}

Step 3: num = 3
  Is 3 in set? NO
  Insert 3 into set
  set = {1, 2, 3}

Step 4: num = 1
  Is 1 in set? YES!
  Return true

DUPLICATE FOUND!
```

## Complexity Analysis Table

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Brute Force | O(n²) | O(1) | Nested loops |
| Sort First | O(n log n) | O(n) | Sorting overhead |
| HashSet | O(n) | O(n) | Linear time |

## Edge Cases to Consider

1. **Empty array**: Return false (no duplicates possible)
2. **Single element**: Return false
3. **All duplicates**: nums = [1, 1, 1, 1]
4. **Negative numbers**: Handle -1, -2 correctly
5. **Large values**: Handle i32::MAX, i32::MIN
6. **Large array**: Handle arrays with millions of elements

## Related Problems

### LeetCode 217: Contains Duplicate
The basic problem - check if any duplicate exists.

### LeetCode 219: Contains Duplicate II
Find if there are two duplicate indices within k distance.

```rust
pub fn contains_nearby_duplicate(nums: &[i32], k: i32) -> bool {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        if let Some(&prev_index) = map.get(&num) {
            if (i as i32) - prev_index <= k {
                return true;
            }
        }
        map.insert(num, i as i32);
    }
    false
}
```

### LeetCode 220: Contains Duplicate III
Find if there are two values where the difference is at most t and indices are at most k apart.

```rust
pub fn contains_nearby_almost_duplicate(nums: &[i32], k: i32, t: i32) -> bool {
    use std::collections::BTreeMap;
    let mut bst = BTreeMap::new();

    for (i, &num) in nums.iter().enumerate() {
        // Find smallest key >= num
        if let Some(&&prev) = bst.range(num..).next() {
            if (prev - num) as i64 <= t as i64 {
                return true;
            }
        }
        // Find largest key < num
        if let Some(&&prev) = bst.range(..num).next_back() {
            if (num - prev) as i64 <= t as i64 {
                return true;
            }
        }

        bst.insert(num, i);

        if i as i32 >= k {
            bst.remove(&nums[i - k as usize]);
        }
    }
    false
}
```

## Variant: Contains Duplicate II (with Index Check)

Given an integer array `nums` and an integer `k`, return `true` if there are two distinct indices `i` and `j` in the array such that `nums[i] == nums[j]` and `abs(i - j) <= k`.

## Exercises

### Exercise 1: Basic Contains Duplicate
Implement using HashSet.

### Exercise 2: Contains Duplicate II
Check if duplicate exists within k distance.

### Exercise 3: Contains Duplicate III
Check if duplicate exists with value difference <= t and index difference <= k.

### Exercise 4: Count Distinct Elements
Count how many distinct elements are in the array.

### Exercise 5: Find All Duplicate Indices
Find all pairs of indices where nums[i] == nums[j].

## Key Takeaways

1. **HashSet is optimal** for duplicate detection - O(n) time
2. **Sort first** is O(n log n) but uses O(1) extra space if sorting in-place
3. **Brute force** is O(n²) - never use for production code
4. **Consider the variant** requirements when choosing an approach

## Real-World Applications

1. **Database deduplication**: Find duplicate records in a dataset
2. **User registration**: Check if username or email already exists
3. **Card game**: Check for duplicate cards in a hand
4. **Inventory systems**: Detect duplicate SKU entries
5. **Document comparison**: Find repeated phrases or words
