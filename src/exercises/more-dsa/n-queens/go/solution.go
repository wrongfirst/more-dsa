func solveNQueens(n int) [][]string {
	col := make(map[int]bool)
	posDiag := make(map[int]bool)
	negDiag := make(map[int]bool)
	res := [][]string{}
	board := make([][]string, n)
	for i := range board {
		board[i] = make([]string, n)
		for j := range board[i] {
			board[i][j] = "."
		}
	}

	var backtrack func(int)
	backtrack = func(r int) {
		if r == n {
			copy := make([]string, n)
			for i := range board {
				copy[i] = strings.Join(board[i], "")
			}
			res = append(res, copy)
			return
		}
		for c := 0; c < n; c++ {
			if col[c] || posDiag[r+c] || negDiag[r-c] {
				continue
			}
			col[c] = true
			posDiag[r+c] = true
			negDiag[r-c] = true
			board[r][c] = "Q"
			backtrack(r + 1)
			delete(col, c)
			delete(posDiag, r+c)
			delete(negDiag, r-c)
			board[r][c] = "."
		}
	}
	backtrack(0)
	return res
}
