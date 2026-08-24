use std::collections::VecDeque;

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let rows = grid.len();
        let cols = grid[0].len();
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let mut fresh = 0;
        let mut time = 0;

        // Find all rotten oranges and count fresh ones
        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == 1 {
                    fresh += 1;
                } else if grid[r][c] == 2 {
                    queue.push_back((r, c));
                }
            }
        }

        let directions: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        while fresh > 0 && !queue.is_empty() {
            let length = queue.len();
            for _ in 0..length {
                if let Some((r, c)) = queue.pop_front() {
                    for (dr, dc) in directions.iter() {
                        let nr = r as i32 + dr;
                        let nc = c as i32 + dc;

                        if nr >= 0
                            && nr < rows as i32
                            && nc >= 0
                            && nc < cols as i32
                            && grid[nr as usize][nc as usize] == 1
                        {
                            grid[nr as usize][nc as usize] = 2;
                            queue.push_back((nr as usize, nc as usize));
                            fresh -= 1;
                        }
                    }
                }
            }
            time += 1;
        }

        if fresh == 0 { time } else { -1 }
    }
}
