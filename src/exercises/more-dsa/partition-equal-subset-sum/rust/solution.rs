use std::collections::HashSet;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let total: i32 = nums.iter().sum();
        if total % 2 != 0 {
            return false;
        }

        let target = total / 2;
        let mut dp: HashSet<i32> = HashSet::new();
        dp.insert(0);

        for i in (0..nums.len()).rev() {
            let mut next_dp: HashSet<i32> = HashSet::new();
            for &t in &dp {
                if t + nums[i] == target {
                    return true;
                }
                next_dp.insert(t + nums[i]);
                next_dp.insert(t);
            }
            dp = next_dp;
        }
        false
    }
}
