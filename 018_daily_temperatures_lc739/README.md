# Daily Temperatures (LeetCode #739)

## Problem Statement

Given an array of integers `temperatures` representing the daily temperatures, return an array `answer` such that `answer[i]` is the number of days you have to wait after the ith day to get a warmer temperature.

If there is no future day for which this is possible, put `answer[i] = 0`.

## Examples

```
Input: temperatures = [73, 74, 75, 71, 69, 72, 76, 73]
Output: [1, 1, 4, 2, 1, 1, 0, 0]

Input: temperatures = [30, 40, 50, 60]
Output: [1, 1, 1, 0]

Input: temperatures = [30, 60, 90]
Output: [1, 1, 0]
```

## Stack-Based Solution

### Key Insight

Use a decreasing stack (monotonic stack):
- Stack stores indices of days with temperatures in decreasing order
- When a warmer day is found, calculate days difference for all smaller temperatures

### Visual Walkthrough

```
temperatures = [73, 74, 75, 71, 69, 72, 76, 73]

Day 0: 73 - Push index 0
  stack: [0] (temp[0]=73)

Day 1: 74 > 73 (warmer than stack top)
  Pop 0, answer[0] = 1 - 0 = 1
  Push index 1
  stack: [1] (temp[1]=74)

Day 2: 75 > 74 (warmer)
  Pop 1, answer[1] = 2 - 1 = 1
  Push index 2
  stack: [2] (temp[2]=75)

Day 3: 71 < 75 (not warmer)
  Push index 3
  stack: [2, 3] (75, 71)

Day 4: 69 < 71 (not warmer)
  Push index 4
  stack: [2, 3, 4] (75, 71, 69)

Day 5: 72 > 69 (warmer)
  Pop 4, answer[4] = 5 - 4 = 1
  72 > 71 (warmer)
  Pop 3, answer[3] = 5 - 3 = 2
  Push index 5
  stack: [2, 5] (75, 72)

Day 6: 76 > 72 (warmer)
  Pop 5, answer[5] = 6 - 5 = 1
  76 > 75 (warmer)
  Pop 2, answer[2] = 6 - 2 = 4
  Push index 6
  stack: [6] (76)

Day 7: 73 < 76 (not warmer)
  Push index 7
  stack: [6, 7] (76, 73)

Remaining indices 6, 7 have no warmer day → 0

Result: [1, 1, 4, 2, 1, 1, 0, 0]
```

## Implementation

```rust
pub fn daily_temperatures(temperatures: &[i32]) -> Vec<i32> {
    let n = temperatures.len();
    let mut answer = vec![0; n];
    let mut stack: Vec<usize> = Vec::new();

    for i in 0..n {
        while let Some(&j) = stack.last() {
            if temperatures[i] > temperatures[j] {
                answer[j] = (i - j) as i32;
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(i);
    }
    answer
}
```

## Complexity Analysis Table

| Approach | Time | Space | Notes |
|----------|------|-------|-------|
| Brute Force | O(n²) | O(1) | Check all future days |
| Stack | O(n) | O(n) | Single pass |

## Key Takeaways

1. **Monotonic decreasing stack** stores indices
2. **Pop when warmer** day found
3. **Single pass** achieves O(n) time
4. **Remaining indices** have answer 0
5. **Stack empty** means no warmer day ahead

## Real-World Applications

1. **Weather forecasting**: Days until warmer weather
2. **Stock prices**: Days until price increases
3. **Event scheduling**: Days until next event
4. **Seasonal planning**: Days until summer
5. **Agriculture**: Growing degree days
