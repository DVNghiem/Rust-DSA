# Reconstruct Itinerary - LeetCode 332

## Problem

Given a list of airline tickets represented by pairs `from -> to`, reconstruct the itinerary in order. Each ticket is used exactly once, and the itinerary must start from "JFK".

```
Example:
Tickets: [["MUC", "LHR"], ["MUC", "SFO"], ["SFO", "MUC"]]
Reconstructed: ["JFK", "MUC", "LHR"]  (or ["JFK", "MUC", "SFO", "MUC", "LHR"])
```

## Hierholzer's Algorithm

This problem is a special case of finding an Eulerian path in a directed graph:
- Each ticket is an edge from `from` airport to `to` airport
- We need to visit every edge exactly once
- Start from "JFK"

### Algorithm Steps

1. Build adjacency list from tickets
2. Sort destinations alphabetically (to get lexicographically smallest path)
3. Use Hierholzer's algorithm to find the path
4. Reverse the result to get the correct order

### Visual Walkthrough

```
Tickets: JFK -> MUC, MUC -> LHR, MUC -> SFO, SFO -> MUC

Graph:
    JFK ---> MUC
           /     \
         LHR      SFO ---> MUC

Hierholzer's Algorithm:
1. Start at JFK
2. Follow edges (MUC), (LHR) backtracking: LHR -> MUC -> JFK
3. Follow edges (MUC), (SFO), (MUC) backtracking: MUC -> SFO -> MUC -> JFK
4. Combined: JFK -> MUC -> LHR | JFK -> MUC -> SFO -> MUC

Result (reversed): JFK -> MUC -> SFO -> MUC -> LHR
```

## Topics Covered
- Graph theory / Eulerian path
- Hierholzer's algorithm
- DFS traversal
- Priority queue / Sorting
- Path reconstruction

## Approaches

### Approach 1: Hierholzer's Algorithm with Stack (Recommended)

Time: O(E log E) for sorting | Space: O(E)
- Uses stack to simulate recursive DFS
- Backtracking builds answer in correct order
- Most efficient for Eulerian path problems

### Approach 2: Naive DFS (Recursion)

Time: O(E!) in worst case | Space: O(E)
- Try all possible paths
- Backtrack when stuck
- Inefficient but conceptually simple

### Approach 3: Iterative with Priority Queue

Time: O(E log E) | Space: O(E)
- Use min-heap to always take lexicographically smallest next destination
- Equivalent to sorting approach

## Complexity Analysis

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Hierholzer (Stack) | O(E log E) | O(E) | Best - sorts edges |
| Naive DFS | O(E!) | O(E) | Exponential - tries all paths |
| Priority Queue | O(E log E) | O(E) | Similar to Approach 1 |

E = number of edges (tickets)

## Additional Notes

- This is finding an Eulerian path in a directed graph where all edges are used exactly once
- The graph is guaranteed to have a valid solution (problem constraint)
- Sorting ensures lexicographically smallest valid itinerary
- Hierholzer's algorithm is optimal: O(E log E) vs exponential naive DFS
