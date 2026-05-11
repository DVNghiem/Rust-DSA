use std::collections::{HashSet, VecDeque};

/// Approach 1: Kahn's Algorithm (BFS-based Topological Sort)
///
/// Build graph, compute in-degrees.
/// Start with courses that have no prerequisites (in-degree 0).
/// Process them, reducing in-degree of dependents.
/// If all courses processed → no cycle → true.
pub fn can_finish(numCourses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let n = numCourses as usize;
    if n == 0 { return true; }

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    let mut in_degree = vec![0usize; n];

    // Build adjacency list and in-degrees
    for prereq in &prerequisites {
        let course = prereq[0] as usize;
        let prereq_course = prereq[1] as usize;
        graph[prereq_course].push(course);
        in_degree[course] += 1;
    }

    // Queue courses with no prerequisites
    let mut queue = VecDeque::new();
    for i in 0..n {
        if in_degree[i] == 0 {
            queue.push_back(i);
        }
    }

    let mut count = 0;
    while let Some(course) = queue.pop_front() {
        count += 1;
        for &next in &graph[course] {
            in_degree[next] -= 1;
            if in_degree[next] == 0 {
                queue.push_back(next);
            }
        }
    }

    // If processed all courses, no cycle
    count == n
}

/// Approach 2: DFS Cycle Detection
///
/// Use DFS to detect if cycle exists.
/// States: 0 = unvisited, 1 = visiting (in stack), 2 = visited (done)
/// If we encounter a node in "visiting" state during DFS, cycle exists.
pub fn can_finish_dfs(numCourses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let n = numCourses as usize;
    if n == 0 { return true; }

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for prereq in &prerequisites {
        let course = prereq[0] as usize;
        let prereq_course = prereq[1] as usize;
        graph[prereq_course].push(course);
    }

    // 0 = unvisited, 1 = visiting (in current path), 2 = finished
    let mut state = vec![0u8; n];

    fn dfs(graph: &[Vec<usize>], state: &mut [u8], node: usize) -> bool {
        if state[node] == 1 { return true; }  // Cycle detected
        if state[node] == 2 { return false; } // Already processed, no cycle from here

        state[node] = 1; // Mark as visiting
        for &next in &graph[node] {
            if dfs(graph, state, next) { return true; }
        }
        state[node] = 2; // Mark as finished
        false
    }

    for i in 0..n {
        if state[i] == 0 {
            if dfs(&graph, &mut state, i) { return false; }
        }
    }

    true
}

