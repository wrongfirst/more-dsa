use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut visit = vec![vec![false; n]; n];
        let mut min_heap: BinaryHeap<Reverse<(i32, usize, usize)>> = BinaryHeap::new();
        let directions: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        min_heap.push(Reverse((grid[0][0], 0, 0)));
        visit[0][0] = true;

        while let Some(Reverse((t, r, c))) = min_heap.pop() {
            if r == n - 1 && c == n - 1 {
                return t;
            }
            for (dr, dc) in directions.iter() {
                let nei_r = r as i32 + dr;
                let nei_c = c as i32 + dc;
                if nei_r >= 0 && nei_c >= 0 && nei_r < n as i32 && nei_c < n as i32 {
                    let nei_r = nei_r as usize;
                    let nei_c = nei_c as usize;
                    if !visit[nei_r][nei_c] {
                        visit[nei_r][nei_c] = true;
                        min_heap.push(Reverse((t.max(grid[nei_r][nei_c]), nei_r, nei_c)));
                    }
                }
            }
        }
        -1
    }
}
