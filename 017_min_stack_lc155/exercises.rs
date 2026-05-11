//! Min Stack Exercises (LeetCode #155)
//!
//! This module contains exercises for the Min Stack problem.

use std::collections::{BinaryHeap, VecDeque};

// ============================================================================
// MinStack definition
// ============================================================================

#[derive(Default)]
pub struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {
    pub fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    pub fn push(&mut self, val: i32) {
        self.stack.push(val);
        let min = self.min_stack.last().copied().unwrap_or(i32::MAX).min(val);
        self.min_stack.push(min);
    }

    pub fn pop(&mut self) {
        self.stack.pop();
        self.min_stack.pop();
    }

    pub fn top(&self) -> i32 {
        self.stack.last().copied().unwrap_or(0)
    }

    pub fn get_min(&self) -> i32 {
        self.min_stack.last().copied().unwrap_or(0)
    }
}

// ============================================================================
// Exercise 1: Min Stack Operations
// ============================================================================

/// Implement MinStack with push, pop, top, get_min in O(1).
///
/// Time Complexity: O(1) for all operations
/// Space Complexity: O(n)
pub fn min_stack_operations() {
    todo!("Use MinStack struct above")
}

// ============================================================================
// Exercise 2: Max Stack
// ============================================================================

/// Implement a MaxStack that supports get_max in O(1).
///
/// Time Complexity: O(1) for all operations
/// Space Complexity: O(n)
#[derive(Default)]
pub struct MaxStack {
    stack: Vec<i32>,
    max_stack: Vec<i32>,
}

impl MaxStack {
    pub fn new() -> Self {
        todo!("Implement MaxStack")
    }

    pub fn push(&mut self, val: i32) {
        todo!("Push to both stacks")
    }

    pub fn pop(&mut self) {
        todo!("Pop from both stacks")
    }

    pub fn top(&self) -> i32 {
        todo!("Return top element")
    }

    pub fn get_max(&self) -> i32 {
        todo!("Return maximum element")
    }
}

// ============================================================================
// Exercise 3: Stack with All Operations O(1)
// ============================================================================

/// Implement a stack that supports get_min, get_max, and get_median in O(1).
///
/// Time Complexity: O(1) for all operations
/// Space Complexity: O(n)
pub struct AdvancedStack {
    // Add your data structures here
}

impl AdvancedStack {
    pub fn new() -> Self {
        todo!("Initialize data structures")
    }

    pub fn push(&mut self, val: i32) {
        todo!("Push element")
    }

    pub fn pop(&mut self) -> Option<i32> {
        todo!("Pop and return element")
    }

    pub fn top(&self) -> Option<i32> {
        todo!("Return top element")
    }

    pub fn get_min(&self) -> Option<i32> {
        todo!("Return minimum element")
    }

    pub fn get_max(&self) -> Option<i32> {
        todo!("Return maximum element")
    }

    pub fn get_median(&self) -> Option<f64> {
        todo!("Return median element")
    }
}

// ============================================================================
// Exercise 4: Min Stack with Single Stack
// ============================================================================

/// Implement MinStack using only one stack (use clever trick).
///
/// Time Complexity: O(1) for all operations
/// Space Complexity: O(1) if we can modify values
pub struct MinStackSingle {
    stack: Vec<i32>,
}

impl MinStackSingle {
    pub fn new() -> Self {
        todo!("Initialize")
    }

    pub fn push(&mut self, val: i32) {
        todo!("Push with encoding trick")
    }

    pub fn pop(&mut self) {
        todo!("Pop")
    }

    pub fn top(&self) -> i32 {
        todo!("Return top")
    }

    pub fn get_min(&self) -> i32 {
        todo!("Return minimum")
    }
}

// ============================================================================
// Exercise 5: Stack with Product
// ============================================================================

/// Implement a stack that supports get_product (product of all elements).
///
/// Time Complexity: O(1) for all operations
/// Space Complexity: O(n)
#[derive(Default)]
pub struct ProductStack {
    stack: Vec<i32>,
    product_stack: Vec<i64>,
}

impl ProductStack {
    pub fn new() -> Self {
        todo!("Initialize ProductStack")
    }

    pub fn push(&mut self, val: i32) {
        todo!("Push and track product")
    }

    pub fn pop(&mut self) {
        todo!("Pop")
    }

    pub fn top(&self) -> i32 {
        todo!("Return top")
    }

    pub fn get_product(&self) -> i64 {
        todo!("Return product of all elements")
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Exercise 1: Basic MinStack Tests
    #[test]
    fn test_min_stack_basic() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        assert_eq!(min_stack.get_min(), -3);
        min_stack.pop();
        assert_eq!(min_stack.top(), 0);
        assert_eq!(min_stack.get_min(), -2);
    }

    #[test]
    fn test_min_stack_empty() {
        let min_stack = MinStack::new();
        assert_eq!(min_stack.top(), 0);
        assert_eq!(min_stack.get_min(), 0);
    }

    #[test]
    fn test_min_stack_single() {
        let mut min_stack = MinStack::new();
        min_stack.push(5);
        assert_eq!(min_stack.top(), 5);
        assert_eq!(min_stack.get_min(), 5);
    }

    // Exercise 2: MaxStack Tests
    #[test]
    fn test_max_stack_basic() {
        let mut max_stack = MaxStack::new();
        max_stack.push(1);
        max_stack.push(3);
        max_stack.push(2);
        assert_eq!(max_stack.get_max(), 3);
        max_stack.pop();
        assert_eq!(max_stack.top(), 3);
        assert_eq!(max_stack.get_max(), 3);
    }

    #[test]
    fn test_max_stack_empty() {
        let max_stack = MaxStack::new();
        assert_eq!(max_stack.top(), 0);
        assert_eq!(max_stack.get_max(), 0);
    }

    // Exercise 3: Advanced Stack Tests
    #[test]
    fn test_advanced_stack_basic() {
        let mut stack = AdvancedStack::new();
        stack.push(5);
        stack.push(2);
        stack.push(8);
        stack.push(1);
        assert_eq!(stack.get_min(), Some(1));
        assert_eq!(stack.get_max(), Some(8));
    }

    // Exercise 4: Single Stack Tests
    #[test]
    fn test_single_stack_basic() {
        let mut min_stack = MinStackSingle::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        assert_eq!(min_stack.get_min(), -3);
        min_stack.pop();
        assert_eq!(min_stack.top(), 0);
        assert_eq!(min_stack.get_min(), -2);
    }

    // Exercise 5: Product Stack Tests
    #[test]
    fn test_product_stack_basic() {
        let mut stack = ProductStack::new();
        stack.push(2);
        stack.push(3);
        stack.push(4);
        assert_eq!(stack.get_product(), 24);
        stack.pop();
        assert_eq!(stack.top(), 3);
        assert_eq!(stack.get_product(), 6);
    }

    #[test]
    fn test_product_stack_empty() {
        let stack = ProductStack::new();
        assert_eq!(stack.get_product(), 1);
    }
}
// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("$name exercises - run tests with cargo test");
}
