func minInterval(intervals [][]int, queries []int) []int {
    sort.Slice(intervals, func(i, j int) bool {
        if intervals[i][0] == intervals[j][0] {
            return intervals[i][1] < intervals[j][1]
        }
        return intervals[i][0] < intervals[j][0]
    })

    sortedQueries := make([]int, len(queries))
    copy(sortedQueries, queries)
    sort.Ints(sortedQueries)

    res := make(map[int]int)
    h := &MinHeap{}
    heap.Init(h)
    i := 0

    for _, q := range sortedQueries {
        for i < len(intervals) && intervals[i][0] <= q {
            l, r := intervals[i][0], intervals[i][1]
            heap.Push(h, []int{r - l + 1, r})
            i++
        }

        for h.Len() > 0 && (*h)[0][1] < q {
            heap.Pop(h)
        }

        if h.Len() > 0 {
            res[q] = (*h)[0][0]
        } else {
            res[q] = -1
        }
    }

    output := make([]int, len(queries))
    for idx, q := range queries {
        output[idx] = res[q]
    }

    return output
}

type MinHeap [][]int

func (h MinHeap) Len() int            { return len(h) }
func (h MinHeap) Less(i, j int) bool  { return h[i][0] < h[j][0] }
func (h MinHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }

func (h *MinHeap) Push(x interface{}) {
	*h = append(*h, x.([]int))
}

func (h *MinHeap) Pop() interface{} {
    old := *h
    n := len(old)
    x := old[n-1]
    *h = old[:n-1]
    return x
}
