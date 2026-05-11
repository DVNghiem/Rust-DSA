# Sliding Window Maximum Solution - LeetCode 239 (Complete)

## Solution Analysis

### Monotonic Deque

```rust
pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n = nums.len();
    if n == 0 || k == 0 { return vec![]; }

    let mut result = Vec::new();
    let mut deque: VecDeque<usize> = VecDeque::new();

    for i in 0..n {
        // Remove indices outside window
        while !deque.is_empty() && deque[0] <= i.saturating_sub(k) {
            deque.pop_front();
        }

        // Remove smaller elements from back (they can never be max)
        while !deque.is_empty() && nums[*deque.back().unwrap()] < nums[i] {
            deque.pop_back();
        }

        deque.push_back(i);

        // Add max to result when window is ready
        if i >= k - 1 {
            result.push(nums[*deque.front().unwrap()]);
        }
    }

    result
}
```

## Why Monotonic Deque?

### Key Properties

1. **Front always has max**: We maintain decreasing order, so front is largest
2. **Remove outdated**: When index < i - k + 1, it's outside current window
3. **Remove smaller**: If an element is smaller than current, it can never be max while current is in window

### Decreasing Order Invariant

The deque stores indices of elements in **decreasing order** of their values:
```
Deque indices: [i1, i2, i3, ...] where nums[i1] >= nums[i2] >= nums[i3] >= ...
```

## Line-by-Line Analysis

### Remove Outdated Indices
```rust
while !deque.is_empty() && deque[0] <= i.saturating_sub(k) {
    deque.pop_front();
}
```
- Keep indices where `index > i - k` (within window)
- Remove front if it's too old

### Remove Smaller Elements
```rust
while !deque.is_empty() && nums[*deque.back().unwrap()] < nums[i] {
    deque.pop_back();
}
```
- If current element is larger than back, back can never be max
- Pop until back >= current, maintaining decreasing order

### Add Current Index
```rust
deque.push_back(i);
```
- Add current index to back

### Record Max
```rust
if i >= k - 1 {
    result.push(nums[*deque.front().unwrap()]);
}
```
- Front is always the maximum in current window

## Visual Example

### Input: nums = [1, 3, -1, -3, 5, 3, 6, 7], k = 3

```
i=0, nums[0]=1:
  deque = []

i=1, nums[1]=3:
  Remove outdated: none
  Remove smaller: pop 0 (1 < 3)
  deque = []
  push 1
  deque = [1]

i=2, nums[2]=-1:
  Remove outdated: none
  Remove smaller: -1 < 3, keep
  push 2
  deque = [1, 2]
  i >= 2, result = [3]

i=3, nums[3]=-3:
  Remove outdated: none (3 <= 3-3=0? no)
  Remove smaller: -3 < 3, -3 < -1, pop all
  deque = []
  push 3
  deque = [3]
  i >= 2, result = [3, 3]

i=4, nums[4]=5:
  Remove outdated: deque[0]=3 <= 4-3=1? no
  Remove smaller: pop 3 (5 > -3)
  deque = []
  push 4
  deque = [4]
  i >= 2, result = [3, 3, 5]

i=5, nums[5]=3:
  Remove outdated: deque[0]=4 <= 5-3=2? no
  Remove smaller: 3 < 5, keep
  push 5
  deque = [4, 5]
  result = [3, 3, 5, 5]

i=6, nums[6]=6:
  Remove outdated: deque[0]=4 <= 6-3=3? no
  Remove smaller: pop 5 (6 > 3), pop 4 (6 > 5)
  deque = []
  push 6
  deque = [6]
  result = [3, 3, 5, 5, 6]

i=7, nums[7]=7:
  Remove outdated: deque[0]=6 <= 7-3=4? no
  Remove smaller: pop 6 (7 > 6)
  deque = []
  push 7
  deque = [7]
  result = [3, 3, 5, 5, 6, 7]

Final: [3, 3, 5, 5, 6, 7]
```

## Why Remove Smaller Elements?

Consider: nums = [1, 3, 2]

When i=2 (nums[2]=2):
- deque might have [1, 3] (indices for 1 and 3)
- 2 is less than 3, so we keep it
- deque becomes [1, 3, 2] but maintaining decreasing order...

Actually, let me trace more carefully:

```
i=0, 1: deque=[0]
i=1, 3: pop 0 (1<3), deque=[], push 1 → deque=[1]
i=2, 2: no pop (2<3 is true, so we pop back?)
```

Wait, the condition is `nums[deque.back()] < nums[i]`, so:
- nums[1] = 3 < nums[2] = 2? No, 3 < 2 is false
- So we don't pop, deque becomes [1, 2]

But then max is at front = nums[1] = 3 ✓

## Complexity Analysis

| Approach | Time | Space |
|----------|------|-------|
| Monotonic Deque | O(n) | O(k) |
| Brute Force | O(n × k) | O(1) |

Each element is pushed and popped at most once.

## Edge Cases

### Empty Input
```rust
Input: [], k = 3
Output: []
```

### k = 1
```rust
Input: [1, 2, 3], k = 1
Output: [1, 2, 3] (each element is its own window)
```

### k = n
```rust
Input: [1, 2, 3], k = 3
Output: [3] (only one window)
```

## Common Mistakes

1. **Not removing outdated indices**: Old indices would give wrong max
2. **Not removing smaller elements**: They would block newer, potentially larger elements
3. **Not checking window readiness**: Only add to result when i >= k-1