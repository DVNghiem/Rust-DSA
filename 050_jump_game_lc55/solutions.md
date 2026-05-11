# Jump Game - Solution Analysis

## Problem Overview

Given an array where each element represents maximum jump length from that position, determine if you can reach the last index. This is a classic greedy problem that demonstrates how local decisions lead to global solutions.

## Solution 1: Forward Greedy

### Code Implementation

```rust
pub fn can_jump(nums: Vec<i32>) -> bool {
    if nums.is_empty() {
        return false;
    }

    let mut max_reach = 0;

    for i in 0..nums.len() {
        if i as i32 > max_reach {
            return false;
        }
        max_reach = max_reach.max(i as i32 + nums[i]);
    }

    true
}
```

### Line-by-Line Analysis

**Line 1-3**: Handle empty input - return false since we can't reach anything from nothing.

**Line 5**: Initialize `max_reach` to 0, tracking the furthest index we can reach.

**Line 7-12**: Iterate through array positions:
- **Line 8-9**: If current position exceeds max_reach, we're stuck - return false
- **Line 10-11**: Update max_reach to maximum of current max and (current index + jump length)
- **Line 12**: If we complete the loop, we can reach the end

**Key insight**: The condition `i > max_reach` means we've encountered a gap we cannot cross.

### Greedy Logic Visualization

```
nums = [2,3,1,1,4]

i=0: max_reach = max(0, 0+2) = 2
i=1: 1 <= 2 → continue, max_reach = max(2, 1+3) = 4
i=2: 2 <= 4 → continue, max_reach = max(4, 2+1) = 4
i=3: 3 <= 4 → continue, max_reach = max(4, 3+1) = 4
i=4: 4 <= 4 → continue, max_reach = max(4, 4+4) = 8

Loop completes → return true
```

```
nums = [3,2,1,0,4]

i=0: max_reach = max(0, 0+3) = 3
i=1: 1 <= 3 → continue, max_reach = max(3, 1+2) = 3
i=2: 2 <= 3 → continue, max_reach = max(3, 2+1) = 3
i=3: 3 <= 3 → continue, max_reach = max(3, 3+0) = 3
i=4: 4 > 3 → return false (stuck at index 3, can't reach index 4)
```

## Solution 2: Dynamic Programming (Bottom-Up)

### Code Implementation

```rust
pub fn can_jump_dp(nums: Vec<i32>) -> bool {
    if nums.is_empty() {
        return false;
    }

    let n = nums.len();
    let mut dp = vec![false; n];
    dp[0] = true;

    for i in 0..n {
        if !dp[i] {
            continue;
        }
        let max_jump = nums[i] as usize;
        for j in 1..=max_jump.min(n - 1 - i) {
            dp[i + j] = true;
        }
    }

    dp[n - 1]
}
```

### Line-by-Line Analysis

**Line 1-3**: Handle empty input.

**Line 5-7**: Create DP array, all false initially. Mark start position as reachable.

**Line 9-15**: For each reachable position, mark all positions it can jump to as reachable.

**Line 16**: Return whether last position is reachable.

**Time complexity**: O(n²) - each position can potentially update many future positions.

**Space complexity**: O(n) - storing the DP array.

## Solution 3: Backward Greedy

### Code Implementation

```rust
pub fn can_jump_backward(nums: Vec<i32>) -> bool {
    if nums.is_empty() {
        return false;
    }

    let mut target = nums.len() - 1;

    for i in (0..target).rev() {
        if i as i32 + nums[i] >= target as i32 {
            target = i;
        }
    }

    target == 0
}
```

### Line-by-Line Analysis

**Line 1-3**: Handle empty input.

**Line 5**: Start from the last index as our target.

**Line 7-10**: Iterate backward - if current position can reach the target, update target to current position.

**Line 12**: If target is 0, we can reach from start.

**Key insight**: Working backward is often more intuitive - we ask "can I get to the target from here?"

### Backward Greedy Visualization

```
nums = [2,3,1,1,4]

target = 4
i=3: 3+1=4 >= 4 → target=3
i=2: 2+1=3 >= 3 → target=2
i=1: 1+3=4 >= 2 → target=1
i=0: 0+2=2 >= 1 → target=0

target=0 → return true
```

## Solution 4: Minimum Jumps (LC 45)

### Code Implementation

```rust
pub fn min_jump(nums: Vec<i32>) -> i32 {
    if nums.is_empty() || nums.len() == 1 {
        return 0;
    }

    let mut jumps = 0;
    let mut current_end = 0;
    let mut furthest = 0;

    for i in 0..nums.len() - 1 {
        furthest = furthest.max(i as i32 + nums[i]) as usize;
        if i == current_end {
            jumps += 1;
            current_end = furthest;
        }
    }

    jumps
}
```

### Line-by-Line Analysis

**Line 1-4**: Handle edge cases - empty or single element needs 0 jumps.

**Line 6-8**: Initialize counters for jumps, current range end, and furthest reachable.

**Line 10-14**: For each position (except last):
- Update furthest with maximum reach from current position
- When we reach the end of current "jump range", increment jumps and set new range

**Key insight**: This is a BFS-like approach - we count how many jumps needed to cover the range.

## Complexity Comparison

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Forward Greedy | O(n) | O(1) | Optimal solution |
| DP | O(n²) | O(n) | Too slow for large n |
| Backward Greedy | O(n) | O(1) | Equivalent to forward |
| Min Jumps | O(n) | O(1) | Extension to count jumps |

## Follow-up Answers

**Q: Why is greedy optimal?**
A: Because if we can reach a position, we don't need to consider how we got there. We only need to know the furthest we can go.

**Q: What does `i > max_reach` mean?**
A: It means there's a gap - position i is beyond everything we can currently reach. No previous position can help us reach i.

**Q: When does greedy fail?**
A: Greedy works for this problem because jump lengths are non-negative. Greedy can fail in variants where jumps have negative costs or other constraints.

**Q: Can we solve in O(n log n)?**
A: We can in the variant that asks for minimum jumps (LC 45), but the simple "can reach" question has an O(n) solution.

## Edge Cases Explained

1. **Empty array**: `[]` → false (can't reach anything)
2. **Single element**: `[5]` → true (already at goal)
3. **Zero at start**: `[0, ...]` → false (can't move)
4. **Zero at end**: `[1,1,0,1]` → false (stuck before end)
5. **All ones**: `[1,1,1,...]` → true (can always make progress)
6. **Large jump**: `[5,0,0,0,...]` → true (single large jump reaches end)

## Test Coverage

- Basic reachable case
- Unreachable (gap at end)
- Empty input
- Single element (0 and non-zero)
- All ones
- First element is zero
- Zero at end but reachable before
- DP and backward approaches
- Minimum jumps variant
- Large input stress test