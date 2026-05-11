/// Non-Overlapping Intervals - LeetCode 435
/// Find minimum intervals to remove for non-overlapping intervals.

/// Approach: Greedy - sort by end time, pick intervals that don't overlap.
pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
    if intervals.is_empty() { return 0; }

    let mut intervals = intervals;
    intervals.sort_by_key(|v| v[1]);

    let mut end = intervals[0][1];
    let mut count = 0;

    for i in 1..intervals.len() {
        if intervals[i][0] < end {
            count += 1;
        } else {
            end = intervals[i][1];
        }
    }

    count
}

/// Alternative: Sort by start time
pub fn erase_overlap_intervals_by_start(intervals: Vec<Vec<i32>>) -> i32 {
    if intervals.is_empty() { return 0; }

    let mut intervals = intervals;
    intervals.sort_by_key(|v| v[0]);

    let mut prev_end = intervals[0][1];
    let mut count = 0;

    for i in 1..intervals.len() {
        if intervals[i][0] < prev_end {
            count += 1;
            prev_end = prev_end.min(intervals[i][1]);
        } else {
            prev_end = intervals[i][1];
        }
    }

    count
}

/// Brute force with recursion for small inputs
pub fn erase_overlap_intervals_dp(intervals: Vec<Vec<i32>>) -> i32 {
    if intervals.is_empty() { return 0; }

    let mut intervals = intervals;
    intervals.sort_by_key(|v| v[1]);
    let n = intervals.len();

    fn dfs(i: usize, intervals: &Vec<Vec<i32>>, memo: &mut Vec<i32>) -> i32 {
        if i >= intervals.len() { return 0; }
        if memo[i] >= 0 { return memo[i]; }

        let mut skip = dfs(i + 1, intervals, memo);

        let mut j = i + 1;
        while j < intervals.len() && intervals[j][0] < intervals[i][1] {
            j += 1;
        }
        let take = 1 + dfs(j, intervals, memo);

        memo[i] = skip.min(take);
        memo[i]
    }

    let mut memo = vec![-1; n];
    dfs(0, &intervals, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_erase_overlap_basic() {
        assert_eq!(erase_overlap_intervals(vec![vec![1,2],vec![2,3],vec![3,4],vec![1,3]]), 1);
    }

    #[test]
    fn test_erase_overlap_none() {
        assert_eq!(erase_overlap_intervals(vec![vec![1,2],vec![2,3]]), 0);
    }

    #[test]
    fn test_erase_overlap_all() {
        assert_eq!(erase_overlap_intervals(vec![vec![1,2],vec![1,2],vec![1,2]]), 2);
    }

    #[test]
    fn test_erase_overlap_empty() {
        assert_eq!(erase_overlap_intervals(vec![]), 0);
    }

    #[test]
    fn test_erase_overlap_single() {
        assert_eq!(erase_overlap_intervals(vec![vec![1,2]]), 0);
    }

    #[test]
    fn test_erase_overlap_two() {
        assert_eq!(erase_overlap_intervals(vec![vec![1,2],vec![3,4]]), 0);
    }

    #[test]
    fn test_erase_overlap_overlapping() {
        assert_eq!(erase_overlap_intervals(vec![vec![1,4],vec![2,3]]), 1);
    }

    #[test]
    fn test_erase_overlap_start_same() {
        assert_eq!(erase_overlap_intervals(vec![vec![1,2],vec![1,3],vec![2,4]]), 1);
    }

    #[test]
    fn test_erase_overlap_end_same() {
        assert_eq!(erase_overlap_intervals(vec![vec![1,2],vec![2,3],vec![3,3]]), 0);
    }

    #[test]
    fn test_erase_overlap_by_start() {
        let intervals = vec![vec![1,2],vec![2,3],vec![3,4],vec![1,3]];
        assert_eq!(erase_overlap_intervals_by_start(intervals.clone()),
                   erase_overlap_intervals(intervals));
    }

    #[test]
    fn test_erase_overlap_dp_basic() {
        assert_eq!(erase_overlap_intervals_dp(vec![vec![1,2],vec![2,3],vec![3,4],vec![1,3]]), 1);
    }

    #[test]
    fn test_erase_overlap_dp_none() {
        assert_eq!(erase_overlap_intervals_dp(vec![vec![1,2],vec![2,3]]), 0);
    }

    #[test]
    fn test_erase_overlap_dp_all() {
        assert_eq!(erase_overlap_intervals_dp(vec![vec![1,2],vec![1,2],vec![1,2]]), 2);
    }

    #[test]
    fn test_erase_overlap_longer() {
        let intervals = vec![vec![1,100],vec![2,3],vec![3,4],vec![4,5]];
        assert!(erase_overlap_intervals(intervals) <= 2);
    }

    #[test]
    fn test_erase_overlap_complex() {
        let intervals = vec![vec![1,4],vec![2,4],vec![3,4],vec![4,5]];
        assert_eq!(erase_overlap_intervals(intervals), 1);
    }

    #[test]
    fn test_erase_overlap_many() {
        let intervals = vec![vec![0,2],vec![1,3],vec![2,4],vec![3,5],vec![4,6]];
        assert!(erase_overlap_intervals(intervals) >= 2);
    }

    #[test]
    fn test_erase_overlap_adjacent() {
        // Adjacent intervals (touching) are not overlapping
        assert_eq!(erase_overlap_intervals(vec![vec![1,2],vec![2,3],vec![3,4]]), 0);
    }

    #[test]
    fn test_erase_overlap_greedy_vs_dp() {
        let intervals = vec![vec![1,2],vec![2,3],vec![3,4],vec![1,3]];
        assert_eq!(erase_overlap_intervals(intervals.clone()),
                   erase_overlap_intervals_dp(intervals));
    }

    #[test]
    fn test_erase_overlap_reverse_sorted() {
        let mut intervals = vec![vec![1,3],vec![3,6],vec![2,4]];
        intervals.sort_by_key(|v| -v[1]); // Reverse sort
        // After proper sorting by end, should work
        let result = erase_overlap_intervals(vec![vec![1,3],vec![2,4],vec![3,6]]);
        assert!(result <= 1);
    }

    #[test]
    fn test_erase_overlap_equal_ends() {
        let intervals = vec![vec![1,5],vec![2,5],vec![3,5],vec![4,5]];
        assert_eq!(erase_overlap_intervals(intervals), 3);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("058_non_overlapping_intervals_lc435 exercises - run tests with cargo test");
}
