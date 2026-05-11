# Course Schedule - LeetCode 207

## Problem Statement

There are a total of `numCourses` courses labeled from `0` to `numCourses-1`. You are given an array `prerequisites` where `prerequisites[i] = [ai, bi]` indicates that you must take course `bi` first if you want to take course `ai`.

For example, to take course `0` you have to first take course `1`, which is expressed as: `prerequisites = [[0,1]]`.

Return `true` if you can finish all courses. Otherwise, return `false`.

## Visual Walkthrough

```
Example 1:
Input: numCourses = 2, prerequisites = [[1,0]]
Output: true

Explanation: Take course 1 first, then course 0. Possible.

Example 2:
Input: numCourses = 2, prerequisites = [[1,0],[0,1]]
Output: false

Explanation: 
- To take course 1, must take course 0 first
- To take course 0, must take course 1 first
- Circular dependency! Impossible.
```

### Cycle Detection Visualization

```
prerequisites = [[1,0],[0,1]]

Course 0 needs 1 → Course 1 needs 0 → Course 0 needs 1 → ...

0 → 1 → 0 (cycle!)
Cannot complete.

prerequisites = [[1,0],[2,0]]

0 → 1
0 → 2
No cycle. Can complete.
```

### Topological Sort (Kahn's Algorithm)

```
A directed graph with no cycles = DAG (Directed Acyclic Graph)
DAG can be topologically sorted.

Algorithm:
1. Calculate in-degree (number of incoming edges) for each node
2. Start with nodes that have in-degree 0 (no prerequisites)
3. Remove these nodes and their outgoing edges
4. Repeat until no nodes left or cycle detected

If all nodes removed → no cycle → can finish
If nodes remain → cycle exists → cannot finish
```

## Approach Comparison

| Approach | Time | Space | Description |
|----------|------|-------|-------------|
| Kahn's Algorithm | O(V+E) | O(V) | BFS-based, count processed nodes |
| DFS (recursive) | O(V+E) | O(V) | Detect back edges, check for cycle |
| DFS (iterative) | O(V+E) | O(V) | Stack-based |

### Why Topological Sort?

- Problem is essentially: "Is the prerequisite graph a DAG?"
- If DAG, can complete all courses
- If not DAG (has cycle), cannot complete

## Implementation Strategy

### Approach 1: Kahn's Algorithm (BFS)

```rust
use std::collections::VecDeque;

pub fn can_finish(numCourses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let n = numCourses as usize;
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    let mut in_degree = vec![0usize; n];

    for prereq in &prerequisites {
        let course = prereq[0] as usize;
        let prereq_course = prereq[1] as usize;
        graph[prereq_course].push(course);
        in_degree[course] += 1;
    }

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

    count == n
}
```

## Edge Cases

1. **No prerequisites**: All courses have in-degree 0 → can finish
2. **Single course**: No prerequisites → can finish
3. **Linear chain**: 0→1→2→3 → can finish
4. **Cycle**: Impossible
5. **Multiple disconnected components**: Each must be acyclic
6. **Self-loop**: course requires itself → immediate cycle

## Test Cases

1. Basic valid (no cycle)
2. Basic invalid (cycle)
3. No prerequisites
4. Single course
5. Linear chain
6. Diamond dependency
7. Self-loop
8. Multiple independent paths

## Solution Explanation

### Key Insight

The prerequisite graph is a directed graph. If there's a cycle, you can never break into the cycle to complete courses. If no cycle (DAG), you can topologically sort and complete all.

### Kahn's Algorithm

1. Build adjacency list and compute in-degrees
2. Queue all nodes with in-degree 0 (can start these courses)
3. Process nodes: reduce in-degree of neighbors, enqueue if becomes 0
4. If we process all n nodes → no cycle → return true
5. If we process fewer than n nodes → cycle exists → return false

## Complexity Analysis

- **Time**: O(V + E) where V = numCourses, E = prerequisites.length
- **Space**: O(V + E) for graph and auxiliary structures

## Follow-up Questions

1. How to return an actual course order?
2. What if multiple valid orderings exist?
3. How to find the cycle if it exists?