# Meeting Rooms - Solution Analysis

## Problem Overview

Determine if a person could attend all meetings without conflicts. Each meeting has start and end time. Two meetings conflict if one starts before the other ends.

## Solution 1: Sort + Linear Scan

### Code Implementation

```rust
pub fn can_attend_meetings(intervals: Vec<Vec<i32>>) -> bool {
    if intervals.is_empty() { return true; }

    let mut intervals = intervals;
    intervals.sort_by_key(|v| v[0]);

    for i in 1..intervals.len() {
        if intervals[i][0] < intervals[i - 1][1] {
            return false;
        }
    }

    true
}
```

### Line-by-Line Analysis

**Line 1-2**: Handle empty case - if no meetings, return true (can attend nothing).

**Line 4**: Sort intervals by start time using `sort_by_key`. This ensures overlapping intervals are adjacent after sorting.

**Line 6-9**: Iterate through sorted intervals starting from index 1. If current interval's start is strictly less than previous interval's end, there's overlap.

**Key insight**: We use `<` not `<=` because meetings that touch at endpoints ([5,10] and [10,15]) don't overlap - you can attend both.

### Example Walkthrough

```
intervals = [[0,30],[5,10],[15,20]]

After sort: [[0,30], [5,10], [15,20]]

i=1: intervals[1][0]=5, intervals[0][1]=30
     5 < 30 → overlap detected → return false
```

```
intervals = [[5,15],[20,30],[35,45]]

After sort: [[5,15], [20,30], [35,45]]

i=1: 20 < 15? No → continue
i=2: 35 < 30? No → return true
```

## Solution 2: Min-Heap Approach

### Code Implementation

```rust
pub fn can_attend_meetings_heap(intervals: Vec<Vec<i32>>) -> bool {
    if intervals.is_empty() { return true; }

    use std::collections::BinaryHeap;
    let mut min_heap: BinaryHeap<i32> = BinaryHeap::new();

    let mut intervals = intervals;
    intervals.sort_by_key(|v| v[0]);

    for interval in intervals {
        if !min_heap.is_empty() && interval[0] >= -min_heap.peek().unwrap() {
            min_heap.pop();
        }
        min_heap.push(-interval[1]);
    }

    min_heap.len() == 1
}
```

### Line-by-Line Analysis

**Line 2-3**: Empty input returns true.

**Line 5-6**: Create a min-heap using BinaryHeap (Rust's max-heap, so we store negative values).

**Line 8**: Sort intervals by start time.

**Line 10-14**: For each interval, check if we can free a meeting room:
  - If current interval starts after (or at) the earliest ending meeting, that room is free
  - Push current interval's end time into heap
  - At the end, if we can fit all meetings in 1 room, heap size is 1

**Line 15**: If heap size is 1, we only needed 1 room, meaning no conflicts.

### Example Walkthrough

```
intervals = [[0,30],[5,10],[15,20]]

After sort: [[0,30], [5,10], [15,20]]

min_heap = []

[0,30]: heap empty, push -30 → [-30]
[5,10]: 5 >= 30? No (5 >= 30 is false), push -10 → [-30, -10]
[15,20]: 15 >= 10? Yes, pop -10, push -20 → [-30, -20]

heap size = 2 → need 2 rooms → cannot attend
```

```
intervals = [[5,15],[20,30],[35,45]]

min_heap = []

[5,15]: push -15 → [-15]
[20,30]: 20 >= 15? Yes, pop -15, push -30 → [-30]
[35,45]: 35 >= 30? Yes, pop -30, push -45 → [-45]

heap size = 1 → need 1 room → can attend
```

## Complexity Analysis

| Metric | Sort + Scan | Min-Heap |
|--------|-------------|----------|
| Time | O(n log n) | O(n log n) |
| Space | O(1) | O(n) |

## Follow-up Answers

**Q: Why sort first?**
A: After sorting by start time, overlapping intervals must be adjacent. This allows checking only adjacent pairs.

**Q: What's the difference between `<` and `<=`?**
A: `<` treats [5,10] and [10,15] as non-overlapping (valid). `<=` would incorrectly mark them as overlapping.

**Q: When would you use heap approach?**
A: When you also need to find minimum number of meeting rooms (LC 253). The heap approach generalizes to that problem.

**Q: Can we do better than O(n log n)?**
A: Not for the general case. Sorting is required since we need to know which intervals come after which.

## Edge Cases Explained

1. **Empty input**: `[]` → true (nothing to conflict)
2. **Single meeting**: `[5,10]` → true (no other meetings)
3. **No overlapping**: Sorted and checked, all clear → true
4. **Touching intervals**: `[1,5], [5,10]` → 5 < 5 is false → true (can attend)
5. **Contained interval**: `[1,10], [3,5]` → 3 < 10 is true → false (conflict)

## Testing Strategy

The test cases cover:
- Basic overlapping scenario
- Empty input edge case
- Single meeting edge case
- Non-overlapping meetings
- Adjacent (touching) meetings
- One interval containing another
- Exactly same intervals
- Unsorted input
- Negative times
- Large gaps between meetings
- Various heap scenarios