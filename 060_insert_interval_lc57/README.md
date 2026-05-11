# Insert Interval - LeetCode 57

## Problem Overview

Given a set of non-overlapping intervals sorted by start time, and a new interval, insert it and merge overlapping intervals.

**Examples:**
```
Input: intervals = [[1,3],[6,9]], newInterval = [2,5]
Output: [[1,9]]

Input: intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
Output: [[1,2],[3,10],[12,16]]
```

## Theory

### Three Cases for Each Interval

1. **No overlap (current.end < new.start)**: keep current, continue
2. **Complete overlap (new covers current)**: skip current
3. **Partial overlap**: merge (min start, max end)

## Implementation

```rust
pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let (mut i, n) = (0, intervals.len());
    let (new_start, new_end) = (new_interval[0], new_interval[1]);

    // Add all intervals before new_interval
    while i < n && intervals[i][1] < new_start {
        result.push(intervals[i].clone());
        i += 1;
    }

    // Merge overlapping intervals
    let mut start = new_start;
    let mut end = new_end;
    while i < n && intervals[i][0] <= end {
        start = start.min(intervals[i][0]);
        end = end.max(intervals[i][1]);
        i += 1;
    }
    result.push(vec![start, end]);

    // Add remaining intervals
    while i < n {
        result.push(intervals[i].clone());
        i += 1;
    }

    result
}
```

## Test Cases

```rust
#[test]
fn test_insert_basic() {
    assert_eq!(insert(vec![vec![1,3],vec![6,9]], vec![2,5]), vec![vec![1,9]]);
}
```