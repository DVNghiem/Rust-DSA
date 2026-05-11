# Remove Nth From End (LeetCode #19)

## Problem Statement

Given the head of a linked list and an integer n, remove the nth node from the end of the list and return its head.

## Examples

```
Input: head = [1, 2, 3, 4, 5], n = 2
Output: [1, 2, 3, 5]

Input: head = [1], n = 1
Output: []

Input: head = [1, 2], n = 1
Output: [1]
```

## Two Pointer Approach

### Key Insight

Use two pointers with a gap of n nodes:
1. Move fast pointer n steps ahead
2. Move both pointers until fast reaches the end
3. The slow pointer is now at the node before the one to remove

### Visual Walkthrough

```
List: 1 -> 2 -> 3 -> 4 -> 5, n = 2

Step 1: Move fast n=2 steps ahead
  slow = 1 -> 2 -> 3 -> 4 -> 5
  fast = 3 -> 4 -> 5

Step 2: Move both until fast reaches end
  slow = 1 -> 2 -> 3 -> 4 -> 5
  fast = 5 -> None

Step 3: slow.next is the node to remove
  slow.next = slow.next.next
  Remove node with value 4

Result: 1 -> 2 -> 3 -> 5
```

## Implementation

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

## Complexity Analysis Table

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Two Pass | O(n) | O(1) | Count first, then remove |
| Two Pointers | O(n) | O(1) | Single pass |

## Edge Cases to Consider

1. **Remove first node**: When n equals list length
2. **Remove last node**: When n = 1
3. **Single node**: When list has only one node
4. **Two nodes**: When list has two nodes

## Key Takeaways

1. **Dummy head** simplifies edge cases
2. **Gap of n** between slow and fast pointers
3. **When fast reaches end**, slow is at previous node
4. **Single pass** achieves O(n) time
5. **Handle n equals length** correctly

## Real-World Applications

1. **Queue operations**: Remove kth element from end
2. **Text editors**: Undo functionality
3. **Browser history**: Remove last visited
4. **Music playlist**: Remove last played
5. **Task management**: Remove completed tasks
