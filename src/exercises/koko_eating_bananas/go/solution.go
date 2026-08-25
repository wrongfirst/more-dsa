func minEatingSpeed(piles []int, h int) int {
    l, r := 1, max(piles)
    res := r
    
    for l <= r {
        k := (l + r) / 2
        totalTime := 0
        for _, p := range piles {
            totalTime += int(math.Ceil(float64(p) / float64(k)))
        }
        
        if totalTime <= h {
            res = k
            r = k - 1
        } else {
            l = k + 1
        }
    }
    
    return res
}

func max(piles []int) int {
    maxVal := piles[0]
    for _, val := range piles {
        if val > maxVal {
            maxVal = val
        }
    }
    return maxVal
}
