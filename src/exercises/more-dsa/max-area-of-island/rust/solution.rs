use std::collections::HashSet;

impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        if rows == 0 {
            return 0;
        }
        let cols = grid[0].len();
        let mut visit: HashSet<(usize, usize)> = HashSet::new();

        fn dfs(
            grid: &Vec<Vec<i32>>,
            visit: &mut HashSet<(usize, usize)>,
            r: usize,
            c: usize,
            rows: usize,
            cols: usize,
        ) -> i32 {
            if r >= rows || c >= cols || grid[r][c] == 0 || visit.contains(&(r, c)) {
                return 0;
            }
            visit.insert((r, c));

            let mut area = 1;
            if r + 1 < rows {
                area += dfs(grid, visit, r + 1, c, rows, cols);
            }
            if r > 0 {
                area += dfs(grid, visit, r - 1, c, rows, cols);
            }
            if c + 1 < cols {
                area += dfs(grid, visit, r, c + 1, rows, cols);
            }
            if c > 0 {
                area += dfs(grid, visit, r, c - 1, rows, cols);
            }
            area
        }

        let mut max_area = 0;
        for r in 0..rows {
            for c in 0..cols {
                max_area = max_area.max(dfs(&grid, &mut visit, r, c, rows, cols));
            }
        }
        max_area
    }
}
