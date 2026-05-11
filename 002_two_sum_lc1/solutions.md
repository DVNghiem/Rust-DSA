# Solutions: Two Sum (LeetCode #1)

This document provides detailed, line-by-line analysis of each exercise implementation.

---

## Exercise 1: Two Sum - One-Pass HashMap

### The Solution

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

### Line-by-Line Analysis

```rust
use std::collections::HashMap;
```
**Purpose:** Import the HashMap from the collections module. HashMap provides O(1) average-case lookup and insertion.

```rust
let mut map = HashMap::new();
```
**Purpose:** Create a new empty HashMap. The type signature `HashMap<i32, usize>` is inferred from usage where keys are array values and values are indices.

```rust
for (i, &num) in nums.iter().enumerate() {
```
**Purpose:** Iterate through the array with both index `i` and value `num`. `enumerate()` returns an iterator yielding `(index, value)` tuples.

```rust
if let Some(&j) = map.get(&(target - num)) {
```
**Purpose:** Check if the complement (target - num) exists in the map. If it does, we found our pair. `get()` returns `Option<&usize>` where `&usize` is a reference to the stored index.

```rust
return Some((j, i));
```
**Purpose:** Return the indices of the pair. `j` is the index of the complement (found in map), `i` is the current element's index. The order is `(j, i)` because we want the indices in ascending order.

```rust
map.insert(num, i);
```
**Purpose:** If complement wasn't found, insert the current number and its index into the map. This is done AFTER checking so we don't match an element with itself.

```rust
None
```
**Purpose:** Return None if no valid pair exists after checking all elements.

### Complexity Analysis

| Metric | Value | Explanation |
|--------|-------|-------------|
| **Time** | O(n) | Single pass through array; each HashMap operation is O(1) average |
| **Space** | O(n) | In worst case, map stores n elements |

---

## Exercise 2: Two Sum - Brute Force

### The Solution

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

### Line-by-Line Analysis

```rust
for i in 0..nums.len() {
```
**Purpose:** Outer loop iterates through each element as the first element of the potential pair. `0..nums.len()` creates a range from 0 to length-1.

```rust
for j in (i + 1)..nums.len() {
```
**Purpose:** Inner loop starts from `i + 1` to avoid counting the same element twice. This ensures each pair is only checked once.

```rust
if nums[i] + nums[j] == target {
```
**Purpose:** Check if the sum of the two elements equals the target. This is the core comparison.

### Complexity Analysis

| Metric | Value | Explanation |
|--------|-------|-------------|
| **Time** | O(n²) | Nested loops checking all pairs: n(n-1)/2 |
| **Space** | O(1) | No additional data structures allocated |

### When to Use

Brute force is acceptable when:
- n is small (n < 1000)
- Memory is extremely constrained
- Simplicity matters more than performance

---

## Exercise 3: Two Sum - Two-Pass HashMap

### The Solution

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

### Line-by-Line Analysis

```rust
let mut map = HashMap::new();
for (i, &num) in nums.iter().enumerate() {
    map.insert(num, i);
}
```
**Purpose:** First pass - build the HashMap with all values as keys and their indices as values.

```rust
for (i, &num) in nums.iter().enumerate() {
    if let Some(&j) = map.get(&(target - num)) {
        if i != j {
```
**Purpose:** Second pass - for each element, look up its complement. The `i != j` check prevents matching an element with itself when values can be the same (e.g., nums = [6], target = 12).

### Complexity Analysis

| Metric | Value | Explanation |
|--------|-------|-------------|
| **Time** | O(n) | Two passes but still linear |
| **Space** | O(n) | HashMap stores all n elements |

### Comparison with One-Pass

| Aspect | One-Pass | Two-Pass |
|--------|----------|----------|
| Passes | 1 | 2 |
| Same element check | Not needed | Required |
| Cache efficiency | Worse | Better |

---

## Exercise 4: Two Sum II - Sorted Array

### The Solution

```rust
pub fn two_sum_sorted(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left < right {
        let sum = nums[left] + nums[right];
        match sum.cmp(&target) {
            std::cmp::Ordering::Equal => return Some((left + 1, right + 1)),
            std::cmp::Ordering::Less => left += 1,
            std::cmp::Ordering::Greater => right -= 1,
        }
    }
    None
}
```

### Line-by-Line Analysis

```rust
let mut left = 0;
let mut right = nums.len() - 1;
```
**Purpose:** Initialize two pointers at opposite ends of the array. This is the classic two-pointer technique for sorted arrays.

```rust
while left < right {
```
**Purpose:** Continue while pointers haven't crossed. When `left == right`, all pairs have been checked.

```rust
let sum = nums[left] + nums[right];
```
**Purpose:** Calculate the sum of the current pair pointed to by left and right.

```rust
match sum.cmp(&target) {
    std::cmp::Ordering::Equal => return Some((left + 1, right + 1)),
```
**Purpose:** If sum equals target, return 1-indexed indices as required by LeetCode 167.

