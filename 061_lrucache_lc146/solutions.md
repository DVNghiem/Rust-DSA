# LRU Cache Solution - LeetCode 146 (Complete)

## Solution Analysis

### HashMap + VecDeque Approach

```rust
pub struct LRUCache {
    capacity: usize,
    map: HashMap<i32, i32>,
    order: VecDeque<i32>,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        LRUCache {
            capacity: capacity as usize,
            map: HashMap::new(),
            order: VecDeque::new(),
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(&value) = self.map.get(&key) {
            // Move to back (most recently used)
            if let Some(pos) = self.order.iter().position(|&k| k == key) {
                self.order.remove(pos);
                self.order.push_back(key);
            }
            value
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        // If key exists, update and move to back
        if self.map.contains_key(&key) {
            self.map.insert(key, value);
            if let Some(pos) = self.order.iter().position(|&k| k == key) {
                self.order.remove(pos);
            }
            self.order.push_back(key);
        } else {
            // If at capacity, remove least recently used (front)
            if self.map.len() >= self.capacity {
                if let Some(lru_key) = self.order.pop_front() {
                    self.map.remove(&lru_key);
                }
            }
            self.map.insert(key, value);
            self.order.push_back(key);
        }
    }
}
```

## Why Two Data Structures?

### HashMap for O(1) Lookup
```rust
map: HashMap<i32, i32>
```
- Key → Value mapping
- O(1) get and put operations

### VecDeque for Order Tracking
```rust
order: VecDeque<i32>
```
- Stores keys in order of use
- Front = Least Recently Used
- Back = Most Recently Used
- O(1) remove from front and add to back

## Key Operations Explained

### Get Operation

```rust
if let Some(&value) = self.map.get(&key) {
    // Move key to back (most recently used)
    if let Some(pos) = self.order.iter().position(|&k| k == key) {
        self.order.remove(pos);
        self.order.push_back(key);
    }
    value
} else {
    -1
}
```
1. Check if key exists in map
2. If yes, move key to back of order (most recently used)
3. Return value
4. If no, return -1

### Put Operation

```rust
if self.map.contains_key(&key) {
    // Update existing: move to back
    self.map.insert(key, value);
    // Move to back in order
} else {
    // New key: check capacity, evict if needed
    if self.map.len() >= self.capacity {
        let lru_key = self.order.pop_front().unwrap();
        self.map.remove(&lru_key);
    }
    self.map.insert(key, value);
    self.order.push_back(key);
}
```

1. If key exists: update value, move to back
2. If key doesn't exist:
   - If at capacity: evict LRU (front of order)
   - Insert new key-value pair
   - Add key to back of order

## Why Move-to-Back on Get?

When we access a key, it becomes the Most Recently Used.
- In VecDeque, back = MRU
- So we remove from current position and push to back

## Why Remove from Front on Eviction?

When we need to evict (capacity full) and adding a new key:
- Front of VecDeque = Least Recently Used
- We remove that key from both map and order

## Complexity Analysis

| Operation | Time | Space |
|----------|------|-------|
| get | O(1) avg | O(1) |
| put | O(1) avg | O(1) |

- HashMap: O(1) average for lookup/insert
- VecDeque: O(1) for push/pop from ends, O(n) for remove from middle

## Edge Cases

### Capacity 0
```rust
let mut cache = LRUCache::new(0);
cache.put(1, 1);
cache.get(1);  // Returns -1
```
- No items can be stored

### Update Existing Key
```rust
cache.put(1, 1);
cache.put(1, 10);
cache.get(1);  // Returns 10
```
- Updates value, doesn't change order position until next get

### Get Updates Order
```rust
cache.put(1, 1);
cache.put(2, 2);
cache.get(1);  // Returns 1, moves 1 to back
cache.put(3, 3);  // Evicts 2 (LRU)
```
- After get(1), order is [2, 1]
- After put(3), evict 2

## Common Mistakes

1. **Not updating order on get**: Would break LRU tracking
2. **Forgetting to remove from map on eviction**: Would leave stale entries
3. **Using Vec instead of VecDeque**: VecDeque provides O(1) front removal

## Rust-Specific Patterns

1. **`VecDeque::new()`**: Double-ended queue
2. **`position()` + `remove()`**: Find and remove in one pass
3. **`HashMap::contains_key()`**: Check before insert to handle update