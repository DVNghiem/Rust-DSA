# Sliding Window Maximum - LeetCode 239

## Problem Overview

Given an array `nums` and a sliding window of size `k`, return the max value in each window as it slides from left to right.

**Examples:**
```
Input: nums = [1,3,-1,-3,5,3,6,7], k = 3
Output: [3,3,5,5,6,7]
```

## Theory

### Monotonic Deque

Use a deque storing indices, maintaining decreasing order:
- Front always has the max for current window
- Remove indices outside current window
- Remove smaller elements from back (they'll never be max)

```
nums = [1,3,-1,-3,5,3,6,7], k = 3

Step 1: [1]        → max = 1
Step 2: [1,3]      → max = 3
Step 3: [3,-1]     → max = 3
Step 4: [3,-1,-3]  → max = 3
Step 5: [-1,-3,5]  → max = 5
...
```

## Implementation

```rust
pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::collections::VecDeque;

    let k = k as usize;
    let n = nums.len();
    if n == 0 || k == 0 { return vec![]; }

    let mut result = Vec::new();
    let mut deque: VecDeque<usize> = VecDeque::new();

    for i in 0..n {
        // Remove indices outside window
        while !deque.is_empty() && deque[0] <= i - k {
            deque.pop_front();
        }

        // Remove smaller elements from back
        while !deque.is_empty() && nums[*deque.back().unwrap()] < nums[i] {
            deque.pop_back();
        }

        deque.push_back(i);

        // Record max (window ready when i >= k - 1)
        if i >= k - 1 {
            result.push(nums[*deque.front().unwrap()]);
        }
    }

    result
}
```

## Test Cases

```rust
#[test]
fn test_max_window_basic() {
    assert_eq!(max_sliding_window(vec![1,3,-1,-3,5,3,6,7], 3), vec![3,3,5,5,6,7]);
}
```