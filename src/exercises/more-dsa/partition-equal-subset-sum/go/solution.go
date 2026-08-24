func canPartition(nums []int) bool {
    sum := 0
    for _, num := range nums {
        sum += num
    }
    if sum%2 != 0 {
        return false
    }

    dp := make(map[int]bool)
    dp[0] = true
    target := sum / 2

    for i := len(nums) - 1; i >= 0; i-- {
        nextDP := make(map[int]bool)
        for t := range dp {
            if (t + nums[i]) == target {
                return true
            }
            nextDP[t+nums[i]] = true
            nextDP[t] = true
        }
        dp = nextDP
    }
    return false
}
