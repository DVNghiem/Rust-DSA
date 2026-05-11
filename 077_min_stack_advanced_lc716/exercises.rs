//! Exercises for Min Stack Advanced (LeetCode 716)
//!
//! # Topics Covered
//! - Stack data structure
//! - O(1) operations design
//! - Auxiliary data structures
//! - Edge case handling
//!
//! # Difficulty: Medium

/// MinStack supports all stack operations plus O(1) minimum retrieval
#[derive(Debug)]
pub struct MinStack {
    main: Vec<i32>,
    min: Vec<i32>,
}

impl MinStack {
    /// Creates a new empty MinStack
    pub fn new() -> Self {
        MinStack {
            main: Vec::new(),
            min: Vec::new(),
        }
    }

    /// Pushes element onto the stack
    pub fn push(&mut self, val: i32) {
        self.main.push(val);
        if self.min.is_empty() || val <= *self.min.last().unwrap() {
            self.min.push(val);
        }
    }

    /// Removes and returns the top element
    pub fn pop(&mut self) -> Option<i32> {
        let val = self.main.pop();
        if let Some(v) = val {
            if !self.min.is_empty() && v == *self.min.last().unwrap() {
                self.min.pop();
            }
            return Some(v);
        }
        None
    }

    /// Returns the top element without removing it
    pub fn top(&self) -> Option<i32> {
        self.main.last().copied()
    }

    /// Returns the minimum element in the stack
    pub fn get_min(&self) -> Option<i32> {
        self.min.last().copied()
    }

    /// Returns the number of elements in the stack
    pub fn len(&self) -> usize {
        self.main.len()
    }

    /// Returns true if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.main.is_empty()
    }
}

impl Default for MinStack {
    fn default() -> Self {
        Self::new()
    }
}

/// Alternative implementation using single stack with pairs
/// Each element stores (value, min_at_this_point)
#[derive(Debug)]
pub struct MinStackSingle {
    stack: Vec<(i32, i32)>,
}

impl MinStackSingle {
    pub fn new() -> Self {
        MinStackSingle { stack: Vec::new() }
    }

    pub fn push(&mut self, val: i32) {
        let current_min = self.stack.last().map(|(_, m)| *m).unwrap_or(i32::MAX);
        let new_min = val.min(current_min);
        self.stack.push((val, new_min));
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.stack.pop().map(|(v, _)| v)
    }

    pub fn top(&self) -> Option<i32> {
        self.stack.last().map(|(v, _)| *v)
    }

