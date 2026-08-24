use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut cols: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut rows: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut squares: Vec<HashSet<char>> = vec![HashSet::new(); 9];

        for r in 0..9 {
            for c in 0..9 {
                let cell = board[r][c];
                if cell == '.' {
                    continue;
                }
                let square_idx = (r / 3) * 3 + c / 3;
                if rows[r].contains(&cell) || cols[c].contains(&cell) || squares[square_idx].contains(&cell) {
                    return false;
                }
                rows[r].insert(cell);
                cols[c].insert(cell);
                squares[square_idx].insert(cell);
            }
        }
        true
    }
}
