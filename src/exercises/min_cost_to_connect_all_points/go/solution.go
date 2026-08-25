func minCostConnectPoints(points [][]int) int {
    n := len(points)
    adj := make(map[int][][2]int)
    for i := 0; i < n; i++ {
        adj[i] = [][2]int{}
    }

    for i := 0; i < n; i++ {
        x1, y1 := points[i][0], points[i][1]
        for j := i + 1; j < n; j++ {
            x2, y2 := points[j][0], points[j][1]
            dist := abs(x1-x2) + abs(y1-y2)
            adj[i] = append(adj[i], [2]int{dist, j})
            adj[j] = append(adj[j], [2]int{dist, i})
        }
    }

    res := 0
    visit := make(map[int]bool)
    minH := &MinHeap{}
    heap.Init(minH)
    heap.Push(minH, [2]int{0, 0})

    for len(visit) < n {
        item := heap.Pop(minH).([2]int)
        cost, i := item[0], item[1]

        if visit[i] {
            continue
        }
        res += cost
        visit[i] = true

        for _, neighbor := range adj[i] {
            neiCost, nei := neighbor[0], neighbor[1]
            if !visit[nei] {
                heap.Push(minH, [2]int{neiCost, nei})
            }
        }
    }
    return res
}

func abs(x int) int {
    if x < 0 {
        return -x
    }
    return x
}

type MinHeap [][2]int

func (h MinHeap) Len() int           { return len(h) }
func (h MinHeap) Less(i, j int) bool { return h[i][0] < h[j][0] }
func (h MinHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *MinHeap) Push(x interface{}) {
    *h = append(*h, x.([2]int))
}

func (h *MinHeap) Pop() interface{} {
    old := *h
    n := len(old)
    x := old[n-1]
    *h = old[0 : n-1]
    return x
}
