use std::collections::HashMap;

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut dp: HashMap<(usize, usize), i32> = HashMap::new();

        fn dfs(
            r: i32,
            c: i32,
            prev_val: i32,
            matrix: &Vec<Vec<i32>>,
            dp: &mut HashMap<(usize, usize), i32>,
            rows: i32,
            cols: i32,
        ) -> i32 {
            if r < 0
                || r >= rows
                || c < 0
                || c >= cols
                || matrix[r as usize][c as usize] <= prev_val
            {
                return 0;
            }

            let key = (r as usize, c as usize);
            if let Some(&cached) = dp.get(&key) {
                return cached;
            }

            let curr_val = matrix[r as usize][c as usize];
            let mut res = 1;
            res = res.max(1 + dfs(r + 1, c, curr_val, matrix, dp, rows, cols));
            res = res.max(1 + dfs(r - 1, c, curr_val, matrix, dp, rows, cols));
            res = res.max(1 + dfs(r, c + 1, curr_val, matrix, dp, rows, cols));
            res = res.max(1 + dfs(r, c - 1, curr_val, matrix, dp, rows, cols));

            dp.insert(key, res);
            res
        }

        for r in 0..rows {
            for c in 0..cols {
                dfs(r as i32, c as i32, -1, &matrix, &mut dp, rows as i32, cols as i32);
            }
        }

        *dp.values().max().unwrap_or(&1)
    }
}
