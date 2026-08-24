use std::collections::BinaryHeap;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap: BinaryHeap<i32> = BinaryHeap::from(stones);

        while heap.len() > 1 {
            let first = heap.pop().unwrap();
            let second = heap.pop().unwrap();
            if second < first {
                heap.push(first - second);
            }
        }

        heap.pop().unwrap_or(0)
    }
}
