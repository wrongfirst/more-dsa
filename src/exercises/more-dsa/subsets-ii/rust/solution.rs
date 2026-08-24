impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut res = Vec::new();
        Self::backtrack(0, &mut Vec::new(), &nums, &mut res);
        res
    }

    fn backtrack(start: usize, subset: &mut Vec<i32>, nums: &[i32], res: &mut Vec<Vec<i32>>) {
        res.push(subset.clone());
        for i in start..nums.len() {
            if i > start && nums[i] == nums[i - 1] {
                continue;
            }
            subset.push(nums[i]);
            Self::backtrack(i + 1, subset, nums, res);
            subset.pop();
        }
    }
}
