# Longest Consecutive Sequence - LeetCode 128

## Problem Statement

Given an unsorted array of integers, find the length of the longest consecutive sequence.

```
Example:
Input: [100, 4, 200, 1, 3, 2]
Output: 4
Explanation: [1, 2, 3, 4] is the longest consecutive sequence
```

## Key Insight

Using a HashSet, we can achieve O(n) time complexity.

```
Algorithm:
1. Insert all elements into HashSet
2. For each element, check if it's the start of a sequence
3. If num-1 is not in set, num is start
4. Count consecutive elements starting from num
```

## Complexity: O(n) time, O(n) space

## Test Cases
- No consecutive elements
- All consecutive
- Duplicate values