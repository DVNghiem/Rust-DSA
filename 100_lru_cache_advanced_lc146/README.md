# LRU Cache - LeetCode 146

## Problem

Design a data structure that follows the constraints of a Least Recently Used (LRU) cache:
- `get(key)` - Returns the value if key exists, otherwise returns -1
- `put(key, value)` - Updates or inserts the value. If cache is at capacity, evict the least recently used key before inserting.

```
Example:
LRUCache cache = new LRUCache(2);
cache.put(1, 1);    // Cache: {1=1}
cache.put(2, 2);    // Cache: {1=1, 2=2}
cache.get(1);       // Returns 1, Cache: {2=2, 1=1} (1 is most recent)
cache.put(3, 3);    // Evicts key 2 (LRU), Cache: {1=1, 3=3}
cache.get(2);       // Returns -1 (not found, evicted)
```

## Design: HashMap + Doubly Linked List

### Key Insight

We need two operations in O(1):
1. **Random access by key** - HashMap provides O(1) lookup
2. **Maintain usage order** - Doubly linked list provides O(1) insert/delete when we know the position

Combining both gives us O(1) for both `get` and `put`.

### Data Structure

```
HashMap<K, Node<K, V>>  <--->  Doubly Linked List  <--->  ... <---> Head <---> Tail
                                                         Head = sentinel (most recent)
                                                         Tail = sentinel (least recent)
```

### Operations

**Get:**
1. Look up key in HashMap - O(1)
2. If found, move node to front of list (most recent) - O(1)
3. Return value

**Put:**
1. If key exists, update value and move to front - O(1)
2. If not, insert at front - O(1)
3. If over capacity, remove from tail (least recent) - O(1)

### Visual Walkthrough

```
Cache capacity = 2

Initial state:
Head <-> Tail (empty)

After put(1, 1):
Head <-> [1,1] <-> Tail

After put(2, 2):
Head <-> [2,2] <-> [1,1] <-> Tail

After get(1):
- Find node for key 1
- Move to front
Head <-> [1,1] <-> [2,2] <-> Tail
- Return 1

After put(3, 3):
- Insert new node at front
Head <-> [3,3] <-> [1,1] <-> [2,2] <-> Tail
- Over capacity! Evict LRU (tail's neighbor = [2,2])
Head <-> [3,3] <-> [1,1] <-> Tail

After get(2):
- Key 2 was evicted
Returns -1
```

## Topics Covered
- HashMap for O(1) lookup
- Doubly linked list for O(1) insertion/deletion
- LRU eviction policy
- Cache design patterns
- Capacity management

## Approaches

### Approach 1: HashMap + Doubly Linked List (Recommended)

Time: O(1) for both get and put | Space: O(capacity)
- Standard approach for LRU cache
- Explicit linked list for ordering
- HashMap for node lookup

### Approach 2: LinkedHashMap (Built-in)

Time: O(1) for both get and put | Space: O(capacity)
- Uses Java's LinkedHashMap with removeEldestEntry
- Cleaner but platform-dependent

### Approach 3: OrderedMap (Rust - BTreeMap)

Time: O(log n) for operations | Space: O(capacity)
- Uses BTreeMap's ordering
- Simpler but slower than linked list approach

## Complexity Analysis

| Approach | Get | Put | Space |
|----------|-----|-----|-------|
| HashMap + List | O(1) | O(1) | O(capacity) |
| LinkedHashMap | O(1) | O(1) | O(capacity) |
| BTreeMap | O(log n) | O(log n) | O(capacity) |

## Additional Notes

- LRU cache is a classic system design primitive
- Used in CPU caches, browser caches, database caches
- The "least recently used" key is the one that hasn't been accessed for the longest time
- Capacity is typically fixed at creation time
- Sentinel nodes (dummy head/tail) simplify linked list edge cases
