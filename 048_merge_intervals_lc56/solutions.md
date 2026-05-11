# Merge Intervals - Solution Analysis

## Problem Overview

Merge overlapping intervals. Sort by start time, then merge if overlapping.

## Solution: Sort + Merge

### Code Implementation

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

### Key Points

1. **Sort by start**: Enables linear scan for merging
2. **Overlap condition**: current.start <= last.end
3. **Extend end**: max(last.end, current.end)

### Example: [[1,3],[2,6],[8,10],[15,18]]

```
After sort: [[1,3], [2,6], [8,10], [15,18]]

result = [[1,3]]
[2,6]: 2 <= 3 (overlap) → extend to [1,6]
[8,10]: 8 > 6 (no overlap) → add new
[15,18]: 15 > 10 (no overlap) → add new

Result: [[1,6], [8,10], [15,18]] ✓
```

## Complexity

| Metric | Value |
|--------|-------|
| Time | O(n log n) for sorting |
| Space | O(n) for result |

## Follow-up Answers

**Q: Why sort first?**
A: After sorting, overlapping intervals must be consecutive. Enables single pass merge.

**Q: What about touching intervals [1,2] and [2,3]?**
A: They overlap because 2 <= 3. Merge to [1,3].

**Q: Can we avoid sorting?**
A: Not efficiently - unsorted requires O(n²) pairwise checking.