# Solutions Analysis: Design HashMap (LeetCode 706)

## Solution Overview

We implement a HashMap using separate chaining with linked lists. Each bucket is a linked list of key-value pairs.

## Data Structure Design

### Main HashMap Structure

```rust
pub struct MyHashMap {
    buckets: Vec<Option<Box<ListNode>>>,
    size: usize,
}
```

**Line-by-line analysis:**

1. `buckets: Vec<Option<Box<ListNode>>>` - Vector of bucket headers
   - Each bucket is `Option<Box<ListNode>>` - either empty (`None`) or head of linked list (`Some(Box)`)
   - `Box<ListNode>` is heap-allocated to avoid stack overflow with large lists

2. `size: usize` - Tracks number of key-value pairs for size queries and load factor calculation

### ListNode Structure

```rust
struct ListNode {
    key: i32,
    val: i32,
    next: Option<Box<ListNode>>,
}
```

Each node stores:
- `key` - The lookup key
- `val` - The associated value
- `next` - Pointer to next node in chain (None if end of chain)

## Hash Function Implementation

```rust
fn hash(&self, key: i32) -> usize {
    (key as usize).wrapping_abs() % self.buckets.len()
}
```

**Analysis:**

1. `(key as usize)` - Convert i32 to usize (unsigned)
2. `.wrapping_abs()` - Handle two's complement negation properly (i32::MIN would overflow otherwise)
3. `% self.buckets.len()` - Modulo to get bucket index

**Why `wrapping_abs`?**
- i32::MIN = -2147483648
- `abs(-2147483648)` would overflow because 2147483648 > i32::MAX (2147483647)
- `wrapping_abs()` returns -2147483648 (the raw bits as unsigned)

## Put Operation Deep Dive

```rust
pub fn put(&mut self, key: i32, value: i32) {
    if self.should_rehash() {
        self.rehash();
    }

    let index = self.hash(key);
    let bucket = &mut self.buckets[index];

    // Traverse existing nodes in this bucket
    if let Some(ref mut node_ref) = bucket {
        let mut node = node_ref.as_mut();
        loop {
            if node.key == key {
                node.val = value;
                return;
            }
            if let Some(ref mut next) = node.next {
                node = next;
            } else {
                break;
            }
        }
        // Key not found, append at end
        node.next = Some(Box::new(ListNode::new(key, value)));
    } else {
        // Empty bucket, create new node
        *bucket = Some(Box::new(ListNode::new(key, value)));
    }
    self.size += 1;
}
```

**Step-by-step:**

1. **Lines 2-4**: Check if we need to rehash (load factor exceeded)
   - `should_rehash()` returns true when `size / buckets.len() > 0.75`

2. **Lines 6-7**: Compute index and get mutable reference to bucket

3. **Lines 9-23**: Traverse linked list looking for existing key
   - If found, update value and return
   - If not found, continue traversing until end

4. **Lines 25-26**: Append new node at end of chain

5. **Line 28**: Increment size (note: this runs even for updates, potential bug!)

**Critical Bug Found!**

The current implementation increments size even when updating an existing key. Should be:

```rust
// After node.val = value, we should return early without incrementing
// Or restructure like this:
if let Some(ref mut node_ref) = bucket {
    let mut node = node_ref.as_mut();
    loop {
        if node.key == key {
            node.val = value;
            return;  // Early return - don't increment size
        }
        // ...
    }
}
```

Wait, looking more carefully, if we update, we return before reaching `size += 1`. So the bug is only if we somehow fall through. Let me trace:

1. Key exists: `node.val = value; return;` - size NOT incremented
2. Key not found: append at end, then `size += 1` - correct

Actually the implementation seems correct for updates.

