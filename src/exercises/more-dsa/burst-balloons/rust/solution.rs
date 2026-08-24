impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut cache = vec![vec![0; n + 2]; n + 2];
        let mut new_nums = vec![0; n + 2];
        new_nums[0] = 1;
        new_nums[n + 1] = 1;
        for i in 0..n {
            new_nums[i + 1] = nums[i];
        }

        for offset in 2..new_nums.len() {
            for left in 0..new_nums.len() - offset {
                let right = left + offset;
                for pivot in (left + 1)..right {
                    let coins = new_nums[left] * new_nums[pivot] * new_nums[right]
                        + cache[left][pivot]
                        + cache[pivot][right];
                    cache[left][right] = cache[left][right].max(coins);
                }
            }
        }
        cache[0][new_nums.len() - 1]
    }
}
