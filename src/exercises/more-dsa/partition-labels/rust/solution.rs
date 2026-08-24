use std::collections::HashMap;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut last_index: HashMap<char, usize> = HashMap::new();
        let chars: Vec<char> = s.chars().collect();
        let length = chars.len();

        for (j, &c) in chars.iter().enumerate() {
            last_index.insert(c, j);
        }

        let mut result: Vec<i32> = Vec::new();
        let mut i = 0;
        let mut cur_len = 0;
        let mut goal = 0;

        while i < length {
            let c = chars[i];
            goal = goal.max(*last_index.get(&c).unwrap());
            cur_len += 1;

            if goal == i {
                result.push(cur_len);
                cur_len = 0;
            }
            i += 1;
        }
        result
    }
}
