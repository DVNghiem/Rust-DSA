# Combination Sum Solution - LeetCode 39 (Complete)

## Solution Analysis

### Backtracking with Index Tracking

```rust
pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut path = Vec::new();

    fn backtrack(start: usize, candidates: &Vec<i32>, target: i32, path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if target == 0 {
            result.push(path.clone());
            return;
        }
        for i in start..candidates.len() {
            if candidates[i] <= target {
                path.push(candidates[i]);
                backtrack(i, candidates, target - candidates[i], path, result);
                path.pop();
            }
        }
    }

    backtrack(0, &candidates, target, &mut path, &mut result);
    result
}
```

## Why Start Index Prevents Duplicates

Without start index:
```
candidates = [2, 3], target = 6

Path [2,2,2] could come from:
- Start with 2, then choose 2, then choose 2
- Start with 3 can't contribute

But [2,3] and [3,2] BOTH appear without start!
```

With start=i:
```
After choosing 2 at index 0, next recursion starts at 0
After choosing 3 at index 1, next recursion starts at 1

[2,2,2] ✓
[2,3] ✓
[3,3] ✓

No duplicates because we enforce ordering!
```

## Pruning Optimization

```rust
if candidates[i] <= target {
    // proceed
}
// else: skip, candidate too large
```

This prevents exploring paths that immediately fail.

## Sorted Version with Early Break

```rust
candidates.sort();
// ...
for i in start..candidates.len() {
    if candidates[i] > target {
        break;  // All subsequent are also too large!
    }
}
```

Sorting enables early termination when candidates[i] exceeds remaining target.

## Complexity Analysis

| Approach | Time | Space |
|----------|------|-------|
| Backtrack (unsorted) | O(n × target) | O(target) |
| Backtrack (sorted) | O(n × target) with pruning | O(target) |

## Step-by-Step Trace

For `candidates = [2, 3, 6, 7]`, `target = 7`:

```
backtrack(start=0, target=7, path=[])
├── Try 2 (2 <= 7)
│   └── backtrack(start=0, target=5, path=[2])
│       ├── Try 2 (2 <= 5) → add [2,2]
│       │   └── backtrack(start=0, target=3, path=[2,2])
│       │       └── Try 3 → add [2,2,3] ✓
│       └── Try 3 (3 <= 5) → add [2,3]
│       └── Try 6,7 → skip (too large)
├── Try 3 (3 <= 7)
│   └── backtrack(start=1, target=4, path=[3])
│       └── Try 3 (3 <= 4) → add [3,3]
├── Try 6,7 → skip

Result: [[2,2,3], [7]]
```

## Edge Cases

1. **Empty candidates**: Returns `[]`
2. **No solution**: Returns `[]`
3. **Target is zero**: Returns `[[]]` (empty combination)

## Common Mistakes

1. **Not passing start=i**: Causes duplicate combinations
2. **Forgetting to check <= target**: Can cause infinite recursion or wrong results
3. **Forgetting to pop**: Leaves stale elements in path