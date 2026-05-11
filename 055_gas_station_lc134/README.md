# Gas Station - LeetCode 134

## Problem Overview

Given `n` gas stations numbered 0 to n-1, you have `gas[i]` units of gas at station i, and `cost[i]` to travel to the next station. Return the starting station's index where you can travel around the circuit once, or -1 if impossible.

**Examples:**
```
Input: gas = [1,2,3,4,5], cost = [3,4,5,1,2]
Output: 3

Input: gas = [2,3,4], cost = [3,4,5]
Output: -1
```

## Theory

### Key Insight: Greedy Algorithm

If total gas >= total cost, a solution always exists. The trick is finding the start.

**Observation**: If starting at station A fails at station B, no station between A and B can be the start.

### Visual Proof

```
gas  = [1, 2, 3, 4, 5]
cost = [3, 4, 5, 1, 2]
diff = [-2,-2,-2, 3, 3]

If start at 0:
  0 → 1: -2 gas (fail)
  
This means start cannot be 0, 1, 2
Only possible start is 3 or 4

Starting at 3:
  3 → 4: 4 + 3 = 7 ✓
  4 → 0: 5 + 3 = 8 ✓
  0 → 1: 1 + 8 = 9 ✓
  1 → 2: 2 + 9 = 11 ✓
  2 → 3: 3 + 11 = 14 ✓
  Success!
```

## Approaches

### Approach 1: Greedy (Optimal)

```rust
pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let mut total_tank = 0;
    let mut curr_tank = 0;
    let mut start = 0;

    for i in 0..gas.len() {
        total_tank += gas[i] - cost[i];
        curr_tank += gas[i] - cost[i];
        if curr_tank < 0 {
            start = (i + 1) as i32;
            curr_tank = 0;
        }
    }

    if total_tank >= 0 { start } else { -1 }
}
```

### Approach 2: Brute Force

```rust
pub fn can_complete_circuit_brute(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    for start in 0..gas.len() {
        let mut tank = 0;
        for i in 0..gas.len() {
            let idx = (start + i) % gas.len();
            tank += gas[idx] - cost[idx];
            if tank < 0 { break; }
        }
        if tank >= 0 { return start as i32; }
    }
    -1
}
```

## Complexity Analysis

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Greedy | O(n) | O(1) | Optimal solution |
| Brute Force | O(n²) | O(1) | Naive |

## Edge Cases

1. **No solution**: total_gas < total_cost
2. **Single station**: Works if gas >= cost
3. **All zeros**: gas=[0,0], cost=[0,0] → start=0
4. **Negative diffs everywhere**: No solution

## Test Cases

```rust
#[test]
fn test_gas_station_basic() {
    assert_eq!(can_complete_circuit(vec![1,2,3,4,5], vec![3,4,5,1,2]), 3);
}
```