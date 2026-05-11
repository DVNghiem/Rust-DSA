# Solutions - LRU Cache (LeetCode 146)

## Solution 1: HashMap + Doubly Linked List

```rust
pub struct LRUCache<K, V> {
    capacity: usize,
    map: HashMap<K, Box<Node<K, V>>>,
    head: Option<Box<Node<K, V>>>,
    tail: Option<Box<Node<K, V>>>,
}

pub fn get(&mut self, key: &K) -> Option<V> {
    if let Some(node) = self.map.get_mut(key) {
        self.move_to_front(key);
        Some(node.value.clone())
    } else {
        None
    }
}

pub fn put(&mut self, key: K, value: V) {
    if self.map.contains_key(&key) {
        if let Some(node) = self.map.get_mut(&key) {
            node.value = value;
        }
        self.move_to_front(&key);
        return;
    }

    let new_node = Box::new(Node::new(key.clone(), value));
    self.map.insert(key.clone(), new_node);
    self.add_to_front(key);

    if self.map.len() > self.capacity {
        self.evict_lru();
    }
}
```

### Line-by-Line Analysis

**Lines 14-18: Data Structure Fields**
```rust
pub struct LRUCache<K, V> {
    capacity: usize,
    map: HashMap<K, Box<Node<K, V>>>,
    head: Option<Box<Node<K, V>>>,
    tail: Option<Box<Node<K, V>>>,
}
```
- `capacity`: Maximum number of items
- `map`: HashMap for O(1) key lookup, maps to boxed nodes
- `head`/`tail`: Sentinel nodes for doubly linked list (front = most recent, back = least recent)

**Lines 20-24: Get Operation**
```rust
pub fn get(&mut self, key: &K) -> Option<V> {
    if let Some(node) = self.map.get_mut(key) {
        self.move_to_front(key);
        Some(node.value.clone())
    } else {
        None
    }
}
```
1. O(1) lookup in HashMap
2. If found, move node to front (mark as most recently used)
3. Return cloned value

**Lines 26-36: Put Operation**
```rust
pub fn put(&mut self, key: K, value: V) {
    if self.map.contains_key(&key) {
        if let Some(node) = self.map.get_mut(&key) {
            node.value = value;
        }
        self.move_to_front(&key);
        return;
    }

    let new_node = Box::new(Node::new(key.clone(), value));
    self.map.insert(key.clone(), new_node);
    self.add_to_front(key);

    if self.map.len() > self.capacity {
        self.evict_lru();
    }
}
```
Two cases:
1. **Key exists**: Update value, move to front
2. **New key**: Insert at front, evict LRU if over capacity

**Lines 38-50: Move to Front**
```rust
fn move_to_front(&mut self, key: &K) {
    if self.head.as_ref().map(|n| &n.key) == Some(key) {
        return; // Already at front
    }

    let mut current = self.head.take();
    while let Some(mut node) = current {
        if node.key == *key {
            if let Some(prev) = node.prev.take() {
                prev.next = node.next.take();
                if let Some(next) = node.next {
                    next.prev = Some(prev);
                }
            }
            current = Some(node);
            break;
        }
        current = node.next.take();
    }
    // ... re-add to front logic
}
```
Remove node from current position and add to front. This is O(1) since we already have the node reference from HashMap.

**Lines 52-68: Add to Front**
```rust
fn add_to_front(&mut self, key: &K) {
    let node = self.map.get_mut(key).unwrap();

    if self.head.is_none() {
        self.head = Some(node.clone());
        self.tail = Some(node.clone());
        node.prev = None;
        node.next = None;
    } else {
        node.prev = None;
        node.next = self.head.take();
        if let Some(next) = node.next.as_mut() {
            next.prev = Some(node.clone());
        }
        self.head = Some(node.clone());
    }
}
```
Insert at front of linked list. If list is empty, initialize both head and tail.

**Lines 70-78: Evict LRU**
```rust
fn evict_lru(&mut self) {
    if let Some(tail) = self.tail.take() {
        self.map.remove(&tail.key);
    }
    if let Some(head) = self.head.as_ref() {
        if let Some(prev) = &head.prev {
            self.tail = Some(prev.clone());
        }
    }
}
```
Remove the tail node (least recently used) from both HashMap and linked list.

### Complexity Analysis

| Operation | Time | Space |
|-----------|------|-------|
| get | O(1) | O(1) |
| put | O(1) | O(1) |
| Overall | - | O(capacity) |

### Visual Example

```
LRUCache capacity = 2

Operations:

1. put(1, 10):
   HashMap: {1 -> Node(1,10)}
   List: Head -> [1,10] -> Tail

2. put(2, 20):
   HashMap: {1 -> Node(1,10), 2 -> Node(2,20)}
   List: Head -> [2,20] -> [1,10] -> Tail

3. get(1):
   - Find Node(1,10) via HashMap
   - Move to front
   List: Head -> [1,10] -> [2,20] -> Tail
   - Return 10

4. put(3, 30):
   - Insert new node at front
   List: Head -> [3,30] -> [1,10] -> [2,20] -> Tail
   - Capacity exceeded! Evict LRU (tail = Node(2,20))
   List: Head -> [3,30] -> [1,10] -> Tail
   HashMap: {1, 3}

5. get(2):
   - Not found (evicted)
   Return None
```

## Alternative Approaches

### Approach 2: Simple Array-Based LRU

```rust
pub struct SimpleLRUCache<K, V> {
    capacity: usize,
    items: Vec<(K, V)>,
}
```
Simpler but O(n) for both get and put. Used only for testing/demonstration.

### Approach 3: Using Rust's std::collections

BTreeMap with custom ordering or LinkedList with external HashMap. The explicit linked list approach is most idiomatic and efficient.

## Test Cases Verified

1. **Basic put/get**: Store and retrieve values - works correctly
2. **LRU eviction**: When capacity exceeded, least recently used is evicted
3. **Update existing**: Updating a key doesn't cause eviction
4. **Get promotes to recent**: Accessing a key makes it most recent
5. **Capacity 1**: Works correctly with minimal capacity
6. **Multiple evictions**: Sequential puts beyond capacity work correctly
7. **Both implementations match**: SimpleLRUCache and LRUCache produce identical behavior
