use std::collections::{BinaryHeap, HashMap, VecDeque};

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut counts: HashMap<char, i32> = HashMap::new();
        for task in tasks {
            *counts.entry(task).or_insert(0) += 1;
        }

        let mut max_heap: BinaryHeap<i32> = BinaryHeap::new();
        for &count in counts.values() {
            max_heap.push(count);
        }

        let mut time = 0;
        let mut queue: VecDeque<(i32, i32)> = VecDeque::new(); // (cnt, idle_time)

        while !max_heap.is_empty() || !queue.is_empty() {
            if !queue.is_empty() && time >= queue.front().unwrap().1 {
                let (cnt, _) = queue.pop_front().unwrap();
                max_heap.push(cnt);
            }
            if let Some(cnt) = max_heap.pop() {
                let cnt = cnt - 1;
                if cnt > 0 {
                    queue.push_back((cnt, time + n + 1));
                }
            }
            time += 1;
        }
        time
    }
}
