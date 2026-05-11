//! Exercises for Design HashMap (LeetCode 706)
//!
//! # Topics Covered
//! - Hash function design
//! - Separate chaining for collision handling
//! - Linked list operations
//! - O(1) average operations
//!
//! # Difficulty: Medium

const INITIAL_CAPACITY: usize = 1000;
const LOAD_FACTOR_THRESHOLD: f64 = 0.75;

/// ListNode for separate chaining in HashMap
#[derive(Debug, Clone)]
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

/// MyHashMap implements a hash map with O(1) average operations
#[derive(Debug)]
pub struct MyHashMap {
    buckets: Vec<Option<Box<ListNode>>>,
    size: usize,
}

impl MyHashMap {
    /// Creates a new empty HashMap
    pub fn new() -> Self {
        MyHashMap {
            buckets: vec![None; INITIAL_CAPACITY],
            size: 0,
        }
    }

    /// Creates a HashMap with specified initial capacity
    pub fn with_capacity(capacity: usize) -> Self {
        MyHashMap {
            buckets: vec![None; capacity],
            size: 0,
        }
    }

    /// Hash function to compute bucket index
    fn hash(&self, key: i32) -> usize {
        key.unsigned_abs() as usize % self.buckets.len()
    }

    /// Returns true if the HashMap needs rehashing
    fn should_rehash(&self) -> bool {
        if self.buckets.is_empty() {
            return true;
        }
        let load_factor = self.size as f64 / self.buckets.len() as f64;
        load_factor > LOAD_FACTOR_THRESHOLD
    }

    /// Inserts a key-value pair into the HashMap
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

    /// Returns the value associated with the key, or -1 if not found
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

    /// Removes the key-value pair from the HashMap
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
            if let Some(ref mut next_node) = node.next {
                if next_node.key == key {
                    node.next = next_node.next.take();
                    self.size -= 1;
                    return;
                }
            }
            current = node.next;
        }
    }

    /// Returns true if the HashMap contains the key
    pub fn contains(&self, key: i32) -> bool {
        self.get(key) != -1
    }

    /// Returns the number of key-value pairs
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns true if the HashMap is empty
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Clears all entries from the HashMap
    pub fn clear(&mut self) {
        for bucket in self.buckets.iter_mut() {
            *bucket = None;
        }
        self.size = 0;
    }

    /// Internal rehashing function
    fn rehash(&mut self) {
        let new_capacity = self.buckets.len().saturating_mul(2).max(1);
        let mut new_buckets: Vec<Option<Box<ListNode>>> = vec![None; new_capacity];

        // Reinsert all existing elements
        for bucket in self.buckets.iter_mut() {
            let mut current = bucket.take();
            while let Some(mut node) = current {
                let next = node.next.take();
                let index = (node.key as usize) % new_buckets.len();
                node.next = new_buckets[index].take();
                new_buckets[index] = Some(node);
                current = next;
            }
        }

        self.buckets = new_buckets;
    }
}

impl Default for MyHashMap {
    fn default() -> Self {
        Self::new()
    }
}

/// Alternative implementation using Vec<Vec<(K,V)>> for simpler implementation
/// This uses separate chaining with vectors instead of linked lists
#[derive(Debug)]
pub struct SimpleHashMap {
    buckets: Vec<Vec<(i32, i32)>>,
    size: usize,
}

impl SimpleHashMap {
    pub fn new() -> Self {
        SimpleHashMap {
            buckets: vec![Vec::new(); 1000],
            size: 0,
        }
    }

    fn hash(&self, key: i32) -> usize {
        key.unsigned_abs() as usize % self.buckets.len()
    }

    pub fn put(&mut self, key: i32, value: i32) {
        let index = self.hash(key);
        let bucket = &mut self.buckets[index];

        for pair in bucket.iter_mut() {
            if pair.0 == key {
                pair.1 = value;
                return;
            }
        }

        bucket.push((key, value));
        self.size += 1;
    }

    pub fn get(&self, key: i32) -> i32 {
        let index = self.hash(key);
        let bucket = &self.buckets[index];

        for pair in bucket.iter() {
            if pair.0 == key {
                return pair.1;
            }
        }
        -1
    }

