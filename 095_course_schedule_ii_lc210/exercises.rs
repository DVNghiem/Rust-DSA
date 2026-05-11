//! Exercises for Course Schedule II (LeetCode 210)
//!
//! # Topics Covered
//! - Topological sort
//! - Course prerequisites
//! - BFS/DFS approaches
//! - Cycle detection
//!
//! # Difficulty: Medium

use std::collections::{HashSet, VecDeque};

/// BFS topological sort approach
pub fn find_order_bfs(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let n = num_courses as usize;
    let mut graph = vec![vec![]; n];
    let mut in_degree = vec![0; n];

    for prereq in &prerequisites {
        let course = prereq[0] as usize;
        let pre = prereq[1] as usize;
        graph[pre].push(course);
        in_degree[course] += 1;
    }

    let mut queue = VecDeque::new();
    for i in 0..n {
        if in_degree[i] == 0 {
            queue.push_back(i);
        }
    }

    let mut result = vec![];
    while let Some(course) = queue.pop_front() {
        result.push(course as i32);
        for &next in &graph[course] {
            in_degree[next] -= 1;
            if in_degree[next] == 0 {
                queue.push_back(next);
            }
        }
    }

    if result.len() == n { result } else { vec![] }
}

/// DFS approach (post-order traversal)
pub fn find_order_dfs(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let n = num_courses as usize;
    let mut graph = vec![vec![]; n];
    let mut visited = vec![0; n]; // 0=unvisited, 1=visiting, 2=done

    for prereq in &prerequisites {
        let course = prereq[0] as usize;
        let pre = prereq[1] as usize;
        graph[pre].push(course);
    }

    let mut result = vec![];
    let mut has_cycle = false;

    fn dfs(node: usize, graph: &[Vec<usize>], visited: &mut [i32], result: &mut Vec<usize>, has_cycle: &mut bool) {
        if *has_cycle { return; }
        if visited[node] == 1 {
            *has_cycle = true;
            return;
        }
        if visited[node] == 2 {
            return;
        }

        visited[node] = 1;
        for &next in &graph[node] {
            dfs(next, graph, visited, result, has_cycle);
        }
        visited[node] = 2;
        result.push(node);
    }

    for i in 0..n {
        if visited[i] == 0 {
            dfs(i, &graph, &mut visited, &mut result, &mut has_cycle);
        }
    }

    if has_cycle { vec![] } else { result.into_iter().rev().map(|x| x as i32).collect() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_example() {
        let result = find_order_bfs(4, vec![vec![1,0], vec![2,0], vec![3,1], vec![3,2]]);
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_no_prerequisites() {
        let result = find_order_bfs(2, vec![]);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_single_course() {
        let result = find_order_bfs(1, vec![]);
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn test_linear_chain() {
        let result = find_order_bfs(3, vec![vec![1,0], vec![2,1]]);
        // 0 -> 1 -> 2
        assert_eq!(result, vec![0, 1, 2]);
    }

    #[test]
    fn test_two_prerequisites() {
        let result = find_order_bfs(4, vec![vec![1,0], vec![2,0], vec![3,1], vec![3,2]]);
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_dfs_basic() {
        let bfs = find_order_bfs(4, vec![vec![1,0], vec![2,0], vec![3,1], vec![3,2]]);
        let dfs = find_order_dfs(4, vec![vec![1,0], vec![2,0], vec![3,1], vec![3,2]]);
        // Both should have same length (valid) and all same courses
        let mut bfs_sorted = bfs.clone();
        let mut dfs_sorted = dfs.clone();
        bfs_sorted.sort();
        dfs_sorted.sort();
        assert_eq!(bfs_sorted, dfs_sorted);
    }

    #[test]
    fn test_dfs_no_prereq() {
        let result = find_order_dfs(2, vec![]);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_self_loop() {
        // Self loop means cycle
        let result = find_order_bfs(1, vec![vec![0,0]]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_simple_two() {
        let result = find_order_bfs(2, vec![vec![1,0]]);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_three_courses() {
        let result = find_order_bfs(3, vec![vec![0,1], vec![1,2], vec![2,0]]);
        // Cycle detected
        assert!(result.is_empty());
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("095_course_schedule_ii_lc210 exercises - run tests with cargo test");
}
