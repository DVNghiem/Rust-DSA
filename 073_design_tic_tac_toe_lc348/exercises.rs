/// Design Tic-Tac-Toe - LeetCode 348
/// Design TicTacToe class with O(1) move operation.

/// O(1) move using row, column, and diagonal counters
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

    /// Make a move at (row, col) for player (1 or 2)
    /// Returns winner (1 or 2) or 0 if no winner yet
    pub fn r#move(&mut self, row: i32, col: i32, player: i32) -> i32 {
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

        // Check win condition
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

/// Alternative: Use arrays for better performance
pub struct TicTacToeArray {
    n: usize,
    rows: [[i32; 2]; 100],  // assuming max n=100
    cols: [[i32; 2]; 100],
    diag: [i32; 2],
    anti_diag: [i32; 2],
}

impl TicTacToeArray {
    pub fn new(n: i32) -> Self {
        TicTacToeArray {
            n: n as usize,
            rows: [[0; 2]; 100],
            cols: [[0; 2]; 100],
            diag: [0; 2],
            anti_diag: [0; 2],
        }
    }

    pub fn r#move(&mut self, row: i32, col: i32, player: i32) -> i32 {
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

/// Board for visual representation (used in testing)
pub struct TicTacToeBoard {
    n: usize,
    board: Vec<Vec<char>>,
}

impl TicTacToeBoard {
    pub fn new(n: usize) -> Self {
        TicTacToeBoard {
            n,
            board: vec![vec![' '; n]; n],
        }
    }

    pub fn r#move(&mut self, row: usize, col: usize, player: i32) -> bool {
        if self.board[row][col] != ' ' { return false; }
        self.board[row][col] = if player == 1 { 'X' } else { 'O' };
        true
    }

    pub fn to_string(&self) -> String {
        self.board.iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_tictactoe_col_win() {
        let mut t = TicTacToe::new(3);
        assert_eq!(t.move(0, 0, 1), 0);
        assert_eq!(t.move(1, 0, 2), 0);
        assert_eq!(t.move(0, 1, 1), 0);
        assert_eq!(t.move(1, 1, 2), 0);
        assert_eq!(t.move(0, 2, 1), 1); // Player 1 wins with column 0
    }

    #[test]
    fn test_tictactoe_row_win() {
        let mut t = TicTacToe::new(3);
        assert_eq!(t.move(0, 0, 1), 0);
        assert_eq!(t.move(1, 0, 2), 0);
        assert_eq!(t.move(0, 1, 1), 0);
        assert_eq!(t.move(1, 1, 2), 0);
        assert_eq!(t.move(0, 2, 1), 1); // Player 1 wins with row 0
    }

    #[test]
    fn test_tictactoe_diag_win() {
        let mut t = TicTacToe::new(3);
        assert_eq!(t.move(0, 0, 1), 0);
        assert_eq!(t.move(0, 1, 2), 0);
        assert_eq!(t.move(1, 1, 1), 0);
        assert_eq!(t.move(0, 2, 2), 0);
        assert_eq!(t.move(2, 2, 1), 1); // Player 1 wins with main diagonal
    }

    #[test]
    fn test_tictactoe_anti_diag_win() {
        let mut t = TicTacToe::new(3);
        assert_eq!(t.move(0, 2, 1), 0);
        assert_eq!(t.move(0, 0, 2), 0);
        assert_eq!(t.move(1, 1, 1), 0);
        assert_eq!(t.move(0, 1, 2), 0);
        assert_eq!(t.move(2, 0, 1), 1); // Player 1 wins with anti-diagonal
    }

    #[test]
    fn test_tictactoe_player2_win() {
        let mut t = TicTacToe::new(3);
        assert_eq!(t.move(0, 0, 1), 0);
        assert_eq!(t.move(0, 1, 2), 0);
        assert_eq!(t.move(1, 0, 1), 0);
        assert_eq!(t.move(1, 1, 2), 0);
        assert_eq!(t.move(2, 0, 1), 0);
        assert_eq!(t.move(2, 2, 2), 1); // Player 2 wins with anti-diagonal
    }

    #[test]
    fn test_tictactoe_no_win_yet() {
        let mut t = TicTacToe::new(3);
        assert_eq!(t.move(0, 0, 1), 0);
        assert_eq!(t.move(0, 1, 2), 0);
        assert_eq!(t.move(0, 2, 1), 0);
        assert_eq!(t.move(1, 0, 2), 0);
        assert_eq!(t.move(1, 1, 1), 0);
        assert_eq!(t.move(1, 2, 2), 0);
        assert_eq!(t.move(2, 0, 1), 0);
        assert_eq!(t.move(2, 1, 2), 0);
        assert_eq!(t.move(2, 2, 1), 0); // Draw, no winner
    }

    #[test]
    fn test_tictactoe_size_4() {
        let mut t = TicTacToe::new(4);
        // Player 1 wins with row 0
        assert_eq!(t.move(0, 0, 1), 0);
        assert_eq!(t.move(1, 0, 2), 0);
        assert_eq!(t.move(0, 1, 1), 0);
        assert_eq!(t.move(1, 1, 2), 0);
        assert_eq!(t.move(0, 2, 1), 0);
        assert_eq!(t.move(1, 2, 2), 0);
        assert_eq!(t.move(0, 3, 1), 1);
    }

    #[test]
    fn test_tictactoe_array_basic() {
        let mut t = TicTacToeArray::new(3);
        assert_eq!(t.move(0, 0, 1), 0);
        assert_eq!(t.move(0, 2, 2), 0);
        assert_eq!(t.move(2, 2, 1), 0);
        assert_eq!(t.move(1, 1, 2), 0);
        assert_eq!(t.move(2, 1, 1), 1);
    }

    #[test]
    fn test_both_approaches_same() {
        let mut t1 = TicTacToe::new(3);
        let mut t2 = TicTacToeArray::new(3);

        let moves = vec![
            (0, 0, 1),
            (0, 2, 2),
            (2, 2, 1),
            (1, 1, 2),
            (2, 0, 1),
            (1, 0, 2),
            (2, 1, 1),
        ];

        for (row, col, player) in moves {
            assert_eq!(t1.move(row, col, player), t2.move(row, col, player));
        }
    }

    #[test]
    fn test_board_visual() {
        let mut board = TicTacToeBoard::new(3);
        board.r#move(0, 0, 1);
        board.r#move(1, 1, 2);
        board.r#move(2, 2, 1);
        assert_eq!(board.board[0][0], 'X');
        assert_eq!(board.board[1][1], 'O');
        assert_eq!(board.board[2][2], 'X');
    }

    #[test]
    fn test_invalid_move() {
        let mut board = TicTacToeBoard::new(3);
        assert!(board.r#move(0, 0, 1));
        assert!(!board.r#move(0, 0, 2)); // Can't overwrite
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("Tic Tac Toe exercises - run tests with cargo test");
}