func orangesRotting(grid [][]int) int {
    q := [][2]int{}
    fresh := 0
    time := 0

    for r := 0; r < len(grid); r++ {
        for c := 0; c < len(grid[0]); c++ {
            if grid[r][c] == 1 {
                fresh++
            }
            if grid[r][c] == 2 {
                q = append(q, [2]int{r, c})
            }
        }
    }

    directions := [][2]int{{0, 1}, {0, -1}, {1, 0}, {-1, 0}}
    for fresh > 0 && len(q) > 0 {
        length := len(q)
        for i := 0; i < length; i++ {
            r, c := q[0][0], q[0][1]
            q = q[1:]

            for _, dir := range directions {
                dr, dc := dir[0], dir[1]
                row, col := r+dr, c+dc
                if row >= 0 && row < len(grid) &&
                    col >= 0 && col < len(grid[0]) &&
                    grid[row][col] == 1 {
                    grid[row][col] = 2
                    q = append(q, [2]int{row, col})
                    fresh--
                }
            }
        }
        time++
    }

    if fresh == 0 {
        return time
    }
    return -1
}
