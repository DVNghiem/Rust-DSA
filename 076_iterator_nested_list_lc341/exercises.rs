//! Iterator for Nested List - LeetCode 341
//!
//! This module contains exercises to practice flattening nested structures.

use std::collections::VecDeque;

/// Represents a nested integer structure
#[derive(PartialEq, Clone, Debug)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

/// NestedIterator implementation using a stack
#[derive(Clone, Debug)]
pub struct NestedIterator {
    stack: VecDeque<<Vec<NestedInteger> as IntoIterator>::IntoIter>,
}

impl NestedIterator {
    pub fn new(nested_list: Vec<NestedInteger>) -> Self {
        let mut iterator = NestedIterator {
            stack: VecDeque::new(),
        };
        iterator.stack.push_back(nested_list.into_iter());
        iterator
    }

    pub fn next(&mut self) -> Option<i32> {
        if !self.has_next() {
            return None;
        }

        loop {
            let current_iter = self.stack.back_mut()?;

            match current_iter.next() {
                Some(NestedInteger::Int(val)) => return Some(val),
                Some(NestedInteger::List(list)) => {
                    if !list.is_empty() {
                        self.stack.push_back(list.into_iter());
                    }
                }
                None => {
                    self.stack.pop_back();
                    if self.stack.is_empty() {
                        return None;
                    }
                }
            }
        }
    }

    pub fn has_next(&mut self) -> bool {
        while let Some(current_iter) = self.stack.back_mut() {
            match current_iter.next() {
                Some(NestedInteger::Int(_)) => return true,
                Some(NestedInteger::List(list)) => {
                    if !list.is_empty() {
                        self.stack.push_back(list.into_iter());
                    }
                }
                None => {
                    self.stack.pop_back();
                }
            }
        }
        false
    }
}

