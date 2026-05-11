//! Exercises for Design HashSet (LeetCode 705)
//!
//! # Topics Covered
//! - Hash function design
//! - Set operations (add, remove, contains)
//! - O(1) average time complexity
//! - Collision handling
//!
//! # Difficulty: Easy

const INITIAL_CAPACITY: usize = 1000;
const LOAD_FACTOR_THRESHOLD: f64 = 0.75;

/// ListNode for separate chaining in HashSet
#[derive(Debug, Clone)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

/// MyHashSet implements a hash set with O(1) average operations
#[derive(Debug)]
pub struct MyHashSet {
    buckets: Vec<Option<Box<ListNode>>>,
    size: usize,
}

impl MyHashSet {
    /// Creates a new empty HashSet
    pub fn new() -> Self {
        MyHashSet {
            buckets: vec![None; INITIAL_CAPACITY],
            size: 0,
        }
    }

    /// Creates a HashSet with specified initial capacity
    pub fn with_capacity(capacity: usize) -> Self {
        MyHashSet {
            buckets: vec![None; capacity],
            size: 0,
        }
    }

    /// Hash function to compute bucket index
    fn hash(&self, val: i32) -> usize {
        val.unsigned_abs() as usize % self.buckets.len()
    }

    /// Returns true if the HashSet needs rehashing
    fn should_rehash(&self) -> bool {
        if self.buckets.is_empty() {
            return true;
        }
        let load_factor = self.size as f64 / self.buckets.len() as f64;
        load_factor > LOAD_FACTOR_THRESHOLD
    }

    /// Inserts value into the set (no duplicates)
    pub fn add(&mut self, val: i32) {
        if self.contains(val) {
            return; // Already exists
        }

        if self.should_rehash() {
            self.rehash();
        }

        let index = self.hash(val);
        let bucket = &mut self.buckets[index];

        // Add at head of bucket
        let new_node = Box::new(ListNode { val, next: bucket.take() });
        *bucket = Some(new_node);
        self.size += 1;
    }

    /// Returns true if value exists in the set
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

    /// Removes value from the set (no-op if not present)
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
            if let Some(ref mut next_node) = node.next {
                if next_node.val == val {
                    node.next = next_node.next.take();
                    self.size -= 1;
                    return;
                }
            }
            current = node.next;
        }
    }

    /// Returns the number of elements
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns true if the set is empty
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Clears all elements from the set
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

        for bucket in self.buckets.iter_mut() {
            let mut current = bucket.take();
            while let Some(mut node) = current {
                let next = node.next.take();
                let index = (node.val as usize) % new_buckets.len();
                node.next = new_buckets[index].take();
                new_buckets[index] = Some(node);
                current = next;
            }
        }

        self.buckets = new_buckets;
    }
}

impl Default for MyHashSet {
    fn default() -> Self {
        Self::new()
    }
}

/// Alternative implementation using Vec<Vec<i32>> for simpler approach
#[derive(Debug)]
pub struct SimpleHashSet {
    buckets: Vec<Vec<i32>>,
    size: usize,
}

impl SimpleHashSet {
    pub fn new() -> Self {
        SimpleHashSet {
            buckets: vec![Vec::new(); 1000],
            size: 0,
        }
    }

    fn hash(&self, val: i32) -> usize {
        val.unsigned_abs() as usize % self.buckets.len()
    }

    pub fn add(&mut self, val: i32) {
        if self.contains(val) {
            return;
        }
        let index = self.hash(val);
        self.buckets[index].push(val);
        self.size += 1;
    }

    pub fn contains(&self, val: i32) -> bool {
        let index = self.hash(val);
        self.buckets[index].iter().any(|&v| v == val)
    }

