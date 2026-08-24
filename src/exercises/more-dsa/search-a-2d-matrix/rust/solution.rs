impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let rows = matrix.len();
        let cols = matrix[0].len();

        let mut top: i32 = 0;
        let mut bot: i32 = rows as i32 - 1;

        while top <= bot {
            let row = (top + bot) / 2;
            if target > matrix[row as usize][cols - 1] {
                top = row + 1;
            } else if target < matrix[row as usize][0] {
                bot = row - 1;
            } else {
                break;
            }
        }

        if top > bot {
            return false;
        }

        let row = ((top + bot) / 2) as usize;
        let mut l: i32 = 0;
        let mut r: i32 = cols as i32 - 1;

        while l <= r {
            let m = (l + r) / 2;
            if target > matrix[row][m as usize] {
                l = m + 1;
            } else if target < matrix[row][m as usize] {
                r = m - 1;
            } else {
                return true;
            }
        }

        false
    }
}
