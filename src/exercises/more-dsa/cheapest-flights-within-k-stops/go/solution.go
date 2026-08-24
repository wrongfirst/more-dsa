func findCheapestPrice(n int, flights [][]int, src int, dst int, k int) int {
    prices := make([]float64, n)
    for i := 0; i < n; i++ {
        prices[i] = math.Inf(1)
    }
    prices[src] = 0

    for i := 0; i < k+1; i++ {
        tmpPrices := make([]float64, len(prices))
        copy(tmpPrices, prices)

        for _, flight := range flights {
            s, d, p := flight[0], flight[1], flight[2]
            if prices[s] == math.Inf(1) {
                continue
            }
            if prices[s]+float64(p) < tmpPrices[d] {
                tmpPrices[d] = prices[s] + float64(p)
            }
        }
        prices = tmpPrices
    }

    if prices[dst] == math.Inf(1) {
        return -1
    }
    return int(prices[dst])
}
