/// Find Median from Data Stream - LeetCode 295
/// Design MedianFinder using two heaps for O(1) median retrieval.

use std::collections::{BinaryHeap, VecDeque};
use std::cmp::Reverse;

/// MedianFinder uses two heaps:
/// - lo: max-heap for lower half (largest element at top)
/// - hi: min-heap for upper half (smallest element at top)
/// Balance: lo.size() == hi.size() or lo.size() == hi.size() + 1
/// Median is always at lo.peek()
pub struct MedianFinder {
    lo: BinaryHeap<i32>,                      // max-heap
    hi: BinaryHeap<Reverse<i32>>,            // min-heap
}

impl MedianFinder {
    pub fn new() -> Self {
        MedianFinder {
            lo: BinaryHeap::new(),
            hi: BinaryHeap::new(),
        }
    }

    pub fn add_num(&mut self, num: i32) {
        // Add to max-heap (lower half)
        self.lo.push(num);

        // Move largest of lower half to min-heap (upper half)
        if let Some(max_lo) = self.lo.pop() {
            self.hi.push(Reverse(max_lo));
        }

        // Balance: ensure lo has >= elements than hi
        if self.hi.len() > self.lo.len() {
            if let Some(min_hi) = self.hi.pop() {
                self.lo.push(min_hi.0);
            }
        }
    }

    pub fn find_median(&self) -> f64 {
        if self.lo.is_empty() {
            return 0.0;
        }

        if self.lo.len() > self.hi.len() {
            *self.lo.peek().unwrap() as f64
        } else {
            (*self.lo.peek().unwrap() as f64 + self.hi.peek().unwrap().0 as f64) / 2.0
        }
    }
}

/// Alternative: Balanced BST approach using VecDeque for order
pub struct MedianFinderBst {
    data: VecDeque<i32>,
}

impl MedianFinderBst {
    pub fn new() -> Self {
        MedianFinderBst { data: VecDeque::new() }
    }

    pub fn add_num(&mut self, num: i32) {
        // Binary search for insertion point
        let pos = self.data.iter().position(|&x| x >= num).unwrap_or(self.data.len());
        self.data.insert(pos, num);
    }

    pub fn find_median(&self) -> f64 {
        if self.data.is_empty() {
            return 0.0;
        }
        let mid = self.data.len() / 2;
        if self.data.len() % 2 == 1 {
            self.data[mid] as f64
        } else {
            (self.data[mid - 1] as f64 + self.data[mid] as f64) / 2.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median_basic() {
        let mut mf = MedianFinder::new();
        mf.add_num(1);
        mf.add_num(2);
        assert_eq!(mf.find_median(), 1.5);
    }

    #[test]
    fn test_median_three_elements() {
        let mut mf = MedianFinder::new();
        mf.add_num(1);
        mf.add_num(2);
        mf.add_num(3);
        assert_eq!(mf.find_median(), 2.0);
    }

    #[test]
    fn test_median_empty() {
        let mf = MedianFinder::new();
        assert_eq!(mf.find_median(), 0.0);
    }

    #[test]
    fn test_median_single() {
        let mut mf = MedianFinder::new();
        mf.add_num(5);
        assert_eq!(mf.find_median(), 5.0);
    }

    #[test]
    fn test_median_even_count() {
        let mut mf = MedianFinder::new();
        mf.add_num(1);
        mf.add_num(2);
        mf.add_num(3);
        mf.add_num(4);
        assert_eq!(mf.find_median(), 2.5);
    }

    #[test]
    fn test_median_odd_count() {
        let mut mf = MedianFinder::new();
        mf.add_num(1);
        mf.add_num(3);
        mf.add_num(5);
        assert_eq!(mf.find_median(), 3.0);
    }

    #[test]
    fn test_median_negative() {
        let mut mf = MedianFinder::new();
        mf.add_num(-1);
        mf.add_num(-2);
        mf.add_num(-3);
        assert_eq!(mf.find_median(), -2.0);
    }

    #[test]
    fn test_median_mixed() {
        let mut mf = MedianFinder::new();
        mf.add_num(-10);
        mf.add_num(5);
        mf.add_num(15);
        assert_eq!(mf.find_median(), 5.0);
    }

    #[test]
    fn test_median_many() {
        let mut mf = MedianFinder::new();
        for i in 1..=100 {
            mf.add_num(i);
        }
        assert_eq!(mf.find_median(), 50.5);
    }

    #[test]
    fn test_median_bst_basic() {
        let mut mf = MedianFinderBst::new();
        mf.add_num(1);
        mf.add_num(2);
        assert_eq!(mf.find_median(), 1.5);
    }

    #[test]
    fn test_median_bst_three() {
        let mut mf = MedianFinderBst::new();
        mf.add_num(1);
        mf.add_num(2);
        mf.add_num(3);
        assert_eq!(mf.find_median(), 2.0);
    }

    #[test]
    fn test_median_bst_vs_heap() {
        let mut mf1 = MedianFinder::new();
        let mut mf2 = MedianFinderBst::new();

        let inputs = vec![1, 2, 3, 4, 5, 6, 7];
        for num in inputs {
            mf1.add_num(num);
            mf2.add_num(num);
            let m1 = mf1.find_median();
            let m2 = mf2.find_median();
            assert!((m1 - m2).abs() < 0.0001);
        }
    }

    #[test]
    fn test_median_reversed_order() {
        let mut mf = MedianFinder::new();
        for i in (1..=5).rev() {
            mf.add_num(i);
        }
        assert_eq!(mf.find_median(), 3.0);
    }

    #[test]
    fn test_median_large_numbers() {
        let mut mf = MedianFinder::new();
        mf.add_num(1_000_000);
        mf.add_num(2_000_000);
        assert_eq!(mf.find_median(), 1_500_000.0);
    }

    #[test]
    fn test_median_insertion_order() {
        let mut mf = MedianFinder::new();
        mf.add_num(5);
        mf.add_num(1);
        mf.add_num(3);
        mf.add_num(2);
        mf.add_num(4);
        assert_eq!(mf.find_median(), 3.0);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("072_find_median_data_stream_lc295 exercises - run tests with cargo test");
}
