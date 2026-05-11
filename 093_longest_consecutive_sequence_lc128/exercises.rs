//! Exercises for Longest Consecutive Sequence (LeetCode 128)
//!
//! # Topics Covered
//! - HashSet for O(1) lookup
//! - Sequence detection
//! - O(n) time complexity
//! - Consecutive number tracking
//!
//! # Difficulty: Hard

use std::collections::{HashMap, HashSet};

/// Longest consecutive sequence using HashSet
/// Time: O(n), Space: O(n)
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let set: HashSet<i32> = nums.into_iter().collect();
    let mut longest = 1;

    for &num in set.iter() {
        // Only start counting if this is the start of a sequence
        if !set.contains(&(num - 1)) {
            let mut current = num;
            let mut count = 1;

            while set.contains(&(current + 1)) {
                current += 1;
                count += 1;
            }

            longest = longest.max(count);
        }
    }

    longest
}

/// Union-Find approach
pub fn longest_consecutive_union_find(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let n = nums.len();
    let mut parent = vec![0usize; n];
    let mut rank = vec![1usize; n];
    let mut index: HashMap<i32, usize> = HashMap::new();

    // Map each number to its index
    for (i, &num) in nums.iter().enumerate() {
        parent[i] = i;
        index.insert(num, i);
    }

    // Find with path compression
    fn find(x: usize, parent: &mut [usize]) -> usize {
        if parent[x] != x {
            parent[x] = find(parent[x], parent);
        }
        parent[x]
    }

    // Union
    fn union(x: usize, y: usize, parent: &mut [usize], rank: &mut [usize]) {
        let px = find(x, parent);
        let py = find(y, parent);
        if px == py { return; }

        if rank[px] < rank[py] {
            parent[px] = py;
        } else if rank[px] > rank[py] {
            parent[py] = px;
        } else {
            parent[py] = px;
            rank[px] += 1;
        }
    }

    // Union adjacent elements
    for &num in nums.iter() {
        if let Some(&i) = index.get(&num) {
            if let Some(&j) = index.get(&(num + 1)) {
                union(i, j, &mut parent, &mut rank);
            }
        }
    }

    // Count distinct roots
    let mut count: HashMap<usize, usize> = HashMap::new();
    for i in 0..n {
        let root = find(i, &mut parent);
        *count.entry(root).or_insert(0) += 1;
    }

    count.values().copied().max().unwrap_or(1) as i32
}

/// Sort-based approach O(n log n)
pub fn longest_consecutive_sort(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut nums = nums;
    nums.sort();
    let mut longest = 1;
    let mut current = 1;

    for i in 1..nums.len() {
        if nums[i] == nums[i - 1] + 1 {
            current += 1;
            longest = longest.max(current);
        } else if nums[i] != nums[i - 1] {
            current = 1;
        }
    }

    longest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_example() {
        assert_eq!(longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
        assert_eq!(longest_consecutive_sort(vec![100, 4, 200, 1, 3, 2]), 4);
    }

    #[test]
    fn test_empty() {
        assert_eq!(longest_consecutive(vec![]), 0);
        assert_eq!(longest_consecutive_union_find(vec![]), 0);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(longest_consecutive(vec![1]), 1);
        assert_eq!(longest_consecutive_union_find(vec![1]), 1);
        assert_eq!(longest_consecutive_sort(vec![1]), 1);
    }

    #[test]
    fn test_all_consecutive() {
        assert_eq!(longest_consecutive(vec![1, 2, 3, 4, 5]), 5);
        assert_eq!(longest_consecutive_union_find(vec![1, 2, 3, 4, 5]), 5);
        assert_eq!(longest_consecutive_sort(vec![1, 2, 3, 4, 5]), 5);
    }

    #[test]
    fn test_no_consecutive() {
        assert_eq!(longest_consecutive(vec![1, 3, 5, 7]), 1);
        assert_eq!(longest_consecutive_union_find(vec![1, 3, 5, 7]), 1);
        assert_eq!(longest_consecutive_sort(vec![1, 3, 5, 7]), 1);
    }

    #[test]
    fn test_two_consecutive() {
        assert_eq!(longest_consecutive(vec![1, 2]), 2);
        assert_eq!(longest_consecutive_union_find(vec![1, 2]), 2);
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(longest_consecutive(vec![-1, 0, 1, 2]), 4);
        assert_eq!(longest_consecutive_union_find(vec![-1, 0, 1, 2]), 4);
    }

    #[test]
    fn test_duplicate_values() {
        assert_eq!(longest_consecutive(vec![1, 2, 2, 3]), 3);
        assert_eq!(longest_consecutive_union_find(vec![1, 2, 2, 3]), 3);
    }

    #[test]
    fn test_large_gaps() {
        assert_eq!(longest_consecutive(vec![1, 10, 20, 30]), 1);
    }

    #[test]
    fn test_consistency_all_approaches() {
        let test_cases = vec![
            vec![100, 4, 200, 1, 3, 2],
            vec![],
            vec![1],
            vec![1, 2, 3, 4, 5],
            vec![1, 3, 5, 7],
            vec![-1, 0, 1, 2],
            vec![1, 2, 2, 3],
        ];

        for nums in test_cases {
            let hash = longest_consecutive(nums.clone());
            let union_find = longest_consecutive_union_find(nums.clone());
            let sort = longest_consecutive_sort(nums.clone());
            assert_eq!(hash, union_find, "hash vs union_find for {:?}", nums);
            assert_eq!(union_find, sort, "union_find vs sort for {:?}", nums);
        }
    }

    #[test]
    fn test_longest_at_end() {
        assert_eq!(longest_consecutive(vec![1, 2, 3, 10]), 3);
    }

    #[test]
    fn test_longest_at_start() {
        assert_eq!(longest_consecutive(vec![10, 1, 2, 3, 4]), 4);
    }

    #[test]
    fn test_multiple_sequences() {
        // [1,2,3] and [5,6,7] - longest is 3
        assert_eq!(longest_consecutive(vec![1, 2, 3, 5, 6, 7]), 3);
    }

    #[test]
    fn test_unsorted_with_duplicates() {
        assert_eq!(longest_consecutive(vec![4, 2, 1, 3, 3]), 3);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("Longest Consecutive Sequence exercises - run tests with cargo test");
}