## Get Operation Deep Dive

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
    -1
}
```

**Step-by-step:**

1. **Lines 2-3**: Compute index and get bucket reference

2. **Lines 5-11**: Traverse linked list
   - Start with `bucket.as_ref()` which gives `Option<&ListNode>`
   - Loop while current is `Some(node)`
   - If key matches, return value
   - Otherwise, advance to `node.next.as_ref()`

3. **Line 12**: Return -1 if not found

**Rust Pattern Explanation:**

```rust
let mut current = bucket.as_ref();  // Option<&ListNode>
while let Some(node) = current {    // Destructure Option
    // ...
    current = node.next.as_ref();    // Option<&ListNode>
}
```

This is idiomatic Rust for iterating an Option-based linked list.

## Remove Operation Deep Dive

```rust
pub fn remove(&mut self, key: i32) {
    let index = self.hash(key);
    let bucket = &mut self.buckets[index];

    if bucket.is_none() {
        return;
    }

    let mut current = bucket.take();

    // Check if head matches
    if current.as_ref().unwrap().key == key {
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

**Step-by-step:**

1. **Lines 2-8**: Hash and check if bucket is empty

2. **Lines 10-16**: Check if head node matches key
   - `bucket.take()` - Takes ownership of head, leaving `None` in bucket
   - If matches, replace bucket with `head.next`
   - Decrement size and return

3. **Lines 18-28**: Traverse rest of list
   - `node.next` is `Option<Box<ListNode>>`
   - `next.take()` takes ownership of the next node

**Critical Rust Pattern - `take()` on Option:**

```rust
node.next = next.take().unwrap().next;
// equivalent to:
let taken_next = node.next.take().unwrap();  // Take ownership
node.next = taken_next.next;                  // Link to rest of list
```

This is how we remove a node from the middle of a linked list in Rust.

## Rehashing Implementation

```rust
fn rehash(&mut self) {
    let new_capacity = self.buckets.len().saturating_mul(2).max(1);
    let mut new_buckets: Vec<Option<Box<ListNode>>> = vec![None; new_capacity];

    // Reinsert all existing elements
    for bucket in self.buckets.iter_mut() {
        let mut current = bucket.take();
        while let Some(mut node) = current {
            let next = node.next.take();
            let index = (node.key as usize).wrapping_abs() % new_buckets.len();
            node.next = new_buckets[index].take();
            new_buckets[index] = Some(node);
            current = next;
        }
    }

    self.buckets = new_buckets;
}
```

**Step-by-step:**

1. **Line 2**: Calculate new capacity (double, minimum 1)
   - `saturating_mul(2)` - Won't overflow on very large values
   - `.max(1)` - Ensure at least 1 bucket

2. **Lines 3-17**: Reinsert all elements
   - For each bucket, take ownership of linked list
   - For each node, compute new index and insert at head of new bucket

**Why insert at head?**

Inserting at head is O(1) and we don't care about order in a hash map. The important thing is that all elements are correctly redistributed.

## Load Factor Management

```rust
fn should_rehash(&self) -> bool {
    if self.buckets.is_empty() {
        return true;
    }
    let load_factor = self.size as f64 / self.buckets.len() as f64;
    load_factor > LOAD_FACTOR_THRESHOLD
}
```

**Load factor** = items / buckets

When load factor > 0.75:
- Hash collisions increase
- Performance degrades

**Solution:** Rehash to more buckets (typically double)

## Alternative Implementation: SimpleHashMap

```rust
pub struct SimpleHashMap {
    buckets: Vec<Vec<(i32, i32)>>,
    size: usize,
}
```

Uses `Vec<Vec<(K,V)>>` instead of linked lists:

- Simpler implementation (no manual linked list)
- Better cache locality for small buckets
- `bucket.push()` and `bucket.remove()` are easier

**Trade-off:** For very large buckets, removing from middle requires O(n) shift.

## Complexity Analysis

### Time Complexity

| Operation | Average | Worst Case |
|-----------|---------|------------|
| put | O(1) | O(n) - all keys same bucket |
| get | O(1) | O(n) |
| remove | O(1) | O(n) |
| contains | O(1) | O(n) |

### Space Complexity

| Metric | Value |
|--------|-------|
| Base space | O(buckets.len()) |
| Per entry | O(1) for pointers |
| Total worst | O(n) |

## Test Case Coverage Analysis

### Basic Tests
- Single put/get/remove
- Multiple operations
- Update existing keys

### Edge Cases
- Empty bucket operations
- Remove from head vs middle vs tail
- Negative keys, key zero, large keys

### Collision Tests
- Keys that hash to same bucket
- Many operations that trigger rehash

### Memory Tests
- Large number of operations
- Clear and reuse

## Common Pitfalls in Rust Implementation

### Pitfall 1: Ownership with `take()`

```rust
// WRONG - trying to use after take
let next = node.next.take();
// node.next is now None!

// CORRECT - use immediately
node.next = node.next.take().unwrap().next;
```

### Pitfall 2: Mutable borrowing across loop

```rust
// This can cause issues with complex nested Option
let mut node = node_ref.as_mut();
loop {
    // node is &mut ListNode
    // Need to carefully handle next borrowing
}
```

### Pitfall 3: Overflow in hash function

```rust
// WRONG - can overflow
(key as usize) % self.buckets.len()

// CORRECT - use wrapping
(key as usize).wrapping_abs() % self.buckets.len()
```

## Alternative Approaches

### 1. Vec<Vec<(K,V)>> (Chaining with vectors)

```rust
struct SimpleHashMap {
    buckets: Vec<Vec<(i32, i32)>>,
    size: usize,
}
```

**Pros:**
- Simpler implementation
- Better cache locality
- No Box overhead

**Cons:**
- Remove is O(n) in bucket

### 2. Open Addressing (Linear Probing)

```rust
struct LinearHashMap {
    slots: Vec<Option<(i32, i32)>>,
    size: usize,
}
```

**Pros:**
- No extra allocations
- Better cache locality

**Cons:**
- More complex removal (tombstones)
- Clustering issues

### 3. Cuckoo Hashing

**Pros:** O(1) worst case for all operations

**Cons:** Complex implementation, limited use cases

## Conclusion

The separate chaining with linked lists approach:

1. **Simple and robust** - Handles arbitrary collision patterns
2. **Predictable performance** - O(1) average, no worst-case clustering
3. **Memory efficient** - Only allocates what's needed
4. **Rust-idiomatic** - Uses Box for heap allocation, Option for nullability

The implementation demonstrates key Rust patterns:
- `Option::take()` for ownership transfer
- `&mut` borrows for exclusive access
- Recursive data structures with Box
- Proper handling of edge cases (empty, negative keys, etc.)