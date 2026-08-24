impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut nums = nums;
        Self::backtrack(&mut nums, 0, &mut res);
        res
    }

    fn backtrack(nums: &mut Vec<i32>, start: usize, res: &mut Vec<Vec<i32>>) {
        if start == nums.len() {
            res.push(nums.clone());
            return;
        }

        for i in start..nums.len() {
            nums.swap(start, i);
            Self::backtrack(nums, start + 1, res);
            nums.swap(start, i);
        }
    }
}
