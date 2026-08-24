impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let rows = board.len();
        if rows == 0 {
            return;
        }
        let cols = board[0].len();

        // Mark all 'O's connected to the border as 'T'
        for r in 0..rows {
            for c in 0..cols {
                if board[r][c] == 'O' && (r == 0 || c == 0 || r == rows - 1 || c == cols - 1) {
                    Self::capture(board, r, c);
                }
            }
        }

        // Convert all remaining 'O's to 'X's
        for r in 0..rows {
            for c in 0..cols {
                if board[r][c] == 'O' {
                    board[r][c] = 'X';
                }
            }
        }

        // Convert all 'T's back to 'O's
        for r in 0..rows {
            for c in 0..cols {
                if board[r][c] == 'T' {
                    board[r][c] = 'O';
                }
            }
        }
    }

    fn capture(board: &mut Vec<Vec<char>>, r: usize, c: usize) {
        let rows = board.len();
        let cols = board[0].len();

        if board[r][c] != 'O' {
            return;
        }

        board[r][c] = 'T';

        if r + 1 < rows {
            Self::capture(board, r + 1, c);
        }
        if r > 0 {
            Self::capture(board, r - 1, c);
        }
        if c + 1 < cols {
            Self::capture(board, r, c + 1);
        }
        if c > 0 {
            Self::capture(board, r, c - 1);
        }
    }
}
