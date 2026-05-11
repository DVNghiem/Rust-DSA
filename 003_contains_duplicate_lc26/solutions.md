# Solutions: Contains Duplicate (LeetCode #26)

This document provides detailed, line-by-line analysis of each exercise implementation.

---

## Exercise 1: Contains Duplicate - HashSet

### The Solution

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

### Line-by-Line Analysis

```rust
use std::collections::HashSet;
```
**Purpose:** Import HashSet from the collections module. HashSet is an efficient container for checking element existence without duplicates.

```rust
let mut set = HashSet::new();
```
**Purpose:** Create a new empty HashSet. The type `HashSet<i32>` is inferred from usage.

```rust
for num in nums {
```
**Purpose:** Iterate through each element in the input array. This is a simple for-each loop.

```rust
if set.contains(num) {
    return true;
}
```
**Purpose:** If the current number already exists in the set, we found a duplicate. Return true immediately.

```rust
set.insert(num);
```
**Purpose:** If not a duplicate, add the number to the set. This is done AFTER checking so we don't match an element with itself in single-element arrays.

```rust
false
```
**Purpose:** Return false if we complete the loop without finding any duplicates.

### Complexity Analysis

| Metric | Value | Explanation |
|--------|-------|-------------|
| **Time** | O(n) | Single pass, each HashSet operation is O(1) average |
| **Space** | O(n) | In worst case, set stores all n elements |

---

## Exercise 2: Contains Duplicate - Sort First

### The Solution

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

### Line-by-Line Analysis

```rust
let mut sorted = nums.to_vec();
sorted.sort();
```
**Purpose:** Create a mutable copy of the array and sort it. Sorting brings duplicate elements adjacent to each other.

```rust
for i in 1..sorted.len() {
```
**Purpose:** Iterate from index 1 to end. Start at 1 because we compare with the previous element.