    pub fn get_min(&self) -> Option<i32> {
        self.stack.last().map(|(_, m)| m).copied()
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

impl Default for MinStackSingle {
    fn default() -> Self {
        Self::new()
    }
}

/// MaxStack implementation - tracks maximum instead of minimum
#[derive(Debug)]
pub struct MaxStack {
    main: Vec<i32>,
    max: Vec<i32>,
}

impl MaxStack {
    pub fn new() -> Self {
        MaxStack {
            main: Vec::new(),
            max: Vec::new(),
        }
    }

    pub fn push(&mut self, val: i32) {
        self.main.push(val);
        if self.max.is_empty() || val >= *self.max.last().unwrap() {
            self.max.push(val);
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        let val = self.main.pop();
        if let Some(v) = val {
            if !self.max.is_empty() && v == *self.max.last().unwrap() {
                self.max.pop();
            }
            return Some(v);
        }
        None
    }

    pub fn top(&self) -> Option<i32> {
        self.main.last().copied()
    }

    pub fn get_max(&self) -> Option<i32> {
        self.max.last().copied()
    }

    pub fn len(&self) -> usize {
        self.main.len()
    }

    pub fn is_empty(&self) -> bool {
        self.main.is_empty()
    }
}

impl Default for MaxStack {
    fn default() -> Self {
        Self::new()
    }
}

/// MinMaxStack - tracks both minimum and maximum
#[derive(Debug)]
pub struct MinMaxStack {
    main: Vec<i32>,
    min_stack: Vec<i32>,
    max_stack: Vec<i32>,
}

impl MinMaxStack {
    pub fn new() -> Self {
        MinMaxStack {
            main: Vec::new(),
            min_stack: Vec::new(),
            max_stack: Vec::new(),
        }
    }

    pub fn push(&mut self, val: i32) {
        self.main.push(val);
        if self.min_stack.is_empty() || val <= *self.min_stack.last().unwrap() {
            self.min_stack.push(val);
        }
        if self.max_stack.is_empty() || val >= *self.max_stack.last().unwrap() {
            self.max_stack.push(val);
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        let val = self.main.pop();
        if let Some(v) = val {
            if !self.min_stack.is_empty() && v == *self.min_stack.last().unwrap() {
                self.min_stack.pop();
            }
            if !self.max_stack.is_empty() && v == *self.max_stack.last().unwrap() {
                self.max_stack.pop();
            }
            return Some(v);
        }
        None
    }

    pub fn top(&self) -> Option<i32> {
        self.main.last().copied()
    }

    pub fn get_min(&self) -> Option<i32> {
        self.min_stack.last().copied()
    }

    pub fn get_max(&self) -> Option<i32> {
        self.max_stack.last().copied()
    }

    pub fn len(&self) -> usize {
        self.main.len()
    }

    pub fn is_empty(&self) -> bool {
        self.main.is_empty()
    }
}

impl Default for MinMaxStack {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_single_element() {
        let mut stack = MinStack::new();
        stack.push(5);
        assert_eq!(stack.get_min(), Some(5));
        assert_eq!(stack.top(), Some(5));
        assert_eq!(stack.len(), 1);
    }

    #[test]
    fn test_push_multiple_elements() {
        let mut stack = MinStack::new();
        stack.push(5);
        stack.push(3);
        stack.push(7);
        assert_eq!(stack.get_min(), Some(3));
        assert_eq!(stack.top(), Some(7));
    }

    #[test]
    fn test_pop_basic() {
        let mut stack = MinStack::new();
        stack.push(5);
        stack.push(3);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.get_min(), Some(5));
    }

    #[test]
    fn test_pop_maintains_correct_min() {
        let mut stack = MinStack::new();
        stack.push(5);
        stack.push(3);
        stack.push(7);
        assert_eq!(stack.pop(), Some(7));
        assert_eq!(stack.get_min(), Some(3));
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.get_min(), Some(5));
    }

    #[test]
    fn test_pop_empty_stack() {
        let mut stack = MinStack::new();
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.top(), None);
        assert_eq!(stack.get_min(), None);
    }

    #[test]
    fn test_duplicate_minimums() {
        let mut stack = MinStack::new();
        stack.push(5);
        stack.push(3);
        stack.push(3);  // duplicate
        assert_eq!(stack.get_min(), Some(3));
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.get_min(), Some(3));
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.get_min(), Some(5));
    }

    #[test]
    fn test_alternate_push_pop() {
        let mut stack = MinStack::new();
        stack.push(10);
        assert_eq!(stack.pop(), Some(10));
        stack.push(5);
        stack.push(3);
        assert_eq!(stack.pop(), Some(3));
        stack.push(7);
        assert_eq!(stack.get_min(), Some(5));
    }

    #[test]
    fn test_negative_numbers() {
        let mut stack = MinStack::new();
        stack.push(-1);
        stack.push(-5);
        stack.push(-3);
        assert_eq!(stack.get_min(), Some(-5));
        assert_eq!(stack.pop(), Some(-3));
        assert_eq!(stack.get_min(), Some(-5));
    }

    #[test]
    fn test_all_same_values() {
        let mut stack = MinStack::new();
        stack.push(5);
        stack.push(5);
        stack.push(5);
        stack.push(5);
        assert_eq!(stack.get_min(), Some(5));
        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.get_min(), Some(5));
    }

    #[test]
    fn test_strictly_decreasing() {
        let mut stack = MinStack::new();
        stack.push(5);
        stack.push(4);
        stack.push(3);
        stack.push(2);
        stack.push(1);
        assert_eq!(stack.get_min(), Some(1));
        // Pop all and verify min updates
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.get_min(), Some(2));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.get_min(), Some(3));
    }

    #[test]
    fn test_strictly_increasing() {
        let mut stack = MinStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack.push(4);
        stack.push(5);
        assert_eq!(stack.get_min(), Some(1));
        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.get_min(), Some(1));
    }

    #[test]
    fn test_large_number_of_operations() {
        let mut stack = MinStack::new();
        for i in 0..1000 {
            stack.push(i);
        }
        assert_eq!(stack.get_min(), Some(0));
        for i in (0..1000).rev() {
            assert_eq!(stack.pop(), Some(i));
        }
        assert!(stack.is_empty());
    }

    #[test]
    fn test_is_empty() {
        let mut stack = MinStack::new();
        assert!(stack.is_empty());
        stack.push(1);
        assert!(!stack.is_empty());
        stack.pop();
        assert!(stack.is_empty());
    }

    #[test]
    fn test_len() {
        let mut stack = MinStack::new();
        assert_eq!(stack.len(), 0);
        stack.push(1);
        assert_eq!(stack.len(), 1);
        stack.push(2);
        assert_eq!(stack.len(), 2);
        stack.pop();
        assert_eq!(stack.len(), 1);
    }

    // Single stack tests
    #[test]
    fn test_single_stack_basic() {
        let mut stack = MinStackSingle::new();
        stack.push(5);
        stack.push(3);
        stack.push(7);
        assert_eq!(stack.get_min(), Some(3));
        assert_eq!(stack.pop(), Some(7));
        assert_eq!(stack.get_min(), Some(3));
    }

    #[test]
    fn test_single_stack_pop_all() {
        let mut stack = MinStackSingle::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert!(stack.is_empty());
    }

    // MaxStack tests
    #[test]
    fn test_max_stack_basic() {
        let mut stack = MaxStack::new();
        stack.push(5);
        stack.push(3);
        stack.push(7);
        assert_eq!(stack.get_max(), Some(7));
        assert_eq!(stack.pop(), Some(7));
        assert_eq!(stack.get_max(), Some(5));
    }

    #[test]
    fn test_max_stack_duplicate_max() {
        let mut stack = MaxStack::new();
        stack.push(5);
        stack.push(7);
        stack.push(7);  // duplicate
        assert_eq!(stack.get_max(), Some(7));
        assert_eq!(stack.pop(), Some(7));
        assert_eq!(stack.get_max(), Some(7));
        assert_eq!(stack.pop(), Some(7));
        assert_eq!(stack.get_max(), Some(5));
    }

    // MinMaxStack tests
    #[test]
    fn test_min_max_stack_basic() {
        let mut stack = MinMaxStack::new();
        stack.push(5);
        stack.push(3);
        stack.push(7);
        stack.push(1);
        assert_eq!(stack.get_min(), Some(1));
        assert_eq!(stack.get_max(), Some(7));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.get_min(), Some(3));
        assert_eq!(stack.get_max(), Some(7));
    }

    #[test]
    fn test_min_max_stack_empty() {
        let stack = MinMaxStack::new();
        assert_eq!(stack.get_min(), None);
        assert_eq!(stack.get_max(), None);
    }

    #[test]
    fn test_min_max_all_same() {
        let mut stack = MinMaxStack::new();
        stack.push(5);
        stack.push(5);
        stack.push(5);
        assert_eq!(stack.get_min(), Some(5));
        assert_eq!(stack.get_max(), Some(5));
        stack.pop();
        assert_eq!(stack.get_min(), Some(5));
        assert_eq!(stack.get_max(), Some(5));
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("Min Stack Advanced exercises - run tests with cargo test");
}