    pub fn remove(&mut self, val: i32) {
        let index = self.hash(val);
        let bucket = &mut self.buckets[index];
        if let Some(pos) = bucket.iter().position(|&v| v == val) {
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

impl Default for SimpleHashSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_single() {
        let mut set = MyHashSet::new();
        set.add(1);
        assert!(set.contains(1));
        assert_eq!(set.size(), 1);
    }

    #[test]
    fn test_add_multiple() {
        let mut set = MyHashSet::new();
        set.add(1);
        set.add(2);
        set.add(3);
        assert!(set.contains(1));
        assert!(set.contains(2));
        assert!(set.contains(3));
        assert_eq!(set.size(), 3);
    }

    #[test]
    fn test_add_duplicate() {
        let mut set = MyHashSet::new();
        set.add(1);
        set.add(1);
        set.add(1);
        assert!(set.contains(1));
        assert_eq!(set.size(), 1); // Should remain 1
    }

    #[test]
    fn test_remove_existing() {
        let mut set = MyHashSet::new();
        set.add(1);
        set.add(2);
        assert_eq!(set.size(), 2);
        set.remove(1);
        assert!(!set.contains(1));
        assert!(set.contains(2));
        assert_eq!(set.size(), 1);
    }

    #[test]
    fn test_remove_nonexistent() {
        let mut set = MyHashSet::new();
        set.add(1);
        set.remove(99);
        assert_eq!(set.size(), 1);
        assert!(set.contains(1));
    }

    #[test]
    fn test_remove_from_empty() {
        let mut set = MyHashSet::new();
        set.remove(1);
        assert!(set.is_empty());
    }

    #[test]
    fn test_empty_set_contains() {
        let set = MyHashSet::new();
        assert!(!set.contains(1));
        assert!(!set.contains(100));
    }

    #[test]
    fn test_negative_values() {
        let mut set = MyHashSet::new();
        set.add(-1);
        set.add(-2);
        set.add(-3);
        assert!(set.contains(-1));
        assert!(set.contains(-2));
        assert!(set.contains(-3));
        assert_eq!(set.size(), 3);
        set.remove(-2);
        assert!(!set.contains(-2));
        assert_eq!(set.size(), 2);
    }

    #[test]
    fn test_zero_value() {
        let mut set = MyHashSet::new();
        set.add(0);
        assert!(set.contains(0));
        set.remove(0);
        assert!(!set.contains(0));
    }

    #[test]
    fn test_large_values() {
        let mut set = MyHashSet::new();
        set.add(1000000);
        set.add(2000000);
        assert!(set.contains(1000000));
        assert!(set.contains(2000000));
    }

    #[test]
    fn test_collision_handling() {
        let mut set = MyHashSet::with_capacity(10);
        // Keys that will collide (same bucket index)
        for i in (0..50).step_by(10) {
            set.add(i);
        }
        for i in (0..50).step_by(10) {
            assert!(set.contains(i));
        }
        assert_eq!(set.size(), 5);
    }

    #[test]
    fn test_many_operations() {
        let mut set = MyHashSet::new();
        // Add many elements
        for i in 0..1000 {
            set.add(i);
        }
        assert_eq!(set.size(), 1000);
        // Verify all
        for i in 0..1000 {
            assert!(set.contains(i));
        }
        // Remove half
        for i in 0..500 {
            set.remove(i);
        }
        assert_eq!(set.size(), 500);
        // Verify remaining
        for i in 500..1000 {
            assert!(set.contains(i));
        }
        // Check removed are gone
        for i in 0..500 {
            assert!(!set.contains(i));
        }
    }

    #[test]
    fn test_clear() {
        let mut set = MyHashSet::new();
        set.add(1);
        set.add(2);
        set.add(3);
        assert_eq!(set.size(), 3);
        set.clear();
        assert!(set.is_empty());
        assert!(!set.contains(1));
        assert!(!set.contains(2));
        assert!(!set.contains(3));
    }

    #[test]
    fn test_is_empty() {
        let mut set = MyHashSet::new();
        assert!(set.is_empty());
        set.add(1);
        assert!(!set.is_empty());
        set.remove(1);
        assert!(set.is_empty());
    }

    #[test]
    fn test_remove_head_of_bucket() {
        let mut set = MyHashSet::with_capacity(10);
        set.add(0);   // collides with 10, 20
        set.add(10);
        set.add(20);
        set.remove(0);
        assert!(!set.contains(0));
        assert!(set.contains(10));
        assert!(set.contains(20));
    }

    #[test]
    fn test_remove_middle_of_bucket() {
        let mut set = MyHashSet::with_capacity(10);
        set.add(0);
        set.add(10);
        set.add(20);
        set.remove(10);
        assert!(set.contains(0));
        assert!(!set.contains(10));
        assert!(set.contains(20));
    }

    #[test]
    fn test_rehash_triggers() {
        let mut set = MyHashSet::with_capacity(10);
        for i in 0..20 {
            set.add(i);
        }
        assert_eq!(set.size(), 20);
        for i in 0..20 {
            assert!(set.contains(i));
        }
    }

    // SimpleHashSet tests
    #[test]
    fn test_simple_basic() {
        let mut set = SimpleHashSet::new();
        set.add(1);
        set.add(2);
        assert!(set.contains(1));
        assert!(set.contains(2));
        assert_eq!(set.size(), 2);
    }

    #[test]
    fn test_simple_duplicate() {
        let mut set = SimpleHashSet::new();
        set.add(1);
        set.add(1);
        assert_eq!(set.size(), 1);
    }

    #[test]
    fn test_simple_remove() {
        let mut set = SimpleHashSet::new();
        set.add(1);
        set.add(2);
        set.remove(1);
        assert!(!set.contains(1));
        assert!(set.contains(2));
    }

    #[test]
    fn test_simple_clear() {
        let mut set = SimpleHashSet::new();
        set.add(1);
        set.add(2);
        set.clear();
        assert!(set.is_empty());
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("Design HashSet exercises - run tests with cargo test");
}