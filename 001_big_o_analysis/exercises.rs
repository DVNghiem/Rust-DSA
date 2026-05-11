//! Big O Analysis Exercises
//!
//! This module contains exercises to practice analyzing time and space complexity
//! of various algorithms.

// ============================================================================
// Exercise 1: Sum of Array Elements
// ============================================================================

/// Calculate the sum of all elements in an array.
/// analyze: What is the time complexity?
/// analyze: What is the space complexity?
pub fn sum_array(arr: &[i32]) -> i32 {
    let mut sum = 0;
    for &val in arr {
        sum += val;
    }
    sum
}

/// Calculate the sum of all elements in an array using recursion.
/// analyze: What is the time complexity?
/// analyze: What is the space complexity?
pub fn sum_array_recursive(arr: &[i32], n: usize) -> i32 {
    if n == 0 {
        return 0;
    }
    arr[n - 1] + sum_array_recursive(arr, n - 1)
}

// ============================================================================
// Exercise 2: Find Duplicates (Brute Force)
// ============================================================================

/// Check if an array contains any duplicate elements using brute force.
/// analyze: What is the time complexity?
/// analyze: What is the space complexity?
pub fn has_duplicate(arr: &[i32]) -> bool {
    for i in 0..arr.len() {
        for j in (i + 1)..arr.len() {
            if arr[i] == arr[j] {
                return true;
            }
        }
    }
    false
}

// ============================================================================
// Exercise 3: Recursive Fibonacci
// ============================================================================

/// Calculate the nth Fibonacci number using naive recursion.
/// analyze: What is the time complexity?
/// analyze: What is the space complexity?
pub fn fib(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}

/// Calculate the nth Fibonacci number using dynamic programming (memoization).
/// analyze: How does this compare to the naive version?
pub fn fib_memoized(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if n <= 1 {
        return n;
    }
    if let Some(&result) = memo.get(&n) {
        return result;
    }
    let result = fib_memoized(n - 1, memo) + fib_memoized(n - 2, memo);
    memo.insert(n, result);
    result
}

// ============================================================================
// Exercise 4: Two Sum with HashMap
// ============================================================================

use std::collections::HashMap;

/// Find two indices in the array that add up to target using a HashMap.
/// analyze: What is the time complexity?
/// analyze: What is the space complexity?
pub fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut map = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        if let Some(&j) = map.get(&(target - num)) {
            return Some((j, i));
        }
        map.insert(num, i);
    }
    None
}

// ============================================================================
// Exercise 5: Mystery Function Analysis
// ============================================================================

/// A mysterious function for analysis.
/// analyze: Trace through the execution and determine complexity.
pub fn mystery(n: usize) -> usize {
    let mut count = 0;
    let mut i = n;
    while i > 0 {
        i /= 2;
        let mut j = 0;
        while j < n {
            j += 1;
            count += 1;
        }
    }
    count
}

// ============================================================================
// Exercise 6: String Reversal
// ============================================================================

/// Reverse a string and return a new string.
/// analyze: What is the time complexity?
/// analyze: What is the space complexity?
pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

/// Reverse a string in-place using a mutable array.
/// analyze: What is the time complexity?
/// analyze: What is the space complexity?
pub fn reverse_string_in_place(s: &mut [char]) {
    let mut left = 0;
    let mut right = s.len().saturating_sub(1);
    while left < right {
        s.swap(left, right);
        left += 1;
        right = right.saturating_sub(1);
    }
}

// ============================================================================
// Exercise 7: BST Search
// ============================================================================

/// A simple BST node structure.
#[derive(Debug, Clone)]
pub struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new(val: i32) -> Self {
        Node { val, left: None, right: None }
    }

    pub fn search(root: Option<&Node>, target: i32) -> bool {
        let mut current = root;
        while let Some(node) = current {
            match target.cmp(&node.val) {
                std::cmp::Ordering::Equal => return true,
                std::cmp::Ordering::Less => current = node.left.as_ref().map(|v| &**v),
                std::cmp::Ordering::Greater => current = node.right.as_ref().map(|v| &**v),
            }
        }
        false
    }
}

/// Search for a value in a BST.
/// analyze: What is the time complexity (best, average, worst)?
pub fn bst_search(root: Option<&Node>, target: i32) -> bool {
    Node::search(root, target)
}

// ============================================================================
// Exercise 8: Recursive String Permutations
// ============================================================================

/// Generate all permutations of a string.
/// analyze: What is the time complexity?
/// analyze: What is the space complexity?
pub fn permutations(s: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut chars: Vec<char> = s.chars().collect();

    fn helper(chars: &mut [char], start: usize, result: &mut Vec<String>) {
        if start == chars.len() {
            result.push(chars.iter().collect());
        } else {
            for i in start..chars.len() {
                chars.swap(start, i);
                helper(chars, start + 1, result);
                chars.swap(start, i);
            }
        }
    }

    helper(&mut chars, 0, &mut result);
    result
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("Rust DSA exercises - run tests with cargo test");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sorted_empty_first() {
        assert_eq!(merge_sorted(&[], &[1, 2, 3]), vec![1, 2, 3]);
    }

    #[test]
    fn test_merge_sorted_empty_second() {
        assert_eq!(merge_sorted(&[1, 2, 3], &[]), vec![1, 2, 3]);
    }
}
// Exercise 9: Sliding Window Maximum
// ============================================================================

