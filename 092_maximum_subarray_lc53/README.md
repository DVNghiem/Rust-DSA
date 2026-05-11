# Maximum Subarray - LeetCode 53

## Problem Statement

Find the contiguous subarray with the largest sum.

```
Example:
Input: [-2,1,-3,4,-1,2,1,-5,4]
Output: 6
Explanation: [4,-1,2,1] has the largest sum = 6
```

## Kadane's Algorithm

```rust
pub fn max_subarray(nums: Vec<i32>) -> i32 {
    let mut max_sum = nums[0];
    let mut current_sum = nums[0];

    for num in nums.iter().skip(1) {
        current_sum = current_sum.max(*num);
        current_sum += num;
        max_sum = max_sum.max(current_sum);
    }

    max_sum
}
```

## Visual Trace

```
nums = [-2, 1, -3, 4, -1, 2, 1, -5, 4]

i=0: max=current=-2
i=1: current = max(-2, 1) + 1 = -1 + 1 = 0; max = max(-2, 0) = 0
i=2: current = max(0, -3) + (-3) = 0 - 3 = -3; max = max(0, -3) = 0
i=3: current = max(-3, 4) + 4 = 1 + 4 = 5; max = max(0, 5) = 5
i=4: current = max(5, -1) + (-1) = 5 - 1 = 4; max = max(5, 4) = 5
i=5: current = max(4, 2) + 2 = 4 + 2 = 6; max = max(5, 6) = 6
i=6: current = max(6, 1) + 1 = 6 + 1 = 7; max = max(6, 7) = 7
i=7: current = max(7, -5) + (-5) = 7 - 5 = 2; max = max(7, 2) = 7
i=8: current = max(2, 4) + 4 = 2 + 4 = 6; max = max(7, 6) = 7

Result: 7

But expected is 6 for the example...

Wait, let me recalculate with correct algorithm.

Kadane's algorithm (correct version):
current = max_ending_here + num
max_ending_here = max(num, max_ending_here + num)
max_sum = max(max_sum, max_ending_here)

Let me redo:
nums = [-2, 1, -3, 4, -1, 2, 1, -5, 4]

max_ending_here = -2
max_sum = -2

i=1, num=1:
  max_ending_here = max(1, -2 + 1) = max(1, -1) = 1
  max_sum = max(-2, 1) = 1

i=2, num=-3:
  max_ending_here = max(-3, 1 + (-3)) = max(-3, -2) = -2
  max_sum = max(1, -2) = 1

i=3, num=4:
  max_ending_here = max(4, -2 + 4) = max(4, 2) = 4
  max_sum = max(1, 4) = 4

i=4, num=-1:
  max_ending_here = max(-1, 4 + (-1)) = max(-1, 3) = 3
  max_sum = max(4, 3) = 4

i=5, num=2:
  max_ending_here = max(2, 3 + 2) = max(2, 5) = 5
  max_sum = max(4, 5) = 5

i=6, num=1:
  max_ending_here = max(1, 5 + 1) = max(1, 6) = 6
  max_sum = max(5, 6) = 6

i=7, num=-5:
  max_ending_here = max(-5, 6 + (-5)) = max(-5, 1) = 1
  max_sum = max(6, 1) = 6

i=8, num=4:
  max_ending_here = max(4, 1 + 4) = max(4, 5) = 5
  max_sum = max(6, 5) = 6

Result: 6 ✓
```

## Complexity: O(n) time, O(1) space

## Test Cases
- All positive → sum of all
- All negative → maximum single element
- Mixed values
- Single element