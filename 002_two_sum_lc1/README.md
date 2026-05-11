# Two Sum (LeetCode #1)

## Problem Statement

Given an array of integers `nums` and an integer `target`, return the indices of the two numbers such that they add up to `target`.

You may assume that each input would have **exactly one solution**, and you may not use the same element twice.

You can return the answer in any order.

## Examples

```
Input: nums = [2, 7, 11, 15], target = 9
Output: [0, 1]
Explanation: nums[0] + nums[1] = 2 + 7 = 9

Input: nums = [3, 2, 4], target = 6
Output: [1, 2]
Explanation: nums[1] + nums[2] = 2 + 4 = 6

Input: nums = [3, 3], target = 6
Output: [0, 1]
```

## Approaches Overview

### Approach 1: Brute Force O(n²)
Check every pair of elements to find the target sum.

```rust
pub fn two_sum_brute_force(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] + nums[j] == target {
                return Some((i, j));
            }
        }
    }
    None
}
```

### Approach 2: Two-Pass HashMap O(n)
Build a HashMap of value -> index, then check for complement.

```rust
pub fn two_sum_two_pass(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        map.insert(num, i);
    }
    for (i, &num) in nums.iter().enumerate() {
        if let Some(&j) = map.get(&(target - num)) {
            if i != j {
                return Some((i, j));
            }
        }
    }
    None
}
```

### Approach 3: One-Pass HashMap O(n) - Preferred
Check and insert in single pass.

```rust
pub fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        if let Some(&j) = map.get(&(target - num)) {
            return Some((j, i));
        }
        map.insert(num, i);
    }
    None
}
```

### Approach 4: Two Pointers O(n log n)
Sort and use two pointers (loses original indices).

```rust
pub fn two_sum_two_pointer(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut indices: Vec<(usize, i32)> = nums.iter().enumerate().map(|(i, &v)| (i, v)).collect();
    indices.sort_by(|a, b| a.1.cmp(&b.1));

    let mut left = 0;
    let mut right = indices.len() - 1;

    while left < right {
        let sum = indices[left].1 + indices[right].1;
        match sum.cmp(&target) {
            std::cmp::Ordering::Equal => {
                let (i1, i2) = (indices[left].0, indices[right].0);
                return Some((i1.min(i2), i1.max(i2)));
            }
            std::cmp::Ordering::Less => left += 1,
            std::cmp::Ordering::Greater => right -= 1,
        }
    }
    None
}
```

## HashMap Deep Dive

### How HashMap Works in Rust

A HashMap in Rust uses a hash function to map keys to indices in an underlying array (bucket array). When you insert a key-value pair:

1. Hash function computes an index from the key
2. If that bucket is empty, the pair is stored there
3. If occupied, the new pair is appended to a linked list or checked for equality

### Why HashMap Lookup is O(1) Average

- **Hash computation**: O(1) - Fixed time hash function
- **Bucket access**: O(1) - Array index access
- **Collision handling**: O(1) average if load factor is maintained

### Collision Resolution

When two keys hash to the same bucket, we handle it via:
- **Separate chaining**: Each bucket is a linked list
- **Open addressing**: Find next available slot (Rust's stdlib uses this)

### Load Factor

Load factor = n / k where n is number of items, k is number of buckets.
Rust automatically resizes when load factor > 0.875.

## Visual Walkthrough: One-Pass HashMap

```
nums = [2, 7, 11, 15], target = 9

Step 1: i=0, num=2
  complement = 9 - 2 = 7
  map = {}
  Is 7 in map? NO
  Insert (2, 0) into map
  map = {2: 0}

Step 2: i=1, num=7
  complement = 9 - 7 = 2
  map = {2: 0}
  Is 2 in map? YES! j = 0
  Return (0, 1)

SUCCESS!
```

## Complexity Analysis Table

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Brute Force | O(n²) | O(1) | No extra space |
| Two-Pass HashMap | O(n) | O(n) | Two full passes |
| One-Pass HashMap | O(n) | O(n) | Single pass |
| Two Pointers | O(n log n) | O(1)* | Loses original indices |

*Note: Sorting is O(n log n), so technically the same, but O(1) extra space beyond input.

## Edge Cases to Consider

1. **Empty array**: Return None
2. **Single element**: Cannot sum to target
3. **Negative numbers**: Handle negative targets and values
4. **Duplicate values**: nums = [3, 3], target = 6 should return (0, 1)
5. **Large numbers**: Handle overflow (use i64 instead of i32)
6. **Same element twice**: nums = [6], target = 12 should not match (6 + 6)

## Variant: Two Sum II (Sorted Array)

Given a sorted array in non-decreasing order, find two numbers that add up to target.

```rust
pub fn two_sum_sorted(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left < right {
        let sum = nums[left] + nums[right];
        match sum.cmp(&target) {
            std::cmp::Ordering::Equal => return Some((left + 1, right + 1)), // 1-indexed
            std::cmp::Ordering::Less => left += 1,
            std::cmp::Ordering::Greater => right -= 1,
        }
    }
    None
}
```

**Why this works for sorted array:**
- If nums[left] + nums[right] > target, we need a smaller sum, so decrement right
- If nums[left] + nums[right] < target, we need a larger sum, so increment left

## Variant: Three Sum

Given an array of integers, find all unique triplets that sum to zero.

```rust
pub fn three_sum(nums: &[i32]) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut nums = nums.to_vec();
    nums.sort();

    for i in 0..nums.len().saturating_sub(2) {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        let mut left = i + 1;
        let mut right = nums.len() - 1;
        let target = -nums[i];

        while left < right {
            let sum = nums[left] + nums[right];
            match sum.cmp(&target) {
                std::cmp::Ordering::Less => left += 1,
                std::cmp::Ordering::Greater => right -= 1,
                std::cmp::Ordering::Equal => {
                    result.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;
                    while left < right && nums[left] == nums[left - 1] {
                        left += 1;
                    }
                }
            }
        }
    }
    result
}
```

## Exercises

### Exercise 1: Basic Two Sum
Implement the one-pass HashMap solution for Two Sum.

### Exercise 2: Two Sum II
Given a sorted array, return indices (1-indexed) of two numbers that sum to target.

### Exercise 3: Three Sum
Find all unique triplets in the array that sum to zero.

### Exercise 4: Four Sum
Find all unique quadruplets that sum to a target value.

### Exercise 5: Count Pair Sum
Count the number of pairs that sum to a specific value (LC 2006 - Count Number of Pairs With Absolute Difference K).

### Exercise 6: Two Sum with Data Stream
Design a data structure that supports:
- `void add(int number)` - Add a number to the data structure
- `boolean find(int value)` - Check if there's a pair that sums to value

## Key Takeaways

1. **HashMap is your friend** for O(n) lookups
2. **Single pass vs two pass** - single pass is more efficient when possible
3. **Two pointers** works when you have sorted input
4. **Complement logic** - Instead of searching for target - num, search for num's complement
5. **Handle edge cases** - Always consider empty input, single element, duplicates

## Real-World Applications

1. **Financial transactions**: Find transaction pairs that balance out
2. **Inventory management**: Match items that complete a set
3. **Duplicate detection**: Find pairs of duplicate entries
4. **Recommendation systems**: Find items that "go together"
5. **Game development**: Match players for balanced teams
