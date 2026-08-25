func islandsAndTreasure(grid [][]int) {
    rows, cols := len(grid), len(grid[0])
    visit := make(map[[2]int]bool)
    q := [][2]int{}

    addCell := func(r, c int) {
        if r < 0 || c < 0 ||
            r == rows ||
            c == cols ||
            visit[[2]int{r, c}] ||
            grid[r][c] == -1 {
            return
        }
        visit[[2]int{r, c}] = true
        q = append(q, [2]int{r, c})
    }

    for r := 0; r < rows; r++ {
        for c := 0; c < cols; c++ {
            if grid[r][c] == 0 {
                q = append(q, [2]int{r, c})
                visit[[2]int{r, c}] = true
            }
        }
    }

    dist := 0
    for len(q) > 0 {
        qLen := len(q)
        for i := 0; i < qLen; i++ {
            cell := q[0]
            q = q[1:]
            r, c := cell[0], cell[1]
            grid[r][c] = dist
            addCell(r+1, c)
            addCell(r-1, c)
            addCell(r, c+1)
            addCell(r, c-1)
        }
        dist++
    }
}
