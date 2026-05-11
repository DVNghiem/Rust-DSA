# Merge Intervals - LeetCode 56

## Problem Statement

Given an array of intervals where intervals[i] = [starti, endi], merge all overlapping intervals, and return an array of the non-overlapping intervals that cover all the intervals in the input.

## Visual Walkthrough

```
Example:
intervals = [[1,3],[2,6],[8,10],[15,18]]

After sorting by start:
[[1,3], [2,6], [8,10], [15,18]]

Merge:
[1,3] and [2,6] overlap → merge to [1,6]
[1,6] and [8,10] don't overlap → keep [1,6]
[15,18] doesn't overlap → keep

Result: [[1,6], [8,10], [15,18]]
```

### Sorting Approach

```
1. Sort intervals by start time
2. Iterate through sorted intervals
3. If current interval overlaps with result's last interval, merge
4. Otherwise, add as new interval
```

## Approach Comparison

| Approach | Time | Space | Description |
|----------|------|-------|-------------|
| Sort + Merge | O(n log n) | O(n) | Standard |
| Sweep Line | O(n log n) | O(n) | Alternative |

## Implementation Strategy

```rust
pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if intervals.is_empty() { return vec![]; }

    let mut intervals = intervals;
    intervals.sort_by_key(|v| v[0]);

    let mut result = vec![intervals[0].clone()];

    for i in 1..intervals.len() {
        let last = result.last_mut().unwrap();
        if intervals[i][0] <= last[1] {
            last[1] = last[1].max(intervals[i][1]);
        } else {
            result.push(intervals[i].clone());
        }
    }

    result
}
```

## Edge Cases

1. **Empty input**: Return empty
2. **Single interval**: Return it
3. **No overlapping**: Return sorted intervals
4. **All overlapping**: Return single merged interval

## Complexity Analysis

- **Time**: O(n log n) for sorting
- **Space**: O(n) for result

## Follow-up Questions

1. What if intervals can touch (end == start)?
2. How to track original indices?