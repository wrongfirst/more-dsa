use std::collections::BinaryHeap;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut min_heap: BinaryHeap<(i64, i32, i32)> = BinaryHeap::new();

        for point in &points {
            let x = point[0] as i64;
            let y = point[1] as i64;
            let dist = x * x + y * y;
            // Use negative distance to simulate a min-heap
            min_heap.push((-dist, point[0], point[1]));
        }

        let mut result = Vec::new();
        for _ in 0..k {
            if let Some((_, x, y)) = min_heap.pop() {
                result.push(vec![x, y]);
            }
        }

        result
    }
}
