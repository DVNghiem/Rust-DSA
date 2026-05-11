# Longest Increasing Subsequence - LeetCode 300

## Problem Statement

Given an integer array `nums`, return the length of the longest strictly increasing subsequence.

A subsequence is a sequence that can be derived by deleting some or no elements without changing the order of the remaining elements.

## Visual Walkthrough

```
Example:
nums = [10, 9, 2, 5, 3, 7, 101, 18]

Longest increasing subsequence: [2, 3, 7, 101] or [2, 3, 7, 18]
Length = 4

Another valid subsequence: [2, 5, 7, 101]
```

### DP Approach

```
dp[i] = length of LIS ending at index i

For each i:
  For each j < i:
    if nums[j] < nums[i]:
      dp[i] = max(dp[i], dp[j] + 1)

LIS = max(dp[i]) over all i

Time: O(n²)
Space: O(n)
```

### Binary Search Approach (Optimal)

```
Use patience sorting technique.
Maintain an array "tails" where tails[i] = smallest tail value for LIS of length i+1.

For each num:
  - Find position using binary search
  - Replace or append

Length of tails = LIS length
```

## Approach Comparison

| Approach | Time | Space | Description |
|----------|------|-------|-------------|
| DP O(n²) | O(n²) | O(n) | Simple but slow |
| Binary Search | O(n log n) | O(n) | Optimal |
| Patience Sorting | O(n log n) | O(n) | Same as binary search |

## Implementation Strategy

```rust
pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    if nums.is_empty() { return 0; }

    let mut tails = Vec::new();

    for num in nums {
        match tails.binary_search(&num) {
            Ok(pos) => tails[pos] = num,
            Err(pos) => {
                if pos == tails.len() {
                    tails.push(num);
                } else {
                    tails[pos] = num;
                }
            }
        }
    }

    tails.len() as i32
}
```

## Edge Cases

1. **Empty array**: Return 0
2. **Single element**: Return 1
3. **All decreasing**: Return 1
4. **All same**: Return 1 (strictly increasing)
5. **Already sorted**: Return n

## Test Cases

1. Basic LIS
2. Empty array
3. Single element
4. Decreasing array
5. All same values

## Solution Explanation

### Key Insight

Binary search approach: maintain smallest tail for each LIS length. For each number, find its position in tails. If larger than all, append; otherwise replace first larger element.

## Complexity Analysis

- **Time**: O(n log n) for binary search approach
- **Space**: O(n) for tails array

## Follow-up Questions

1. How to return actual subsequence?
2. What if we need non-decreasing?