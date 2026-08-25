func change(amount int, coins []int) int {
    dp := make([]int, amount+1)
    dp[0] = 1

    for i := len(coins) - 1; i >= 0; i-- {
        nextDP := make([]int, amount+1)
        nextDP[0] = 1

        for a := 1; a <= amount; a++ {
            nextDP[a] = dp[a]
            if a-coins[i] >= 0 {
                nextDP[a] += nextDP[a-coins[i]]
            }
        }
        dp = nextDP
    }
    return dp[amount]
}