use std::collections::VecDeque;

/// Find the maximum element in each sliding window of size k.
/// analyze: What is the time complexity?
/// analyze: What is the space complexity?
pub fn max_sliding_window(nums: &[i32], k: usize) -> Vec<i32> {
    let mut result = Vec::new();
    let mut deque = VecDeque::new();

    for (i, &num) in nums.iter().enumerate() {
        while let Some(&back) = deque.back() {
            if nums[back] < num {
                deque.pop_back();
            } else {
                break;
            }
        }
        deque.push_back(i);

        if deque.front() == Some(&(i - k)) {
            deque.pop_front();
        }

        if i >= k.saturating_sub(1) {
            result.push(nums[*deque.front().unwrap()]);
        }
    }
    result
}

// ============================================================================
// Exercise 10: Nested Loop with Early Exit
// ============================================================================

/// Find a pair of elements that sum to target (returns on first match).
/// analyze: What is the time complexity (best, average, worst)?
pub fn find_pair_sum(arr: &[i32], target: i32) -> Option<(i32, i32)> {
    for i in 0..arr.len() {
        for j in 0..arr.len() {
            if arr[i] + arr[j] == target {
                return Some((arr[i], arr[j]));
            }
        }
    }
    None
}

// ============================================================================
// Exercise 11: Find All Pairs with Sum
// ============================================================================

/// Find all unique pairs of indices that sum to target.
/// analyze: What is the time complexity?
/// analyze: What is the space complexity?
pub fn all_pairs_with_sum(nums: &[i32], target: i32) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    let mut map = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        if let Some(&j) = map.get(&(target - num)) {
            result.push((j, i));
        }
        map.insert(num, i);
    }
    result
}

// ============================================================================
// Exercise 12: Recursive Binary Search
// ============================================================================

/// Perform binary search recursively.
/// analyze: What is the time complexity?
/// analyze: What is the space complexity?
pub fn binary_search_recursive(arr: &[i32], target: i32, left: usize, right: usize) -> Option<usize> {
    if left >= right {
        return None;
    }
    let mid = left + (right - left) / 2;
    match arr[mid].cmp(&target) {
        std::cmp::Ordering::Equal => Some(mid),
        std::cmp::Ordering::Less => binary_search_recursive(arr, target, mid + 1, right),
        std::cmp::Ordering::Greater => binary_search_recursive(arr, target, left, mid),
    }
}

// ============================================================================
// Exercise 13: Insertion Sort
// ============================================================================

/// Sort an array using insertion sort.
/// analyze: What is the time complexity (best, average, worst)?
/// analyze: What is the space complexity?
pub fn insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let key = arr[i];
        let mut j = i;
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = key;
    }
}

// ============================================================================
// Exercise 14: Power Function
// ============================================================================

/// Calculate base^exponent using binary exponentiation.
/// analyze: What is the time complexity?
pub fn power(base: f64, exponent: i32) -> f64 {
    if exponent < 0 {
        return 1.0 / power(base, -exponent);
    }
    if exponent == 0 {
        return 1.0;
    }
    if exponent == 1 {
        return base;
    }
    let half = power(base, exponent / 2);
    if exponent % 2 == 0 {
        half * half
    } else {
        half * half * base
    }
}

// ============================================================================
// Exercise 15: Merge Two Sorted Arrays
// ============================================================================