```rust
std::cmp::Ordering::Less => left += 1,
```
**Purpose:** If sum is less than target, we need a larger sum. Since the array is sorted, moving `left` rightward increases the sum.

```rust
std::cmp::Ordering::Greater => right -= 1,
```
**Purpose:** If sum is greater than target, we need a smaller sum. Moving `right` leftward decreases the sum.

### Visual Example

```
nums = [2, 3, 4], target = 6

Step 1: left=0, right=2
  sum = 2 + 4 = 6 = target
  Return (1, 3) [1-indexed]

SUCCESS!
```

### Why Two Pointers Works

1. **Monotonic property:** In a sorted array, moving left increases sum, moving right decreases sum.
2. **No backtracking:** Each step moves exactly one pointer, guaranteeing O(n) runtime.
3. **Complete coverage:** All pairs are checked exactly once.

---

## Exercise 5: Three Sum

### The Solution

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

### Line-by-Line Analysis

```rust
let mut nums = nums.to_vec();
nums.sort();
```
**Purpose:** Create a mutable copy and sort it. Sorting enables the two-pointer technique and helps with deduplication.

```rust
for i in 0..nums.len().saturating_sub(2) {
```
**Purpose:** Iterate through each element as the "anchor" of our triplet. We only need to go up to len-2 because we need at least two more elements for left and right pointers.

```rust
if i > 0 && nums[i] == nums[i - 1] {
    continue;
}
```
**Purpose:** Skip duplicates for the first element. If nums[i] equals nums[i-1], we've already processed all triplets starting with nums[i-1].

```rust
let mut left = i + 1;
let mut right = nums.len() - 1;
let target = -nums[i];
```
**Purpose:** Initialize two pointers for the remaining subarray and calculate the target sum for the two-pointer search. Since a + b + c = 0, we need a + b = -c where c = nums[i].

```rust
while left < right {
    let sum = nums[left] + nums[right];
    match sum.cmp(&target) {
```
**Purpose:** Standard two-pointer loop: find two numbers that sum to target.

```rust
result.push(vec![nums[i], nums[left], nums[right]]);
left += 1;
while left < right && nums[left] == nums[left - 1] {
    left += 1;
}
```
**Purpose:** When we find a valid triplet, add it to result and skip all duplicates for the `left` pointer to avoid duplicate triplets.

### Complexity Analysis

| Metric | Value | Explanation |
|--------|-------|-------------|
| **Time** | O(n²) | Sort O(n log n) + nested loops O(n²) |
| **Space** | O(1) | Sorting in-place; output not counted |

---

## Exercise 6: Three Sum Closest

### The Solution

```rust
pub fn three_sum_closest(nums: &[i32], target: i32) -> i32 {
    let mut nums = nums.to_vec();
    nums.sort();
    let mut closest = i32::MAX;
    let mut best_diff = i32::MAX;

    for i in 0..nums.len().saturating_sub(2) {
        let mut left = i + 1;
        let mut right = nums.len() - 1;

        while left < right {
            let sum = nums[left] + nums[right] + nums[i];
            let diff = (sum - target).abs();

            if diff < best_diff {
                best_diff = diff;
                closest = sum;
            }

            match sum.cmp(&target) {
                std::cmp::Ordering::Less => left += 1,
                std::cmp::Ordering::Greater => right -= 1,
                std::cmp::Ordering::Equal => return sum,
            }
        }
    }
    closest
}
```

### Key Differences from Three Sum

1. **Track best diff:** Instead of checking for exact match, track the smallest difference from target.
2. **Early exit:** Return immediately when we find an exact match (sum == target).
3. **Update closest:** When a better (smaller) diff is found, update both `closest` and `best_diff`.

---

## Exercise 7: Four Sum

### The Solution

```rust
pub fn four_sum(nums: &[i32], target: i32) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut nums = nums.to_vec();
    nums.sort();

    for i in 0..nums.len().saturating_sub(3) {
        if i > 0 && nums[i] == nums[i - 1] { continue; }
        for j in (i + 1)..nums.len().saturating_sub(2) {
            if j > i + 1 && nums[j] == nums[j - 1] { continue; }

            let mut left = j + 1;
            let mut right = nums.len() - 1;

            while left < right {
                let sum = nums[i] + nums[j] + nums[left] + nums[right];
                match sum.cmp(&target) {
                    std::cmp::Ordering::Less => left += 1,
                    std::cmp::Ordering::Greater => right -= 1,
                    std::cmp::Ordering::Equal => {
                        result.push(vec![nums[i], nums[j], nums[left], nums[right]]);
                        left += 1;
                        while left < right && nums[left] == nums[left - 1] { left += 1; }
                    }
                }
            }
        }
    }
    result
}
```

### Two Pairs of Two Pointers

The pattern extends naturally: use two outer loops for the first two elements, then two pointers for the remaining two.

---

## Exercise 8: Count Pairs with Diff K

### The Solution

