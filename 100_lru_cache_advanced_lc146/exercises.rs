//! Exercises for LRU Cache (LeetCode 146)
//!
//! # Topics Covered
//! - HashMap for O(1) lookup
//! - VecDeque for ordering
//! - LRU eviction policy
//! - Cache design
//! - Capacity management
//!
//! # Difficulty: Hard

use std::collections::{HashMap, VecDeque};

/// Simple LRU Cache using HashMap and VecDeque for ordering
pub struct LRUCache {
    capacity: usize,
    map: HashMap<i32, i32>,
    order: VecDeque<i32>,
}

impl LRUCache {
    /// Create a new LRU cache with the given capacity
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            map: HashMap::new(),
            order: VecDeque::new(),
        }
    }

    /// Get a value from the cache, returns None if not found
    pub fn get(&mut self, key: i32) -> Option<i32> {
        if let Some(value) = self.map.get(&key).copied() {
            if let Some(pos) = self.order.iter().position(|&k| k == key) {
                self.order.remove(pos);
            }
            self.order.push_front(key);
            Some(value)
        } else {
            None
        }
    }

    /// Put a key-value pair into the cache
    pub fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }

        if self.map.contains_key(&key) {
            self.map.insert(key, value);
            if let Some(pos) = self.order.iter().position(|&k| k == key) {
                self.order.remove(pos);
            }
            self.order.push_front(key);
            return;
        }

        if self.map.len() >= self.capacity {
            if let Some(lru_key) = self.order.pop_back() {
                self.map.remove(&lru_key);
            }
        }

        self.map.insert(key, value);
        self.order.push_front(key);
    }

    /// Check if cache contains a key
    pub fn contains(&self, key: &i32) -> bool {
        self.map.contains_key(key)
    }

    /// Get the current size of the cache
    pub fn size(&self) -> usize {
        self.map.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_put_get() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 10);
        cache.put(2, 20);
        assert_eq!(cache.get(1), Some(10));
        assert_eq!(cache.get(2), Some(20));
    }

    #[test]
    fn test_eviction_lru() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 10);
        cache.put(2, 20);
        cache.put(3, 30);
        assert_eq!(cache.get(1), None);
        assert_eq!(cache.get(2), Some(20));
        assert_eq!(cache.get(3), Some(30));
    }

    #[test]
    fn test_update_existing_key() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 10);
        cache.put(2, 20);
        cache.put(1, 100);
        assert_eq!(cache.get(1), Some(100));
        assert_eq!(cache.get(2), Some(20));
    }

    #[test]
    fn test_get_makes_recent() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 10);
        cache.put(2, 20);
        cache.get(1);
        cache.put(3, 30);
        assert_eq!(cache.get(1), Some(10));
        assert_eq!(cache.get(2), None);
    }

    #[test]
    fn test_capacity_one() {
        let mut cache = LRUCache::new(1);
        cache.put(1, 10);
        cache.put(2, 20);
        assert_eq!(cache.get(1), None);
        assert_eq!(cache.get(2), Some(20));
    }

    #[test]
    fn test_contains() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 10);
        assert!(cache.contains(&1));
        assert!(!cache.contains(&2));
    }

    #[test]
    fn test_size() {
        let mut cache = LRUCache::new(3);
        cache.put(1, 10);
        cache.put(2, 20);
        cache.put(3, 30);
        assert_eq!(cache.size(), 3);
        cache.put(4, 40);
        assert_eq!(cache.size(), 3);
    }

    #[test]
    fn test_empty_cache() {
        let mut cache = LRUCache::new(2);
        assert_eq!(cache.get(1), None);
        assert_eq!(cache.size(), 0);
    }

    #[test]
    fn test_zero_capacity() {
        let mut cache = LRUCache::new(0);
        cache.put(1, 10);
        assert_eq!(cache.get(1), None);
    }

    #[test]
    fn test_multiple_evictions() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 10);
        cache.put(2, 20);
        cache.put(3, 30);
        cache.put(4, 40);
        assert_eq!(cache.get(1), None);
        assert_eq!(cache.get(2), None);
        assert_eq!(cache.get(3), Some(30));
        assert_eq!(cache.get(4), Some(40));
    }

    #[test]
    fn test_put_after_get() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 10);
        cache.put(2, 20);
        cache.get(1);
        cache.put(3, 30);
        assert_eq!(cache.get(1), Some(10));
        assert_eq!(cache.get(2), None);
    }
}
