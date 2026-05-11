# Trapping Rain Water - LeetCode 42

## Problem Overview

Given `n` non-negative integers representing an elevation map where the width of each bar is 1, compute how much water it can trap after raining.

**Examples:**
```
Input: height = [0,1,0,2,1,0,1,3,2,1,2,1]
Output: 6

Input: height = [4,2,0,3,2,5]
Output: 9
```

## Theory

### Two Pointer Approach

For each position, water trapped = min(max_left, max_right) - height[i]

```
       |
   |   | |
 | | | | |
 | | | | |
-----------
```

### Visual

```
height: [0,1,0,2,1,0,1,3,2,1,2,1]

At index 2: water = min(1,2) - 0 = 1
At index 5: water = min(2,3) - 0 = 2
```

## Implementation

```rust
pub fn trap(height: Vec<i32>) -> i32 {
    let n = height.len();
    if n < 3 { return 0; }

    let mut left = 0;
    let mut right = n - 1;
    let mut left_max = height[left];
    let mut right_max = height[right];
    let mut water = 0;

    while left < right {
        if left_max < right_max {
            left += 1;
            left_max = left_max.max(height[left]);
            water += left_max - height[left];
        } else {
            right -= 1;
            right_max = right_max.max(height[right]);
            water += right_max - height[right];
        }
    }

    water
}
```

## Test Cases

```rust
#[test]
fn test_trap_basic() {
    assert_eq!(trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]), 6);
}
```