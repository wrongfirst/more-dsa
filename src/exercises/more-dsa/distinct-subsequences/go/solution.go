func numDistinct(s string, t string) int {
    cache := make(map[[2]int]int)

    for i := 0; i <= len(s); i++ {
        cache[[2]int{i, len(t)}] = 1
    }
    for j := 0; j < len(t); j++ {
        cache[[2]int{len(s), j}] = 0
    }

    for i := len(s) - 1; i >= 0; i-- {
        for j := len(t) - 1; j >= 0; j-- {
            if s[i] == t[j] {
                cache[[2]int{i, j}] = cache[[2]int{i + 1, j + 1}] + cache[[2]int{i + 1, j}]
            } else {
                cache[[2]int{i, j}] = cache[[2]int{i + 1, j}]
            }
        }
    }
    return cache[[2]int{0, 0}]
}
