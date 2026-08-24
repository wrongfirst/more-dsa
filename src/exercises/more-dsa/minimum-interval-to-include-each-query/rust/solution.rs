use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

impl Solution {
    pub fn min_interval(intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut intervals = intervals;
        intervals.sort();

        let mut min_heap: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();
        let mut res: HashMap<i32, i32> = HashMap::new();
        let mut i = 0;

        let mut sorted_queries = queries.clone();
        sorted_queries.sort();

        for q in sorted_queries {
            while i < intervals.len() && intervals[i][0] <= q {
                let (l, r) = (intervals[i][0], intervals[i][1]);
                min_heap.push(Reverse((r - l + 1, r)));
                i += 1;
            }

            while let Some(&Reverse((_, r))) = min_heap.peek() {
                if r < q {
                    min_heap.pop();
                } else {
                    break;
                }
            }

            if let Some(&Reverse((size, _))) = min_heap.peek() {
                res.insert(q, size);
            } else {
                res.insert(q, -1);
            }
        }

        queries.iter().map(|q| *res.get(q).unwrap()).collect()
    }
}
