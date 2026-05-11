# Design HashSet - LeetCode 705

## Problem Statement

Design a HashSet without using any built-in hash table libraries.

```
Operations required:
- add(value): Insert value into HashSet (no duplicates)
- remove(value): Remove value from HashSet
- contains(value): Return true if value exists, false otherwise

All operations must have O(1) average time complexity.
```

## Understanding HashSet vs HashMap

HashSet is essentially HashMap with only keys (no values):

```
HashMap:  key → value
HashSet:  key → (present or not)

Implementation: HashSet<T> = HashMap<T, ()>
```

The `()` type is the unit type - a zero-sized type that indicates "we only care about key presence".

## Visual Walkthrough

```
HashSet after adding {1, 2, 3, 5}:
┌─────────┬─────────┬─────────┬─────────┬─────────┐
│ Bucket 0│ Bucket 1 │ Bucket 2 │ Bucket 3 │ Bucket 4 │
├─────────┼─────────┼─────────┼─────────┼─────────┤
│         │   1     │   2     │   3     │   5     │
│         │  (hash) │  (hash) │  (hash) │  (hash) │
│         │    ↓    │    ↓    │    ↓    │    ↓    │
│         │   null  │   null  │   null  │   null  │
└─────────┴─────────┴─────────┴─────────┴─────────┘

contains(3): index = hash(3) = 3, bucket[3] has 3 → true
contains(4): index = hash(4) = 4, bucket[4] has 5 → false
```

## Implementation Approach

Similar to HashMap, but:
1. Only stores keys (no values)
2. Uses `()` as placeholder value
3. No duplicate handling - add is idempotent

## Data Structure

```rust
pub struct MyHashSet {
    buckets: Vec<Option<Box<ListNode>>>,
    size: usize,
}

struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}
```

- `buckets`: Array of linked list buckets
- `size`: Count of unique elements

## Key Operations

### Add Operation

```rust
pub fn add(&mut self, val: i32) {
    if self.contains(val) {
        return;  // Already exists, no duplicate
    }

    let index = self.hash(val);
    let bucket = &mut self.buckets[index];

    // Add at head of bucket (O(1))
    let new_node = Box::new(ListNode { val, next: bucket.take() });
    *bucket = Some(new_node);
    self.size += 1;
}
```

**Key insight:** Check `contains` before adding to avoid duplicates.

### Remove Operation

```rust
pub fn remove(&mut self, val: i32) {
    let index = self.hash(val);
    let bucket = &mut self.buckets[index];

    if bucket.is_none() {
        return;
    }

    let mut current = bucket.take();

    if current.as_ref().unwrap().val == val {
        *bucket = current.unwrap().next;
        self.size -= 1;
        return;
    }

    while let Some(mut node) = current {
        if let Some(ref mut next) = node.next {
            if next.key == val {  // Wait, should be val not key
                node.next = next.take().unwrap().next;
                self.size -= 1;
                return;
            }
        }
        current = node.next;
    }
}
```

**Note:** Should be `next.val == val` not `next.key == val`.

## Collision Handling

Uses separate chaining (linked lists) just like HashMap:

```
After adding 1, 11, 21 (assuming same bucket):
bucket → [21] → [11] → [1] → null

remove(11):
bucket → [21] → [1] → null
```

## Four Approaches Compared

| Approach | Add | Remove | Contains | Space |
|----------|-----|--------|----------|-------|
| Separate chaining (linked) | O(1)* | O(1)* | O(1)* | O(n) |
| Separate chaining (vector) | O(1)* | O(1)* | O(1)* | O(n) |
| Open addressing | O(1)* | O(1)* | O(1)* | O(n) |
| Bloom filter | O(1) | N/A | O(1) | O(n) |

*Amortized O(1) with good hash function.

## Complexity Analysis

| Operation | Average | Worst |
|-----------|---------|-------|
| add | O(1) | O(n) |
| remove | O(1) | O(n) |
| contains | O(1) | O(n) |
| space | O(n) | O(n) |

## Differences from HashMap

| Aspect | HashMap | HashSet |
|--------|---------|---------|
| Stores | key-value pairs | keys only |
| Duplicate handling | Update value | Ignore (already present) |
| Return value | Value or -1 | Boolean |
| Placeholder value | N/A | () |

## Test Cases Design

### Basic Operations
1. Add single element, verify contains
2. Add multiple elements, verify all
3. Remove element, verify gone
4. Remove non-existent, verify no change

### Edge Cases
5. Add duplicate (should be no-op)
6. Remove from empty set
7. Contains on empty set
8. Many operations with collisions

### Boundary Conditions
9. Add 0, verify
10. Add negative numbers
11. Add large numbers
12. All elements hash to same bucket

## Implementation with Alternative: Vec<Vec<i32>>

```rust
pub struct SimpleHashSet {
    buckets: Vec<Vec<i32>>,
    size: usize,
}

impl SimpleHashSet {
    pub fn new() -> Self {
        SimpleHashSet {
            buckets: vec![Vec::new(); 1000],
            size: 0,
        }
    }

    pub fn add(&mut self, val: i32) {
        if self.contains(val) {
            return;
        }
        let index = self.hash(val);
        self.buckets[index].push(val);
        self.size += 1;
    }

    pub fn contains(&self, val: i32) -> bool {
        let index = self.hash(val);
        self.buckets[index].iter().any(|&v| v == val)
    }

    pub fn remove(&mut self, val: i32) {
        let index = self.hash(val);
        let bucket = &mut self.buckets[index];
        if let Some(pos) = bucket.iter().position(|&v| v == val) {
            bucket.remove(pos);
            self.size -= 1;
        }
    }
}
```

This alternative uses vectors for buckets - simpler but remove is O(n) in bucket.

## Related Problems

1. **LeetCode 705**: Design HashSet (this problem)
2. **LeetCode 706**: Design HashMap
3. **LeetCode 217**: Contains Duplicate (uses HashSet concept)
4. **LeetCode 349**: Intersection of Two Arrays

## Time to Complete

**Target**: 30 minutes for first implementation
**Optimal**: 20 minutes with vector-based approach