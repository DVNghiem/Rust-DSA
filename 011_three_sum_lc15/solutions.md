# Solutions: Three Sum (LeetCode #15)

This document provides detailed, line-by-line analysis of each exercise implementation.

---

## Exercise 1: Three Sum - Sort + Two Pointers

### The Solution

```rust
pub fn three_sum(nums: &[i32]) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut nums = nums.to_vec();
    nums.sort();

    for i in 0..nums.len().saturating_sub(2) {
        if i > 0 && nums[i] == nums[i - 1] { continue; }

        let mut left = i + 1;
        let mut right = nums.len() - 1;

        while left < right {
            let sum = nums[i] + nums[left] + nums[right];
            match sum.cmp(&0) {
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
**Purpose:** Sort the array. Sorting is essential because:
1. It enables the two-pointer technique for finding pairs
2. It groups duplicate values together for easy skipping
3. It establishes a predictable ordering

```rust
for i in 0..nums.len().saturating_sub(2) {
```
**Purpose:** Iterate with pointer `i` as the first element of the triplet. We only need to go up to len-2 because we need at least two more elements for `left` and `right` pointers.

```rust
if i > 0 && nums[i] == nums[i - 1] { continue; }
```
**Purpose:** Skip duplicates for the first element. If nums[i] equals nums[i-1], we've already found all triplets starting with nums[i-1], so we skip to avoid duplicate triplets.

```rust
let mut left = i + 1;
let mut right = nums.len() - 1;
```
**Purpose:** Initialize two pointers for the remaining subarray after the fixed element at position `i`.

```rust
let sum = nums[i] + nums[left] + nums[right];
match sum.cmp(&0) {
    std::cmp::Ordering::Less => left += 1,
    std::cmp::Ordering::Greater => right -= 1,
```
**Purpose:** Standard two-pointer logic:
- If sum < 0, we need a larger sum (move left forward)
- If sum > 0, we need a smaller sum (move right backward)

```rust
std::cmp::Ordering::Equal => {
    result.push(vec![nums[i], nums[left], nums[right]]);
    left += 1;
    while left < right && nums[left] == nums[left - 1] {
        left += 1;
    }
}
```
**Purpose:** Found a triplet! Add it to results and skip all duplicate values for the `left` pointer to avoid duplicate triplets.

### Complexity Analysis

| Metric | Value | Explanation |
|--------|-------|-------------|
| **Time** | O(n²) | Sort O(n log n) + O(n²) for two-pointer search |
| **Space** | O(n) | Sorting copy and result vector |

### Key Insight: Why Sort + Two Pointers Works

1. Fix one element (nums[i])
2. We need nums[left] + nums[right] = -nums[i]
3. In a sorted array, moving left increases sum, moving right decreases sum
4. Each element is processed O(n) times, so O(n²) total

---

## Exercise 2: Three Sum Closest

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
            let sum = nums[i] + nums[left] + nums[right];
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

### Key Difference from Three Sum

Instead of checking for exact match, we track the smallest difference from target. Early exit when we find an exact match.

---

## Summary Table

| Exercise | Time | Space | Key Technique |
|----------|------|-------|---------------|
| 1: Three Sum | O(n²) | O(n) | Sort + Two Pointers |
| 2: Closest | O(n²) | O(n) | Track best diff |
| 3: Four Sum | O(n³) | O(n) | Nested + Two Pointers |
| 4: Count | O(n²) | O(n) | Count matches |
| 5: Smaller | O(n²) | O(1) | Count < target |
| 6: Two Sum Unique | O(n) | O(n) | HashMap |
| 7: Sorted | O(n²) | O(1) | Two Pointers |
| 8: Min Sum | O(n²) | O(1) | Closest to zero |

## Key Takeaways

1. **Sorting enables two-pointer** approach
2. **Skip duplicates** to avoid duplicate triplets
3. **Fix one, use two pointers** for the rest
4. **Track best diff** for closest problems
5. **O(n²) is optimal** for three-sum problems
