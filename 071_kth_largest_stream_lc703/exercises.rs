/// Kth Largest Element in a Stream - LeetCode 703
/// Design KthLargest class using min-heap of size k.

use std::collections::BinaryHeap;
use std::cmp::Reverse;

/// KthLargest maintains a min-heap of size k
/// The root is always the kth largest element
pub struct KthLargest {
    k: usize,
    heap: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    /// Initialize with initial array and k
    pub fn new(k: i32, mut nums: Vec<i32>) -> Self {
        let k = k as usize;
        let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

        for num in nums.drain(..) {
            if heap.len() < k {
                heap.push(Reverse(num));
            } else if num > heap.peek().unwrap().0 {
                heap.push(Reverse(num));
                heap.pop();
            }
        }

        KthLargest { k, heap }
    }

    /// Add a value and return kth largest
    pub fn add(&mut self, val: i32) -> i32 {
        if self.heap.len() < self.k {
            self.heap.push(Reverse(val));
        } else if val > self.heap.peek().unwrap().0 {
            self.heap.push(Reverse(val));
            self.heap.pop();
        }
        self.heap.peek().map(|r| r.0).unwrap_or(0)
    }
}

/// Alternative implementation without using Reverse
pub struct KthLargestAlt {
    k: usize,
    heap: BinaryHeap<i32>,
}

impl KthLargestAlt {
    pub fn new(k: i32, mut nums: Vec<i32>) -> Self {
        let k = k as usize;
        let mut heap: BinaryHeap<i32> = BinaryHeap::new();

        for num in nums.drain(..) {
            if heap.len() < k {
                heap.push(num);
            } else if num > *heap.peek().unwrap() {
                heap.push(num);
                heap.pop();
            }
        }

        KthLargestAlt { k, heap }
    }

    pub fn add(&mut self, val: i32) -> i32 {
        if self.heap.len() < self.k {
            self.heap.push(val);
        } else if val > *self.heap.peek().unwrap() {
            self.heap.push(val);
            self.heap.pop();
        }
        *self.heap.peek().unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kth_largest_basic() {
        let mut kth = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(kth.add(3), 4);
        assert_eq!(kth.add(5), 5);
        assert_eq!(kth.add(10), 5);
        assert_eq!(kth.add(9), 8);
    }

    #[test]
    fn test_kth_largest_k1() {
        let mut kth = KthLargest::new(1, vec![]);
        assert_eq!(kth.add(5), 5);
        assert_eq!(kth.add(10), 10);
        assert_eq!(kth.add(3), 10);
    }

    #[test]
    fn test_kth_largest_k2() {
        let mut kth = KthLargest::new(2, vec![0]);
        assert_eq!(kth.add(-1), -1);
        assert_eq!(kth.add(-2), -2);
        assert_eq!(kth.add(-3), -2);
        assert_eq!(kth.add(-4), -3);
    }

    #[test]
    fn test_kth_largest_empty_initial() {
        let mut kth = KthLargest::new(3, vec![]);
        assert_eq!(kth.add(5), 5);
        assert_eq!(kth.add(10), 10);
    }

    #[test]
    fn test_kth_largest_many_adds() {
        let mut kth = KthLargest::new(4, vec![4, 5, 8, 2, 1, 6, 7]);
        assert_eq!(kth.add(3), 4);
        assert_eq!(kth.add(9), 5);
        assert_eq!(kth.add(8), 6);
    }

    #[test]
    fn test_kth_largest_alt_basic() {
        let mut kth = KthLargestAlt::new(3, vec![4, 5, 8, 2]);
        assert_eq!(kth.add(3), 4);
        assert_eq!(kth.add(5), 5);
        assert_eq!(kth.add(10), 5);
        assert_eq!(kth.add(9), 8);
    }

    #[test]
    fn test_kth_largest_alt_k1() {
        let mut kth = KthLargestAlt::new(1, vec![]);
        assert_eq!(kth.add(5), 5);
        assert_eq!(kth.add(10), 10);
    }

    #[test]
    fn test_both_same_result() {
        let mut kth1 = KthLargest::new(3, vec![4, 5, 8, 2]);
        let mut kth2 = KthLargestAlt::new(3, vec![4, 5, 8, 2]);

        let inputs = vec![3, 5, 10, 9, 7];
        for val in inputs {
            assert_eq!(kth1.add(val), kth2.add(val));
        }
    }

    #[test]
    fn test_kth_largest_decreasing() {
        let mut kth = KthLargest::new(3, vec![7, 6, 5, 4, 3, 2, 1]);
        assert_eq!(kth.add(8), 5);
        assert_eq!(kth.add(9), 6);
    }

    #[test]
    fn test_kth_largest_increasing() {
        let mut kth = KthLargest::new(3, vec![1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(kth.add(0), 3);
        assert_eq!(kth.add(8), 5);
    }

    #[test]
    fn test_kth_largest_duplicates() {
        let mut kth = KthLargest::new(2, vec![5, 5, 5, 5]);
        assert_eq!(kth.add(5), 5);
        assert_eq!(kth.add(5), 5);
    }

    #[test]
    fn test_kth_largest_negative() {
        let mut kth = KthLargest::new(2, vec![-5, -3, -2, -4]);
        assert_eq!(kth.add(-1), -2);
        assert_eq!(kth.add(-6), -4);
    }

    #[test]
    fn test_kth_largest_large_k() {
        let mut kth = KthLargest::new(10, vec![1; 20]);
        assert_eq!(kth.add(0), 0);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("071_kth_largest_stream_lc703 exercises - run tests with cargo test");
}
