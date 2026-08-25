func solve(board [][]byte) {
    rows, cols := len(board), len(board[0])

    var capture func(r, c int)
    capture = func(r, c int) {
        if r < 0 || c < 0 || r == rows || c == cols || board[r][c] != 'O' {
            return
        }
        board[r][c] = 'T'
        capture(r+1, c)
        capture(r-1, c)
        capture(r, c+1)
        capture(r, c-1)
    }

    for r := 0; r < rows; r++ {
        for c := 0; c < cols; c++ {
            if board[r][c] == 'O' && (r == 0 || r == rows-1 || c == 0 || c == cols-1) {
                capture(r, c)
            }
        }
    }

    for r := 0; r < rows; r++ {
        for c := 0; c < cols; c++ {
            if board[r][c] == 'O' {
                board[r][c] = 'X'
            }
        }
    }

    for r := 0; r < rows; r++ {
        for c := 0; c < cols; c++ {
            if board[r][c] == 'T' {
                board[r][c] = 'O'
            }
        }
    }
}
