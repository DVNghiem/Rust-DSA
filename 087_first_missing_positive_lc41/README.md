# First Missing Positive - LeetCode 41

## Problem Statement

Given an unsorted integer array, find the smallest missing positive integer.

```
Example 1:
Input: [1,2,0] → Output: 3

Example 2:
Input: [3,4,-1,1] → Output: 2

Example 3:
Input: [7,8,9,11,12] → Output: 1
```

## Key Insight

The answer is always between 1 and n+1 where n is the array length. This is because with n numbers, we can at most fill positions 1 through n.

## In-Place Marking Approach

Put each number in its correct position (index = value - 1) if possible.

```rust
pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    let n = nums.len() as i32;

    for i in 0..n as usize {
        while nums[i] > 0 && nums[i] <= n && nums[(nums[i] - 1) as usize] != nums[i] {
            let idx = (nums[i] - 1) as usize;
            nums.swap(i, idx);
        }
    }

    for i in 0..n as usize {
        if nums[i] != (i + 1) as i32 {
            return (i + 1) as i32;
        }
    }
    n + 1
}
```

## Complexity: O(n) time, O(1) space

## Test Cases
- Empty array → 1
- Single element → depends
- All positives from 1 to n → n+1
- Missing in middle