```rust
if sorted[i] == sorted[i - 1] {
    return true;
}
```
**Purpose:** If current element equals the previous element, we found a duplicate (they're adjacent after sorting).

### Complexity Analysis

| Metric | Value | Explanation |
|--------|-------|-------------|
| **Time** | O(n log n) | Sorting is the dominant operation |
| **Space** | O(n) | The sorted copy requires n elements |

### Why O(n log n) but Still Useful?

- Sorting provides additional information (ordering)
- Useful when you need both sorting and duplicate detection
- Simple to understand and implement correctly

---

## Exercise 3: Contains Duplicate - Brute Force

### The Solution

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

### Line-by-Line Analysis

```rust
for i in 0..nums.len() {
```
**Purpose:** Outer loop iterates through each element as potential first element of a pair.

```rust
for j in (i + 1)..nums.len() {
```
**Purpose:** Inner loop starts from `i + 1` to avoid comparing an element with itself and to avoid checking the same pair twice.

```rust
if nums[i] == nums[j] {
    return true;
}
```
**Purpose:** If we find matching elements, return true immediately (early exit optimization).

### Complexity Analysis

| Metric | Value | Explanation |
|--------|-------|-------------|
| **Time** | O(n²) | Nested loops: n(n-1)/2 comparisons |
| **Space** | O(1) | No additional data structures |

### When to Use

- n is small (< 1000) and memory is extremely constrained
- Educational purposes to illustrate inefficiency
- Never in production code with potentially large inputs

---

## Exercise 4: Contains Duplicate II

### The Solution

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

### Line-by-Line Analysis

```rust
use std::collections::HashMap;
```
**Purpose:** Import HashMap to store value -> most recent index mapping.

```rust
let mut map = HashMap::new();
```
**Purpose:** Create empty HashMap to track the most recent index of each value.

```rust
for (i, &num) in nums.iter().enumerate() {
```
**Purpose:** Iterate with both index and value.

```rust
if let Some(&prev_index) = map.get(&num) {
    if (i as i32) - prev_index <= k {
        return true;
    }
}
```
**Purpose:** If we've seen this value before, check if the index difference is at most k. If so, we found a valid pair.

```rust
map.insert(num, i as i32);
```
**Purpose:** Update the map with the current index of this value.

### Key Insight

We only need to track the most recent index for each value. If we find a duplicate within k distance, all previous occurrences are irrelevant.

### Complexity Analysis

| Metric | Value | Explanation |
|--------|-------|-------------|
| **Time** | O(n) | Single pass through array |
| **Space** | O(min(n, k)) | Map stores at most k recent entries |

---

## Exercise 5: Contains Duplicate III

### The Solution

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

### Line-by-Line Analysis

```rust
use std::collections::BTreeMap;
```
**Purpose:** Use BTreeMap for ordered key-value storage. The ordered property allows us to efficiently find neighbors.

```rust
if let Some(&&prev) = bst.range(num..).next() {
    if (prev - num) as i64 <= t as i64 {
        return true;
    }
}
```
**Purpose:** Find the smallest element >= num. If it differs by at most t, we found a valid pair.

```rust
if let Some(&&prev) = bst.range(..num).next_back() {
    if (num - prev) as i64 <= t as i64 {
        return true;
    }
}
```
**Purpose:** Find the largest element < num. If it differs by at most t, we found a valid pair.

```rust
bst.insert(num, i);
```
**Purpose:** Add current element to the BST.

```rust
if i as i32 >= k {
    bst.remove(&nums[i - k as usize]);
}
```
**Purpose:** Remove elements that are now more than k positions away. This keeps the BST size bounded by k+1.

### Why BTreeMap?

- **Ordered**: Allows finding immediate neighbors in O(log n)
- **Efficient range queries**: The `range()` method finds elements in a specific range in O(log n)
- **Size bounded**: We only keep k+1 most recent elements

### Complexity Analysis

| Metric | Value | Explanation |
|--------|-------|-------------|
| **Time** | O(n log k) | Each iteration does O(log k) work for BST operations |
| **Space** | O(k) | BST stores at most k+1 elements |

---

## Exercise 6: Count Distinct Elements

### The Solution

```rust
pub fn count_distinct(nums: &[i32]) -> usize {
    use std::collections::HashSet;
    let set: HashSet<i32> = nums.iter().copied().collect();
    set.len()
}
```

### Line-by-Line Analysis

```rust
let set: HashSet<i32> = nums.iter().copied().collect();
```
**Purpose:** Convert the array to a HashSet. Duplicates are automatically removed.

```rust
set.len()
```
**Purpose:** Return the number of elements in the set, which equals the number of distinct elements.

### Complexity Analysis

| Metric | Value | Explanation |
|--------|-------|-------------|
| **Time** | O(n) | HashSet insertion is O(1) average |
| **Space** | O(n) | Set stores all distinct elements |

---

## Exercise 7: Find All Duplicates

### The Solution

```rust
pub fn find_all_duplicates(nums: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    let mut seen: HashMap<i32, bool> = HashMap::new();

    for &num in nums.iter() {
        if seen.contains_key(&num) {
            result.push(num);
        } else {
            seen.insert(num, true);
        }
    }
    result
}
```

### Line-by-Line Analysis

```rust
let mut seen: HashMap<i32, bool> = HashMap::new();
```
**Purpose:** Create a HashMap to track which values we've seen.

```rust
if seen.contains_key(&num) {
    result.push(num);
}
```
**Purpose:** If we've seen this number before, it's a duplicate - add to result.

```rust
} else {
    seen.insert(num, true);
}
```
**Purpose:** First time seeing this number - mark as seen.

### Complexity Analysis

| Metric | Value | Explanation |
|--------|-------|-------------|
| **Time** | O(n) | Single pass |
| **Space** | O(n) | HashMap and result vector |

### Alternative: In-Place O(1) Space

A clever solution uses the fact that values are in range [1, n]. For each value v, we negate the element at index v-1. If it's already negative, v is a duplicate.

```rust
pub fn find_all_duplicates_inplace(nums: &mut Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for &num in nums.iter() {
        let abs_num = num.abs() as usize;
        if nums[abs_num - 1] < 0 {
            result.push(abs_num as i32);
        } else {
            nums[abs_num - 1] = -nums[abs_num - 1];
        }
    }
    result
}
```

---

## Exercise 8-10: Simpler Implementations

Exercises 8-10 use the same HashSet pattern but with different counting/logic:

```rust
pub fn contains_duplicate_in_range(nums: &[i32], left: i32, right: i32) -> bool {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    for &num in nums.iter() {
        if num >= left && num <= right {
            if set.contains(&num) {
                return true;
            }
            set.insert(num);
        }
    }
    false
}

pub fn count_unique(nums: &[i32]) -> usize {
    nums.iter().collect::<HashSet<&i32>>().len()
}

pub fn has_all_duplicates(nums: &[i32]) -> bool {
    use std::collections::HashMap;
    let mut counts: HashMap<i32, i32> = HashMap::new();
    for &num in nums.iter() {
        *counts.entry(num).or_insert(0) += 1;
    }
    counts.values().all(|&c| c >= 2)
}
```

---

## Summary Table

| Exercise | Time | Space | Key Technique |
|----------|------|-------|---------------|
| 1: HashSet | O(n) | O(n) | Linear scan |
| 2: Sort | O(n log n) | O(n) | Sort + adjacent compare |
| 3: Brute Force | O(n²) | O(1) | Nested loops |
| 4: Nearby II | O(n) | O(k) | Index tracking |
| 5: Nearby III | O(n log k) | O(k) | BTreeMap |
| 6: Count Distinct | O(n) | O(n) | HashSet |
| 7: Find Duplicates | O(n) | O(n) | HashMap |
| 8: Range | O(n) | O(n) | Filter + HashSet |
| 9: Count Unique | O(n) | O(n) | HashSet |
| 10: All Duplicates | O(n) | O(n) | HashMap counts |

## Key Takeaways

1. **HashSet is the goto** for duplicate detection - O(n) time
2. **Sorting** provides additional information at O(n log n) cost
3. **Brute force is never optimal** - O(n²) is too slow for production
4. **For index-based variants**, track recent indices or use BTreeMap
5. **Range queries** (contains almost duplicate) require ordered data structures
