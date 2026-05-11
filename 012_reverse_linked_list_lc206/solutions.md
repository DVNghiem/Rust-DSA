# Solutions: Reverse Linked List (LeetCode #206)

This document provides detailed, line-by-line analysis of each exercise implementation.

---

## Exercise 1: Reverse Linked List - Iterative

### The Solution

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

### Line-by-Line Analysis

```rust
let mut prev = None;
let mut current = head;
```
**Purpose:** Initialize two pointers:
- `prev`: The previously processed node (starts as None, becomes the new head)
- `current`: The node currently being processed

```rust
while let Some(mut node) = current {
```
**Purpose:** Loop while current is not None. `Some(mut node)` takes ownership of the current node and allows mutation.

```rust
let next = node.next;
```
**Purpose:** Save the next pointer before we overwrite it. This is critical because we need to continue to the next node after reversing the link.

```rust
node.next = prev;
```
**Purpose:** Reverse the link. The current node now points to the previous node instead of the next node.

```rust
prev = Some(node);
```
**Purpose:** Move prev forward to the current node (which we've now reversed).

```rust
current = next;
```
**Purpose:** Move current forward to the saved next node.

```rust
prev
```
**Purpose:** When current becomes None, prev is the new head of the reversed list.

### Complexity Analysis

| Metric | Value | Explanation |
|--------|-------|-------------|
| **Time** | O(n) | Traverse each node once |
| **Space** | O(1) | Only three pointers used |

---

## Exercise 5: Palindrome Check

### The Solution

```rust
pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    if head.is_none() {
        return true;
    }

    // Find middle using slow/fast pointers
    let mut slow = &head;
    let mut fast = &head;
    while fast.as_ref().unwrap().next.is_some() {
        slow = &slow.as_ref().unwrap().next;
        match fast.as_ref().unwrap().next.as_ref() {
            Some(next) => fast = &next.next,
            None => break,
        }
    }

    // Reverse second half
    let mut prev = None;
    let mut current = slow.clone();
    while let Some(mut node) = current {
        let next = node.next;
        node.next = prev;
        prev = Some(node);
        current = next;
    }

    // Compare first half with reversed second half
    let mut left = &head;
    let mut right = &prev;
    while right.is_some() {
        if left.as_ref().unwrap().val != right.as_ref().unwrap().val {
            return false;
        }
        left = &left.as_ref().unwrap().next;
        right = &right.as_ref().unwrap().next;
    }
    true
}
```

### Key Insight

1. Find middle using slow/fast pointers
2. Reverse the second half
3. Compare first half with reversed second half
4. If all values match, it's a palindrome

---

## Exercise 7: Add Two Numbers

### The Solution

```rust
pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    let mut current = &mut dummy;
    let mut carry = 0;

    let mut n1 = l1;
    let mut n2 = l2;

    while n1.is_some() || n2.is_some() || carry > 0 {
        let val1 = n1.as_ref().map_or(0, |n| n.val);
        let val2 = n2.as_ref().map_or(0, |n| n.val);

        let sum = val1 + val2 + carry;
        carry = sum / 10;
        let digit = sum % 10;

        current.next = Some(Box::new(ListNode::new(digit)));
        current = current.next.as_mut().unwrap();

        n1 = n1.and_then(|n| n.next);
        n2 = n2.and_then(|n| n.next);
    }

    dummy.next
}
```

### Key Insight

- Use a dummy head to simplify the code
- Process each digit with carry
- Handle different length lists with `map_or(0, ...)`
- Continue while there are digits or carry remaining

---

## Summary Table

| Exercise | Time | Space | Key Technique |
|----------|------|-------|---------------|
| 1: Iterative | O(n) | O(1) | Three pointers |
| 2: Recursive | O(n) | O(n) | Call stack |
| 3: K Group | O(n) | O(1) | Reverse + skip |
| 4: Between | O(n) | O(1) | Boundary tracking |
| 5: Palindrome | O(n) | O(1) | Reverse second half |
| 6: Even Pos | O(n) | O(1) | Track positions |
| 7: Add Numbers | O(n) | O(n) | Digit + carry |
| 8: Rotate | O(n) | O(1) | Circular linking |

## Key Takeaways

1. **Three pointers** (prev, current, next) is the pattern for iterative reversal
2. **Link reversal** is `node.next = prev`
3. **Dummy head** simplifies adding new nodes
4. **Slow/fast pointers** find the middle
5. **Recursive reversal** uses the call stack as implicit storage
