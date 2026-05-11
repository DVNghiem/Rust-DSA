# Linked List Cycle (LeetCode #141)

## Problem Statement

Given a linked list, determine if it has a cycle in it.

There is a cycle in a linked list if there is some node in the list that can be reached again by continuously following the `next` pointer.

## Examples

```
Input: head = [3, 2, 0, -4], tail connects to node index 1
Output: true
Explanation: There is a cycle where tail connects to index 1.

Input: head = [1, 2], tail connects to node index 0
Output: true
Explanation: There is a cycle where tail connects to index 0.

Input: head = [1], tail connects to null
Output: false
```

## Floyd's Cycle Detection Algorithm

### The Key Insight

Use two pointers:
- **Slow pointer**: Moves 1 step at a time
- **Fast pointer**: Moves 2 steps at a time

If there's a cycle, the fast pointer will eventually "lap" the slow pointer - they'll point to the same node.

### Why It Works

In a cycle of length `c`:
- Slow moves `t` steps, Fast moves `2t` steps
- When they meet: `2t - t ≡ 0 (mod c)` → `t ≡ 0 (mod c)`
- They meet after fast has lapped slow

If there's no cycle, fast reaches the end.

## Visual Walkthrough

```
List with cycle: 3 -> 2 -> 0 -> -4 ----+
            ^--------------------------+
(cycle back to node at index 1)

Step 1: slow=3, fast=3
Step 2: slow=2, fast=0
Step 3: slow=0, fast=-4
Step 4: slow=-4, fast=2
Step 5: slow=2, fast=-4 (first meeting!)

Cycle detected!
```

## Implementation

```rust
pub fn has_cycle(head: Option<Box<ListNode>>) -> bool {
    let mut slow = &head;
    let mut fast = &head;

    while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
        slow = &slow.as_ref().unwrap().next;
        fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;

        if slow == fast {
            return true;
        }
    }
    false
}
```

## Complexity Analysis Table

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Floyd's Cycle Detection | O(n) | O(1) | Slow/fast pointers |
| HashSet | O(n) | O(n) | Track visited nodes |

## Edge Cases to Consider

1. **Empty list**: Return false
2. **Single node, no cycle**: Return false
3. **Single node, self-loop**: Return true
4. **Two nodes, no cycle**: Return false
5. **Two nodes, cycle**: Return true

## Floyd's Algorithm Variants

### Find Cycle Start

To find where the cycle starts:

```rust
pub fn detect_cycle(head: Option<Box<ListNode>>) -> Option<usize> {
    let mut slow = &head;
    let mut fast = &head;
    let mut start = 0;

    while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
        slow = &slow.as_ref().unwrap().next;
        fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        if slow == fast {
            // Found cycle, find start
            slow = &head;
            while slow != fast {
                slow = &slow.as_ref().unwrap().next;
                fast = &fast.as_ref().unwrap().next;
                start += 1;
            }
            return Some(start);
        }
    }
    None
}
```

## Related Problems

### LeetCode 142: Linked List Cycle II
Find the node where cycle begins.

### LeetCode 287: Find the Duplicate Number
Find duplicate using cycle detection (Pigeonhole principle).

### LeetCode 876: Middle of Linked List
Find middle using slow/fast pointers.

## Exercises

### Exercise 1: Basic Cycle Detection
Implement Floyd's algorithm.

### Exercise 2: Find Cycle Start
Return the index where cycle begins.

### Exercise 3: Cycle Length
Find the length of the cycle.

### Exercise 4: Happy Number
Check if number is happy using cycle detection.

### Exercise 5: Has Cycle with HashSet
Implement using HashSet.

## Key Takeaways

1. **Floyd's algorithm** uses O(1) space
2. **Slow moves 1x, fast moves 2x**
3. **If they meet, there's a cycle**
4. **Meeting point proves cycle exists**
5. **Same technique** finds cycle start

## Real-World Applications

1. **Operating systems**: Deadlock detection
2. **Network routing**: Cycle detection in routing tables
3. **Pseudo-random number generators**: Cycle detection
4. **Formula 1 racing**: Lap counting
5. **Memory management**: Circular buffer detection
