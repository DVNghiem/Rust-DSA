# Solutions Analysis: Design HashSet (LeetCode 705)

## Solution Overview

HashSet is essentially a HashMap that stores only keys with a placeholder value. We implement two versions:
1. **Linked list chaining** (primary)
2. **Vector chaining** (alternative simpler approach)

## Core Insight: HashSet = HashMap<K, ()>

```rust
// HashSet is just HashMap where value is ()
struct MyHashSet {
    buckets: Vec<Option<Box<ListNode>>>,  // Node only has val, not key-val
    size: usize,
}
```

The key difference is that HashSet nodes only store `val`, not a key-value pair.

## Data Structure Comparison

### HashMap ListNode
```rust
struct ListNode {
    key: i32,
    val: i32,
    next: Option<Box<ListNode>>,
}
```

### HashSet ListNode
```rust
struct ListNode {
    val: i32,  // Only value, no key
    next: Option<Box<ListNode>>,
}
```

## Add Operation Deep Dive

```rust
pub fn add(&mut self, val: i32) {
    if self.contains(val) {
        return; // Already exists - HashSet has no duplicates
    }

    if self.should_rehash() {
        self.rehash();
    }

    let index = self.hash(val);
    let bucket = &mut self.buckets[index];

    // Add at head of bucket (O(1))
    let new_node = Box::new(ListNode { val, next: bucket.take() });
    *bucket = Some(new_node);
    self.size += 1;
}
```

**Step-by-step:**

1. **Lines 2-4**: Check if already present - HashSet has no duplicates
   - If contains, return early (no-op)

2. **Lines 5-7**: Rehash if load factor exceeded

3. **Lines 9-14**: Insert at head of bucket
   - `bucket.take()` - Takes ownership of current head (None if empty)
   - `next: bucket.take()` - New node points to old head
   - `*bucket = Some(new_node)` - New node becomes head

**Why insert at head?**
- O(1) operation vs O(n) at tail
- Order doesn't matter in a set

## Contains Operation Deep Dive

```rust
pub fn contains(&self, val: i32) -> bool {
    let index = self.hash(val);
    let bucket = &self.buckets[index];

    let mut current = bucket.as_ref();
    while let Some(node) = current {
        if node.val == val {
            return true;
        }
        current = node.next.as_ref();
    }
    false
}
```

**Step-by-step:**

1. **Lines 2-3**: Hash and get bucket reference

2. **Lines 5-10**: Traverse linked list
   - Start with `bucket.as_ref()` which gives `Option<&ListNode>`
   - If node.val matches, return true
   - Otherwise, advance to `node.next.as_ref()`

3. **Line 11**: Return false if not found

## Remove Operation Deep Dive

```rust
pub fn remove(&mut self, val: i32) {
    let index = self.hash(val);
    let bucket = &mut self.buckets[index];

    if bucket.is_none() {
        return;
    }

    let mut current = bucket.take();

    // Check if head matches
    if current.as_ref().unwrap().val == val {
        *bucket = current.unwrap().next;
        self.size -= 1;
        return;
    }

    // Search rest of list
    while let Some(mut node) = current {
        if let Some(ref mut next) = node.next {
            if next.val == val {
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

1. **Lines 2-8**: Hash and handle empty bucket

2. **Lines 10-16**: Check if head matches
   - `bucket.take()` - Takes ownership of head
   - If head.val == target, replace bucket with head.next
   - Decrement size and return

3. **Lines 18-27**: Traverse rest of list
   - For each node, check if `node.next.val == target`
   - If found: `node.next = next.next` (skip the node)
   - Decrement size and return

**Critical Rust pattern - `take()` on Option:**

```rust
node.next = next.take().unwrap().next;
// This is equivalent to:
// 1. take() takes ownership of Some(Box)
// 2. unwrap() gets the Box
// 3. .next gets the next pointer
// 4. node.next is set to that next pointer
```

## Hash Function

```rust
fn hash(&self, val: i32) -> usize {
    (val as usize).wrapping_abs() % self.buckets.len()
}
```

- `(val as usize)` - Convert to unsigned
- `.wrapping_abs()` - Handle i32::MIN properly
- `% self.buckets.len()` - Map to bucket index

## Load Factor and Rehashing

```rust
fn should_rehash(&self) -> bool {
    if self.buckets.is_empty() {
        return true;
    }
    let load_factor = self.size as f64 / self.buckets.len() as f64;
    load_factor > LOAD_FACTOR_THRESHOLD
}
```

When `size / buckets.len() > 0.75`, we double the bucket count and reinsert all elements.

## Alternative Implementation: Vec<Vec<i32>>

```rust
pub struct SimpleHashSet {
    buckets: Vec<Vec<i32>>,
    size: usize,
}

