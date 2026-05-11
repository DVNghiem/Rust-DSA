/// Approach: Sort by start time, then check adjacent overlaps
pub fn can_attend_meetings(intervals: Vec<Vec<i32>>) -> bool {
    if intervals.is_empty() {
        return true;
    }

    let mut intervals = intervals;
    intervals.sort_by_key(|v| v[0]);

    for i in 1..intervals.len() {
        if intervals[i][0] < intervals[i - 1][1] {
            return false;
        }
    }

    true
}

/// Alternative: Check using min-heap of end times
pub fn can_attend_meetings_heap(intervals: Vec<Vec<i32>>) -> bool {
    if intervals.is_empty() {
        return true;
    }

    use std::collections::BinaryHeap;
    let mut min_heap: BinaryHeap<i32> = BinaryHeap::new();

    let mut intervals = intervals;
    intervals.sort_by_key(|v| v[0]);

    for interval in intervals {
        if !min_heap.is_empty() && interval[0] >= -min_heap.peek().unwrap() {
            min_heap.pop();
        }
        min_heap.push(-interval[1]);
    }

    min_heap.len() == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let intervals = vec![vec![0, 30], vec![5, 10], vec![15, 20]];
        assert!(!can_attend_meetings(intervals));
    }

    #[test]
    fn test_empty() {
        let intervals: Vec<Vec<i32>> = vec![];
        assert!(can_attend_meetings(intervals));
    }

    #[test]
    fn test_single() {
        let intervals = vec![vec![5, 10]];
        assert!(can_attend_meetings(intervals));
    }

    #[test]
    fn test_no_overlap() {
        let intervals = vec![vec![5, 15], vec![20, 30], vec![35, 45]];
        assert!(can_attend_meetings(intervals));
    }

    #[test]
    fn test_adjacent_meetings() {
        // [5, 10] and [10, 15] don't overlap (touch at endpoint)
        let intervals = vec![vec![5, 10], vec![10, 15]];
        assert!(can_attend_meetings(intervals));
    }

    #[test]
    fn test_one_contains_another() {
        let intervals = vec![vec![5, 20], vec![10, 15]];
        assert!(!can_attend_meetings(intervals));
    }

    #[test]
    fn test_exactly_same() {
        let intervals = vec![vec![5, 10], vec![5, 10]];
        assert!(!can_attend_meetings(intervals));
    }

    #[test]
    fn test_unsorted_input() {
        let intervals = vec![vec![15, 20], vec![0, 30], vec![5, 10]];
        assert!(!can_attend_meetings(intervals));
    }

    #[test]
    fn test_negative_times() {
        let intervals = vec![vec![-10, -5], vec![-3, 3]];
        assert!(can_attend_meetings(intervals));
    }

    #[test]
    fn test_large_gap() {
        let intervals = vec![vec![0, 10], vec![100, 110], vec![200, 210]];
        assert!(can_attend_meetings(intervals));
    }

    #[test]
    fn test_two_overlapping() {
        let intervals = vec![vec![1, 10], vec![2, 5]];
        assert!(!can_attend_meetings(intervals));
    }

    #[test]
    fn test_three_sequential() {
        let intervals = vec![vec![1, 5], vec![5, 10], vec![10, 15]];
        assert!(can_attend_meetings(intervals));
    }

    #[test]
    fn test_heap_basic() {
        let intervals = vec![vec![0, 30], vec![5, 10], vec![15, 20]];
        assert!(!can_attend_meetings_heap(intervals));
    }

    #[test]
    fn test_heap_empty() {
        let intervals: Vec<Vec<i32>> = vec![];
        assert!(can_attend_meetings_heap(intervals));
    }

    #[test]
    fn test_heap_no_overlap() {
        let intervals = vec![vec![5, 15], vec![20, 30], vec![35, 45]];
        assert!(can_attend_meetings_heap(intervals));
    }

    #[test]
    fn test_heap_one_contains() {
        let intervals = vec![vec![5, 20], vec![10, 15]];
        assert!(!can_attend_meetings_heap(intervals));
    }

    #[test]
    fn test_all_same_start() {
        let intervals = vec![vec![1, 5], vec![1, 10], vec![1, 15]];
        assert!(!can_attend_meetings_heap(intervals));
    }

    #[test]
    fn test_all_same_end() {
        let intervals = vec![vec![0, 5], vec![1, 5], vec![2, 5]];
        assert!(!can_attend_meetings_heap(intervals));
    }

    #[test]
    fn test_equal_meetings() {
        let intervals = vec![vec![5, 10], vec![5, 10]];
        assert!(!can_attend_meetings_heap(intervals));
    }

    #[test]
    fn test_back_to_back_non_overlapping() {
        let intervals = vec![vec![1, 3], vec![3, 5], vec![5, 7]];
        assert!(can_attend_meetings(intervals));
    }

    #[test]
    fn test_multiple_non_overlapping() {
        let intervals = vec![
            vec![0, 5],
            vec![5, 10],
            vec![10, 15],
            vec![15, 20],
            vec![20, 25],
        ];
        assert!(can_attend_meetings(intervals));
    }

    #[test]
    fn test_partial_overlap() {
        let intervals = vec![vec![1, 8], vec![3, 6]];
        assert!(!can_attend_meetings(intervals));
    }

    #[test]
    fn test_one_inside_another() {
        let intervals = vec![vec![10, 20], vec![12, 15]];
        assert!(!can_attend_meetings(intervals));
    }

    #[test]
    fn test_heap_partial_overlap() {
        let intervals = vec![vec![1, 8], vec![3, 6]];
        assert!(!can_attend_meetings_heap(intervals));
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("049_meeting_rooms_lc252 exercises - run tests with cargo test");
}
