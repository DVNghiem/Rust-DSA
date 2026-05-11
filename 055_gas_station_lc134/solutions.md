# Gas Station Solution - LeetCode 134

## Solution Code

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

## Line-by-Line Analysis

### Lines 1-2: Function Signature
```rust
/// Approach 1: Greedy (Optimal O(n))
pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
```
- **Input**: `gas` - gas available at each station, `cost` - gas needed to reach next station
- **Output**: Starting station index, or -1 if impossible
- **Guarantee**: If total gas >= total cost, a solution exists

### Lines 3-5: Initialize Variables
```rust
    let mut total_tank = 0;
    let mut curr_tank = 0;
    let mut start = 0;
```
- **total_tank**: Tracks overall net gas (gas - cost) across all stations
- **curr_tank**: Tracks current journey's net gas from starting point
- **start**: Potential starting station index

### Lines 6-12: Single Pass Algorithm
```rust
    for i in 0..gas.len() {
        total_tank += gas[i] - cost[i];
        curr_tank += gas[i] - cost[i];
        if curr_tank < 0 {
            start = (i + 1) as i32;
            curr_tank = 0;
        }
    }
```
- **total_tank update**: Accumulate net gas for all stations
- **curr_tank update**: Accumulate net gas for current attempt
- **Failure detection**: If curr_tank goes negative at station i, we cannot start before i
- **Reset**: Set new potential start to i+1 and reset curr_tank

### Lines 13-14: Final Validation
```rust
    if total_tank >= 0 { start } else { -1 }
```
- **Total check**: If total_tank >= 0, total gas is sufficient for a complete circuit
- **Return**: Return the computed start, or -1 if no solution exists

## Key Insight: Why the Greedy Works

### The Proof

If starting from station `S` fails at station `F` (we run out of gas), then no station between `S` and `F` can be the answer.

**Reason**: When we fail at `F`, we had `curr_tank < 0`. This means even with the best start between `S` and `F`, we couldn't accumulate enough gas to reach `F`. If we started later (closer to `F`), we'd have even less accumulated gas.

### Visual Example

```
gas  = [1, 2, 3, 4, 5]
cost = [3, 4, 5, 1, 2]
diff = [-2,-2,-2, 3, 3]

Starting at 0:
  At station 0: curr_tank = -2 < 0 → FAIL
  
This means:
  - Station 0 cannot be the start
  - Station 1 cannot be the start (we already passed through it)
  - Station 2 cannot be the start (we already passed through it)
  
Only station 3 can be the start.
```

## Brute Force Comparison

```rust
pub fn can_complete_circuit_brute(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    for start in 0..gas.len() {           // O(n) starts
        let mut tank = 0;
        for i in 0..gas.len() {           // O(n) per start
            let idx = (start + i) % gas.len();
            tank += gas[idx] - cost[idx];
            if tank < 0 { break; }
        }
        if tank >= 0 { return start as i32; }
    }
    -1
}
```
- **Time**: O(n²) - try each station as starting point
- **Space**: O(1) - only local variables

## Complexity Comparison

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Greedy | O(n) | O(1) | Single pass |
| Brute Force | O(n²) | O(1) | Check each start |

## Step-by-Step Execution Trace

### Input: `gas = [1,2,3,4,5]`, `cost = [3,4,5,1,2]`

```
i=0: total_tank += 1-3 = -2
      curr_tank += 1-3 = -2
      curr_tank < 0, start=1, curr_tank=0

i=1: total_tank += 2-4 = -4
      curr_tank += 2-4 = -2
      curr_tank < 0, start=2, curr_tank=0

i=2: total_tank += 3-5 = -6
      curr_tank += 3-5 = -2
      curr_tank < 0, start=3, curr_tank=0

i=3: total_tank += 4-1 = -3
      curr_tank += 4-1 = 3
      curr_tank >= 0, continue

i=4: total_tank += 5-2 = 0
      curr_tank += 5-2 = 6
      curr_tank >= 0, continue

total_tank = 0 >= 0, return start = 3
```

### Verification

Starting at station 3:
```
Station 3: tank = 4, cost to go to 4 = 1 → tank = 3
Station 4: tank = 3 + 5 = 8, cost to go to 0 = 2 → tank = 6
Station 0: tank = 6 + 1 = 7, cost to go to 1 = 3 → tank = 4
Station 1: tank = 4 + 2 = 6, cost to go to 2 = 4 → tank = 2
Station 2: tank = 2 + 3 = 5, cost to go to 3 = 5 → tank = 0
Complete!
```

## Edge Cases

### No Solution
```rust
Input: gas = [2,3,4], cost = [3,4,5]
diff = [-1,-1,-1]

total_tank = -3 < 0 → return -1
```

### Single Station - Valid
```rust
Input: gas = [5], cost = [2]
diff = [3]

total_tank = 3 >= 0 → return 0
```

### Single Station - Invalid
```rust
Input: gas = [1], cost = [2]
diff = [-1]

total_tank = -1 < 0 → return -1
```

## Why This Works

The problem has a special structure:
1. If total_gas >= total_cost, a solution always exists
2. If you can't go from A to B, no station between A and B can be the solution

The algorithm leverages both properties in a single pass.

## Common Mistakes

1. **Not checking total_tank**: Would return a start even when no solution exists
2. **Off-by-one on start**: When failing at index i, next possible start is i+1 (modulo n)
3. **Not resetting curr_tank**: Would cause incorrect state for next attempt

## Rust-Specific Patterns

1. **Integer casting**: `(i + 1) as i32` for proper return type
2. **Vec indexing**: Direct `gas[i]` access (bounds checked in debug)
3. **Single pass**: No intermediate storage, just tracking state
4. **Early exit**: Return immediately after checking solution existence