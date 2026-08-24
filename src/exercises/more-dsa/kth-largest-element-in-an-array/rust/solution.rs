impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        let k = nums.len() - k as usize;
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let pivot = Self::partition(&mut nums, left, right);

            if pivot < k {
                left = pivot + 1;
            } else if pivot > k {
                right = pivot - 1;
            } else {
                break;
            }
        }

        nums[k]
    }

    fn partition(nums: &mut Vec<i32>, left: usize, right: usize) -> usize {
        let pivot = nums[right];
        let mut fill = left;

        for i in left..right {
            if nums[i] <= pivot {
                nums.swap(fill, i);
                fill += 1;
            }
        }

        nums.swap(fill, right);
        fill
    }
}
