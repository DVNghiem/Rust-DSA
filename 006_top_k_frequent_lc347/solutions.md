# Solutions: Top K Frequent Elements (LeetCode #347)

This document provides detailed, line-by-line analysis of each exercise implementation.

---

## Exercise 1: Top K Frequent - Binary Heap

### The Solution

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

### Line-by-Line Analysis

```rust
use std::collections::{BinaryHeap, HashMap};
```
**Purpose:** Import BinaryHeap for priority queue and HashMap for counting.

```rust
let mut freq = HashMap::new();
for &num in nums {
    *freq.entry(num).or_insert(0) += 1;
}
```
**Purpose:** Build frequency map. Each occurrence increments the count for that number.

```rust
let mut heap = BinaryHeap::new();
for (num, count) in freq {
    heap.push((count, num));
}
```
**Purpose:** Push all (count, num) pairs into the heap. BinaryHeap orders by the first tuple element (count) in descending order by default.

```rust
for _ in 0..k {
    if let Some((_, num)) = heap.pop() {
        result.push(num);
    }
}
```
**Purpose:** Pop the top k elements from the heap. The `if let Some` handles the case where k might be larger than unique elements.

### Complexity Analysis

| Metric | Value | Explanation |
|--------|-------|-------------|
| **Time** | O(n log k) | n insert + k pops from heap |
| **Space** | O(n) | HashMap and heap store n elements |

### Why O(n log k)?

Building the HashMap takes O(n). Adding n elements to the heap is O(n log n) in worst case, but we only keep top k, so it's O(n log k). Popping k elements is O(k log n).

For large n and small k, this is much better than O(n log n).

---

## Exercise 2: Top K Frequent - Bucket Sort

### The Solution

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

### Line-by-Line Analysis

```rust
let max_freq = *freq.values().max().unwrap_or(&1) as usize;
```
**Purpose:** Find the maximum frequency to size the buckets array.

```rust
let mut buckets: Vec<Vec<i32>> = vec![vec![]; max_freq + 1];
```
**Purpose:** Create buckets where index represents frequency. Buckets[frequency] contains all numbers with that frequency.

```rust
for (num, count) in freq {
    buckets[count as usize].push(num);
}
```
**Purpose:** Place each number in its frequency bucket.

```rust
for bucket in buckets.iter_mut().rev() {
    for num in bucket.drain(..) {
        result.push(num);
        if result.len() == k as usize {
            return result;
        }
    }
}
```
**Purpose:** Iterate buckets from highest frequency to lowest, collecting elements until we have k results.

### Complexity Analysis

| Metric | Value | Explanation |
|--------|-------|-------------|
| **Time** | O(n) | Single pass + bucket iteration |
| **Space** | O(n) | Buckets store all elements |

### Why O(n)?

Counting is O(n). Creating buckets is O(max_freq). Iterating from high to low frequency visits each element once. Total is O(n + max_freq) which is O(n).

---

## Exercise 3: Top K Frequent Words

### The Solution

```rust
pub fn top_k_frequent_words(words: &[&str], k: i32) -> Vec<String> {
    use std::collections::HashMap;
    let mut freq = HashMap::new();

    for &word in words {
        *freq.entry(word).or_insert(0) += 1;
    }

    let mut sorted: Vec<(&&str, &i32)> = freq.iter().collect();
    sorted.sort_by(|a, b| {
        if b.1.cmp(a.1) != std::cmp::Ordering::Equal {
            b.1.cmp(a.1)
        } else {
            a.0.cmp(b.0)
        }
    });

    sorted.into_iter().take(k as usize).map(|(w, _)| w.to_string()).collect()
}
```

### Line-by-Line Analysis

```rust
sorted.sort_by(|a, b| {
    if b.1.cmp(a.1) != std::cmp::Ordering::Equal {
        b.1.cmp(a.1)
    } else {
        a.0.cmp(b.0)
    }
});
```
**Purpose:** Sort by frequency descending (b.1.cmp(a.1)), with lexicographic tiebreaker ascending (a.0.cmp(b.0)).

### Complexity Analysis

| Metric | Value | Explanation |
|--------|-------|-------------|
| **Time** | O(n log n) | Sorting dominates |
| **Space** | O(n) | Storing frequencies |

---

## Exercise 4: Sort Characters by Frequency

### The Solution

```rust
pub fn frequency_sort(s: &str) -> String {
    use std::collections::HashMap;
    let mut freq = HashMap::new();

    for c in s.chars() {
        *freq.entry(c).or_insert(0) += 1;
    }

    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_by(|a, b| {
        let freq_a = freq.get(a).unwrap();
        let freq_b = freq.get(b).unwrap();
        freq_b.cmp(freq_a)
    });

    chars.into_iter().collect()
}
```

### Line-by-Line Analysis

```rust
chars.sort_by(|a, b| {
    let freq_a = freq.get(a).unwrap();
    let freq_b = freq.get(b).unwrap();
    freq_b.cmp(freq_a)
});
```
**Purpose:** Sort characters by frequency in descending order.

### Alternative: O(n) Solution

```rust
pub fn frequency_sort_fast(s: &str) -> String {
    use std::collections::HashMap;
    let mut freq = HashMap::new();
    for c in s.chars() {
        *freq.entry(c).or_insert(0) += 1;
    }

    let max_freq = *freq.values().max().unwrap_or(&1) as usize;
    let mut buckets: Vec<Vec<char>> = vec![vec![]; max_freq + 1];

    for (c, count) in freq {
        buckets[count].push(c);
    }

    let mut result = String::new();
    for bucket in buckets.iter_mut().rev() {
        for c in bucket.iter() {
            result.push(*c);
            result.push_str(&c.to_string().repeat(bucket.len()));
        }
    }
    result
}
```

---

## Exercises 5-8: Summary

| Exercise | Time | Space | Key Technique |
|----------|------|-------|---------------|
| 5: Kth Most Frequent | O(n log k) | O(n) | Binary heap |
| 6: Min Operations | O(n) | O(n) | Frequency analysis |
| 7: Memory Check | O(n) | O(n) | Size estimation |
| 8: Sum Frequent | O(n) | O(n) | Filter by frequency |

## Key Takeaways

1. **Binary heap** efficiently tracks top k - O(n log k)
2. **Bucket sort** is O(n) when max frequency is bounded
3. **Tuple ordering** provides custom comparisons
4. **Frequency counting** is the foundation for all these problems
5. **k may be larger** than unique elements - handle gracefully
