# Non-Overlapping Intervals - LeetCode 435

## Problem Overview

Given an array of intervals `intervals`, return the minimum number of intervals you need to remove to make the rest non-overlapping.

**Examples:**
```
Input: intervals = [[1,2],[2,3],[3,4],[1,3]]
Output: 1 (remove [1,3])

Input: intervals = [[1,2],[2,3]]
Output: 0 (no removal needed)
```

## Theory

### Greedy Approach: Sort by End Time

Key insight: Always remove the interval that ends latest while keeping earlier end times.

```
Sort by end: [[1,2], [2,3], [3,4], [1,3]]
Pick:        [1,2] ✓
Pick:        [2,3] ✓ (doesn't overlap)
Pick:        [3,4] ✓
Skip:        [1,3] (overlaps with [1,2])

Result: 1 removal
```

## Implementation

```rust
pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
    if intervals.is_empty() { return 0; }

    let mut intervals = intervals;
    intervals.sort_by_key(|v| v[1]);

    let mut end = intervals[0][1];
    let mut count = 0;

    for i in 1..intervals.len() {
        if intervals[i][0] < end {
            count += 1;
        } else {
            end = intervals[i][1];
        }
    }

    count
}
```

## Test Cases

```rust
#[test]
fn test_erase_overlap_basic() {
    assert_eq!(erase_overlap_intervals(vec![vec![1,2],vec![2,3],vec![3,4],vec![1,3]]), 1);
}
```