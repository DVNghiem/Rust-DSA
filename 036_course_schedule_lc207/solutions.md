# Course Schedule - Solution Analysis

## Problem Overview

Given numCourses and prerequisite pairs [a, b] meaning "to take course a, must first take course b", determine if all courses can be finished. This is essentially detecting if the prerequisite graph has a cycle.

## Solution 1: Kahn's Algorithm (BFS Topological Sort)

### Code Implementation

```rust
pub fn can_finish(numCourses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let n = numCourses as usize;
    if n == 0 { return true; }

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    let mut in_degree = vec![0usize; n];

    // Build graph: prereq_course -> [courses that need it]
    for prereq in &prerequisites {
        let course = prereq[0] as usize;
        let prereq_course = prereq[1] as usize;
        graph[prereq_course].push(course);
        in_degree[course] += 1;
    }

    // Start with courses having no prerequisites
    let mut queue = VecDeque::new();
    for i in 0..n {
        if in_degree[i] == 0 {
            queue.push_back(i);
        }
    }

    let mut count = 0;
    while let Some(course) = queue.pop_front() {
        count += 1;
        // Remove this course's edges
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

### Line-by-Line Analysis

1. **`let n = numCourses as usize; if n == 0 { return true; }`**: Handle edge case of 0 courses.

2. **`let mut graph: Vec<Vec<usize>> = vec![vec![]; n];`**: Adjacency list - for each course, list of courses that depend on it.

3. **`let mut in_degree = vec![0usize; n];`**: In-degree = number of prerequisites for each course.

4. **`graph[prereq_course].push(course); in_degree[course] += 1;`**: Edge from prereq_course to course. If you take prereq_course, you can then take course.

5. **`if in_degree[i] == 0 { queue.push_back(i); }`**: Courses with no prerequisites can be started immediately.

6. **`while let Some(course) = queue.pop_front() { count += 1; ... }`**: Process each course from queue.

7. **`in_degree[next] -= 1; if in_degree[next] == 0 { queue.push_back(next); }`**: After "removing" this course, reduce in-degree of dependent courses. If becomes 0, they can now be started.

8. **`count == n`**: If we processed all courses, no cycle. If less, cycle prevented some processing.

### Kahn's Algorithm Visualization

```
prerequisites = [[1,0], [2,0], [3,1], [3,2]]

Graph:
0 → 1 → 3
  → 2 ↗

In-degrees: [0, 1, 1, 2]

Step 1: Queue starts with {0} (in-degree 0)
Process 0: count=1, reduce in-degrees of 1,2
In-degrees: [0, 0, 0, 2]
Queue: {1, 2}

Step 2: Process 1: count=2, reduce in-degree of 3
In-degrees: [0, 0, 0, 1]
Queue: {2}

Step 3: Process 2: count=3, reduce in-degree of 3
In-degrees: [0, 0, 0, 0]
Queue: {3}

Step 4: Process 3: count=4
Queue: empty

count = 4 = n → No cycle → Can finish ✓
```

## Solution 2: DFS Cycle Detection

```rust
pub fn can_finish_dfs(numCourses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let n = numCourses as usize;
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for prereq in &prerequisites {
        graph[prereq[1] as usize].push(prereq[0] as usize);
    }

    let mut state = vec![0u8; n]; // 0=unvisited, 1=visiting, 2=done

    fn dfs(graph: &[Vec<usize>], state: &mut [u8], node: usize) -> bool {
        if state[node] == 1 { return true; }  // Back edge = cycle
        if state[node] == 2 { return false; }

        state[node] = 1;
        for &next in &graph[node] {
            if dfs(graph, state, next) { return true; }
        }
        state[node] = 2;
        false
    }

    for i in 0..n {
        if state[i] == 0 {
            if dfs(&graph, &mut state, i) { return false; }
        }
    }
    true
}
```

### DFS State Machine

```
State transitions:
0 (unvisited) → 1 (visiting) → 2 (finished)

Cycle detection:
If during DFS we encounter a node in state 1 (visiting),
it means we're going back to a node in current recursion stack.
This indicates a cycle.
```

## Solution 3: Union-Find

```rust
pub fn can_finish_union_find(numCourses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let n = numCourses as usize;
    let mut parent: Vec<usize> = (0..n).collect();

    fn find(parent: &mut [usize], x: usize) -> usize {
        if parent[x] != x {
            parent[x] = find(parent, parent[x]);
        }
        parent[x]
    }

    fn union(parent: &mut [usize], rank: &mut [usize], x: usize, y: usize) -> bool {
        let px = find(parent, x);
        let py = find(parent, y);
        if px == py { return false; } // Cycle!
        // ... union logic
        true
    }

    for prereq in &prerequisites {
        // course depends on prereq_course
        // If already in same set, adding edge creates cycle
        if find(&mut parent, prereq[0] as usize) == find(&mut parent, prereq[1] as usize) {
            return false;
        }
        union(...);
    }
    true
}
```

## Complexity Comparison

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Kahn's (BFS) | O(V+E) | O(V+E) | Clean, intuitive |
| DFS | O(V+E) | O(V+E) | Recursive, need state tracking |
| Union-Find | O(V+E×α(n)) | O(V) | Almost O(1) per operation |

## Key Insights

1. **DAG detection = course completion**: If prerequisite graph is acyclic, all courses can be completed.

2. **Topological sort gives order**: Kahn's not only detects cycle but also produces a valid course order.

3. **In-degree 0 = can start**: Courses with no prerequisites can always be started.

4. **Removing edges reduces problem**: When we "complete" a course, we effectively remove its outgoing edges, which may enable other courses to start.

## Test Case Analysis

### Test: `test_basic_invalid`

```
prerequisites = [[1,0], [0,1]]
Course 1 depends on 0, course 0 depends on 1.

In-degrees: [1, 1]
No course has in-degree 0!
Queue starts empty.
count = 0 < n = 2 → Cycle detected → false ✓
```

### Test: `test_linear_chain`

```
prerequisites = [[1,0], [2,1], [3,2]]
0 → 1 → 2 → 3

In-degrees: [0, 1, 1, 1]
Queue starts with {0}

Process 0 → in-degree[1] = 0, queue {1}
Process 1 → in-degree[2] = 0, queue {2}
Process 2 → in-degree[3] = 0, queue {3}
Process 3 → count = 4 = n → No cycle ✓
```

## Follow-up Answers

**Q: Why Kahn's algorithm works?**
A: It simulates course completion. If there's a cycle, no course in that cycle will ever reach in-degree 0, so we won't process them all.

**Q: How to return actual order?**
A: Modify Kahn's to record the order we process nodes. That's a valid topological ordering.

**Q: Union-Find cycle detection logic?**
A: If two nodes are already connected (same set), adding an edge between them creates a cycle. This is because there exists a path between them already.

**Q: Time complexity O(V+E)?**
A: Building graph O(E), processing each node once O(V), each edge once O(E). Total O(V+E).

**Q: Space complexity O(V+E)?**
A: Graph stores adjacency list O(V+E), plus queues/auxiliary structures O(V).