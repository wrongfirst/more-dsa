func search(nums []int, target int) int {
	l, r := 0, len(nums)-1

	for l <= r {
		m := l + ((r - l) / 2)  // (l + r) // 2 can lead to overflow
		if nums[m] > target {
			r = m - 1
		} else if nums[m] < target {
			l = m + 1
		} else {
			return m
		}
	}

	return -1
}
