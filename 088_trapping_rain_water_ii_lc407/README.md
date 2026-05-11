# Trapping Rain Water II - LeetCode 407

## Problem Statement

Given an m x n matrix of heights, calculate how much water can be trapped after raining.

```
Input: heightMap = [
  [1,4,3,1,3,2],
  [3,2,1,3,2,4],
  [2,3,3,2,3,1]
]
Output: 4 (units of water)
```

## Visual Explanation

```
Water trapped between barriers:
    ▼ water
[1, 4, 3, 1, 3, 2]
  ┌───────────────┐
[3, 2, 1, 3, 2, 4]│
  │   water   │   │
  └───────────┘   │
[2, 3, 3, 2, 3, 1] │
```

## Key Insight: Priority Queue + BFS

Use a min-heap to process cells from outside inward. The water level at each cell is determined by the maximum height along the boundary path.

```
Algorithm:
1. Add all border cells to min-heap
2. Process cells in order of height (lowest first)
3. For each cell, try to "flow" to neighbors
4. If neighbor is lower, water can be trapped
5. Track the maximum water level seen so far
```

## Data Structures

```rust
// Min-heap for cells ordered by height
// (height, row, col)
let mut heap: BinaryHeap<(i32, i32, i32), Reversed> = ...

// Visited cells
let mut visited = vec![vec![false; n]; m];
```

## Complexity: O(mn log(mn)) time, O(mn) space

## Test Cases
- Empty grid → 0
- Single cell → 0
- Already low in center → water
- All same height → 0
- Peak in center → water around it