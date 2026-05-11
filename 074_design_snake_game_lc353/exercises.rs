/// Design Snake Game - LeetCode 353
/// Implement snake game with deque for body tracking.

use std::collections::{HashSet, VecDeque};

/// SnakeGame using deque for snake body
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

    /// Move snake in direction, return score or -1 if game over
    pub fn r#move(&mut self, direction: String) -> i32 {
        let head = *self.snake.front().unwrap();
        let (r, c) = head;

        let (nr, nc) = match direction.as_str() {
            "U" => (r - 1, c),
            "D" => (r + 1, c),
            "L" => (r, c - 1),
            "R" => (r, c + 1),
            _ => return self.score,
        };

        // Check wall collision
        if nr < 0 || nr >= self.height || nc < 0 || nc >= self.width {
            return -1;
        }

        // Check self collision (excluding tail that will move)
        let tail = *self.snake.back().unwrap();
        if self.body.contains(&(nr, nc)) && (nr, nc) != tail {
            return -1;
        }

        // Remove tail
        let removed = self.snake.pop_back().unwrap();
        self.body.remove(&removed);

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
            self.snake.push_back(removed);
            self.body.insert(removed);
        }

        self.score
    }

    /// Get current snake body for debugging
    pub fn get_snake(&self) -> Vec<(i32, i32)> {
        self.snake.iter().copied().collect()
    }

    /// Get current score
    pub fn get_score(&self) -> i32 {
        self.score
    }
}

/// Alternative: Using Vec for simpler implementation
pub struct SnakeGameSimple {
    width: i32,
    height: i32,
    food: Vec<Vec<i32>>,
    food_index: usize,
    snake: Vec<(i32, i32)>,
    score: i32,
}

impl SnakeGameSimple {
    pub fn new(width: i32, height: i32, food: Vec<Vec<i32>>) -> Self {
        SnakeGameSimple {
            width,
            height,
            food,
            food_index: 0,
            snake: vec![(0, 0)],
            score: 0,
        }
    }

    pub fn r#move(&mut self, direction: String) -> i32 {
        let head = self.snake[0];
        let (r, c) = head;

        let (nr, nc) = match direction.as_str() {
            "U" => (r - 1, c),
            "D" => (r + 1, c),
            "L" => (r, c - 1),
            "R" => (r, c + 1),
            _ => return self.score,
        };

        // Check wall collision
        if nr < 0 || nr >= self.height || nc < 0 || nc >= self.width {
            return -1;
        }

        // Check self collision
        if self.snake.contains(&(nr, nc)) {
            return -1;
        }

        // Add new head
        self.snake.insert(0, (nr, nc));

        // Check if eating food
        let eating = self.food_index < self.food.len() &&
           self.food[self.food_index][0] == nr &&
           self.food[self.food_index][1] == nc;

        if eating {
            self.score += 1;
            self.food_index += 1;
        } else {
            self.snake.pop();
        }

        self.score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snake_basic() {
        let mut game = SnakeGame::new(3, 2, vec![vec![1,0], vec![2,0]]);
        assert_eq!(game.r#move("R".to_string()), 0);
        assert_eq!(game.move("D".to_string()), 0);
        assert_eq!(game.move("R".to_string()), 1); // ate food
    }

    #[test]
    fn test_snake_game_over_wall() {
        let mut game = SnakeGame::new(2, 2, vec![]);
        assert_eq!(game.move("L".to_string()), -1); // hits wall
    }

    #[test]
    fn test_snake_game_over_self() {
        let mut game = SnakeGame::new(3, 3, vec![]);
        game.r#move("R".to_string());
        game.r#move("D".to_string());
        game.r#move("L".to_string());
        assert_eq!(game.move("U".to_string()), -1); // hits self
    }

    #[test]
    fn test_snake_grow() {
        let mut game = SnakeGame::new(3, 3, vec![vec![1,0]]);
        game.r#move("R".to_string());
        game.r#move("D".to_string());
        assert_eq!(game.get_snake().len(), 3); // grew from 2 to 3
    }

    #[test]
    fn test_snake_eat_food() {
        let mut game = SnakeGame::new(3, 3, vec![vec![1,0]]);
        game.r#move("R".to_string()); // (0,1)
        game.r#move("D".to_string()); // (1,1)
        assert_eq!(game.move("R".to_string()), 1); // ate food at (1,0)
    }

    #[test]
    fn test_snake_eat_multiple() {
        let mut game = SnakeGame::new(3, 3, vec![vec![1,0], vec![2,0]]);
        game.r#move("R".to_string());
        game.r#move("D".to_string());
        assert_eq!(game.move("R".to_string()), 1); // ate first food
        game.r#move("R".to_string()); // (1,1)
        game.r#move("D".to_string()); // (2,1)
        assert_eq!(game.move("R".to_string()), 2); // ate second food
    }

    #[test]
    fn test_snake_no_movement() {
        let game = SnakeGame::new(3, 3, vec![]);
        assert_eq!(game.get_score(), 0);
        assert_eq!(game.get_snake().len(), 1);
    }

    #[test]
    fn test_snake_simple_basic() {
        let mut game = SnakeGameSimple::new(3, 2, vec![vec![1,0], vec![2,0]]);
        assert_eq!(game.move("R".to_string()), 0);
        assert_eq!(game.move("D".to_string()), 0);
        assert_eq!(game.move("R".to_string()), 1);
    }

    #[test]
    fn test_snake_simple_game_over() {
        let mut game = SnakeGameSimple::new(2, 2, vec![]);
        assert_eq!(game.move("L".to_string()), -1);
    }

    #[test]
    fn test_both_same_result() {
        let mut g1 = SnakeGame::new(3, 3, vec![vec![1,0]]);
        let mut g2 = SnakeGameSimple::new(3, 3, vec![vec![1,0]]);

        let moves = vec!["R", "D", "R"];
        for m in moves {
            assert_eq!(g1.move(m.to_string()), g2.move(m.to_string()));
        }
    }

    #[test]
    fn test_snake_empty_food() {
        let mut game = SnakeGame::new(3, 3, vec![]);
        game.r#move("R".to_string());
        game.r#move("R".to_string());
        assert_eq!(game.get_score(), 0);
    }

    #[test]
    fn test_snake_long_game() {
        let mut game = SnakeGame::new(10, 10, vec![]);
        for _ in 0..100 {
            game.r#move("R".to_string());
        }
        assert!(game.get_score() >= 0);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("Snake Game exercises - run tests with cargo test");
}