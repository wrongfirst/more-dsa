use std::collections::{BinaryHeap, HashSet};
use std::cmp::Reverse;

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        if n == 0 {
            return 0;
        }

        // Build adjacency list with manhattan distances
        let mut adj: Vec<Vec<(i32, usize)>> = vec![Vec::new(); n];
        for i in 0..n {
            let (x1, y1) = (points[i][0], points[i][1]);
            for j in (i + 1)..n {
                let (x2, y2) = (points[j][0], points[j][1]);
                let dist = (x1 - x2).abs() + (y1 - y2).abs();
                adj[i].push((dist, j));
                adj[j].push((dist, i));
            }
        }

        // Prim's algorithm
        let mut res = 0;
        let mut visit: HashSet<usize> = HashSet::new();
        let mut min_heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
        min_heap.push(Reverse((0, 0)));

        while visit.len() < n {
            let Reverse((cost, i)) = min_heap.pop().unwrap();
            if visit.contains(&i) {
                continue;
            }
            res += cost;
            visit.insert(i);
            for &(nei_cost, nei) in &adj[i] {
                if !visit.contains(&nei) {
                    min_heap.push(Reverse((nei_cost, nei)));
                }
            }
        }

        res
    }
}
