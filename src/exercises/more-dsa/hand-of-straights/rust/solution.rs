use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        let group_size = group_size as usize;
        if hand.len() % group_size != 0 {
            return false;
        }

        let mut count: HashMap<i32, i32> = HashMap::new();
        for &n in &hand {
            *count.entry(n).or_insert(0) += 1;
        }

        let mut min_heap: BinaryHeap<Reverse<i32>> = count.keys().map(|&k| Reverse(k)).collect();

        while let Some(Reverse(first)) = min_heap.peek().copied() {
            for i in first..first + group_size as i32 {
                if !count.contains_key(&i) {
                    return false;
                }
                *count.get_mut(&i).unwrap() -= 1;
                if count[&i] == 0 {
                    if let Some(Reverse(top)) = min_heap.peek() {
                        if i != *top {
                            return false;
                        }
                    }
                    min_heap.pop();
                    count.remove(&i);
                }
            }
        }
        true
    }
}
