func swimInWater(grid [][]int) int {
    n := len(grid)
    visit := make(map[[2]int]bool)
    minH := &MinHeap{}
    heap.Init(minH)
    heap.Push(minH, [3]int{grid[0][0], 0, 0})
    directions := [][2]int{{0, 1}, {0, -1}, {1, 0}, {-1, 0}}
    visit[[2]int{0, 0}] = true

    for minH.Len() > 0 {
        item := heap.Pop(minH).([3]int)
        t, r, c := item[0], item[1], item[2]

        if r == n-1 && c == n-1 {
            return t
        }

        for _, dir := range directions {
            dr, dc := dir[0], dir[1]
            neiR, neiC := r+dr, c+dc

            if neiR < 0 ||
                neiC < 0 ||
                neiR == n ||
                neiC == n ||
                visit[[2]int{neiR, neiC}] {
                continue
            }
            visit[[2]int{neiR, neiC}] = true
            heap.Push(minH, [3]int{max(t, grid[neiR][neiC]), neiR, neiC})
        }
    }
    return 0
}

type MinHeap [][3]int

func (h MinHeap) Len() int           { return len(h) }
func (h MinHeap) Less(i, j int) bool { return h[i][0] < h[j][0] }
func (h MinHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *MinHeap) Push(x interface{}) {
	*h = append(*h, x.([3]int))
}

func (h *MinHeap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}