/// Merge two sorted arrays into one sorted array.
/// analyze: What is the time complexity?
/// analyze: What is the space complexity?
pub fn merge_sorted(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(arr1.len() + arr2.len());
    let mut i = 0;
    let mut j = 0;

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] <= arr2[j] {
            result.push(arr1[i]);
            i += 1;
        } else {
            result.push(arr2[j]);
            j += 1;
        }
    }

    while i < arr1.len() {
        result.push(arr1[i]);
        i += 1;
    }

    while j < arr2.len() {
        result.push(arr2[j]);
        j += 1;
    }

    result
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Exercise 1: Sum of Array Elements
    #[test]
    fn test_sum_array_empty() {
        let arr: [i32; 0] = [];
        assert_eq!(sum_array(&arr), 0);
    }

    #[test]
    fn test_sum_array_single() {
        assert_eq!(sum_array(&[42]), 42);
    }

    #[test]
    fn test_sum_array_multiple() {
        assert_eq!(sum_array(&[1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn test_sum_array_negative() {
        assert_eq!(sum_array(&[-1, -2, -3, -4, -5]), -15);
    }

    // Exercise 2: Find Duplicates
    #[test]
    fn test_has_duplicate_false() {
        assert!(!has_duplicate(&[1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_has_duplicate_true() {
        assert!(has_duplicate(&[1, 2, 3, 2, 5]));
    }

    #[test]
    fn test_has_duplicate_empty() {
        assert!(!has_duplicate(&[]));
    }

    // Exercise 3: Fibonacci
    #[test]
    fn test_fib_zero() {
        assert_eq!(fib(0), 0);
    }

    #[test]
    fn test_fib_one() {
        assert_eq!(fib(1), 1);
    }

    #[test]
    fn test_fib_seven() {
        assert_eq!(fib(7), 13);
    }

    #[test]
    fn test_fib_memoized() {
        let mut memo = HashMap::new();
        assert_eq!(fib_memoized(10, &mut memo), 55);
    }

    // Exercise 4: Two Sum
    #[test]
    fn test_two_sum_found() {
        assert_eq!(two_sum(&[2, 7, 11, 15], 9), Some((0, 1)));
    }

    #[test]
    fn test_two_sum_not_found() {
        assert_eq!(two_sum(&[1, 2, 3, 4], 10), None);
    }

    // Exercise 5: Mystery
    #[test]
    fn test_mystery_n8() {
        // n=8: outer loop runs log2(8)=3 times, inner loop runs n times each
        assert_eq!(mystery(8), 24);
    }

    #[test]
    fn test_mystery_n16() {
        assert_eq!(mystery(16), 64);
    }

    // Exercise 6: String Reversal
    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("hello"), "olleh");
    }

    #[test]
    fn test_reverse_string_palindrome() {
        assert_eq!(reverse_string("racecar"), "racecar");
    }

    #[test]
    fn test_reverse_string_empty() {
        assert_eq!(reverse_string(""), "");
    }

    // Exercise 7: BST Search
    #[test]
    fn test_bst_search_found() {
        let root = Some(Box::new(Node {
            val: 5,
            left: Some(Box::new(Node::new(3))),
            right: Some(Box::new(Node::new(7))),
        }));
        assert!(bst_search(root.as_ref(), 3));
    }

    #[test]
    fn test_bst_search_not_found() {
        let root = Some(Box::new(Node {
            val: 5,
            left: Some(Box::new(Node::new(3))),
            right: Some(Box::new(Node::new(7))),
        }));
        assert!(!bst_search(root.as_ref(), 4));
    }

    // Exercise 8: Permutations
    #[test]
    fn test_permutations_abc() {
        let perms = permutations("abc");
        assert_eq!(perms.len(), 6);
        assert!(perms.contains(&"abc".to_string()));
        assert!(perms.contains(&"acb".to_string()));
        assert!(perms.contains(&"bac".to_string()));
    }

    // Exercise 9: Sliding Window Maximum
    #[test]
    fn test_max_sliding_window() {
        assert_eq!(max_sliding_window(&[1, 3, -1, -3, 5, 3, 6, 7], 3), vec![3, 3, 5, 5, 6, 7]);
    }

    #[test]
    fn test_max_sliding_window_single() {
        assert_eq!(max_sliding_window(&[1], 1), vec![1]);
    }

    // Exercise 10: Find Pair Sum
    #[test]
    fn test_find_pair_sum_found() {
        assert_eq!(find_pair_sum(&[1, 2, 3, 4, 5], 9), Some((4, 5)));
    }

    #[test]
    fn test_find_pair_sum_not_found() {
        assert_eq!(find_pair_sum(&[1, 2, 3, 4, 5], 10), None);
    }

    // Exercise 11: All Pairs with Sum
    #[test]
    fn test_all_pairs_with_sum() {
        let pairs = all_pairs_with_sum(&[1, 2, 3, 2, 5], 5);
        assert!(pairs.contains(&(1, 4)));
        assert!(pairs.contains(&(3, 2)));
    }

    // Exercise 12: Binary Search Recursive
    #[test]
    fn test_binary_search_recursive_found() {
        assert_eq!(binary_search_recursive(&[1, 3, 5, 7, 9], 7, 0, 5), Some(3));
    }

    #[test]
    fn test_binary_search_recursive_not_found() {
        assert_eq!(binary_search_recursive(&[1, 3, 5, 7, 9], 4, 0, 5), None);
    }

    // Exercise 13: Insertion Sort
    #[test]
    fn test_insertion_sort() {
        let mut arr = vec![5, 2, 8, 1, 9];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 5, 8, 9]);
    }

    // Exercise 14: Power Function
    #[test]
    fn test_power_positive() {
        assert!((power(2.0, 10) - 1024.0).abs() < 1e-10);
    }

    #[test]
    fn test_power_negative_exp() {
        assert!((power(2.0, -2) - 0.25).abs() < 1e-10);
    }

    // Exercise 15: Merge Sorted
    #[test]
    fn test_merge_sorted() {
        assert_eq!(merge_sorted(&[1, 3, 5], &[2, 4, 6]), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_merge_sorted_empty_first() {
        assert_eq!(merge_sorted(&[], &[1, 2, 3]), vec![1, 2, 3]);
    }

    #[test]
    fn test_merge_sorted_empty_second() {
        assert_eq!(merge_sorted(&[1, 2, 3], &[]), vec![1, 2, 3]);
    }
}
