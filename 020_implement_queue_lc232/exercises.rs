//! Implement Queue Using Stacks Exercises (LeetCode #232)
//!
//! This module contains exercises for implementing Queue with Stacks.

use std::collections::{BinaryHeap, VecDeque};

// ============================================================================
// MyQueue definition
// ============================================================================

#[derive(Default)]
pub struct MyQueue {
    in_stack: Vec<i32>,
    out_stack: Vec<i32>,
}

impl MyQueue {
    pub fn new() -> Self {
        MyQueue {
            in_stack: Vec::new(),
            out_stack: Vec::new(),
        }
    }

    pub fn push(&mut self, x: i32) {
        self.in_stack.push(x);
    }

    pub fn pop(&mut self) -> i32 {
        if self.out_stack.is_empty() {
            self.transfer();
        }
        self.out_stack.pop().unwrap()
    }

    pub fn peek(&self) -> i32 {
        if self.out_stack.is_empty() {
            self.in_stack[0]
        } else {
            self.out_stack[self.out_stack.len() - 1]
        }
    }

    pub fn empty(&self) -> bool {
        self.in_stack.is_empty() && self.out_stack.is_empty()
    }

    fn transfer(&mut self) {
        while let Some(x) = self.in_stack.pop() {
            self.out_stack.push(x);
        }
    }
}

// ============================================================================
// Exercise 1: Basic Queue Operations
// ============================================================================

/// Use the MyQueue struct above to implement FIFO queue operations.
///
/// Time Complexity: O(1) amortized
/// Space Complexity: O(n)
pub fn queue_operations() {
    todo!("Use MyQueue implementation")
}

// ============================================================================
// Exercise 2: Implement Stack Using Queues (LC 225)
// ============================================================================

/// Implement a stack using two queues.
///
/// Time Complexity: O(1) for push, O(n) for pop
/// Space Complexity: O(n)
#[derive(Default)]
pub struct MyStack {
    // Add your data structures
}

impl MyStack {
    pub fn new() -> Self {
        todo!("Initialize")
    }

    pub fn push(&mut self, x: i32) {
        todo!("Push element")
    }

    pub fn pop(&mut self) -> i32 {
        todo!("Pop element")
    }

    pub fn top(&self) -> i32 {
        todo!("Return top element")
    }

    pub fn empty(&self) -> bool {
        todo!("Check if empty")
    }
}

// ============================================================================
// Exercise 3: Queue with Max (Similar to Max Stack)
// ============================================================================

/// Implement a queue that also supports get_max in O(1).
///
/// Time Complexity: O(1) for all operations
/// Space Complexity: O(n)
#[derive(Default)]
pub struct MaxQueue {
    // Add your data structures
}

impl MaxQueue {
    pub fn new() -> Self {
        todo!("Initialize MaxQueue")
    }

    pub fn push(&mut self, x: i32) {
        todo!("Push element")
    }

    pub fn pop(&mut self) -> i32 {
        todo!("Pop element")
    }

    pub fn max(&self) -> i32 {
        todo!("Return maximum element")
    }
}

// ============================================================================
// Exercise 4: Moving Average
// ============================================================================

/// Implement a moving average data structure.
///
/// Time Complexity: O(1)
/// Space Complexity: O(window)
pub struct MovingAverage {
    // Add your data structures
}

impl MovingAverage {
    pub fn new(size: usize) -> Self {
        todo!("Initialize with window size")
    }

    pub fn next(&mut self, val: f64) -> f64 {
        todo!("Add value and return average")
    }
}

// ============================================================================
// Exercise 5: BFS Using Queue
// ============================================================================

/// Implement breadth-first search traversal using queue.
///
/// Time Complexity: O(n)
/// Space Complexity: O(n)
pub fn bfs_traversal(graph: &[Vec<i32>], start: usize) -> Vec<i32> {
    todo!("Use queue for BFS")
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Exercise 1: Basic Queue Tests
    #[test]
    fn test_queue_basic() {
        let mut q = MyQueue::new();
        q.push(1);
        q.push(2);
        assert_eq!(q.peek(), 1);
        assert_eq!(q.pop(), 1);
        assert_eq!(q.pop(), 2);
        assert!(q.empty());
    }

    #[test]
    fn test_queue_empty() {
        let q = MyQueue::new();
        assert!(q.empty());
    }

    #[test]
    fn test_queue_single() {
        let mut q = MyQueue::new();
        q.push(5);
        assert_eq!(q.peek(), 5);
        assert_eq!(q.pop(), 5);
        assert!(q.empty());
    }

    #[test]
    fn test_queue_multiple() {
        let mut q = MyQueue::new();
        for i in 1..=5 {
            q.push(i);
        }
        for i in 1..=5 {
            assert_eq!(q.pop(), i);
        }
        assert!(q.empty());
    }

    // Exercise 2: Stack Tests
    #[test]
    fn test_stack_basic() {
        let mut stack = MyStack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.top(), 2);
        assert_eq!(stack.pop(), 2);
        assert_eq!(stack.pop(), 1);
        assert!(stack.empty());
    }

    #[test]
    fn test_stack_empty() {
        let stack = MyStack::new();
        assert!(stack.empty());
    }

    // Exercise 3: Max Queue Tests
    #[test]
    fn test_max_queue_basic() {
        let mut q = MaxQueue::new();
        q.push(1);
        q.push(3);
        q.push(2);
        assert_eq!(q.max(), 3);
        assert_eq!(q.pop(), 1);
        assert_eq!(q.max(), 3);
    }

    // Exercise 4: Moving Average Tests
    #[test]
    fn test_moving_average_basic() {
        let mut avg = MovingAverage::new(3);
        assert!((avg.next(1.0) - 1.0).abs() < 1e-5);
        assert!((avg.next(2.0) - 1.5).abs() < 1e-5);
        assert!((avg.next(3.0) - 2.0).abs() < 1e-5);
    }

    // Exercise 5: BFS Tests
    #[test]
    fn test_bfs_basic() {
        let graph = vec![vec![1, 2], vec![2], vec![]];
        let result = bfs_traversal(&graph, 0);
        assert_eq!(result, vec![0, 1, 2]);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("020_implement_queue_lc232 exercises - run tests with cargo test");
}
