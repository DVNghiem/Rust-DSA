# Solutions: Merge Two Sorted Lists (LeetCode #21)

This document provides detailed, line-by-line analysis of each exercise implementation.

---

## Exercise 1: Merge Two Sorted Lists - Iterative

### The Solution

```rust
pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    let mut current = &mut dummy;

    let mut l1 = l1;
    let mut l2 = l2;

    while l1.is_some() && l2.is_some() {
        if l1.as_ref().unwrap().val <= l2.as_ref().unwrap().val {
            current.next = l1;
            l1 = l1.and_then(|n| n.next);
        } else {
            current.next = l2;
            l2 = l2.and_then(|n| n.next);
        }
        current = current.next.as_mut().unwrap();
    }

    if l1.is_some() {
        current.next = l1;
    } else {
        current.next = l2;
    }

    dummy.next
}
```

### Line-by-Line Analysis

```rust
let mut dummy = Box::new(ListNode::new(0));
```
**Purpose:** Create a dummy head node with value 0. This simplifies handling the head pointer - we don't need special cases for the first node.

```rust
let mut current = &mut dummy;
```
**Purpose:** Track our current position in the merged list. `current.next` is where we'll attach the next node.

```rust
while l1.is_some() && l2.is_some() {
```
**Purpose:** Continue while both lists have nodes. When one list is exhausted, we can directly attach the remaining list.

```rust
if l1.as_ref().unwrap().val <= l2.as_ref().unwrap().val {
    current.next = l1;
    l1 = l1.and_then(|n| n.next);
} else {
    current.next = l2;
    l2 = l2.and_then(|n| n.next);
}
```
**Purpose:** Compare heads and attach the smaller one. `and_then` takes the current node and returns its next node, effectively advancing the list.

```rust
current = current.next.as_mut().unwrap();
```
**Purpose:** Move current forward to the node we just attached.

```rust
if l1.is_some() {
    current.next = l1;
} else {
    current.next = l2;
}
```
**Purpose:** One list may still have nodes remaining. Attach whatever remains to the end of our merged list.

### Complexity Analysis

| Metric | Value | Explanation |
|--------|-------|-------------|
| **Time** | O(n + m) | Each node visited once |
| **Space** | O(1) | Only pointers used, dummy is just one node |

---

## Exercise 3: Merge K Sorted Lists

### The Solution (Using BinaryHeap)

```rust
use std::collections::BinaryHeap;

pub fn merge_k_lists(lists: &[Option<Box<ListNode>>]) -> Option<Box<ListNode>> {
    let mut heap = BinaryHeap::new();

    // Insert head of each list
    for list in lists.iter() {
        if let Some(node) = list {
            heap.push((node.val, node));
        }
    }

    let mut dummy = Box::new(ListNode::new(0));
    let mut current = &mut dummy;

    while let Some((_, node)) = heap.pop() {
        current.next = Some(node.clone());
        current = current.next.as_mut().unwrap();

        if let Some(next) = &node.next {
            heap.push((next.val, next));
        }
    }

    dummy.next
}
```

### Key Insight

Using a min-heap (BinaryHeap in Rust), we can efficiently find the smallest head among all k lists in O(log k) time. This gives O(n log k) total time.

---

## Summary Table

| Exercise | Time | Space | Key Technique |
|----------|------|-------|---------------|
| 1: Iterative | O(n+m) | O(1) | Dummy head |
| 2: Recursive | O(n+m) | O(n+m) | Recursion |
| 3: K Lists | O(n log k) | O(k) | Min-heap |
| 4: In Place | O(n+m) | O(1) | Modify pointers |
| 5: Arrays | O(n+m) | O(n+m) | Two pointers |
| 6: Middle | O(n+m) | O(1) | Slow/fast |
| 7: Limited | O(k) | O(1) | Counter |

## Key Takeaways

1. **Dummy head** simplifies head pointer handling
2. **Compare heads** to choose the next node
3. **Attach remaining** when one list exhausted
4. **BinaryHeap** for efficient k-way merge
5. **Time O(n + m)** is optimal for merging two lists
