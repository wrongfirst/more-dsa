func isNStraightHand(hand []int, groupSize int) bool {
    if len(hand)%groupSize != 0 {
        return false
    }

    count := make(map[int]int)
    for _, n := range hand {
        count[n]++
    }

    minH := &MinHeap{}
    heap.Init(minH)
    for key := range count {
        heap.Push(minH, key)
    }

    for minH.Len() > 0 {
        first := (*minH)[0]
        for i := first; i < first+groupSize; i++ {
            if _, exists := count[i]; !exists {
                return false
            }
            count[i]--
            if count[i] == 0 {
                if i != (*minH)[0] {
                    return false
                }
                heap.Pop(minH)
            }
        }
    }
    return true
}

type MinHeap []int

func (h MinHeap) Len() int           { return len(h) }
func (h MinHeap) Less(i, j int) bool { return h[i] < h[j] }
func (h MinHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *MinHeap) Push(x interface{}) {
    *h = append(*h, x.(int))
}

func (h *MinHeap) Pop() interface{} {
    old := *h
    n := len(old)
    x := old[n-1]
    *h = old[0 : n-1]
    return x
}
