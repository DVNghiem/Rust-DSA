# Solutions: Container With Most Water (LeetCode #11)

This document provides detailed, line-by-line analysis of each exercise implementation.

---

## Exercise 1: Max Area - Two Pointers

### The Solution

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

### Line-by-Line Analysis

```rust
let (mut left, mut right) = (0, height.len() - 1);
```
**Purpose:** Initialize two pointers at opposite ends. Starting with maximum width gives us the best chance of finding a large area early.

```rust
while left < right {
```
**Purpose:** Continue while pointers haven't crossed.

```rust
let width = (right - left) as i32;
```
**Purpose:** Calculate width. Subtracting usize requires cast to i32 for the multiplication.

```rust
let h = height[left].min(height[right]);
```
**Purpose:** Water height is limited by the shorter line. The container overflows at the shorter height.

```rust
max_area = max_area.max(width * h);
```
**Purpose:** Update maximum area if current area is larger.

```rust
if height[left] < height[right] {
    left += 1;
} else {
    right -= 1;
}
```
**Purpose:** Move the pointer with the shorter height inward. This is the key insight - we might find a taller line that compensates for reduced width.

### Why Move the Shorter Line?

If height[left] < height[right]:
- If we move right inward, width decreases and height is still limited by height[left]
- If we move left inward, width decreases but we might find a taller line

We move the shorter line because that's the limiting factor. Finding a taller line at the left side might increase the limiting height.

### Complexity Analysis

| Metric | Value | Explanation |
|--------|-------|-------------|
| **Time** | O(n) | Each pointer moves at most n times |
| **Space** | O(1) | Only a few variables |

---

## Exercise 4: Trapping Rain Water

### The Solution (Two Pointer Approach)

```rust
pub fn trap(height: &[i32]) -> i32 {
    if height.is_empty() {
        return 0;
    }

    let (mut left, mut right) = (0, height.len() - 1);
    let (mut left_max, mut right_max) = (0, 0);
    let mut water = 0;

    while left < right {
        if height[left] < height[right] {
            if height[left] >= left_max {
                left_max = height[left];
            } else {
                water += left_max - height[left];
            }
            left += 1;
        } else {
            if height[right] >= right_max {
                right_max = height[right];
            } else {
                water += right_max - height[right];
            }
            right -= 1;
        }
    }
    water
}
```

### Key Insight

For each position, water trapped = min(left_max, right_max) - current_height.

We track the maximum height seen from each side. If current height is less than the max, water can be trapped.

### Why It Works

At each position, we know the highest bar on the left and right. The water level is capped by the shorter of the two maxes. If current bar is shorter than that, water fills the gap.

---

## Exercise 5: Largest Rectangle in Histogram

### The Solution (Stack-Based)

```rust
pub fn largest_rectangle_area(heights: &[i32]) -> i32 {
    let mut stack: Vec<usize> = Vec::new();
    let mut max_area = 0;
    let mut i = 0;

    while i < heights.len() {
        if stack.is_empty() || heights[i] >= heights[*stack.last().unwrap()] {
            stack.push(i);
            i += 1;
        } else {
            let top = stack.pop().unwrap();
            let width = if stack.is_empty() {
                i as i32
            } else {
                (i - *stack.last().unwrap() - 1) as i32
            };
            max_area = max_area.max(heights[top] * width);
        }
    }

    while let Some(top) = stack.pop() {
        let width = if stack.is_empty() {
            i as i32
        } else {
            (i - *stack.last().unwrap() - 1) as i32
        };
        max_area = max_area.max(heights[top] * width);
    }

    max_area
}
```

### Key Insight

Using a monotonic stack (increasing heights), we can efficiently compute the largest rectangle.

When we encounter a shorter bar, the bar at the stack top has found its right boundary. We can now calculate the area with that bar as the shortest.

---

## Summary Table

| Exercise | Time | Space | Key Technique |
|----------|------|-------|---------------|
| 1: Max Area | O(n) | O(1) | Two pointers |
| 2: Brute Force | O(n²) | O(1) | All pairs |
| 3: With Indices | O(n) | O(1) | Track indices |
| 4: Trap Water | O(n) | O(1) | Two pointers |
| 5: Rectangle | O(n) | O(n) | Monotonic stack |
| 6: Min Water | O(n) | O(1) | Greedy |
| 7: Limited Moves | O(n*k) | O(1) | Bounded moves |
| 8: 2D | O(n) | O(1) | Widths |

## Key Takeaways

1. **Two pointers** reduces O(n²) to O(n)
2. **Move shorter line** to find potentially taller one
3. **Start widest** for maximum potential width
4. **Trap water** uses left_max and right_max tracking
5. **Monotonic stack** solves rectangle problems efficiently