/// Approach 3: Union-Find (Cycle Detection)
///
/// Use Disjoint Set Union to detect if adding an edge creates a cycle.
/// If two nodes are already in same set, adding edge between them creates cycle.
pub fn can_finish_union_find(numCourses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let n = numCourses as usize;
    if n == 0 { return true; }

    // DSU implementation
    let mut parent: Vec<usize> = (0..n).collect();
    let mut rank = vec![0usize; n];

    fn find(parent: &mut [usize], x: usize) -> usize {
        if parent[x] != x {
            parent[x] = find(parent, parent[x]);
        }
        parent[x]
    }

    fn union(parent: &mut [usize], rank: &mut [usize], x: usize, y: usize) -> bool {
        let px = find(parent, x);
        let py = find(parent, y);
        if px == py { return false; } // Already in same set - cycle!

        if rank[px] < rank[py] {
            parent[px] = py;
        } else if rank[px] > rank[py] {
            parent[py] = px;
        } else {
            parent[py] = px;
            rank[px] += 1;
        }
        true
    }

    for prereq in &prerequisites {
        let course = prereq[0] as usize;
        let prereq_course = prereq[1] as usize;
        // course depends on prereq_course, so edge: prereq_course -> course
        // For cycle detection, check if they share same root before union
        if find(&mut parent, course) == find(&mut parent, prereq_course) {
            return false; // Cycle detected
        }
        union(&mut parent, &mut rank, course, prereq_course);
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_valid() {
        // 0 -> 1 (take 0 first to take 1)
        let result = can_finish(2, vec![vec![1, 0]]);
        assert!(result);
    }

    #[test]
    fn test_basic_invalid() {
        // 0 -> 1 and 1 -> 0 (cycle!)
        let result = can_finish(2, vec![vec![1, 0], vec![0, 1]]);
        assert!(!result);
    }

    #[test]
    fn test_no_prerequisites() {
        let result = can_finish(3, vec![]);
        assert!(result);
    }

    #[test]
    fn test_single_course() {
        let result = can_finish(1, vec![]);
        assert!(result);
    }

    #[test]
    fn test_linear_chain() {
        // 0 -> 1 -> 2 -> 3
        let result = can_finish(4, vec![vec![1, 0], vec![2, 1], vec![3, 2]]);
        assert!(result);
    }

    #[test]
    fn test_diamond_dependency() {
        //   0
        //  / \
        // 1   2
        //  \ /
        //   3
        // 0 -> 1, 0 -> 2, 1 -> 3, 2 -> 3
        let result = can_finish(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]);
        assert!(result);
    }

    #[test]
    fn test_self_loop() {
        // Course 0 requires itself
        let result = can_finish(1, vec![vec![0, 0]]);
        assert!(!result);
    }

    #[test]
    fn test_multiple_independent_paths() {
        // Path 0->1 and path 2->3
        let result = can_finish(4, vec![vec![1, 0], vec![3, 2]]);
        assert!(result);
    }

    #[test]
    fn test_large_chain() {
        let mut prereqs = Vec::new();
        for i in 1..100 {
            prereqs.push(vec![i, i - 1]);
        }
        let result = can_finish(100, prereqs);
        assert!(result);
    }

    #[test]
    fn test_large_chain_with_cycle() {
        let mut prereqs = Vec::new();
        for i in 1..100 {
            prereqs.push(vec![i, i - 1]);
        }
        prereqs.push(vec![0, 99]); // Creates cycle
        let result = can_finish(100, prereqs);
        assert!(!result);
    }

    #[test]
    fn test_dfs_same_as_bfs() {
        let prereqs = vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]];
        let bfs = can_finish(4, prereqs.clone());
        let dfs = can_finish_dfs(4, prereqs.clone());
        assert_eq!(bfs, dfs);
    }

    #[test]
    fn test_union_find_same_as_bfs() {
        let prereqs = vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]];
        let bfs = can_finish(4, prereqs.clone());
        let uf = can_finish_union_find(4, prereqs.clone());
        assert_eq!(bfs, uf);
    }

    #[test]
    fn test_complex_dag() {
        // More complex graph
        let prereqs = vec![
            vec![1, 0],
            vec![2, 0],
            vec![3, 1],
            vec![3, 2],
            vec![4, 3],
            vec![5, 3],
        ];
        let result = can_finish(6, prereqs);
        assert!(result);
    }

    #[test]
    fn test_three_course_cycle() {
        // 0 -> 1 -> 2 -> 0
        let result = can_finish(3, vec![vec![1, 0], vec![2, 1], vec![0, 2]]);
        assert!(!result);
    }

    #[test]
    fn test_partial_cycle() {
        // 0 -> 1 -> 2 -> 3
        //          ↑
        //          └── 4 (4 -> 2 creates cycle)
        let result = can_finish(5, vec![vec![1, 0], vec![2, 1], vec![3, 2], vec![4, 3], vec![2, 4]]);
        assert!(!result);
    }

    #[test]
    fn test_disconnected_components() {
        // Component 1: 0 -> 1
        // Component 2: 2 -> 3
        // Component 3: 4 (isolated)
        let prereqs = vec![vec![1, 0], vec![3, 2]];
        let result = can_finish(5, prereqs);
        assert!(result);
    }

    #[test]
    fn test_zero_courses() {
        let result = can_finish(0, vec![]);
        assert!(result);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("036_course_schedule_lc207 exercises - run tests with cargo test");
}
