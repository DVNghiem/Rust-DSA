# LRU Cache - LeetCode 146

## Problem Overview

Design a data structure that follows the constraints of a Least Recently Used (LRU) cache. Implement the LRUCache class with `get(key)` and `put(key, value)` operations, both O(1).

**Examples:**
```
Input: ["LRUCache","put","put","get","put","get","put","get","get","get"]
       [[2],[1,1],[2,2],[1],[3,3],[2],[4,4],[1],[3],[4]]
Output: [null,null,null,1,null,-1,null,-1,3,4]
```

## Theory

### Key Insight: HashMap + Doubly Linked List

- **HashMap**: O(1) key lookup
- **Doubly Linked List**: O(1) insertion/deletion to track recency
- **Head/Tail sentinels**: Simplify edge cases

```
HEAD ↔ [recent] ↔ ... ↔ [old] ↔ TAIL
         ↑
    Most recently used
```

## Implementation

```rust
use std::collections::HashMap;

pub struct LRUCache {
    capacity: i32,
    map: HashMap<i32, Node>,
    head: Node,
    tail: Node,
}

struct Node {
    key: i32,
    val: i32,
    prev: Option<*mut Node>,
    next: Option<*mut Node>,
}
```

## Complexity Analysis

| Operation | Time | Space |
|----------|------|-------|
| get | O(1) | O(1) |
| put | O(1) | O(1) |

## Test Cases

```rust
#[test]
fn test_lru_cache_basic() {
    let mut cache = LRUCache::new(2);
    cache.put(1, 1);
    cache.put(2, 2);
    assert_eq!(cache.get(1), 1);
}
```