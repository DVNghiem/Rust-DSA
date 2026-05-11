# Task Scheduler - LeetCode 621

## Problem Overview

Given a characters array `tasks` and an integer `n` representing the cooldown period, return the minimum interval needed to finish all tasks. CPU can either execute a task or idle.

**Examples:**
```
Input: tasks = ["A","A","A","B","B","B"], n = 2
Output: 8 (A → idle → A → idle → A → B → B → B)

Input: tasks = ["A","A","A","B","C","D","E"], n = 2
Output: 8
```

## Theory

### Key Insight: Max Frequency Task

The most frequent task determines the lower bound:
```
If A appears 3 times and n = 2:
A _ _ A _ _ A
```
Result: (max_count - 1) * (n + 1) + num_max_tasks

## Implementation

```rust
pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
    use std::collections::HashMap;
    let mut freq = HashMap::new();
    for t in &tasks {
        *freq.entry(t).or_insert(0) += 1;
    }

    let max_freq = freq.values().max().copied().unwrap_or(0);
    let num_max = freq.values().filter(|&&v| v == max_freq).count();

    let part_len = n as usize + 1;
    let num_parts = max_freq - 1;
    let available = num_parts * part_len;
    let remaining = tasks.len() - num_max;

    let idle_slots = num_parts * part_len - num_max.max(remaining);
    tasks.len() as i32 + idle_slots.max(0) as i32
}
```

## Complexity Analysis

| Approach | Time | Space |
|----------|------|-------|
| Formula | O(n) | O(1) |

## Test Cases

```rust
#[test]
fn test_least_interval_basic() {
    assert_eq!(least_interval(vec!['A','A','A','B','B','B'], 2), 8);
}
```