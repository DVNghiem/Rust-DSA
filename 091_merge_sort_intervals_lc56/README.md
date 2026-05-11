# Merge Intervals - LeetCode 56

## Problem Statement

Given an array of intervals, merge all overlapping intervals.

```
Example:
Input: [[1,3],[2,6],[8,10],[15,18]]
Output: [[1,6],[8,10],[15,18]]
```

## Visual Walkthrough

```
Intervals sorted by start:
[[1,3],[2,6],[8,10],[15,18]]

Process:
[1,3] and [2,6] overlap → merge to [1,6]
[1,6] and [8,10] don't overlap → add [1,6], process [8,10]
[8,10] and [15,18] don't overlap → add [8,10], [15,18]

Result: [[1,6],[8,10],[15,18]]
```

## Algorithm

```rust
pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if intervals.is_empty() { return vec![]; }

    let mut intervals = intervals;
    intervals.sort_by_key(|i| i[0]);

    let mut result = vec![intervals[0].clone()];

    for interval in intervals.iter().skip(1) {
        let last = result.last_mut().unwrap();
        if interval[0] <= last[1] {
            last[1] = last[1].max(interval[1]);
        } else {
            result.push(interval.clone());
        }
    }

    result
}
```

## Complexity: O(n log n) time, O(n) space

## Test Cases
- No overlaps
- Complete overlaps
- Adjacent intervals (touching but not overlapping)