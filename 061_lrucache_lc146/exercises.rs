/// LRU Cache - LeetCode 146
/// Implement a Least Recently Used cache with O(1) get and put operations.

use std::collections::{HashMap, VecDeque};

/// LRU Cache using VecDeque and HashMap for simpler implementation
/// Maintains order in deque, most recent at back
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
                self.order.push_back(key);
            }
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

/// Alternative: Using std::collections::VecDeque for order tracking
pub struct LRUCacheVec {
    capacity: usize,
    map: HashMap<i32, i32>,
    order: VecDeque<i32>,
}

impl LRUCacheVec {
    pub fn new(capacity: i32) -> Self {
        LRUCacheVec {
            capacity: capacity as usize,
            map: HashMap::new(),
            order: VecDeque::new(),
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(value) = self.map.get(&key).copied() {
            // Remove and re-add to back
            if let Some(idx) = self.order.iter().position(|&k| k == key) {
                self.order.remove(idx);
            }
            self.order.push_back(key);
            value
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if self.map.contains_key(&key) {
            self.map.insert(key, value);
            if let Some(idx) = self.order.iter().position(|&k| k == key) {
                self.order.remove(idx);
            }
            self.order.push_back(key);
        } else {
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

/// Linked List based approach for interview clarity
pub struct LRUCacheList {
    capacity: usize,
    map: HashMap<i32, usize>,
    storage: Vec<(i32, i32)>, // key-value pairs
    lru_order: VecDeque<usize>, // indices in storage
}

impl LRUCacheList {
    pub fn new(capacity: i32) -> Self {
        LRUCacheList {
            capacity: capacity as usize,
            map: HashMap::new(),
            storage: Vec::new(),
            lru_order: VecDeque::new(),
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(&idx) = self.map.get(&key) {
            // Move to back of LRU order
            if let Some(pos) = self.lru_order.iter().position(|&i| i == idx) {
                self.lru_order.remove(pos);
            }
            self.lru_order.push_back(idx);
            self.storage[idx].1
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(&idx) = self.map.get(&key) {
            // Update existing
            self.storage[idx].1 = value;
            if let Some(pos) = self.lru_order.iter().position(|&i| i == idx) {
                self.lru_order.remove(pos);
            }
            self.lru_order.push_back(idx);
        } else {
            // Evict if needed
            if self.map.len() >= self.capacity {
                if let Some(old_idx) = self.lru_order.pop_front() {
                    let old_key = self.storage[old_idx].0;
                    self.map.remove(&old_key);
                    self.storage[old_idx] = (key, value); // Reuse slot
                    self.map.insert(key, old_idx);
                    self.lru_order.push_back(old_idx);
                    return;
                }
            }
            let idx = self.storage.len();
            self.storage.push((key, value));
            self.map.insert(key, idx);
            self.lru_order.push_back(idx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lru_cache_basic() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);
        assert_eq!(cache.get(2), 2);
    }

    #[test]
    fn test_lru_cache_eviction() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        cache.put(3, 3);
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(2), 2);
        assert_eq!(cache.get(3), 3);
    }

    #[test]
    fn test_lru_cache_update() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        cache.put(1, 10);
        assert_eq!(cache.get(1), 10);
    }

    #[test]
    fn test_lru_cache_capacity_one() {
        let mut cache = LRUCache::new(1);
        cache.put(1, 1);
        assert_eq!(cache.get(1), 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(2), 2);
    }

    #[test]
    fn test_lru_cache_empty() {
        let mut cache = LRUCache::new(0);
        cache.put(1, 1);
        assert_eq!(cache.get(1), -1);
    }

    #[test]
    fn test_lru_cache_get_nonexistent() {
        let mut cache = LRUCache::new(2);
        assert_eq!(cache.get(1), -1);
    }

    #[test]
    fn test_lru_cache_vec_basic() {
        let mut cache = LRUCacheVec::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);
    }

    #[test]
    fn test_lru_cache_list_basic() {
        let mut cache = LRUCacheList::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);
    }

    #[test]
    fn test_lru_cache_multiple_gets() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);
        assert_eq!(cache.get(1), 1);
        assert_eq!(cache.get(2), 2);
    }

    #[test]
    fn test_lru_cache_lru_order() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        cache.put(3, 3); // Should evict 1
        assert_eq!(cache.get(1), -1);
        cache.put(4, 4); // Should evict 2
        assert_eq!(cache.get(2), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);
    }

    #[test]
    fn test_lru_cache_same_key() {
        let mut cache = LRUCache::new(1);
        cache.put(1, 1);
        cache.put(1, 2);
        assert_eq!(cache.get(1), 2);
        cache.put(2, 3);
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(2), 3);
    }

    #[test]
    fn test_lru_cache_stress() {
        let mut cache = LRUCache::new(3);
        for i in 0..100 {
            cache.put(i, i as i32);
        }
        // Last 3 should be accessible
        assert_eq!(cache.get(97), 97);
        assert_eq!(cache.get(98), 98);
        assert_eq!(cache.get(99), 99);
        assert_eq!(cache.get(96), -1);
    }

    #[test]
    fn test_lru_cache_all_approaches() {
        let mut cache1 = LRUCache::new(2);
        let mut cache2 = LRUCacheVec::new(2);
        let mut cache3 = LRUCacheList::new(2);

        for cache in [&mut cache1, &mut cache2, &mut cache3] {
            cache.put(1, 1);
            cache.put(2, 2);
            assert_eq!(cache.get(1), 1);
            cache.put(3, 3);
            assert_eq!(cache.get(2), -1);
        }
    }

    #[test]
    fn test_lru_cache_update_order() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        cache.put(1, 10);
        cache.put(3, 3);
        // 1 was updated, 2 should be evicted
        assert_eq!(cache.get(1), 10);
        assert_eq!(cache.get(2), -1);
        assert_eq!(cache.get(3), 3);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("LRU Cache exercises - run tests with cargo test");
}