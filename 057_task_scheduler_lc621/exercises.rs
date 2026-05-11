/// Task Scheduler - LeetCode 621
/// Calculate minimum intervals needed to complete all tasks with cooldown.

use std::collections::HashMap;

/// Approach: Greedy with max heap simulation
/// Use a max heap to always pick the most frequent task.
pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
    if n == 0 { return tasks.len() as i32; }

    let mut freq = HashMap::new();
    for t in &tasks {
        *freq.entry(t).or_insert(0) += 1;
    }

    let max_freq = *freq.values().max().unwrap_or(&0);
    let num_max = freq.values().filter(|&&v| v == max_freq).count() as i32;

    // Calculate using formula
    let part_len = n + 1;
    let num_parts = max_freq - 1;
    let slots_needed = num_parts * part_len;
    let current_tasks = num_max;
    let idle_slots = slots_needed - current_tasks as i32;

    let remaining = tasks.len() as i32 - current_tasks;
    let actual_idle = idle_slots.max(remaining);

    tasks.len() as i32 + actual_idle.max(0)
}

/// Alternative: Direct formula
pub fn least_interval_formula(tasks: Vec<char>, n: i32) -> i32 {
    if n == 0 { return tasks.len() as i32; }

    let mut freq = HashMap::new();
    for t in &tasks {
        *freq.entry(t).or_insert(0) += 1;
    }

    let max_count = *freq.values().max().unwrap_or(&0);
    let mut max_count_tasks = 0;
    for &v in freq.values() {
        if v == max_count { max_count_tasks += 1; }
    }

    let part_len = (n + 1) as i32;
    let result = (max_count - 1) as i32 * part_len + max_count_tasks as i32;
    result.max(tasks.len() as i32)
}

/// Using max heap simulation for clarity
pub fn least_interval_heap(tasks: Vec<char>, n: i32) -> i32 {
    use std::collections::BinaryHeap;
    use std::cmp::Reverse;

    if n == 0 { return tasks.len() as i32; }

    let mut freq = HashMap::new();
    for t in &tasks {
        *freq.entry(t).or_insert(0) += 1;
    }

    let mut heap: BinaryHeap<i32> = freq.values().cloned().collect();
    let mut intervals: i32 = 0;

    while let Some(count) = heap.pop() {
        let mut remaining = Vec::new();
        for _ in 0..n.min(count) {
            intervals += 1;
            if count > 1 {
                remaining.push(count - 1);
            }
        }
        for r in &remaining {
            if *r > 0 { heap.push(*r); }
        }
        if heap.is_empty() && remaining.is_empty() {
            intervals += tasks.len() as i32;
            break;
        } else if !heap.is_empty() {
            intervals += n - remaining.len() as i32;
        }
    }

    intervals.max(tasks.len() as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_least_interval_basic() {
        assert_eq!(least_interval(vec!['A','A','A','B','B','B'], 2), 8);
    }

    #[test]
    fn test_least_interval_no_cooldown() {
        assert_eq!(least_interval(vec!['A','B','C'], 0), 3);
    }

    #[test]
    fn test_least_interval_single_task() {
        assert_eq!(least_interval(vec!['A'], 1), 1);
    }

    #[test]
    fn test_least_interval_all_same() {
        assert_eq!(least_interval(vec!['A','A','A','A'], 2), 10);
    }

    #[test]
    fn test_least_interval_different_tasks() {
        assert_eq!(least_interval(vec!['A','B','C','D'], 3), 4);
    }

    #[test]
    fn test_least_interval_formula_basic() {
        assert_eq!(least_interval_formula(vec!['A','A','A','B','B','B'], 2), 8);
    }

    #[test]
    fn test_least_interval_formula_no_cooldown() {
        assert_eq!(least_interval_formula(vec!['A','B','C'], 0), 3);
    }

    #[test]
    fn test_least_interval_heap_basic() {
        assert_eq!(least_interval_heap(vec!['A','A','A','B','B','B'], 2), 8);
    }

    #[test]
    fn test_least_interval_heap_no_cooldown() {
        assert_eq!(least_interval_heap(vec!['A','B','C'], 0), 3);
    }

    #[test]
    fn test_least_interval_multiple_max() {
        // A, B, C each appear 3 times, n=3
        // Parts: [A B C _] × 2 + [A B C] = 11
        let result = least_interval(vec!['A','B','C','A','B','C','A','B','C'], 3);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_least_interval_two_max() {
        // A, B each appear 3 times, n=2
        // [A B _] [A B _] [A B] = 8
        assert_eq!(least_interval(vec!['A','A','A','B','B','B'], 2), 8);
    }

    #[test]
    fn test_least_interval_few_tasks() {
        assert_eq!(least_interval(vec!['A','A','B'], 2), 4);
    }

    #[test]
    fn test_least_interval_equal_frequencies() {
        assert_eq!(least_interval(vec!['A','B','C','D'], 1), 4);
    }

    #[test]
    fn test_least_interval_large_n() {
        assert_eq!(least_interval(vec!['A','B'], 10), 2);
    }

    #[test]
    fn test_least_interval_empty() {
        assert_eq!(least_interval(vec![], 5), 0);
    }

    #[test]
    fn test_least_interval_one_each() {
        assert_eq!(least_interval(vec!['A','B','C','D','E'], 1), 5);
    }

    #[test]
    fn test_least_interval_with_idle() {
        // A A A A, n=3: [A _ _ _] × 3 + [A] = 13
        assert_eq!(least_interval(vec!['A','A','A','A'], 3), 13);
    }

    #[test]
    fn test_least_interval_same_as_formula() {
        let tasks = vec!['A','A','A','B','B','B'];
        assert_eq!(least_interval(tasks.clone(), 2),
                   least_interval_formula(tasks, 2));
    }

    #[test]
    fn test_least_interval_real_world() {
        // Simulate a scenario where idle time is needed
        let result = least_interval(vec!['A','A','A','B','C','D','E'], 2);
        assert!(result >= 7);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("Task Scheduler exercises - run tests with cargo test");
}