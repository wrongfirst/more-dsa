func isValidSudoku(board [][]byte) bool {
	cols := make(map[int]map[byte]bool)
	rows := make(map[int]map[byte]bool)
	squares := make(map[[2]int]map[byte]bool) // key = (r /3, c /3)
	for r := 0; r < 9; r++ {
		for c := 0; c < 9; c++ {
			if board[r][c] == '.' {
				continue
			}
			if cols[c] == nil {
				cols[c] = make(map[byte]bool)
			}
			if rows[r] == nil {
				rows[r] = make(map[byte]bool)
			}
			squareKey := [2]int{r / 3, c / 3}
			if squares[squareKey] == nil {
				squares[squareKey] = make(map[byte]bool)
			}
			if cols[c][board[r][c]] || rows[r][board[r][c]] || squares[squareKey][board[r][c]] {
				return false
			}
			cols[c][board[r][c]] = true
			rows[r][board[r][c]] = true
			squares[squareKey][board[r][c]] = true
		}
	}
	return true
}
