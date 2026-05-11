# Design HashMap - LeetCode 706

## Problem Statement

Design a HashMap without using any built-in hash table libraries.

```
Operations required:
- put(key, value): Insert a key-value pair into HashMap
- get(key): Return the value associated with key, or -1 if not found
- remove(key): Remove the key-value pair from HashMap

All operations must have O(1) average time complexity.
```

## Understanding Hash Functions

A hash function maps keys to array indices. A good hash function:
1. Distributes keys uniformly across the array
2. Is deterministic (same input always produces same output)
3. Is fast to compute

```
key "hello" → hash(key) → index 4 → bucket 4 → [ (key, value), ... ]
```

## Visual Walkthrough of HashMap Structure

```
HashMap Structure:
┌─────────┬─────────┬─────────┬─────────┬─────────┐
│ Bucket 0│ Bucket 1 │ Bucket 2 │ Bucket 3 │ Bucket 4 │
├─────────┼─────────┼─────────┼─────────┼─────────┤
│  null   │ (k,v)   │  null   │ (k,v)→  │  null   │
│         │  ↓       │         │  (k,v)  │         │
│         │  null    │         │  ↓      │         │
│         │          │         │  null   │         │
└─────────┴─────────┴─────────┴─────────┴─────────┘

Each bucket can contain a linked list of key-value pairs (separate chaining)
```

## Collision Handling: Separate Chaining

When two keys hash to the same index, we use linked lists to store multiple entries:

```
Index 4 after inserting "hello" and "world" (both hash to 4):
┌──────────────────────────────────────┐
│  bucket[4] → [(hello, 100)] → null   │
└──────────────────────────────────────┘

After inserting "foo" (also hashes to 4):
┌──────────────────────────────────────────────┐
│  bucket[4] → [(foo, 200)] → [(hello, 100)] → null
└──────────────────────────────────────────────┘
```

## Hash Function Design

### Simple Hash Function

```rust
fn hash(&self, key: i32) -> usize {
    (key as usize) % self.buckets.len()
}
```

For integer keys, simple modulo works well with good distribution.

### Handling Negative Keys

```rust
fn hash(&self, key: i32) -> usize {
    // Use unsigned conversion to handle negative numbers
    (key.wrapping_abs() as usize) % self.buckets.len()
}
```

Better approach: Use `unsigned_abs()` or handle two's complement properly.

### Improved Hash for Better Distribution

```rust
fn hash(&self, key: i32) -> usize {
    let mut h = key.hash_value();  // Use Rust's built-in hash
    h ^= h >> 16;
    (h as usize) % self.buckets.len()
}
```

## Data Structure Design

### Array of Buckets

```rust
pub struct MyHashMap {
    buckets: Vec<Option<Box<ListNode>>>,
    size: usize,
}
```

- `buckets`: Vector of optional linked list heads
- `size`: Number of key-value pairs stored

### Linked List Node

```rust
struct ListNode {
    key: i32,
    val: i32,
    next: Option<Box<ListNode>>,
}
```

Each node stores key, value, and pointer to next node.

## Four Approaches Compared

| Approach | Get | Put | Remove | Space |
|----------|-----|-----|--------|-------|
| Separate chaining (linked list) | O(1)* | O(1)* | O(1)* | O(n) |
| Separate chaining (vector) | O(1)* | O(1)* | O(1)* | O(n) |
| Open addressing (linear probing) | O(1)* | O(1)* | O(1)* | O(n) |
| Cuckoo hashing | O(1) | O(1) | O(1) | O(n) |

*Amortized O(1) with good hash function and load factor management.

## Implementation Walkthrough

### Step 1: Initialize HashMap

```rust
pub fn new() -> Self {
    let capacity = INITIAL_CAPACITY;
    let buckets: Vec<Option<Box<ListNode>>> = vec![None; capacity];
    MyHashMap {
        buckets,
        size: 0,
    }
}
```

Creates array of empty buckets (None = empty linked list).

### Step 2: Put Operation

```rust
pub fn put(&mut self, key: i32, value: i32) {
    let index = self.hash(key);
    let bucket = &mut self.buckets[index];

    // Check if key exists, update if so
    if let Some(ref mut node) = bucket {
        let mut current = node.as_mut();
        while let Some(next) = current.next.take() {
            if current.key == key {
                current.val = value;
                current.next = Some(next);
                return;
            }
            current = next;
        }
        // Key not found, add new node at end
        current.next = Some(Box::new(ListNode::new(key, value)));
    } else {
        // Empty bucket, create new node
        *bucket = Some(Box::new(ListNode::new(key, value)));
    }
    self.size += 1;
}
```

**Algorithm:**
1. Compute hash index
2. If bucket empty, create new node
3. If bucket has nodes, traverse list
4. If key found, update value
5. If key not found, append new node
6. Increment size

