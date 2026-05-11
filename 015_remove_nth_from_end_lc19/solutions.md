# Solutions: Remove Nth From End (LeetCode #19)

This document provides detailed, line-by-line analysis of each exercise implementation.

---

## Exercise 1: Remove Nth From End - Two Pointers

### The Solution

```rust
pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    dummy.next = head;

    let mut fast = &dummy;
    let mut slow = &mut dummy;

    // Move fast n steps ahead
    for _ in 0..n {
        if let Some(node) = fast.next.as_ref() {
            fast = node;
        }
    }

    // Move both until fast reaches end
    while fast.next.is_some() {
        slow = slow.next.as_mut().unwrap();
        fast = fast.next.as_ref().unwrap();
    }

    // Remove the node
    slow.next = slow.next.as_mut().unwrap().next.clone();

    dummy.next
}
```

### Line-by-Line Analysis

```rust
let mut dummy = Box::new(ListNode::new(0));
dummy.next = head;
```
**Purpose:** Create a dummy head node before the actual head. This simplifies removing the first node - we don't need special handling.

```rust
let mut fast = &dummy;
let mut slow = &mut dummy;
```
**Purpose:** Initialize both pointers at dummy. We need `&mut` for slow because we'll modify it.

```rust
for _ in 0..n {
    if let Some(node) = fast.next.as_ref() {
        fast = node;
    }
}
```
**Purpose:** Move fast pointer n steps ahead. This creates the gap of n nodes between slow and fast.

```rust
while fast.next.is_some() {
    slow = slow.next.as_mut().unwrap();
    fast = fast.next.as_ref().unwrap();
}
```
**Purpose:** Move both pointers forward until fast reaches the end. When fast.next is None, fast is at the last node, and slow is at the node before the one to remove.

```rust
slow.next = slow.next.as_mut().unwrap().next.clone();
```
**Purpose:** Remove the target node by bypassing it - setting slow.next to skip one node.

### Complexity Analysis

| Metric | Value | Explanation |
|--------|-------|-------------|
| **Time** | O(n) | Single pass through list |
| **Space** | O(1) | Only pointers used |

### Why Dummy Head?

Without dummy, removing the first node requires special handling:
- With dummy: `slow` starts at dummy, `slow.next` is the first node
- Without dummy: We'd need to check if n equals length and handle head pointer separately

---

## Summary Table

| Exercise | Time | Space | Key Technique |
|----------|------|-------|---------------|
| 1: Remove Nth | O(n) | O(1) | Two pointers |
| 2: Two Pass | O(n) | O(1) | Count + remove |
| 3: Get Nth | O(n) | O(1) | Two pointers |
| 4: Every Kth | O(n) | O(1) | Counter |
| 5: Swap Nth | O(n) | O(1) | Pointer swap |
| 6: Remove Middle | O(n) | O(1) | Slow/fast |
| 7: Remove Last K | O(n) | O(1) | Two pointers |

## Key Takeaways

1. **Dummy head** simplifies head removal
2. **Gap of n** nodes between slow and fast
3. **When fast at end**, slow is at previous node
4. **Single pass** is optimal
5. **Clone when needed** to avoid ownership issues
