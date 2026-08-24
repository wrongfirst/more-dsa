use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Reverse;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let mut edges: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
        for time in &times {
            let (u, v, w) = (time[0], time[1], time[2]);
            edges.entry(u).or_insert_with(Vec::new).push((v, w));
        }

        let mut min_heap: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();
        min_heap.push(Reverse((0, k)));
        let mut visited: HashSet<i32> = HashSet::new();
        let mut t = 0;

        while let Some(Reverse((w1, n1))) = min_heap.pop() {
            if visited.contains(&n1) {
                continue;
            }
            visited.insert(n1);
            t = w1;

            if let Some(neighbors) = edges.get(&n1) {
                for &(n2, w2) in neighbors {
                    if !visited.contains(&n2) {
                        min_heap.push(Reverse((w1 + w2, n2)));
                    }
                }
            }
        }

        if visited.len() as i32 == n {
            t
        } else {
            -1
        }
    }
}
