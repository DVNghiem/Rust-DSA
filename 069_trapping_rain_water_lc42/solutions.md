# Trapping Rain Water Solution - LeetCode 42 (Complete)

## Solution Analysis

### Two-Pointer Approach

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
            if left_max > height[left] {
                water += left_max - height[left];
            }
        } else {
            right -= 1;
            right_max = right_max.max(height[right]);
            if right_max > height[right] {
                water += right_max - height[right];
            }
        }
    }

    water
}
```

## Key Insight

### Why Two Pointers Work

For any position `i`:
- Water at `i` = min(max_left, max_right) - height[i]
- max_left = highest bar to the left of i
- max_right = highest bar to the right of i

**Key observation**: The amount of water is limited by the LOWER of the two max heights.

### Why Move the Shorter Side?

If `left_max < right_max`:
- The water at position `left` is determined by `left_max` (not `right_max`)
- No matter how we move `right`, we can't increase `left_max`
- So we move `left` inward and possibly find a taller bar

## Line-by-Line Analysis

### Initialize
```rust
let mut left = 0;
let mut right = n - 1;
let mut left_max = height[left];
let mut right_max = height[right];
let mut water = 0;
```
- Start from both ends
- Track maximum heights seen so far

### While Loop
```rust
while left < right {
    if left_max < right_max {
        left += 1;
        left_max = left_max.max(height[left]);
        if left_max > height[left] {
            water += left_max - height[left];
        }
    } else {
        right -= 1;
        right_max = right_max.max(height[right]);
        if right_max > height[right] {
            water += right_max - height[right];
        }
    }
}
```

When `left_max < right_max`:
1. Move `left` pointer right
2. Update `left_max` if new bar is taller
3. If new bar is shorter than `left_max`, water can be trapped: `left_max - height[left]`

Same logic for right side when `right_max <= left_max`.

## Visual Example

### Input: height = [0,1,0,2,1,0,1,3,2,1,2,1]

```
Bar heights:
    |
    |     |
  | |   | |
  | | | | |
  | | | | | |
-------------

Step by step:
- left=0, right=11, left_max=0, right_max=1
- left_max < right_max, so move left
  - left=1, left_max=max(0,1)=1, water+=1-1=0
- left_max < right_max, so move left
  - left=2, left_max=max(1,0)=1, water+=1-0=1
- left_max < right_max, so move left
  - left=3, left_max=max(1,2)=2, water+=2-2=0
- left_max < right_max, so move left
  - left=4, left_max=max(2,1)=2, water+=2-1=1
- left_max < right_max, so move left
  - left=5, left_max=max(2,0)=2, water+=2-0=2
- left_max < right_max, so move left
  - left=6, left_max=max(2,1)=2, water+=2-1=3
- left_max < right_max, so move left
  - left=7, left_max=max(2,3)=3, water+=3-3=0
- left_max < right_max, so move left
  - left=8, left_max=max(3,2)=3, water+=3-2=4
- left_max < right_max, so move left
  - left=9, left_max=max(3,1)=3, water+=3-1=5
- left_max < right_max, so move left
  - left=10, left_max=max(3,2)=3, water+=3-2=6
- left >= right, done

Result: 6
```

## Why This Works

### Invariant Maintained

At each step, we maintain:
- `left_max` = maximum of all heights from 0 to left
- `right_max` = maximum of all heights from right to n-1

Since we always move the pointer with smaller max height:
- We're guaranteed to never miss a higher bar that could hold water
- When we add water, we're adding `left_max - height[left]` which is correct because `left_max` is the limiting factor (it's <= `right_max`)

## Complexity Analysis

| Approach | Time | Space |
|----------|------|-------|
| Two-pointer | O(n) | O(1) |
| DP | O(n) | O(n) |
| Stack | O(n) | O(n) |

## Edge Cases

### Too Few Bars
```rust
Input: [1, 2]
Output: 0 (can't trap water with less than 3 bars)
```

### No Water Trapped
```rust
Input: [1, 2, 3, 4, 5] (ascending)
Output: 0
```

### Valley Shape
```rust
Input: [2, 0, 2]
Output: 2
```

## Common Mistakes

1. **Using wrong comparison**: Should compare `left_max` and `right_max`, not heights
2. **Forgetting to update max**: Must use `max()` to track the highest bar seen
3. **Off-by-one errors**: Remember indices go from 0 to n-1