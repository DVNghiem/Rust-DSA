# Top K Frequent Elements (LeetCode #347)

## Problem Statement

Given an integer array `nums` and an integer `k`, return the `k` most frequent elements. You may return the answer in any order.

## Examples

```
Input: nums = [1, 1, 1, 2, 2, 3], k = 2
Output: [1, 2]
Explanation: 1 appears 3 times, 2 appears 2 times.

Input: nums = [1], k = 1
Output: [1]
```

## Approaches Overview

### Approach 1: Sort by Frequency O(n log n)
Count frequencies, sort by value, take top k.

```rust
pub fn top_k_frequent_sort(nums: &[i32], k: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let mut freq = HashMap::new();
    for &num in nums {
        *freq.entry(num).or_insert(0) += 1;
    }

    let mut pairs: Vec<(i32, i32)> = freq.into_iter().collect();
    pairs.sort_by(|a, b| b.1.cmp(&a.1));

    pairs.into_iter().take(k as usize).map(|(n, _)| n).collect()
}
```

### Approach 2: Binary Heap O(n log k) - Preferred
Use a max-heap of size k to track top k elements.

```rust
pub fn top_k_frequent(nums: &[i32], k: i32) -> Vec<i32> {
    use std::collections::{BinaryHeap, HashMap};
    let mut freq = HashMap::new();
    for &num in nums {
        *freq.entry(num).or_insert(0) += 1;
    }

    let mut heap = BinaryHeap::new();
    for (num, count) in freq {
        heap.push((count, num));
    }

    let mut result = Vec::new();
    for _ in 0..k {
        if let Some((_, num)) = heap.pop() {
            result.push(num);
        }
    }
    result
}
```

### Approach 3: Bucket Sort O(n)
Use index as frequency for direct access.

```rust
pub fn top_k_frequent_bucket(nums: &[i32], k: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let mut freq = HashMap::new();
    for &num in nums {
        *freq.entry(num).or_insert(0) += 1;
    }

    let max_freq = *freq.values().max().unwrap_or(&1) as usize;
    let mut buckets: Vec<Vec<i32>> = vec![vec![]; max_freq + 1];

    for (num, count) in freq {
        buckets[count as usize].push(num);
    }

    let mut result = Vec::new();
    for bucket in buckets.iter_mut().rev() {
        for num in bucket.drain(..) {
            result.push(num);
            if result.len() == k as usize {
                return result;
            }
        }
    }
    result
}
```

## Binary Heap Deep Dive

### What is a Binary Heap?

A binary heap is a complete binary tree where each parent node is greater than or equal to (max-heap) or less than or equal to (min-heap) its children.

### Rust's BinaryHeap

Rust's `std::collections::BinaryHeap` is a max-heap by default:
- `push()` adds an element
- `pop()` removes and returns the largest element
- No direct iteration support

### Custom Comparator with Tuple

```rust
heap.push((count, num));
```

By pushing a tuple, Rust compares tuples lexicographically. The first element (count) is compared first, so higher counts come first.

## Visual Walkthrough: Binary Heap Approach

```
nums = [1, 1, 1, 2, 2, 3], k = 2

Step 1: Count frequencies
  freq = {1: 3, 2: 2, 3: 1}

Step 2: Build heap with (count, num)
  heap = [(3, 1), (2, 2), (1, 3)]

Step 3: Pop top k elements
  Pop (3, 1) → result = [1]
  Pop (2, 2) → result = [1, 2]

Return [1, 2]
```

## Complexity Analysis Table

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Sort | O(n log n) | O(n) | Full sort |
| Binary Heap | O(n log k) | O(n) | Only track top k |
| Bucket Sort | O(n) | O(n) | Best for frequency problems |

## Edge Cases to Consider

1. **Empty array**: Return empty vector
2. **Single element**: Return [element]
3. **All same elements**: Return [element]
4. **k equals number of elements**: Return all elements
5. **k larger than unique elements**: Return all unique elements
6. **Negative numbers**: Handle correctly in HashMap

## Related Problems

### LeetCode 692: Top K Frequent Words
Return k most frequent words with proper ordering.

### LeetCode 347: Top K Frequent Elements
Same as this problem but may require specific ordering.

### LeetCode 973: K Closest Points
Similar pattern with different data.

## Exercises

### Exercise 1: Basic Top K Frequent
Implement using binary heap.

### Exercise 2: Top K Frequent with Bucket Sort
Implement using bucket sort approach.

### Exercise 3: Top K Frequent Words
Given words with frequencies, return top k words.

### Exercise 4: K Most Frequent Elements II
Handle case where k may be larger than unique elements.

### Exercise 5: Sort Characters by Frequency
Return characters sorted by frequency.

## Key Takeaways

1. **Binary heap** efficiently tracks top k elements
2. **Bucket sort** is optimal when frequency range is bounded
3. **Tuple ordering** enables custom heap comparison
4. **Count first, then select** is the standard pattern
5. **Handle edge cases** before optimization

## Real-World Applications

1. **Search autocomplete**: Most frequently searched terms
2. **Recommendation systems**: Most popular items
3. **Analytics**: Most active users
4. **Word frequency**: Document analysis
5. **Traffic analysis**: Most frequent routes
