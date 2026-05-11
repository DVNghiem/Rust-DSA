# Design Snake Game Solution - LeetCode 353 (Complete)

## Solution Analysis

### Deque for Snake Body

```rust
pub struct SnakeGame {
    width: i32,
    height: i32,
    food: Vec<Vec<i32>>,
    food_index: usize,
    snake: VecDeque<(i32, i32)>,
    body: HashSet<(i32, i32)>,
    score: i32,
}
```

## Key Data Structures

### VecDeque for Snake Body
- Head at front, tail at back
- O(1) add head, O(1) remove tail

### HashSet for Body Tracking
- O(1) collision check
- Store all positions snake occupies

## Line-by-Line Analysis

### Initialize
```rust
snake.push_back((0, 0));  // Snake starts at top-left
body.insert((0, 0));      // Mark body position
```
- Snake is a deque starting with single position (0, 0)

### Move Operation

```rust
pub fn move(&mut self, direction: String) -> i32 {
    let head = *self.snake.front().unwrap();
    let (r, c) = head;

    // Calculate new head position
    let (nr, nc) = match direction.as_str() {
        "U" => (r - 1, c),
        "D" => (r + 1, c),
        "L" => (r, c - 1),
        "R" => (r, c + 1),
        _ => (r, c),
    };

    // Check wall collision
    if nr < 0 || nr >= self.height || nc < 0 || nc >= self.width {
        return -1;  // Game over
    }

    // Check self collision (excluding tail)
    let tail = *self.snake.back().unwrap();
    if self.body.contains(&(nr, nc)) && (nr, nc) != tail {
        return -1;  // Game over
    }

    // Remove tail (will re-add if eating)
    self.snake.pop_back().unwrap();
    self.body.remove(&tail);

    // Add new head
    self.snake.push_front((nr, nc));
    self.body.insert((nr, nc));

    // Check if eating food
    if self.food_index < self.food.len() &&
       self.food[self.food_index][0] == nr &&
       self.food[self.food_index][1] == nc {
        self.score += 1;
        self.food_index += 1;
        // Add tail back (grow)
        self.snake.push_back(tail);
        self.body.insert(tail);
    }

    self.score
}
```

## Move Process Steps

### 1. Calculate New Head Position
Based on direction "U", "D", "L", "R".

### 2. Check Wall Collision
If new position is outside board, game over.

### 3. Check Self Collision
- The tail will move, so it's OK if new head == tail
- But if new head is anywhere else in body, game over

### 4. Move Snake
- Pop tail
- Push new head to front

### 5. Check Food
- If new head position matches current food:
  - Increment score
  - DON'T remove tail (snake grows)
  - Advance food index

## Visual Example

### Initial: 3x3 board, food at [[1,0], [2,0]]

```
Board:
(S) . .
. . .
. . .

S = snake head
```

### Move "R"
```
New head: (0, 1)
Snake: [(0,1), (0,0)]
No food eaten
```

### Move "D"
```
New head: (1, 1)
Snake: [(1,1), (0,1), (0,0)]
No food eaten
```

### Move "D"
```
New head: (2, 1)
Snake: [(2,1), (1,1), (0,1), (0,0)]
No food eaten (food is at [1,0])
```

### Move "L"
```
New head: (2, 0)
Snake: [(2,0), (2,1), (1,1), (0,1)]
FOOD! Score = 1, grow (don't remove tail)
Snake stays at 5 cells: [(2,0), (2,1), (1,1), (0,1), (0,0)]
```

## Why Two Data Structures?

### VecDeque
- Maintains order of snake body
- O(1) operations at both ends
- Direct iteration for debugging

### HashSet
- O(1) collision detection
- Check if position is part of snake body

## Complexity Analysis

| Operation | Time |
|-----------|------|
| move | O(1) |

All operations are constant time.

## Edge Cases

### Eating Food
```rust
// Don't remove tail when eating
snake.push_back(tail);
body.insert(tail);
```
Snake grows by one, doesn't shrink.

### Self Collision
```rust
let tail = *self.snake.back().unwrap();
if self.body.contains(&(nr, nc)) && (nr, nc) != tail {
    return -1;
}
```
New head == tail is OK (tail will move anyway).

### Wall Collision
```rust
if nr < 0 || nr >= self.height || nc < 0 || nc >= self.width {
    return -1;
}
```
Any position outside board is invalid.

## Common Mistakes

1. **Not handling food properly**: Should grow (keep tail) when eating
2. **Not handling self-collision**: Forgetting to exclude tail
3. **Wrong order of operations**: Must add new head before checking food