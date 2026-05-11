# Non-Overlapping Intervals Solution - LeetCode 435 (Complete)

## Solution Analysis

### Greedy: Sort by End Time

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

## Why Sort by End Time?

### The Greedy Choice

When selecting intervals to keep:
- Always pick the interval that ends earliest
- This leaves maximum room for future intervals

### Proof of Correctness

```
Suppose optimal solution picks interval I that ends later than available interval J.
If we replace I with J (which ends earlier):
- We don't lose any potential intervals (J ends earlier, leaves more room)
- We're still valid (J doesn't overlap with intervals before I)
```

Therefore, picking earliest-ending interval is always safe.

## Line-by-Line Analysis

### Handle Empty Case
```rust
if intervals.is_empty() { return 0; }
```
- No intervals to remove

### Sort by End Time
```rust
intervals.sort_by_key(|v| v[1]);
```
- Sort by the end coordinate (index 1)
- `[1, 3], [2, 4], [1, 2]` becomes `[1, 2], [1, 3], [2, 4]`

### Initialize
```rust
let mut end = intervals[0][1];  // End time of first interval
let mut count = 0;              // Number of intervals to remove
```
- Start with first interval (we'll keep it)

### Iterate and Check Overlap
```rust
for i in 1..intervals.len() {
    if intervals[i][0] < end {
        // Overlaps with previous interval, need to remove one
        count += 1;
    } else {
        // No overlap, can keep this interval
        end = intervals[i][1];
    }
}
```
- If current interval starts before `end`, they overlap
- Keep the one that ends earliest (already in `end`)
- If no overlap, update `end` to current interval's end

## Visual Example

```
intervals = [[1,2],[2,3],[3,4],[1,3]]

After sorting by end:
[1,2], [2,3], [3,4], [1,3]

Keep [1,2], end=2
Check [2,3]: start=2 >= 2, no overlap, keep, end=3
Check [3,4]: start=3 >= 3, no overlap, keep, end=4
Check [1,3]: start=1 < 4, overlap, remove

Result: 1 interval removed
```

## Complexity Analysis

| Approach | Time | Space |
|----------|------|-------|
| Greedy | O(n log n) | O(1) |
| DP | O(n²) | O(n) |

Sort dominates, then single pass.

## Edge Cases

### Empty Input
```rust
Input: []
Output: 0
```

### Single Interval
```rust
Input: [[1, 5]]
Output: 0 (no removal needed)
```

### All Overlap
```rust
Input: [[1, 5], [2, 6], [3, 7]]
After sort: [[1, 5], [2, 6], [3, 7]]
Keep [1, 5], remove [2, 6], remove [3, 7]
Output: 2
```

### None Overlap
```rust
Input: [[1, 2], [3, 4], [5, 6]]
Output: 0
```

## Why Not Sort by Start?

Sorting by start might work, but sorting by end is more robust:
- Guarantees we always consider intervals that end earliest
- Simpler overlap check

## Common Mistakes

1. **Not sorting**: Without sorting, greedy doesn't work
2. **Using wrong comparison**: Should check `intervals[i][0] < end` not `<=`
3. **Forgetting empty case**: Could cause index out of bounds