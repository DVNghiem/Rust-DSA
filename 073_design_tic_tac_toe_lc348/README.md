# Design Tic-Tac-Toe - LeetCode 348

## Problem Overview

Design a `TicTacToe` class that supports:
- `move(row, col, player)` - make a move at position
- Returns the winner (1 or 2) or 0 if no winner yet

**Examples:**
```
TicTacToe t = new TicTacToe(3);
t.move(0, 0, 1); // Returns 0 (no winner)
t.move(0, 2, 2); // Returns 0 (no winner)
t.move(2, 2, 1); // Returns 0 (no winner)
t.move(1, 1, 2); // Returns 0 (no winner)
t.move(2, 0, 1); // Returns 0 (no winner)
t.move(1, 0, 2); // Returns 0 (no winner)
t.move(2, 1, 1); // Returns 1 (Player 1 wins)
```

## Theory

### O(1) Move with Row/Col/Diag Counters

Instead of checking entire board, maintain counters:
- `rows[n][2]`: Player's count in each row
- `cols[n][2]`: Player's count in each column
- `diag[2]`: Player's count on two diagonals

Win when count reaches n (size of board).

## Implementation

```rust
pub struct TicTacToe {
    n: usize,
    rows: Vec<Vec<i32>>,
    cols: Vec<Vec<i32>>,
    diag: Vec<i32>,
    anti_diag: Vec<i32>,
}

impl TicTacToe {
    pub fn new(n: i32) -> Self {
        let n = n as usize;
        TicTacToe {
            n,
            rows: vec![vec![0; 2]; n],
            cols: vec![vec![0; 2]; n],
            diag: vec![0; 2],
            anti_diag: vec![0; 2],
        }
    }

    pub fn move(&mut self, row: i32, col: i32, player: i32) -> i32 {
        let row = row as usize;
        let col = col as usize;
        let p = (player - 1) as usize;

        self.rows[row][p] += 1;
        self.cols[col][p] += 1;

        if row == col {
            self.diag[p] += 1;
        }
        if row + col == self.n - 1 {
            self.anti_diag[p] += 1;
        }

        if self.rows[row][p] == self.n as i32 ||
           self.cols[col][p] == self.n as i32 ||
           self.diag[p] == self.n as i32 ||
           self.anti_diag[p] == self.n as i32 {
            player
        } else {
            0
        }
    }
}
```

## Test Cases

```rust
#[test]
fn test_tictactoe_basic() {
    let mut t = TicTacToe::new(3);
    assert_eq!(t.move(0, 0, 1), 0);
    assert_eq!(t.move(0, 2, 2), 0);
    assert_eq!(t.move(2, 2, 1), 0);
    assert_eq!(t.move(1, 1, 2), 0);
    assert_eq!(t.move(2, 0, 1), 0);
    assert_eq!(t.move(1, 0, 2), 0);
    assert_eq!(t.move(2, 1, 1), 1);
}
```