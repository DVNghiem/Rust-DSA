# Jump Game - LeetCode 55

## Problem Statement

Given an array of non-negative integers where each element represents your maximum jump length from that position, determine if you can reach the last index.

## Visual Walkthrough

```
Example:
nums = [2,3,1,1,4]

Index 0: can jump 1-2 steps
Index 1: can jump 1-3 steps
Index 2: can jump 1-1 step
Index 3: can jump 1-4 steps
Index 4: last index

From 0 (jump 2): to index 2
From 2 (jump 1): to index 3
From 3 (jump 4): to index 4 ✓

Result: true
```

## Greedy Approach

```
nums = [2,3,1,1,4]

Iterate and track furthest reachable:
i=0: reach=2, max_reach=2
i=1: reach=4, max_reach=4
i=2: reach=4, max_reach=4
i=3: reach=4, max_reach=4
i=4: within bounds → true
```

## Approach Comparison

| Approach | Time | Space | Description |
|----------|------|-------|-------------|
| Backtracking | O(2^n) | O(n) | Exponential - too slow |
| DP (top-down) | O(n²) | O(n) | Memorization |
| Greedy | O(n) | O(1) | Track max reach |
| BFS | O(n²) | O(n) | Queue-based |

## Implementation Strategy

```rust
pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut max_reach = 0;

    for i in 0..nums.len() {
        if i > max_reach {
            return false;
        }
        max_reach = max_reach.max(i as i32 + nums[i]);
    }

    true
}
```

## Edge Cases

1. **Empty input**: Return false (can't reach anything)
2. **Single element**: Return true (already at goal)
3. **Zero jump**: If stuck at index 0, return false
4. **Can reach end**: Track maximum reachable index