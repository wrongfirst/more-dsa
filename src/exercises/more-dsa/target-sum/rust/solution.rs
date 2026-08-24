use std::collections::HashMap;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp: HashMap<(usize, i32), i32> = HashMap::new();
        Self::backtrack(0, 0, &nums, target, &mut dp)
    }

    fn backtrack(
        i: usize,
        total: i32,
        nums: &Vec<i32>,
        target: i32,
        dp: &mut HashMap<(usize, i32), i32>,
    ) -> i32 {
        if i == nums.len() {
            return if total == target { 1 } else { 0 };
        }
        let key = (i, total);
        if let Some(&val) = dp.get(&key) {
            return val;
        }

        let ways = Self::backtrack(i + 1, total + nums[i], nums, target, dp)
            + Self::backtrack(i + 1, total - nums[i], nums, target, dp);
        dp.insert(key, ways);
        ways
    }
}