```rust
pub fn count_pairs_with_diff_k(nums: &[i32], k: i32) -> i32 {
    use std::collections::HashSet;
    let set: HashSet<i32> = nums.iter().copied().collect();
    let mut count = 0;

    for &num in nums.iter() {
        if set.contains(&(num + k)) {
            count += 1;
        }
        if k != 0 && set.contains(&(num - k)) {
            count += 1;
        }
    }
    count / 2
}
```

### Why Divide by 2?

Each pair (a, b) where |a - b| = k is counted twice:
- Once when iterating with a: found b = a + k
- Once when iterating with b: found a = b - k

Dividing by 2 corrects for this double-counting.

---

## Exercise 9: Find All Pairs with Sum

### The Solution

```rust
pub fn find_pairs(nums: &[i32], target: i32) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    let mut map = HashMap::new();

    for &num in nums.iter() {
        if let Some(&count) = map.get(&(target - num)) {
            if count > 0 {
                result.push((target - num, num));
                // Decrement to avoid duplicate pairs with same values
                map.insert(target - num, count - 1);
            }
        }
        *map.entry(num).or_insert(0) += 1;
    }
    result
}
```

### Key Insight

Using a count-based approach handles duplicate values correctly. We track how many times each value has been "used" to form a pair.

---

## Exercise 10: Two Sum Data Stream

### The Solution

```rust
#[derive(Default)]
pub struct TwoSum {
    numbers: HashSet<i32>,
    sums: HashSet<i32>,
}

impl TwoSum {
    pub fn new() -> Self {
        Self {
            numbers: HashSet::new(),
            sums: HashSet::new(),
        }
    }

    pub fn add(&mut self, number: i32) {
        // Check if adding this number creates a sum we've seen before
        if self.numbers.contains(&number) {
            self.sums.insert(number * 2);
        }
        // Generate all new sums with existing numbers
        for &n in &self.numbers {
            self.sums.insert(n + number);
        }
        self.numbers.insert(number);
    }

    pub fn find(&self, value: i32) -> bool {
        self.sums.contains(&value)
    }
}
```

### Design Choices

1. **HashSet for numbers:** O(1) lookup and insertion
2. **HashSet for sums:** Pre-compute all possible sums for O(1) find
3. **Trade-off:** add() is O(n) but find() is O(1)

This is a classic space-time tradeoff.

---

## Exercise 11: Minimum Operations to Reduce X to Zero

### The Solution

```rust
pub fn min_operations(nums: &[i32], x: i32) -> i32 {
    let total: i32 = nums.iter().sum();
    let target = total - x;

    if target < 0 {
        return -1;
    }

    let mut max_len = -1;
    let mut curr_sum = 0;
    let mut left = 0;

    for right in 0..nums.len() {
        curr_sum += nums[right];

        while curr_sum > target && left <= right {
            curr_sum -= nums[left];
            left += 1;
        }

        if curr_sum == target {
            max_len = max_len.max((right - left + 1) as i32);
        }
    }

    if max_len == -1 {
        -1
    } else {
        (nums.len() as i32) - max_len
    }
}
```

### The Trick

Instead of finding elements from both ends, find the longest subarray whose sum equals `total - x`. Then the answer is `n - longest_subarray_length`.

---

## Exercise 12: Subarray Sum Equals K

### The Solution

```rust
pub fn subarray_sum_equals_k(nums: &[i32], k: i32) -> i32 {
    let mut count = 0;
    let mut prefix_sum = 0;
    let mut prefix_counts: HashMap<i32, i32> = HashMap::new();
    prefix_counts.insert(0, 1);

    for &num in nums.iter() {
        prefix_sum += num;
        count += prefix_counts.get(&(prefix_sum - k)).unwrap_or(&0);
        *prefix_counts.entry(prefix_sum).or_insert(0) += 1;
    }
    count
}
```

### Key Insight

For any prefix sum `p`, the number of subarrays ending at current position with sum k equals the number of previous prefix sums equal to `p - k`.

This is essentially "Two Sum" but for prefix sums!

---

## Summary Table

| Exercise | Time | Space | Key Technique |
|----------|------|-------|---------------|
| 1: Two Sum (One-Pass) | O(n) | O(n) | HashMap |
| 2: Brute Force | O(n²) | O(1) | Nested loops |
| 3: Two-Pass | O(n) | O(n) | HashMap |
| 4: Sorted | O(n) | O(1) | Two pointers |
| 5: Three Sum | O(n²) | O(1)* | Sort + Two pointers |
| 6: Three Sum Closest | O(n²) | O(1)* | Track best diff |
| 7: Four Sum | O(n³) | O(1)* | Nested + Two pointers |
| 8: Count Pairs Diff K | O(n) | O(n) | HashSet |
| 9: Find All Pairs | O(n) | O(n) | Count-based HashMap |
| 10: Data Stream | O(n)/O(1) | O(n) | Pre-computed sums |
| 11: Min Operations | O(n) | O(1) | Sliding window |
| 12: Subarray Sum K | O(n) | O(n) | Prefix sums + HashMap |

*Excluding output space
