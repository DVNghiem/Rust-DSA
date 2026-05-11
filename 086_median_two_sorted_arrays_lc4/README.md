# Median of Two Sorted Arrays - LeetCode 4

## Problem Statement

Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.

```
Example:
Input: nums1 = [1, 3], nums2 = [2]
Output: 2.0

Input: nums1 = [1, 2], nums2 = [3, 4]
Output: 2.5
```

## Understanding the Problem

Finding the median of two sorted arrays is harder than it seems because we need O(log n) complexity, not O(m+n).

```
Two sorted arrays:
[1, 3, 5, 7, 9]  (length 5)
[2, 4, 6, 8]     (length 4)

Combined sorted: [1, 2, 3, 4, 5, 6, 7, 8, 9] (length 9)
Median at position 4 (0-indexed) = 5

We need O(log(m+n)) algorithm, not O(m+n) merge!
```

## Core Insight: Binary Search on Partition

Think of partitioning both arrays such that:
- Left part has same number of elements as right part (or differs by 1)
- All elements in left are ≤ all elements in right

```
Array A: [1, 3, 5, 7, 9]
Array B: [2, 4, 6, 8]

Partition A at index 2: A_left=[1,3], A_right=[5,7,9]
Partition B at index 2: B_left=[2,4], B_right=[6,8]

Combined left: [1,3,2,4] = [1,2,3,4]
Combined right: [5,7,6,8] = [5,6,7,8]

If max(left) ≤ min(right), we found the partition!
```

## Key Observations

1. If we know how many elements go to left side, we can find median
2. Binary search on one array to find partition in other
3. If partition is at i in A and j in B, then i+j = (m+n+1)/2 (left side count)

## Visual Walkthrough

```
A = [1, 3, 5, 7] (m=4)
B = [2, 4, 6]    (n=3)
Total = 7, median position = 3 (0-indexed)

Find i (partition in A) and j (partition in B) such that:
- i + j = 3 (half of total)
- A[i-1] ≤ B[j] and B[j-1] ≤ A[i]

Try i = 2:
j = 3 - 2 = 1

A_left = [1, 3]  max = 3
B_left = [2]      max = 2
A_right = [5, 7] min = 5
B_right = [4, 6] min = 4

3 > 4? No wait, A[i-1]=3, B[j]=4 → 3 ≤ 4 ✓
B[j-1]=2, A[i]=5 → 2 ≤ 5 ✓

Valid partition! Left = [1,3,2] = [1,2,3], max=3
                 Right = [5,7,4,6] = [4,5,6,7], min=4

Median = max(left) = 3 (since odd total)
```

## Four Approaches

| Approach | Time | Space | Description |
|----------|------|-------|-------------|
| Merge (simple) | O(m+n) | O(m+n) | Merge and find median |
| Two pointers | O(m+n) | O(1) | Walk through without storing |
| Binary search | O(log m) | O(1) | Find partition via BS |
| Recursive | O(log(m+n)) | O(log(m+n)) | Recursive partition |

## Binary Search Approach

```rust
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (m, n) = (nums1.len(), nums2.len());
    // Ensure nums1 is smaller for simpler binary search
    if m > n {
        return find_median_sorted_arrays(nums2, nums1);
    }

    let mut left = 0;
    let mut right = m;

    while left <= right {
        let partition_x = (left + right) / 2;
        let partition_y = (m + n + 1) / 2 - partition_x;

        let max_left_x = if partition_x == 0 { i32::MIN } else { nums1[partition_x - 1] };
        let min_right_x = if partition_x == m { i32::MAX } else { nums1[partition_x] };

        let max_left_y = if partition_y == 0 { i32::MIN } else { nums2[partition_y - 1] };
        let min_right_y = if partition_y == n { i32::MAX } else { nums2[partition_y] };

        if max_left_x <= min_right_y && max_left_y <= min_right_x {
            // Found correct partition
            if (m + n) % 2 == 0 {
                return (max_left_x.max(max_left_y) + min_right_x.min(min_right_y)) as f64 / 2.0;
            } else {
                return max_left_x.max(max_left_y) as f64;
            }
        } else if max_left_x > min_right_y {
            right = partition_x - 1;
        } else {
            left = partition_x + 1;
        }
    }

    0.0 // Should never reach here
}
```

## Complexity Analysis

| Approach | Time | Space |
|----------|------|-------|
| Merge | O(m+n) | O(m+n) |
| Two pointers | O(m+n) | O(1) |
| Binary search | O(log m) | O(1) |

## Edge Cases

### Empty arrays

```rust
nums1 = [], nums2 = [1]
→ median = 1
```

### One element each

```rust
nums1 = [1], nums2 = [2]
→ median = 1.5
```

### Different sizes

```rust
nums1 = [1, 2], nums2 = [3, 4, 5, 6]
→ combined = [1, 2, 3, 4, 5, 6]
→ median = (3+4)/2 = 3.5
```

## Related Problems

1. **LeetCode 4**: Median of Two Sorted Arrays (this problem)
2. **LeetCode 876**: Middle of the Linked List
3. **LeetCode 295**: Find Median from Data Stream

## Time to Complete

**Target**: 60 minutes
**Optimal**: 45 minutes