func jump(nums []int) int {
    l, r := 0, 0
    res := 0

    for r < (len(nums) - 1) {
        maxJump := 0
        for i := l; i <= r; i++ {
            maxJump = max(maxJump, i+nums[i])
        }
        l = r + 1
        r = maxJump
        res++
    }
    return res
}
