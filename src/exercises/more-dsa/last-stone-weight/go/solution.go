func lastStoneWeight(stones []int) int {
    h := make([]int, len(stones))
    for i, s := range stones {
        h[i] = -s
    }
    heap.Init(&maxHeap{&h})
    
    for len(h) > 1 {
        first := heap.Pop(&maxHeap{&h}).(int)
        second := heap.Pop(&maxHeap{&h}).(int)
        
        if second > first {
            heap.Push(&maxHeap{&h}, first-second)
        }
    }
    
    if len(h) == 0 {
        h = append(h, 0)
    }
    return abs(h[0])
}

func abs(x int) int {
    if x < 0 {
        return -x
    }
    return x
}

type maxHeap struct {
    nums *[]int
}

func (h maxHeap) Len() int           { return len(*h.nums) }
func (h maxHeap) Less(i, j int) bool { return (*h.nums)[i] < (*h.nums)[j] }
func (h maxHeap) Swap(i, j int)      { (*h.nums)[i], (*h.nums)[j] = (*h.nums)[j], (*h.nums)[i] }

func (h *maxHeap) Push(x interface{}) {
    *h.nums = append(*h.nums, x.(int))
}

func (h *maxHeap) Pop() interface{} {
    old := *h.nums
    n := len(old)
    x := old[n-1]
    *h.nums = old[0 : n-1]
    return x
}