impl SimpleHashSet {
    pub fn add(&mut self, val: i32) {
        if self.contains(val) {
            return;
        }
        let index = self.hash(val);
        self.buckets[index].push(val);
        self.size += 1;
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

**Trade-offs:**

| Aspect | Linked List | Vec of Vec |
|--------|-------------|------------|
| Remove from middle | O(1)* | O(n) |
| Memory overhead | More (Box pointers) | Less |
| Cache locality | Poor | Good |
| Implementation | Complex | Simple |

*O(1) with pointer manipulation but more ownership complexity.

## Complexity Analysis

### Linked List Implementation

| Operation | Average | Worst Case |
|-----------|---------|------------|
| add | O(1) | O(n) |
| remove | O(1) | O(n) |
| contains | O(1) | O(n) |
| space | O(n) | O(n) |

### Vec Implementation

| Operation | Average | Worst Case |
|-----------|---------|------------|
| add | O(1)* | O(n) |
| remove | O(n) | O(n) |
| contains | O(n) | O(n) |
| space | O(n) | O(n) |

*Amortized O(1) for add (push to Vec is amortized O(1)).

## Key Differences from HashMap

| Aspect | HashMap | HashSet |
|--------|---------|---------|
| Data stored | (key, val) pairs | single value |
| Return on get | value or -1 | boolean |
| Duplicate handling | Update existing value | Ignore (already present) |
| Add behavior | Always insert | Check first, then insert |
| Node size | 2 fields + pointer | 1 field + pointer |

## Test Coverage Analysis

### Basic Tests
- Single add operations
- Multiple adds
- Duplicate adds (verify size unchanged)
- Contains checks

### Edge Cases
- Empty set operations
- Remove from empty
- Remove non-existent
- Negative numbers, zero, large values

### Collision Tests
- Multiple values in same bucket
- Remove from head vs middle
- Rehash verification

### Memory Tests
- Many operations
- Clear and verify

## Common Pitfalls in Rust

### Pitfall 1: Forgetting duplicate check

```rust
// WRONG - always inserts, even if duplicate
pub fn add(&mut self, val: i32) {
    let index = self.hash(val);
    // ... insert at head
    self.size += 1;
}

// CORRECT - check first
pub fn add(&mut self, val: i32) {
    if self.contains(val) {
        return;
    }
    // ... insert
}
```

### Pitfall 2: Incorrect next pointer handling in remove

```rust
// WRONG - loses rest of list
node.next = next;  // next is Box, not Option

// CORRECT - use take() and access .next
node.next = next.take().unwrap().next;
```

### Pitfall 3: Not handling i32::MIN in hash

```rust
// WRONG - overflow on i32::MIN
fn hash(&self, val: i32) -> usize {
    (val.abs() as usize) % self.buckets.len()  // panics on MIN
}

// CORRECT - use wrapping_abs
fn hash(&self, val: i32) -> usize {
    (val as usize).wrapping_abs() % self.buckets.len()
}
```

## When to Use Which Implementation

**Use linked list approach when:**
- Need guaranteed O(1) remove
- Memory overhead is acceptable
- Writing production-grade HashSet

**Use Vec approach when:**
- Simpler code is preferred
- Buckets are small (remove is O(n) but n is small)
- Performance not critical

## Related Problems and Patterns

1. **Contains Duplicate (LeetCode 217)** - Uses HashSet to track seen elements
2. **Intersection of Two Arrays (LeetCode 349)** - HashSet for intersection
3. **Happy Number (LeetCode 202)** - HashSet for cycle detection
4. **Linked List Cycle** - Similar pattern with set/dict for tracking visited

## Conclusion

HashSet implementation demonstrates:

1. **Reusing HashMap patterns** - HashSet is conceptually HashMap with () values
2. **Set-specific behavior** - No duplicates, add is idempotent
3. **Rust ownership patterns** - take(), unwrap(), Option handling
4. **Trade-offs** - Simpler Vec approach vs efficient linked list approach

The linked list implementation provides true O(1) operations while the Vec implementation trades that for simplicity. Both are valid depending on requirements.