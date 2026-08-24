func kClosest(points [][]int, k int) [][]int {
    minHeap := [][]int{}
    for _, point := range points {
        x, y := point[0], point[1]
        dist := (x * x) + (y * y)
        minHeap = append(minHeap, []int{dist, x, y})
    }
    
    heap.Init(&minHeapPoints{&minHeap})
    res := [][]int{}
    for i := 0; i < k; i++ {
        popped := heap.Pop(&minHeapPoints{&minHeap}).([]int)
        _, x, y := popped[0], popped[1], popped[2]
        res = append(res, []int{x, y})
    }
    return res
}

type minHeapPoints struct {
    points *[][]int
}

func (h minHeapPoints) Len() int           { return len(*h.points) }
func (h minHeapPoints) Less(i, j int) bool { return (*h.points)[i][0] < (*h.points)[j][0] }
func (h minHeapPoints) Swap(i, j int)      { (*h.points)[i], (*h.points)[j] = (*h.points)[j], (*h.points)[i] }

func (h *minHeapPoints) Push(x interface{}) {
    *h.points = append(*h.points, x.([]int))
}

func (h *minHeapPoints) Pop() interface{} {
    old := *h.points
    n := len(old)
    x := old[n-1]
    *h.points = old[0 : n-1]
    return x
}
