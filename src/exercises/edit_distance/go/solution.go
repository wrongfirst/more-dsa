func minDistance(word1 string, word2 string) int {
    dp := make([][]float64, len(word1)+1)
    for i := 0; i <= len(word1); i++ {
        dp[i] = make([]float64, len(word2)+1)
        for j := 0; j <= len(word2); j++ {
            dp[i][j] = math.Inf(1)
        }
    }

    for j := 0; j <= len(word2); j++ {
        dp[len(word1)][j] = float64(len(word2) - j)
    }
    for i := 0; i <= len(word1); i++ {
        dp[i][len(word2)] = float64(len(word1) - i)
    }

    for i := len(word1) - 1; i >= 0; i-- {
        for j := len(word2) - 1; j >= 0; j-- {
            if word1[i] == word2[j] {
                dp[i][j] = dp[i+1][j+1]
            } else {
                dp[i][j] = 1 + min(dp[i+1][j], dp[i][j+1], dp[i+1][j+1])
            }
        }
    }
    return int(dp[0][0])
}
