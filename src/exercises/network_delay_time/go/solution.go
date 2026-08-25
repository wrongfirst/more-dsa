func networkDelayTime(times [][]int, n int, k int) int {
    edges := make(map[int][][2]int)
    for _, time := range times {
        u, v, w := time[0], time[1], time[2]
        edges[u] = append(edges[u], [2]int{v, w})
    }

    minHeap := &MinHeap{}
    heap.Init(minHeap)
    heap.Push(minHeap, [2]int{0, k})
    visit := make(map[int]bool)
    t := 0

    for minHeap.Len() > 0 {
        item := heap.Pop(minHeap).([2]int)
        w1, n1 := item[0], item[1]

        if visit[n1] {
            continue
        }
        visit[n1] = true
        t = w1

        for _, edge := range edges[n1] {
            n2, w2 := edge[0], edge[1]
            if !visit[n2] {
                heap.Push(minHeap, [2]int{w1 + w2, n2})
            }
        }
    }

    if len(visit) == n {
        return t
    }
    return -1
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
