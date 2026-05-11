# Merge K Sorted Lists - LeetCode 23

## Problem Statement

You are given an array of k linked-lists, each linked-list is sorted in ascending order.

Merge all the linked-lists into one sorted linked-list and return it.

## Visual Walkthrough

```
Example 1:
Input: lists = [[1,4,5],[1,3,4],[2,6]]
Output: [1,1,2,3,4,4,5,6]

List 1: 1 → 4 → 5
List 2: 1 → 3 → 4
List 3: 2 → 6

Merged: 1 → 1 → 2 → 3 → 4 → 4 → 5 → 6

Example 2:
Input: lists = []
Output: []

Example 3:
Input: lists = [[]]
Output: []
```

### Merge Process Visualization

```
List 1: 1 → 4 → 5
List 2: 1 → 3 → 4
List 3: 2 → 6

Step 1: Pick smallest head among all lists
- List 1: 1
- List 2: 1 ← tie, pick any
- List 3: 2
Pick 1 (from List 1 or 2)

Step 2:
- List 1: 4 → 5 (now at 4)
- List 2: 1 ← pick this one
- List 3: 2
Pick 1 (from List 2)

Step 3:
- List 1: 4 → 5
- List 2: empty
- List 3: 2
Pick 2 (from List 3)

Continue until all merged...
Result: 1 → 1 → 2 → 3 → 4 → 4 → 5 → 6
```

### Min-Heap Approach

```
At each step, we need the smallest element among k lists.
This is exactly what a min-heap does efficiently!

Heap stores (value, list_index, node)
- value: the node's value
- list_index: which list this node came from (for reference)
- node: the actual ListNode pointer

Operations:
1. Push head of each list into heap: O(k)
2. Pop minimum: O(log k)
3. Push next from same list: O(log k)
4. Repeat until heap empty

Total: O(n log k) where n = total nodes, k = number of lists
```

## Approach Comparison

| Approach | Time | Space | Description |
|----------|------|-------|-------------|
| Merge pairwise | O(kN) | O(1) | Merge list 1&2, result&3, etc |
| Divide & conquer | O(N log k) | O(log k) | Merge pairs recursively |
| Min-Heap | O(N log k) | O(k) | Always pick smallest head |
| Naive concatenation | O(N log k) | O(1) | Collect all, sort |

### Why Min-Heap?

- At any point, we need to know which list has the smallest head
- Heap gives O(log k) insertion and extraction
- Efficiently handles dynamic "which is smallest" queries

## Implementation Strategy

### Approach 1: Min-Heap with Dummy Node

```rust
pub fn mergeKLists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut heap: BinaryHeap<(i32, usize)> = BinaryHeap::new();
    let dummy = Box::new(ListNode::new(0));
    let mut tail = &mut dummy.as_mut();

    // Push head of each non-empty list
    for (i, node) in lists.iter().enumerate() {
        if let Some(n) = node {
            heap.push((n.val, i));
        }
    }

    while let Some((val, i)) = heap.pop() {
        // Find and remove the next node from list i
        // This is tricky - we'd need to track position
    }
}
```

### Approach 2: Simplified Min-Heap (Recommended)

```rust
use std::collections::BinaryHeap;

pub fn mergeKLists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut heap: BinaryHeap<(i32, Box<ListNode>)> = BinaryHeap::new();

    // Push head of each list
    for list in lists {
        if let Some(node) = list {
            heap.push((node.val, node));
        }
    }

    let mut dummy = Box::new(ListNode::new(0));
    let mut tail = &mut dummy;

    while let Some((_, mut node)) = heap.pop() {
        // Take the node out
        let next = node.next.take();
        tail.next = Some(node);
        tail = tail.next.as_mut().unwrap();

        // Push next from same list
        if let Some(n) = next {
            heap.push((n.val, n));
        }
    }

    dummy.next
}
```

## Edge Cases

1. **Empty lists array**: Return empty list
2. **All empty lists**: Return empty list
3. **Single list**: Return that list
4. **Single element in multiple lists**: Handle properly
5. **Lists of different lengths**: Handle naturally
6. **Duplicate values**: Heap handles ties

## Test Cases

1. Basic merge of 3 lists
2. Empty input
3. Single list
4. All empty lists
5. Lists with different lengths
6. Single element per list
7. All same values
8. Already sorted (one per list)

## Solution Explanation

### Key Data Structure: Min-Heap

A min-heap (BinaryHeap with reversed ordering) efficiently finds the minimum element among k sorted sequences.

### Algorithm Steps

1. **Initialize**: Push head node of each non-empty list into heap
2. **Extract**: Pop node with smallest value from heap
3. **Append**: Add popped node to result list
4. **Advance**: Push next node from same list (if exists)
5. **Repeat**: Until heap is empty

### Why It Works

- Each list is sorted, so the smallest unprocessed element of each list is at its head
- Heap tracks minimum among all list heads
- After extracting min, advance that list by one step
- Repeat maintains sorted order in result

## Complexity Analysis

- **Time**: O(N log k) where N = total nodes, k = number of lists
  - Each node pushed and popped once: O(log k) per node
- **Space**: O(k) for heap (at most k nodes at any time)

## Follow-up Questions

1. How does this compare to merge sort?
2. Can you solve without a heap?
3. What if you need to merge 2 sorted lists repeatedly?