    pub fn remove(&mut self, key: i32) {
        let index = self.hash(key);
        let bucket = &mut self.buckets[index];

        if let Some(pos) = bucket.iter().position(|p| p.0 == key) {
            bucket.remove(pos);
            self.size -= 1;
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

impl Default for SimpleHashMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_put_single() {
        let mut map = MyHashMap::new();
        map.put(1, 1);
        assert_eq!(map.get(1), 1);
        assert_eq!(map.size(), 1);
    }

    #[test]
    fn test_put_multiple() {
        let mut map = MyHashMap::new();
        map.put(1, 1);
        map.put(2, 2);
        map.put(3, 3);
        assert_eq!(map.get(1), 1);
        assert_eq!(map.get(2), 2);
        assert_eq!(map.get(3), 3);
        assert_eq!(map.size(), 3);
    }

    #[test]
    fn test_get_nonexistent() {
        let map = MyHashMap::new();
        assert_eq!(map.get(1), -1);
        assert_eq!(map.get(100), -1);
    }

    #[test]
    fn test_update_existing_key() {
        let mut map = MyHashMap::new();
        map.put(1, 1);
        assert_eq!(map.get(1), 1);
        map.put(1, 100);
        assert_eq!(map.get(1), 100);
        assert_eq!(map.size(), 1); // Size should remain same
    }

    #[test]
    fn test_remove_existing() {
        let mut map = MyHashMap::new();
        map.put(1, 1);
        map.put(2, 2);
        assert_eq!(map.size(), 2);
        map.remove(1);
        assert_eq!(map.get(1), -1);
        assert_eq!(map.get(2), 2);
        assert_eq!(map.size(), 1);
    }

    #[test]
    fn test_remove_nonexistent() {
        let mut map = MyHashMap::new();
        map.put(1, 1);
        map.remove(99); // Remove non-existent
        assert_eq!(map.size(), 1);
        assert_eq!(map.get(1), 1);
    }

    #[test]
    fn test_remove_from_empty_bucket() {
        let mut map = MyHashMap::new();
        map.remove(1); // Remove from empty map
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn test_empty_map_operations() {
        let map = MyHashMap::new();
        assert_eq!(map.get(1), -1);
        assert!(map.is_empty());
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn test_negative_keys() {
        let mut map = MyHashMap::new();
        map.put(-1, -10);
        map.put(-2, -20);
        assert_eq!(map.get(-1), -10);
        assert_eq!(map.get(-2), -20);
        assert_eq!(map.size(), 2);
    }

    #[test]
    fn test_key_zero() {
        let mut map = MyHashMap::new();
        map.put(0, 100);
        assert_eq!(map.get(0), 100);
        map.remove(0);
        assert_eq!(map.get(0), -1);
    }

    #[test]
    fn test_large_keys() {
        let mut map = MyHashMap::new();
        map.put(1000000, 1);
        map.put(2000000, 2);
        assert_eq!(map.get(1000000), 1);
        assert_eq!(map.get(2000000), 2);
    }

    #[test]
    fn test_collision_handling() {
        // Create map and add many keys that might collide
        let mut map = MyHashMap::with_capacity(10);
        for i in 0..20 {
            map.put(i, i * 10);
        }
        for i in 0..20 {
            assert_eq!(map.get(i), i * 10);
        }
        assert_eq!(map.size(), 20);
    }

    #[test]
    fn test_many_operations() {
        let mut map = MyHashMap::new();
        // Insert 1000 elements
        for i in 0..1000 {
            map.put(i, i);
        }
        assert_eq!(map.size(), 1000);
        // Verify all
        for i in 0..1000 {
            assert_eq!(map.get(i), i);
        }
        // Remove half
        for i in 0..500 {
            map.remove(i);
        }
        assert_eq!(map.size(), 500);
        // Verify remaining
        for i in 500..1000 {
            assert_eq!(map.get(i), i);
        }
        // Check removed are gone
        for i in 0..500 {
            assert_eq!(map.get(i), -1);
        }
    }

    #[test]
    fn test_contains() {
        let mut map = MyHashMap::new();
        map.put(1, 1);
        map.put(2, 2);
        assert!(map.contains(1));
        assert!(map.contains(2));
        assert!(!map.contains(3));
    }

    #[test]
    fn test_clear() {
        let mut map = MyHashMap::new();
        map.put(1, 1);
        map.put(2, 2);
        assert_eq!(map.size(), 2);
        map.clear();
        assert!(map.is_empty());
        assert_eq!(map.get(1), -1);
        assert_eq!(map.get(2), -1);
    }

    #[test]
    fn test_all_same_hash() {
        // Keys that all hash to same bucket
        let mut map = MyHashMap::with_capacity(100);
        // Assuming modulo 100, keys 0, 100, 200, etc. will collide
        for i in (0..1000).step_by(100) {
            map.put(i, i);
        }
        for i in (0..1000).step_by(100) {
            assert_eq!(map.get(i), i);
        }
    }

    // SimpleHashMap tests
    #[test]
    fn test_simple_hashmap_basic() {
        let mut map = SimpleHashMap::new();
        map.put(1, 1);
        map.put(2, 2);
        assert_eq!(map.get(1), 1);
        assert_eq!(map.get(2), 2);
        assert_eq!(map.size(), 2);
    }

    #[test]
    fn test_simple_hashmap_remove() {
        let mut map = SimpleHashMap::new();
        map.put(1, 1);
        map.remove(1);
        assert_eq!(map.get(1), -1);
        assert!(map.is_empty());
    }

    #[test]
    fn test_simple_hashmap_update() {
        let mut map = SimpleHashMap::new();
        map.put(1, 1);
        map.put(1, 100);
        assert_eq!(map.get(1), 100);
        assert_eq!(map.size(), 1);
    }

    #[test]
    fn test_rehash_triggers() {
        let mut map = MyHashMap::with_capacity(10);
        // Insert enough elements to trigger rehash (load factor > 0.75)
        for i in 0..20 {
            map.put(i, i);
        }
        assert_eq!(map.size(), 20);
        // Verify all elements still accessible after rehash
        for i in 0..20 {
            assert_eq!(map.get(i), i);
        }
    }

    #[test]
    fn test_remove_head_of_bucket() {
        let mut map = MyHashMap::with_capacity(10);
        map.put(0, 100);  // Will collide with 10, 20, etc.
        map.put(10, 200);
        map.put(20, 300);
        // Remove first element
        map.remove(0);
        assert_eq!(map.get(0), -1);
        assert_eq!(map.get(10), 200);
        assert_eq!(map.get(20), 300);
    }

    #[test]
    fn test_remove_middle_of_bucket() {
        let mut map = MyHashMap::with_capacity(10);
        map.put(0, 100);
        map.put(10, 200);
        map.put(20, 300);
        // Remove middle element
        map.remove(10);
        assert_eq!(map.get(0), 100);
        assert_eq!(map.get(10), -1);
        assert_eq!(map.get(20), 300);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("Design HashMap exercises - run tests with cargo test");
}