# Course Schedule II - LeetCode 210

## Problem

Find the order of courses to take given prerequisites.

```
Input: numCourses=4, prerequisites=[[1,0],[2,0],[3,1],[3,2]]
Output: [0,2,1,3]
```

## Topological Sort with BFS

```rust
pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let n = numCourses as usize;
    let mut graph = vec![vec![]; n];
    let mut in_degree = vec![0; n];

    for prereq in prerequisites {
        let (course, pre) = (prereq[0] as usize, prereq[1] as usize);
        graph[pre].push(course);
        in_degree[course] += 1;
    }

    // BFS topological sort
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

    if result.len() == n as usize { result } else { vec![] }
}
```

## Complexity: O(V+E) time and space