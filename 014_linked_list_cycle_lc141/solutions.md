# Solutions: Linked List Cycle (LeetCode #141)

This document provides detailed, line-by-line analysis of each exercise implementation.

---

## Exercise 1: Has Cycle - Floyd's Algorithm

### The Solution

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

### Line-by-Line Analysis

```rust
let mut slow = &head;
let mut fast = &head;
```
**Purpose:** Initialize both pointers to the head. Slow and fast will move at different speeds to detect cycles.

```rust
while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
```
**Purpose:** Continue while fast can advance. We need fast and fast.next to both be Some for the 2-step advance.

```rust
slow = &slow.as_ref().unwrap().next;
```
**Purpose:** Slow moves 1 step (dereference next).

```rust
fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
```
**Purpose:** Fast moves 2 steps (dereference next twice).

```rust
if slow == fast {
    return true;
}
```
**Purpose:** If pointers meet, fast has lapped slow - cycle detected!

```rust
false
```
**Purpose:** If loop exits, fast reached end - no cycle.

### Why O(n) Time?

In a cycle of length `c`:
- Slow moves `t` steps, visits `t` nodes
- Fast moves `2t` steps, visits `2t` nodes
- They meet when `2t ≡ t (mod c)` → `t ≡ 0 (mod c)` → `t = c`

So they meet after O(c) = O(n) steps.

Without cycle, fast visits O(n) nodes and exits.

---

## Exercise 2: Detect Cycle Start

### The Solution

```rust
pub fn detect_cycle_start(head: Option<Box<ListNode>>) -> Option<usize> {
    let mut slow = &head;
    let mut fast = &head;

    // Phase 1: Find meeting point
    while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
        slow = &slow.as_ref().unwrap().next;
        fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        if slow == fast {
            break;
        }
    }

    if slow != fast {
        return None; // No cycle
    }

    // Phase 2: Find cycle start
    let mut start = 0;
    slow = &head;

    while slow != fast {
        slow = &slow.as_ref().unwrap().next;
        fast = &fast.as_ref().unwrap().next;
        start += 1;
    }
    Some(start)
}
```

### Mathematical Proof

Let:
- `L` = distance from head to cycle start
- `C` = cycle length
- When slow and fast meet:
  - slow has moved `L + a` steps
  - fast has moved `L + a + b` steps (a full cycle + a extra)
  - fast moved `2(L + a)` steps
  - So: `2(L + a) = L + a + b` → `L + a = b`

This means: the distance from head to cycle start (L) equals the remaining distance from meeting point to cycle start (b). So moving both pointers at same speed will meet at cycle start.

---

## Exercise 4: Happy Number

### The Solution

```rust
pub fn is_happy(n: i32) -> bool {
    let mut slow = n;
    let mut fast = n;

    fn next_square_sum(x: i32) -> i32 {
        let mut sum = 0;
        let mut x = x;
        while x > 0 {
            let digit = x % 10;
            sum += digit * digit;
            x /= 10;
        }
        sum
    }

    loop {
        slow = next_square_sum(slow);
        fast = next_square_sum(next_square_sum(fast));

        if slow == fast {
            break;
        }
    }
    slow == 1
}
```

### Key Insight

The happy number process either:
1. Reaches 1 (happy!)
2. Enters a cycle that doesn't include 1 (unhappy)

By applying Floyd's cycle detection to the sequence of square-sum values, we can detect if there's a cycle.

---

## Summary Table

| Exercise | Time | Space | Key Technique |
|----------|------|-------|---------------|
| 1: Has Cycle | O(n) | O(1) | Floyd's algorithm |
| 2: Cycle Start | O(n) | O(1) | Two phases |
| 3: Cycle Length | O(n) | O(1) | Count after meeting |
| 4: Happy Number | O(log n) | O(1) | Cycle detection |
| 5: HashSet | O(n) | O(n) | Track visited |
| 6: Remove Cycle | O(n) | O(1) | Connect to None |
| 7: Is Circular | O(n) | O(1) | All nodes in cycle |

## Key Takeaways

1. **Floyd's algorithm** uses two pointers at different speeds
2. **If they meet, there's a cycle**
3. **Same speed pointers after meeting find cycle start**
4. **Happy number** uses cycle detection on digit sums
5. **O(1) space** makes Floyd's optimal
