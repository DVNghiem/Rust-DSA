# Find Median from Data Stream - LeetCode 295

## Problem Overview

Design a data structure that supports adding numbers and finding the median in O(1) time.

**Examples:**
```
add_num(1), add_num(2) → median = 1.5
add_num(3) → median = 2.0
```

## Theory

### Two Heaps Approach

- **Max-heap (left)**: Lower half, largest element at top
- **Min-heap (right)**: Upper half, smallest element at top
- Balance: left.size() = right.size() or left.size() = right.size() + 1

```
Numbers: [1, 2, 3]

After 1:    Left: [1]    Right: []     → median = 1
After 2:    Left: [1]    Right: [2]    → median = 1.5
After 3:    Left: [1,2]  Right: [3]    → median = 2
```

## Implementation

```rust
use std::collections::{BinaryHeap, Ord};
use std::cmp::Reverse;

struct MedianFinder {
    lo: BinaryHeap<i32>,      // max-heap for lower half
    hi: BinaryHeap<Reverse<i32>>, // min-heap for upper half
}

impl MedianFinder {
    fn new() -> Self {
        MedianFinder {
            lo: BinaryHeap::new(),
            hi: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        self.lo.push(num);
        // Move max from lo to hi
        if let Some(max_lo) = self.lo.pop() {
            self.hi.push(Reverse(max_lo));
        }
        // Balance sizes
        if self.hi.len() > self.lo.len() {
            if let Some(min_hi) = self.hi.pop() {
                self.lo.push(min_hi.0);
            }
        }
    }

    fn find_median(&self) -> f64 {
        if self.lo.is_empty() { return 0.0; }
        if self.lo.len() > self.hi.len() {
            *self.lo.peek().unwrap() as f64
        } else {
            (*self.lo.peek().unwrap() as f64 + self.hi.peek().unwrap().0 as f64) / 2.0
        }
    }
}
```

## Test Cases

```rust
#[test]
fn test_median_basic() {
    let mut mf = MedianFinder::new();
    mf.add_num(1);
    mf.add_num(2);
    assert_eq!(mf.find_median(), 1.5);
}
```