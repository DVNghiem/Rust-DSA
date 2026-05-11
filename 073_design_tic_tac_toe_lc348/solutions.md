# Design Tic-Tac-Toe Solution - LeetCode 348 (Complete)

## Solution Analysis

### O(1) Move with Counters

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

## Why Counters?

### Traditional Approach

Check all rows, columns, diagonals after each move - O(n) per move.

### Counter Approach

Maintain counts for each:
- Row: how many of player's marks in each row
- Column: how many of player's marks in each column
- Diagonal: how many on main diagonal
- Anti-diagonal: how many on anti-diagonal

When count reaches n (board size), that player wins!

## Line-by-Line Analysis

### Initialize
```rust
rows: vec![vec![0; 2]; n]  // n rows, 2 players
cols: vec![vec![0; 2]; n]  // n cols, 2 players
diag: vec![0; 2]           // main diagonal, 2 players
anti_diag: vec![0; 2]      // anti-diagonal, 2 players
```
- `[i][0]` = player 1's count
- `[i][1]` = player 2's count

### Move
```rust
let p = (player - 1) as usize;  // player 1 → 0, player 2 → 1

self.rows[row][p] += 1;
self.cols[col][p] += 1;

if row == col {
    self.diag[p] += 1;
}
if row + col == self.n - 1 {
    self.anti_diag[p] += 1;
}
```
- Increment appropriate counters for the player
- Only increment diagonals if position is on that diagonal

### Win Check
```rust
if self.rows[row][p] == self.n as i32 ||
   self.cols[col][p] == self.n as i32 ||
   self.diag[p] == self.n as i32 ||
   self.anti_diag[p] == self.n as i32 {
    player  // Win!
} else {
    0       // No win
}
```
- If any counter equals n, player wins
- Otherwise no winner yet

## Visual Example

### 3x3 Board, Player 1 Win by Row

```
Move sequence:
1. move(0, 0, 1): rows[0][0] = 1, cols[0][0] = 1, diag[0] = 1
2. move(1, 1, 1): rows[1][0] = 1, cols[1][0] = 1, diag[0] = 2
3. move(2, 0, 2): rows[2][1] = 1, cols[0][1] = 1, anti_diag[1] = 1
4. move(0, 1, 1): rows[0][0] = 2, cols[1][0] = 1
5. move(0, 2, 1): rows[0][0] = 3 = n → Player 1 WINS!
```

## Diagonal Conditions

### Main Diagonal (row == col)
```
(0,0) (1,1) (2,2)
```
All positions where row index equals column index.

### Anti Diagonal (row + col == n - 1)
```
(0, 2) (1, 1) (2, 0) for 3x3
```
All positions where row + col equals board size - 1.

## Complexity Analysis

| Operation | Time | Space |
|----------|------|-------|
| move | O(1) | O(n) |

## Edge Cases

### Center Position
```rust
move(1, 1, 1) on 3x3
row == col (1 == 1) → increment diag
row + col == 2 (1 + 1 == 2) → increment anti_diag
```
Center is on both diagonals.

### Corner Position
```rust
move(0, 0, 1) on 3x3
row == col → increment diag
row + col == 2? No (0 + 0 = 0)
```
Top-left is only on main diagonal.

## Common Mistakes

1. **Wrong player index**: player - 1 maps player 1 to index 0
2. **Checking wrong diagonal**: row == col for main, row + col == n-1 for anti
3. **Not checking all win conditions**: Must check row, col, diag, anti_diag

## Why This Works

When player marks a cell:
- We increment the count for that row
- If the row count reaches n, all n cells in that row are marked by the same player
- Same logic applies to columns and diagonals

Since each move only modifies one row, one column, and at most two diagonals, we only need to check those counters after each move.