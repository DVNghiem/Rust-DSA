# Insert Interval Solution - LeetCode 57 (Complete)

## Solution Analysis

### Three-Phase Insertion

```rust
pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut i = 0;
    let n = intervals.len();
    let (new_start, new_end) = (new_interval[0], new_interval[1]);

    // Phase 1: Add intervals before new_interval (completely before)
    while i < n && intervals[i][1] < new_start {
        result.push(intervals[i].clone());
        i += 1;
    }

    // Phase 2: Merge all overlapping intervals
    let mut start = new_start;
    let mut end = new_end;
    while i < n && intervals[i][0] <= end {
        start = start.min(intervals[i][0]);
        end = end.max(intervals[i][1]);
        i += 1;
    }
    result.push(vec![start, end]);

    // Phase 3: Add remaining intervals
    while i < n {
        result.push(intervals[i].clone());
        i += 1;
    }

    result
}
```

## Three Phases Explained

### Phase 1: Before New Interval

```rust
while i < n && intervals[i][1] < new_start {
    result.push(intervals[i].clone());
    i += 1;
}
```
- Add all intervals that end BEFORE new interval starts
- These don't overlap with new_interval

### Phase 2: Overlapping Intervals

```rust
while i < n && intervals[i][0] <= end {
    start = start.min(intervals[i][0]);
    end = end.max(intervals[i][1]);
    i += 1;
}
result.push(vec![start, end]);
```
- Merge new_interval with all intervals that overlap
- Keep expanding start/end to encompass all
- Condition: `intervals[i][0] <= end` means current interval starts before or at current end (overlap)

### Phase 3: After Merged Interval

```rust
while i < n {
    result.push(intervals[i].clone());
    i += 1;
}
```
- Add all remaining intervals
- These start AFTER the merged interval ends

## Visual Example

### Input: intervals = [[1,3],[6,9]], new_interval = [2,5]

```
Phase 1: [1,3] ends at 3, new starts at 2
         3 < 2? No, so don't add yet

Phase 2: Merge [1,3] with [2,5]
         start = min(1, 2) = 1
         end = max(3, 5) = 5
         Check [6,9]: 6 <= 5? No, stop
         Add merged: [1,5]

Phase 3: Add [6,9]

Result: [[1,5], [6,9]]
```

### Another Example

```
Input: [[1,2],[3,5],[6,7],[8,10],[12,16]], new = [4,8]

Phase 1: [1,2] doesn't overlap (2 < 4) → add
Phase 2: [3,5] overlaps (3 <= 5) → merge to [3,5]
         [6,7] overlaps (6 <= 5? No) → stop
         Add [3,5]? Wait, we haven't added yet...
         
Actually:
start = 4, end = 8
[3,5] overlaps (3 <= 8), start=min(4,3)=3, end=max(8,5)=8
[6,7] overlaps (6 <= 8), start=min(3,6)=3, end=max(8,7)=8
Now result = [[1,2], [3,8]]
Phase 3: add [8,10], [12,16]

Result: [[1,2], [3,8], [8,10], [12,16]]
```

Wait, let me trace more carefully:
- After Phase 1: result = [[1,2]], i = 1
- Phase 2: start=4, end=8
  - intervals[1] = [3,5], 3 <= 8 → merge: start=min(4,3)=3, end=max(8,5)=8, i=2
  - intervals[2] = [6,7], 6 <= 8 → merge: start=min(3,6)=3, end=max(8,7)=8, i=3
  - intervals[3] = [8,10], 8 <= 8 → merge: start=min(3,8)=3, end=max(8,10)=10, i=4
  - intervals[4] = [12,16], 12 <= 10? No, stop
- Add [3,10] to result
- Phase 3: add [12,16]
- Final: [[1,2], [3,10], [12,16]]

## Complexity Analysis

| Approach | Time | Space |
|----------|------|-------|
| Three-phase | O(n) | O(n) |

Single pass through intervals.

## Edge Cases

### Empty Input
```rust
Input: [], new = [5, 8]
Output: [[5, 8]]
```

### No Overlap Before
```rust
Input: [[1,2]], new = [4,6]
Output: [[1,2], [4,6]]
```

### No Overlap After
```rust
Input: [[5,8]], new = [1,3]
Output: [[1,3], [5,8]]
```

### Completely Contained
```rust
Input: [[2,6]], new = [3,5]
Output: [[2,6]]
```

### New Contains Existing
```rust
Input: [[2,4]], new = [1,10]
Output: [[1,10]]
```

## Common Mistakes

1. **Confusing overlap conditions**:
   - Phase 1: `intervals[i][1] < new_start` (ends before new starts)
   - Phase 2: `intervals[i][0] <= end` (starts before/when merged ends)

2. **Not updating start/end during merge**: Must use min/max to expand

3. **Forgetting to add merged interval**: Phase 2 doesn't add until after loop