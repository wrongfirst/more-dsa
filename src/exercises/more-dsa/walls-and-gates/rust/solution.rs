use std::collections::VecDeque;

impl Solution {
    pub fn islands_and_treasure(grid: &mut Vec<Vec<i32>>) {
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let m = grid.len();
        let n = grid[0].len();

        // Find all treasure chests (cells with value 0)
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 0 {
                    queue.push_back((i, j));
                }
            }
        }

        if queue.is_empty() {
            return;
        }

        let dirs: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

        while let Some((row, col)) = queue.pop_front() {
            for (dr, dc) in dirs.iter() {
                let x = row as i32 + dr;
                let y = col as i32 + dc;

                if x < 0 || y < 0 || x >= m as i32 || y >= n as i32 {
                    continue;
                }

                let x = x as usize;
                let y = y as usize;

                if grid[x][y] != i32::MAX {
                    continue;
                }

                queue.push_back((x, y));
                grid[x][y] = grid[row][col] + 1;
            }
        }
    }
}
