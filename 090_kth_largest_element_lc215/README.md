# Kth Largest Element in an Array - LeetCode 215

## Problem Statement

Find the kth largest element in an unsorted array.

```
Example:
Input: [3,2,1,5,6,4], k = 2
Output: 5

Second largest is 5 (sorted: [6,5,4,3,2,1])
```

## Approaches

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Sort | O(n log n) | O(1) | Simple |
| Quickselect | O(n) avg | O(1) | Faster average |
| Min-heap | O(n log k) | O(k) | Good for streaming |

## Quickselect

```rust
pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut nums = nums;
    quickselect(&mut nums, 0, nums.len() - 1, nums.len() - k)
}
```

## Test Cases
- Basic kth largest
- k=1 (maximum)
- k=n (minimum)
- Duplicate values