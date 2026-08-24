func partition(s string) [][]string {
	res, part := [][]string{}, []string{}
	var dfs func(int)
	dfs = func(i int) {
		if i >= len(s) {
			res = append(res, append([]string{}, part...))
			return
		}
		for j := i; j < len(s); j++ {
			if isPali(s, i, j) {
				part = append(part, s[i:j+1])
				dfs(j + 1)
				part = part[:len(part)-1]
			}
		}
	}
	dfs(0)
	return res
}

func isPali(s string, l, r int) bool {
	for l < r {
		if s[l] != s[r] {
			return false
		}
		l, r = l+1, r-1
	}
	return true
}
