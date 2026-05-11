# Sliding Window Maximum - LeetCode 239

## Problem Statement

Given an array nums, there is a sliding window of size k which moves from left to right. Return the max value in each window position.

```
Example:
Input: nums = [1,3,-1,-3,5,3,6,7], k = 3
Output: [3,3,5,5,6,7]
```

## Visual Walkthrough

```
Window at each step:
[1, 3, -1] → max = 3
  [3, -1, -3] → max = 3
    [-1, -3, 5] → max = 5
      [-3, 5, 3] → max = 5
        [5, 3, 6] → max = 6
          [3, 6, 7] → max = 7

Output: [3, 3, 5, 5, 6, 7]
```

## Key Insight: Monotonic Queue

Use a deque that maintains indices in decreasing order of their values.

```
When new element arrives:
1. Remove indices from back while their values <= new value
2. Add new index to back
3. Remove indices from front if outside window
4. Front of deque is the maximum

Process: [1, 3, -1, -3, 5, 3, 6, 7], k=3

i=0: deque=[0] → result=[]
i=1: nums[1]=3 > nums[0]=1, remove 0 → deque=[1] → result=[]
i=2: nums[2]=-1, keep → deque=[1,2] → result=[3]
i=3: nums[3]=-3, keep → deque=[1,2,3] → remove front if < i-k+1 (0) → result=[3,3]
i=4: nums[4]=5 > 3,1,-1 → deque=[4] → remove 1,2,3 → result=[3,3,5]
i=5: nums[5]=3, keep → deque=[4,5] → result=[3,3,5,5]
i=6: nums[6]=6 > 5,3 → deque=[6] → result=[3,3,5,5,6]
i=7: nums[7]=7 > 6 → deque=[7] → result=[3,3,5,5,6,7]
```

## Complexity: O(n) time, O(k) space

## Test Cases
- Single element window
- All same values
- Decreasing array
- Increasing array