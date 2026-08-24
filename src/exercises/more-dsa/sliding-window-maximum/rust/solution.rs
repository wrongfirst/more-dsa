use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut output = Vec::new();
        let mut deque: VecDeque<usize> = VecDeque::new();
        let mut l = 0;

        for r in 0..nums.len() {
            while !deque.is_empty() && nums[*deque.back().unwrap()] < nums[r] {
                deque.pop_back();
            }
            deque.push_back(r);

            if l > *deque.front().unwrap() {
                deque.pop_front();
            }

            if r + 1 >= k {
                output.push(nums[*deque.front().unwrap()]);
                l += 1;
            }
        }

        output
    }
}
