# Meeting Rooms - LeetCode 252

## Problem Statement

Given an array of meeting time intervals where intervals[i] = [starti, endi], determine if a person could attend all meetings.

## Visual Walkthrough

```
Example:
intervals = [[0,30],[5,10],[15,20]]

Sort by start:
[[0,30], [5,10], [15,20]]

Check overlap:
[0,30] and [5,10]: 5 < 30 (overlap) → cannot attend all
[0,30] and [15,20]: 15 < 30 (overlap) → cannot attend all

Result: false
```

## Approach Comparison

| Approach | Time | Space | Description |
|----------|------|-------|-------------|
| Sort + Check | O(n log n) | O(1) | Check adjacent pairs |

## Implementation Strategy

```rust
pub fn can_attend_meetings(intervals: Vec<Vec<i32>>) -> bool {
    if intervals.is_empty() { return true; }

    let mut intervals = intervals;
    intervals.sort_by_key(|v| v[0]);

    for i in 1..intervals.len() {
        if intervals[i][0] < intervals[i-1][1] {
            return false;
        }
    }

    true
}
```

## Edge Cases

1. **Empty input**: Return true
2. **Single meeting**: Return true
3. **No overlapping**: Return true
4. **Touching meetings**: Return true (not overlapping)
