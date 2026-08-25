func maxProfit(prices []int) int {
    dp := make(map[[2]int]int)

    var dfs func(i int, buying bool) int
    dfs = func(i int, buying bool) int {
        if i >= len(prices) {
            return 0
        }

        buyingInt := 0
        if buying {
            buyingInt = 1
        }
        key := [2]int{i, buyingInt}

        if val, exists := dp[key]; exists {
            return val
        }

        cooldown := dfs(i+1, buying)
        if buying {
            buy := dfs(i+1, !buying) - prices[i]
            dp[key] = max(buy, cooldown)
        } else {
            sell := dfs(i+2, !buying) + prices[i]
            dp[key] = max(sell, cooldown)
        }
        return dp[key]
    }

    return dfs(0, true)
}
