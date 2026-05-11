//! Exercises for Merge Intervals (LeetCode 56)
//!
//! # Topics Covered
//! - Interval merging
//! - Sorting by start time
//! - Linear scan merging
//! - O(n log n) algorithm
//!
//! # Difficulty: Medium

/// Merge overlapping intervals
/// Time: O(n log n), Space: O(n)
pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if intervals.is_empty() {
        return vec![];
    }

    let mut intervals = intervals;
    intervals.sort_by_key(|i| i[0]);

    let mut result = vec![intervals[0].clone()];

    for interval in intervals.iter().skip(1) {
        let last = result.last_mut().unwrap();
        if interval[0] <= last[1] {
            last[1] = last[1].max(interval[1]);
        } else {
            result.push(interval.clone());
        }
    }

    result
}

/// Insert interval into already merged intervals
/// Time: O(n), Space: O(n)
pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = merge(intervals);

    let mut i = 0;
    while i < result.len() && result[i][0] < new_interval[0] {
        i += 1;
    }

    if i > 0 && result[i - 1][1] >= new_interval[0] {
        result[i - 1][1] = result[i - 1][1].max(new_interval[1]);
    } else {
        result.insert(i, new_interval.clone());
    }

    // Merge if needed with neighbors
    let mut j = if i > 0 && result[i - 1][1] >= new_interval[0] { i - 1 } else { i };
    while j < result.len() - 1 && result[j][1] >= result[j + 1][0] {
        result[j][1] = result[j][1].max(result[j + 1][1]);
        result.remove(j + 1);
    }

    result
}

/// Check if intervals overlap
pub fn has_overlap(intervals: &[Vec<i32>]) -> bool {
    for i in 1..intervals.len() {
        if intervals[i][0] <= intervals[i - 1][1] {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_example() {
        let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let result = merge(intervals);
        assert_eq!(result, vec![vec![1, 6], vec![8, 10], vec![15, 18]]);
    }

    #[test]
    fn test_no_overlap() {
        let intervals = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        let result = merge(intervals);
        assert_eq!(result, vec![vec![1, 2], vec![3, 4], vec![5, 6]]);
    }

    #[test]
    fn test_complete_overlap() {
        let intervals = vec![vec![1, 10], vec![2, 3], vec![4, 5], vec![6, 7]];
        let result = merge(intervals);
        assert_eq!(result, vec![vec![1, 10]]);
    }

    #[test]
    fn test_adjacent_intervals() {
        let intervals = vec![vec![1, 2], vec![3, 4]];
        let result = merge(intervals);
        // Adjacent (touching) but not overlapping - should remain separate
        assert_eq!(result, vec![vec![1, 2], vec![3, 4]]);
    }

    #[test]
    fn test_single_interval() {
        let intervals = vec![vec![1, 5]];
        let result = merge(intervals);
        assert_eq!(result, vec![vec![1, 5]]);
    }

    #[test]
    fn test_empty_input() {
        let intervals: Vec<Vec<i32>> = vec![];
        let result = merge(intervals);
        assert!(result.is_empty());
    }

    #[test]
    fn test_unsorted_input() {
        let intervals = vec![vec![8, 10], vec![1, 3], vec![15, 18], vec![2, 6]];
        let result = merge(intervals);
        assert_eq!(result, vec![vec![1, 6], vec![8, 10], vec![15, 18]]);
    }

    #[test]
    fn test_identical_intervals() {
        let intervals = vec![vec![1, 5], vec![1, 5], vec![1, 5]];
        let result = merge(intervals);
        assert_eq!(result, vec![vec![1, 5]]);
    }

    #[test]
    fn test_negative_numbers() {
        let intervals = vec![vec![-5, -3], vec![-2, 2], vec![0, 5]];
        let result = merge(intervals);
        assert_eq!(result, vec![vec![-5, -3], vec![-2, 5]]);
    }

    #[test]
    fn test_chain_merging() {
        // [1,5], [2,3], [4,6], [7,8] should all merge to [1,8]
        let intervals = vec![vec![1, 5], vec![2, 3], vec![4, 6], vec![7, 8]];
        let result = merge(intervals);
        assert_eq!(result, vec![vec![1, 8]]);
    }

    #[test]
    fn test_insert_basic() {
        let intervals = vec![vec![1, 3], vec![6, 9]];
        let result = insert(intervals, vec![2, 5]);
        assert_eq!(result, vec![vec![1, 5], vec![6, 9]]);
    }

    #[test]
    fn test_insert_merges() {
        let intervals = vec![vec![1, 3], vec![6, 9]];
        let result = insert(intervals, vec![4, 7]);
        assert_eq!(result, vec![vec![1, 9]]);
    }

    #[test]
    fn test_insert_at_start() {
        let intervals = vec![vec![5, 7]];
        let result = insert(intervals, vec![1, 3]);
        assert_eq!(result, vec![vec![1, 3], vec![5, 7]]);
    }

    #[test]
    fn test_insert_at_end() {
        let intervals = vec![vec![1, 3]];
        let result = insert(intervals, vec![5, 7]);
        assert_eq!(result, vec![vec![1, 3], vec![5, 7]]);
    }

    #[test]
    fn test_has_overlap_true() {
        let intervals = vec![vec![1, 5], vec![4, 6]];
        assert!(has_overlap(&intervals));
    }

    #[test]
    fn test_has_overlap_false() {
        let intervals = vec![vec![1, 3], vec![5, 7]];
        assert!(!has_overlap(&intervals));
    }

    #[test]
    fn test_has_overlap_empty() {
        let intervals: Vec<Vec<i32>> = vec![];
        assert!(!has_overlap(&intervals));
    }

    #[test]
    fn test_two_elements() {
        let intervals = vec![vec![1, 4], vec![2, 3]];
        let result = merge(intervals);
        assert_eq!(result, vec![vec![1, 4]]);
    }

    #[test]
    fn test_wide_range() {
        let intervals = vec![vec![0, 100], vec![1, 2], vec![50, 60]];
        let result = merge(intervals);
        assert_eq!(result, vec![vec![0, 100]]);
    }

    #[test]
    fn test_overlapping_at_end() {
        let intervals = vec![vec![1, 5], vec![5, 10]];
        let result = merge(intervals);
        // [1,5] and [5,10] touch at 5, should merge to [1,10]
        assert_eq!(result, vec![vec![1, 10]]);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("091_merge_sort_intervals_lc56 exercises - run tests with cargo test");
}
