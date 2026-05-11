# Pacific Atlantic Water Flow - LeetCode 417

## Problem

Given an m x n matrix of heights, find cells where water can flow to both the Pacific and Atlantic oceans.

```
Pacific: top and left borders
Atlantic: bottom and right borders
```

## Multi-Source BFS

Start BFS from both oceans and find cells reachable from both.