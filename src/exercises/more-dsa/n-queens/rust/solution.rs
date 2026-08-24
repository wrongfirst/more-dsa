use std::collections::HashSet;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;
        let mut col: HashSet<usize> = HashSet::new();
        let mut pos_diag: HashSet<i32> = HashSet::new();
        let mut neg_diag: HashSet<i32> = HashSet::new();

        let mut res: Vec<Vec<String>> = Vec::new();
        let mut board: Vec<Vec<char>> = vec![vec!['.'; n]; n];

        Self::backtrack(n, 0, &mut col, &mut pos_diag, &mut neg_diag, &mut board, &mut res);

        res
    }

    fn backtrack(
        n: usize,
        r: usize,
        col: &mut HashSet<usize>,
        pos_diag: &mut HashSet<i32>,
        neg_diag: &mut HashSet<i32>,
        board: &mut Vec<Vec<char>>,
        res: &mut Vec<Vec<String>>,
    ) {
        if r == n {
            let copy: Vec<String> = board.iter().map(|row| row.iter().collect()).collect();
            res.push(copy);
            return;
        }

        for c in 0..n {
            let pd = (r + c) as i32;
            let nd = (r as i32) - (c as i32);

            if col.contains(&c) || pos_diag.contains(&pd) || neg_diag.contains(&nd) {
                continue;
            }

            col.insert(c);
            pos_diag.insert(pd);
            neg_diag.insert(nd);
            board[r][c] = 'Q';

            Self::backtrack(n, r + 1, col, pos_diag, neg_diag, board, res);

            col.remove(&c);
            pos_diag.remove(&pd);
            neg_diag.remove(&nd);
            board[r][c] = '.';
        }
    }
}