fn main() {
    println!("Run `cargo test` to test your solutions!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_nested_iterator() {
        let nested_list = vec![
            NestedInteger::Int(1),
            NestedInteger::Int(2),
        ];
        let mut iterator = NestedIterator::new(nested_list);
        assert_eq!(iterator.next(), Some(1));
        assert_eq!(iterator.next(), Some(2));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_single_nested_list() {
        let nested_list = vec![
            NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(2)]),
        ];
        let mut iterator = NestedIterator::new(nested_list);
        assert_eq!(iterator.next(), Some(1));
        assert_eq!(iterator.next(), Some(2));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_multiple_nested_lists() {
        let nested_list = vec![
            NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(2)]),
            NestedInteger::List(vec![NestedInteger::Int(3), NestedInteger::Int(4)]),
        ];
        let mut iterator = NestedIterator::new(nested_list);
        assert_eq!(iterator.next(), Some(1));
        assert_eq!(iterator.next(), Some(2));
        assert_eq!(iterator.next(), Some(3));
        assert_eq!(iterator.next(), Some(4));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_deeply_nested() {
        let nested_list = vec![
            NestedInteger::List(vec![
                NestedInteger::List(vec![NestedInteger::Int(1)]),
            ]),
        ];
        let mut iterator = NestedIterator::new(nested_list);
        assert_eq!(iterator.next(), Some(1));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_empty_list() {
        let nested_list: Vec<NestedInteger> = vec![];
        let mut iterator = NestedIterator::new(nested_list);
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_empty_nested_list() {
        let nested_list = vec![NestedInteger::List(vec![])];
        let mut iterator = NestedIterator::new(nested_list);
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_has_next_before_next() {
        let nested_list = vec![
            NestedInteger::Int(1),
            NestedInteger::Int(2),
        ];
        let mut iterator = NestedIterator::new(nested_list);
        assert!(iterator.has_next());
        assert_eq!(iterator.next(), Some(1));
        assert!(iterator.has_next());
        assert_eq!(iterator.next(), Some(2));
        assert!(!iterator.has_next());
    }

    #[test]
    fn test_mixed_integers_and_lists() {
        let nested_list = vec![
            NestedInteger::Int(1),
            NestedInteger::List(vec![NestedInteger::Int(2), NestedInteger::Int(3)]),
            NestedInteger::Int(4),
        ];
        let mut iterator = NestedIterator::new(nested_list);
        assert_eq!(iterator.next(), Some(1));
        assert_eq!(iterator.next(), Some(2));
        assert_eq!(iterator.next(), Some(3));
        assert_eq!(iterator.next(), Some(4));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_multiple_empty_lists() {
        let nested_list = vec![
            NestedInteger::List(vec![]),
            NestedInteger::Int(1),
            NestedInteger::List(vec![]),
        ];
        let mut iterator = NestedIterator::new(nested_list);
        assert_eq!(iterator.next(), Some(1));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_single_integer() {
        let nested_list = vec![NestedInteger::Int(42)];
        let mut iterator = NestedIterator::new(nested_list);
        assert_eq!(iterator.next(), Some(42));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_interleaved_nested_levels() {
        let nested_list = vec![
            NestedInteger::Int(1),
            NestedInteger::List(vec![
                NestedInteger::Int(2),
                NestedInteger::List(vec![NestedInteger::Int(3)]),
                NestedInteger::Int(4),
            ]),
            NestedInteger::Int(5),
        ];
        let mut iterator = NestedIterator::new(nested_list);
        assert_eq!(iterator.next(), Some(1));
        assert_eq!(iterator.next(), Some(2));
        assert_eq!(iterator.next(), Some(3));
        assert_eq!(iterator.next(), Some(4));
        assert_eq!(iterator.next(), Some(5));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_sequential_lists() {
        let nested_list = vec![
            NestedInteger::List(vec![NestedInteger::Int(1)]),
            NestedInteger::List(vec![NestedInteger::Int(2)]),
            NestedInteger::List(vec![NestedInteger::Int(3)]),
        ];
        let mut iterator = NestedIterator::new(nested_list);
        assert_eq!(iterator.next(), Some(1));
        assert_eq!(iterator.next(), Some(2));
        assert_eq!(iterator.next(), Some(3));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_list_with_multiple_integers() {
        let nested_list = vec![
            NestedInteger::List(vec![
                NestedInteger::Int(1),
                NestedInteger::Int(2),
                NestedInteger::Int(3),
            ]),
        ];
        let mut iterator = NestedIterator::new(nested_list);
        assert_eq!(iterator.next(), Some(1));
        assert_eq!(iterator.next(), Some(2));
        assert_eq!(iterator.next(), Some(3));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_negative_integers() {
        let nested_list = vec![
            NestedInteger::Int(-1),
            NestedInteger::Int(-2),
            NestedInteger::Int(-3),
        ];
        let mut iterator = NestedIterator::new(nested_list);
        assert_eq!(iterator.next(), Some(-1));
        assert_eq!(iterator.next(), Some(-2));
        assert_eq!(iterator.next(), Some(-3));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_has_next_after_exhaustion() {
        let nested_list = vec![NestedInteger::Int(1)];
        let mut iterator = NestedIterator::new(nested_list);
        assert_eq!(iterator.next(), Some(1));
        assert!(!iterator.has_next());
    }

    #[test]
    fn test_double_nested() {
        let nested_list = vec![
            NestedInteger::List(vec![
                NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(2)]),
                NestedInteger::Int(3),
            ]),
        ];
        let mut iterator = NestedIterator::new(nested_list);
        assert_eq!(iterator.next(), Some(1));
        assert_eq!(iterator.next(), Some(2));
        assert_eq!(iterator.next(), Some(3));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_complex_nesting() {
        let nested_list = vec![
            NestedInteger::Int(1),
            NestedInteger::List(vec![
                NestedInteger::List(vec![
                    NestedInteger::Int(2),
                    NestedInteger::List(vec![NestedInteger::Int(3)]),
                ]),
                NestedInteger::Int(4),
            ]),
            NestedInteger::List(vec![NestedInteger::Int(5)]),
        ];
        let mut iterator = NestedIterator::new(nested_list);
        assert_eq!(iterator.next(), Some(1));
        assert_eq!(iterator.next(), Some(2));
        assert_eq!(iterator.next(), Some(3));
        assert_eq!(iterator.next(), Some(4));
        assert_eq!(iterator.next(), Some(5));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_zero_integer() {
        let nested_list = vec![NestedInteger::Int(0)];
        let mut iterator = NestedIterator::new(nested_list);
        assert_eq!(iterator.next(), Some(0));
        assert_eq!(iterator.next(), None);
    }
}