### Step 3: Get Operation

```rust
pub fn get(&self, key: i32) -> i32 {
    let index = self.hash(key);
    let bucket = &self.buckets[index];

    let mut current = bucket.as_ref();
    while let Some(node) = current {
        if node.key == key {
            return node.val;
        }
        current = node.next.as_ref();
    }
    -1  // Not found
}
```

**Algorithm:**
1. Compute hash index
2. Traverse linked list in bucket
3. If key found, return value
4. If not found, return -1

### Step 4: Remove Operation

```rust
pub fn remove(&mut self, key: i32) {
    let index = self.hash(key);
    let bucket = &mut self.buckets[index];

    if bucket.is_none() {
        return;  // Key not found
    }

    let mut current = bucket.take();
    if current.as_ref().unwrap().key == key {
        // Remove head
        *bucket = current.unwrap().next;
        self.size -= 1;
        return;
    }

    // Search rest of list
    while let Some(mut node) = current {
        if let Some(ref mut next) = node.next {
            if next.key == key {
                node.next = next.take().unwrap().next;
                self.size -= 1;
                return;
            }
        }
        current = node.next;
    }
}
```

**Algorithm:**
1. Compute hash index
2. If bucket empty, return
3. If head matches, remove head
4. Otherwise, traverse and remove from middle
5. Decrement size

## Load Factor and Rehashing

When the HashMap gets too full, performance degrades. We should rehashed when load factor exceeds threshold.

```
Load Factor = size / num_buckets

If load_factor > 0.75, rehash to double capacity
```

### Rehashing Process

```rust
fn rehash(&mut self) {
    let new_capacity = self.buckets.len() * 2;
    let mut new_buckets: Vec<Option<Box<ListNode>>> = vec![None; new_capacity];

    // Reinsert all existing elements
    for bucket in self.buckets.iter_mut() {
        let mut current = bucket.take();
        while let Some(mut node) = current {
            let next = node.next.take();
            let index = (node.key.wrapping_abs() as usize) % new_capacity;
            node.next = new_buckets[index].take();
            new_buckets[index] = Some(node);
            current = next;
        }
    }

    self.buckets = new_buckets;
}
```

## Complete Implementation

```rust
const INITIAL_CAPACITY: usize = 1000;
const LOAD_FACTOR_THRESHOLD: f64 = 0.75;

pub struct MyHashMap {
    buckets: Vec<Option<Box<ListNode>>>,
    size: usize,
}

struct ListNode {
    key: i32,
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(key: i32, val: i32) -> Self {
        ListNode { key, val, next: None }
    }
}

impl MyHashMap {
    pub fn new() -> Self {
        MyHashMap {
            buckets: vec![None; INITIAL_CAPACITY],
            size: 0,
        }
    }

    fn hash(&self, key: i32) -> usize {
        (key as usize) % self.buckets.len()
    }

    fn should_rehash(&self) -> bool {
        let load_factor = self.size as f64 / self.buckets.len() as f64;
        load_factor > LOAD_FACTOR_THRESHOLD
    }

    pub fn put(&mut self, key: i32, value: i32) { /* ... */ }
    pub fn get(&self, key: i32) -> i32 { /* ... */ }
    pub fn remove(&mut self, key: i32) { /* ... */ }
}
```

## Test Cases Design

### Basic Operations
1. Put single element, get returns correct value
2. Put multiple elements, verify correct retrieval
3. Remove element, verify it's gone
4. Remove from empty bucket

### Edge Cases
5. Update existing key's value
6. Remove non-existent key
7. Get on empty HashMap
8. Large number of operations

### Collision Handling
9. Many keys hashing to same bucket
10. All keys hash to different buckets
11. Keys with modulo collisions

### Boundary Conditions
12. Negative keys
13. Key of zero
14. Large key values
15. Empty HashMap operations

## Complexity Analysis

| Operation | Average | Worst Case |
|-----------|---------|------------|
| put | O(1) | O(n) |
| get | O(1) | O(n) |
| remove | O(1) | O(n) |
| space | O(n) | O(n) |

Worst case O(n) occurs when all keys hash to the same bucket (degenerate hash function or malicious input).

## Related Problems

1. **LeetCode 705**: Design HashSet (similar problem)
2. **LeetCode 706**: Design HashMap (this problem)
3. **LeetCode 146**: LRU Cache (uses HashMap + Linked List)
4. **LeetCode 380**: Insert Delete GetRandom O(1)

## Time to Complete

**Target**: 45 minutes for first implementation
**Optimal**: 30-35 minutes with clean separate chaining approach

## Next Steps

After completing this problem:
1. Move to HashSet design (similar approach)
2. Explore more complex data structures like LRU Cache
3. Practice with hash-based problems