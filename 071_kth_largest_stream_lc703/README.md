# Kth Largest Element in a Stream - LeetCode 703

## Problem Overview

Design a class `KthLargest` that accepts integers and an integer `k`. It has an `add(val)` method that adds an integer and returns the kth largest element.

**Examples:**
```
KthLargest k=3, arr=[4,5,8,2]
add(3) → returns 4
add(5) → returns 5
add(10) → returns 5
add(9) → returns 8
```

## Theory

### Min-Heap of Size k

Maintain a min-heap with at most k elements:
- When heap size < k: push
- When heap size >= k and new val > min: replace
- Kth largest is always at heap root (minimum of top k)

## Implementation

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub struct KthLargest {
    k: usize,
    heap: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    pub fn new(k: i32, mut nums: Vec<i32>) -> Self {
        let k = k as usize;
        let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

        for num in nums.drain(..) {
            if heap.len() < k {
                heap.push(Reverse(num));
            } else if num > heap.peek().unwrap().0 {
                heap.push(Reverse(num));
                heap.pop();
            }
        }

        KthLargest { k, heap }
    }

    pub fn add(&mut self, val: i32) -> i32 {
        if self.heap.len() < self.k {
            self.heap.push(Reverse(val));
        } else if val > self.heap.peek().unwrap().0 {
            self.heap.push(Reverse(val));
            self.heap.pop();
        }
        self.heap.peek().unwrap().0
    }
}
```

## Test Cases

```rust
#[test]
fn test_kth_largest_basic() {
    let mut kth = KthLargest::new(3, vec![4, 5, 8, 2]);
    assert_eq!(kth.add(3), 4);
    assert_eq!(kth.add(5), 5);
}
```