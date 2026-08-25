func isMatch(s string, p string) bool {
    cache := make([][]bool, len(s)+1)
    for i := 0; i <= len(s); i++ {
        cache[i] = make([]bool, len(p)+1)
    }
    cache[len(s)][len(p)] = true

    for i := len(s); i >= 0; i-- {
        for j := len(p) - 1; j >= 0; j-- {
            match := i < len(s) && (s[i] == p[j] || p[j] == '.')

            if (j+1) < len(p) && p[j+1] == '*' {
                cache[i][j] = cache[i][j+2]
                if match {
                    cache[i][j] = cache[i+1][j] || cache[i][j]
                }
            } else if match {
                cache[i][j] = cache[i+1][j+1]
            }
        }
    }
    return cache[0][0]
}
