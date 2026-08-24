use std::collections::HashMap;

struct CountSquares {
    pts_count: HashMap<(i32, i32), i32>,
    pts: Vec<(i32, i32)>,
}

impl CountSquares {
    pub fn new() -> Self {
        CountSquares {
            pts_count: HashMap::new(),
            pts: Vec::new(),
        }
    }

    pub fn add(&mut self, point: Vec<i32>) {
        let p = (point[0], point[1]);
        *self.pts_count.entry(p).or_insert(0) += 1;
        self.pts.push(p);
    }

    pub fn count(&self, point: Vec<i32>) -> i32 {
        let mut res = 0;
        let px = point[0];
        let py = point[1];

        for &(x, y) in &self.pts {
            if (py - y).abs() != (px - x).abs() || x == px || y == py {
                continue;
            }
            let count1 = *self.pts_count.get(&(x, py)).unwrap_or(&0);
            let count2 = *self.pts_count.get(&(px, y)).unwrap_or(&0);
            res += count1 * count2;
        }

        res
    }
}
