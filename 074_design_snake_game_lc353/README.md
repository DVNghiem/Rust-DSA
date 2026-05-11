# Design Snake Game - LeetCode 353

## Problem Overview

Design a snake game with given width, height, and food positions. Snake moves continuously and eats food to grow.

**Methods:**
- `SnakeGame(width, height, food)` - initialize
- `move(direction)` - returns score (0 if game over)

## Theory

### Deque for Snake Body

Use a deque to represent snake body segments:
- Head at front, tail at back
- On move: add new head, remove tail (unless eating food)

```
Initial: [head(0,0)]
After move right: [head(0,1), tail(0,0)]
```

## Implementation

```rust
use std::collections::{HashSet, VecDeque};

pub struct SnakeGame {
    width: i32,
    height: i32,
    food: Vec<Vec<i32>>,
    food_index: usize,
    snake: VecDeque<(i32, i32)>,
    body: HashSet<(i32, i32)>,
    score: i32,
}

impl SnakeGame {
    pub fn new(width: i32, height: i32, food: Vec<Vec<i32>>) -> Self {
        let mut snake = VecDeque::new();
        snake.push_back((0, 0));
        let mut body = HashSet::new();
        body.insert((0, 0));

        SnakeGame {
            width,
            height,
            food,
            food_index: 0,
            snake,
            body,
            score: 0,
        }
    }

    pub fn move(&mut self, direction: String) -> i32 {
        let head = self.snake.front().unwrap();
        let (r, c) = *head;

        let (nr, nc) = match direction.as_str() {
            "U" => (r - 1, c),
            "D" => (r + 1, c),
            "L" => (r, c - 1),
            "R" => (r, c + 1),
            _ => (r, c),
        };

        // Check wall collision
        if nr < 0 || nr >= self.height || nc < 0 || nc >= self.width {
            return -1;
        }

        // Check self collision (excluding tail which will move)
        let tail = self.snake.back().unwrap();
        if self.body.contains(&(nr, nc)) && (nr, nc) != *tail {
            return -1;
        }

        // Remove tail
        let tail = self.snake.pop_back().unwrap();
        self.body.remove(&tail);

        // Add new head
        self.snake.push_front((nr, nc));
        self.body.insert((nr, nc));

        // Check food
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
}
```

## Test Cases

```rust
#[test]
fn test_snake_basic() {
    let mut game = SnakeGame::new(3, 3, vec![vec![1,2], vec![0,1]]);
    assert_eq!(game.move("R".to_string()), 0);
    assert_eq!(game.move("D".to_string()), 0);
    assert_eq!(game.move("R".to_string()), 1); // ate food
}
```