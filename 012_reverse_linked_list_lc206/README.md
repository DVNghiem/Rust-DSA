# Reverse Linked List (LeetCode #206)

## Problem Statement

Given the head of a singly linked list, reverse the list and return the reversed list.

## Examples

```
Input: head = [1, 2, 3, 4, 5]
Output: [5, 4, 3, 2, 1]

Input: head = [1, 2]
Output: [2, 1]
```

## Linked List Structure

```rust
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
```

## Approaches Overview

### Approach 1: Iterative O(n)
Use three pointers: prev, current, next.

```rust
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut current = head;

    while let Some(mut node) = current {
        let next = node.next;
        node.next = prev;
        prev = Some(node);
        current = next;
    }
    prev
}
```

### Approach 2: Recursive O(n)
Recurse to the end, then reverse links on the way back.

```rust
pub fn reverse_list_recursive(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() || head.as_ref().unwrap().next.is_none() {
        return head;
    }

    let mut new_head = reverse_list_recursive(head.as_ref().unwrap().next.clone());
    let head_node = new_head.as_mut().unwrap();
    let mut tail = head_node;
    while tail.next.is_some() {
        tail = tail.next.as_mut().unwrap();
    }
    tail.next = head;
    head.unwrap().next = None;
    new_head
}
```

## Visual Walkthrough: Iterative Approach

```
Original: 1 -> 2 -> 3 -> 4 -> 5 -> None

Step 1:
  prev = None
  current = 1 -> 2 -> 3 -> 4 -> 5 -> None
  After: 1 -> None (prev), current = 2 -> 3 -> 4 -> 5 -> None

Step 2:
  prev = 1 -> None
  current = 2 -> 3 -> 4 -> 5 -> None
  After: 2 -> 1 -> None (prev), current = 3 -> 4 -> 5 -> None

Step 3:
  prev = 2 -> 1 -> None
  current = 3 -> 4 -> 5 -> None
  After: 3 -> 2 -> 1 -> None (prev), current = 4 -> 5 -> None

Step 4:
  prev = 3 -> 2 -> 1 -> None
  current = 4 -> 5 -> None
  After: 4 -> 3 -> 2 -> 1 -> None (prev), current = 5 -> None

Step 5:
  prev = 4 -> 3 -> 2 -> 1 -> None
  current = 5 -> None
  After: 5 -> 4 -> 3 -> 2 -> 1 -> None (prev), current = None

Final: 5 -> 4 -> 3 -> 2 -> 1 -> None
```

## Complexity Analysis Table

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Iterative | O(n) | O(1) | In-place |
| Recursive | O(n) | O(n) | Call stack |

## Edge Cases to Consider

1. **Empty list**: Return None
2. **Single node**: Return that node
3. **Two nodes**: Reverse correctly
4. **Circular list**: Not applicable for this problem

## Related Problems

### LeetCode 92: Reverse Linked List II
Reverse a portion of the list.

### LeetCode 234: Palindrome Linked List
Check if linked list is palindrome.

### LeetCode 143: Reorder List
Reorder list in specific pattern.

## Exercises

### Exercise 1: Basic Reverse List
Implement iterative solution.

### Exercise 2: Recursive Reverse
Implement recursive solution.

### Exercise 3: Reverse in Groups
Reverse linked list in groups of k.

### Exercise 4: Reverse Between
Reverse nodes between left and right.

### Exercise 5: Check Palindrome
Check if list is palindrome.

## Key Takeaways

1. **Three pointers** track prev, current, next
2. **Link reversal** is the core operation
3. **Iterative is O(1) space** - preferred
4. **Recursive uses call stack** - O(n) space
5. **Handle None gracefully** - empty or single node

## Real-World Applications

1. **Undo functionality**: Reverse operations
2. **Browser history**: Back button traversal
3. **Text editing**: Undo/redo operations
4. **Music playlists**: Shuffle play
5. **Task scheduling**: Reverse execution order
