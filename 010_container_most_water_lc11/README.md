# Container With Most Water (LeetCode #11)

## Problem Statement

You are given an integer array `height` of length `n`. There are `n` vertical lines drawn such that the two endpoints of the ith line are `(i, 0)` and `(i, height[i])`.

Find two lines that together with the x-axis form a container that holds the most water.

Return the maximum amount of water a container can store.

## Examples

```
Input: height = [1, 8, 6, 2, 5, 4, 8, 3, 7]
Output: 49
Explanation: Container formed by lines at index 1 (height 8) and index 8 (height 7).
```

## The Key Insight

The area of water a container can hold is determined by:
- **Width**: The distance between the two lines (right index - left index)
- **Height**: The shorter of the two line heights (water overflows if taller)

```
Area = width * min(height[left], height[right])
```

## Visual Walkthrough

```
height = [1, 8, 6, 2, 5, 4, 8, 3, 7]

Visual representation:
    |
|   |         |
|   |       | |
|   |     | | |
|   |     | | |
|   | | | | | |
|   | | | | | |
|_|_|_|_|_|_|_|
  0 1 2 3 4 5 6 7 8

Step 1: left=0, right=8, width=8, min_height=1, area=8
Step 2: left=0, right=7, width=7, min_height=1, area=7
... (moving inward)
Best container: left=1 (height=8), right=8 (height=7), width=7, height=7, area=49
```

## Approaches Overview

### Approach 1: Brute Force O(n²)
Check all pairs of lines.

```rust
pub fn max_area_brute(height: &[i32]) -> i32 {
    let mut max_area = 0;
    for i in 0..height.len() {
        for j in (i + 1)..height.len() {
            let width = (j - i) as i32;
            let h = height[i].min(height[j]);
            let area = width * h;
            max_area = max_area.max(area);
        }
    }
    max_area
}
```

### Approach 2: Two Pointers O(n) - Preferred
Start with widest container, move the shorter line inward.

```rust
pub fn max_area(height: &[i32]) -> i32 {
    let (mut left, mut right) = (0, height.len() - 1);
    let mut max_area = 0;

    while left < right {
        let width = (right - left) as i32;
        let h = height[left].min(height[right]);
        max_area = max_area.max(width * h);

        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }
    max_area
}
```

## Why Two Pointers Works

Starting with the widest container gives us the maximum possible width. Moving the shorter line inward is the only way to potentially find a better area because:

1. If we move the taller line inward, the width decreases and the height is still bounded by the shorter line
2. If we move the shorter line inward, we might find a taller line that compensates for reduced width

This is a **greedy approach** - at each step, we make the locally optimal choice (moving the shorter line) that leads to the globally optimal solution.

## Complexity Analysis Table

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Brute Force | O(n²) | O(1) | Check all pairs |
| Two Pointers | O(n) | O(1) | Greedy approach |

## Edge Cases to Consider

1. **Empty array**: Return 0
2. **Single element**: Return 0 (can't form container)
3. **Two elements**: Calculate directly
4. **Equal heights**: Area = width * height
5. **Very large heights**: Use i64 for calculations
6. **Maximum width**: height[0] = 1, height[n-1] = 1

## Related Problems

### LeetCode 42: Trapping Rain Water
Find how much water can be trapped.

### LeetCode 407: Trapping Rain Water II
2D version with elevation map.

### LeetCode 11: Container With Most Water
Same as this problem.

## Exercises

### Exercise 1: Basic Max Area
Implement two pointers solution.

### Exercise 2: Max Area with Height Array
Return the actual height pair that forms max area.

### Exercise 3: Trapping Rain Water
Calculate trapped water units.

### Exercise 4: Max Area in Histogram
Find max rectangle in histogram.

### Exercise 5: Minimum Water to Drink
Calculate minimum water to drink while moving.

## Key Takeaways

1. **Area = width × min(height[left], height[right])**
2. **Two pointers** gives O(n) solution
3. **Move shorter line** to potentially find taller one
4. **Start widest** (maximum possible width)
5. **Greedy works** because width reduction is offset by potentially taller line

## Real-World Applications

1. **Water reservoir design**: Maximize water storage
2. **Urban planning**: Building placement optimization
3. **Agriculture**: Irrigation channel design
4. **Container design**: Maximize container volume
5. **Portfolio optimization**: Risk-reward balance
