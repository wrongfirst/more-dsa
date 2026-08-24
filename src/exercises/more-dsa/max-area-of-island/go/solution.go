func maxAreaOfIsland(grid [][]int) int {
    rows, cols := len(grid), len(grid[0])
    visit := make(map[[2]int]bool)

    var dfs func(r, c int) int
    dfs = func(r, c int) int {
        if r < 0 ||
            r == rows ||
            c < 0 ||
            c == cols ||
            grid[r][c] == 0 ||
            visit[[2]int{r, c}] {
            return 0
        }
        visit[[2]int{r, c}] = true
        return 1 + dfs(r+1, c) + dfs(r-1, c) + dfs(r, c+1) + dfs(r, c-1)
    }

    area := 0
    for r := 0; r < rows; r++ {
        for c := 0; c < cols; c++ {
            area = max(area, dfs(r, c))
        }
    }
    return area
}
