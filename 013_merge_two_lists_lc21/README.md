# Merge Two Sorted Lists (LeetCode #21)

## Problem Statement

You are given the heads of two sorted linked lists `list1` and `list2`.

Merge the two lists into one sorted list. The list should be made by splicing together the nodes of the first two lists.

Return the head of the merged linked list.

## Examples

```
Input: list1 = [1, 2, 4], list2 = [1, 3, 4]
Output: [1, 1, 2, 3, 4, 4]

Input: list1 = [], list2 = []
Output: []
```

## Approaches Overview

### Approach 1: Iterative - Dummy Head
Use a dummy head to simplify edge cases.

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

### Approach 2: Recursive
Recursively choose the smaller head.

```rust
pub fn merge_two_lists_recursive(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (None, None) => None,
        (Some(node), None) => Some(node),
        (None, Some(node)) => Some(node),
        (Some(n1), Some(n2)) => {
            if n1.val <= n2.val {
                Some(Box::new(ListNode {
                    val: n1.val,
                    next: merge_two_lists_recursive(n1.next, Some(n2)),
                }))
            } else {
                Some(Box::new(ListNode {
                    val: n2.val,
                    next: merge_two_lists_recursive(Some(n1), n2.next),
                }))
            }
        }
    }
}
```

## Visual Walkthrough

```
l1 = [1, 3, 5], l2 = [2, 4, 6]

dummy -> [dummy.val=0]
Step 1: 1 <= 2, take from l1
  dummy.next = l1 (1), current = 1
  l1 = [3, 5]

Step 2: 3 > 2, take from l2
  current.next = l2 (2), current = 2
  l2 = [4, 6]

Step 3: 3 <= 4, take from l1
  current.next = l1 (3), current = 3
  l1 = [5]

Step 4: 5 > 4, take from l2
  current.next = l2 (4), current = 4
  l2 = [6]

Step 5: 5 <= 6, take from l1
  current.next = l1 (5), current = 5
  l1 = None

Step 6: l1 is None, attach remaining l2
  current.next = l2 (6)

Result: [1, 2, 3, 4, 5, 6]
```

## Complexity Analysis Table

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Iterative | O(n + m) | O(1) | No extra allocation |
| Recursive | O(n + m) | O(n + m) | Call stack |

## Key Takeaways

1. **Dummy head** simplifies handling of head pointer
2. **Pick smaller head** each iteration
3. **Attach remaining** list when one is exhausted
4. **Time O(n + m)** since each node visited once
5. **Space O(1)** for iterative solution

## Real-World Applications

1. **Database merging**: Merge sorted query results
2. **File merging**: Combine sorted log files
3. **Queue merging**: Merge priority queues
4. **Interview scheduling**: Merge interview slots
5. **Version control**: Merge sorted commit histories
