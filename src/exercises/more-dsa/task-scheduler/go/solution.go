func leastInterval(tasks []byte, n int) int {
    count := make(map[byte]int)
    for _, task := range tasks {
        count[task]++
    }
    
    maxHeap := make([]int, 0)
    for _, cnt := range count {
        maxHeap = append(maxHeap, -cnt)
    }
    heap.Init(&maxHeapInt{&maxHeap})
    
    time := 0
    q := [][]int{} // pairs of [cnt, idleTime]
    
    for len(maxHeap) > 0 || len(q) > 0 {
        time++
        
        if len(maxHeap) == 0 {
            time = q[0][1]
        } else {
            cnt := 1 + heap.Pop(&maxHeapInt{&maxHeap}).(int)
            if cnt != 0 {
                q = append(q, []int{cnt, time + n})
            }
        }
        
        if len(q) > 0 && q[0][1] == time {
            heap.Push(&maxHeapInt{&maxHeap}, q[0][0])
            q = q[1:]
        }
    }
    
    return time
}

type maxHeapInt struct {
    nums *[]int
}

func (h maxHeapInt) Len() int           { return len(*h.nums) }
func (h maxHeapInt) Less(i, j int) bool { return (*h.nums)[i] < (*h.nums)[j] }
func (h maxHeapInt) Swap(i, j int)      { (*h.nums)[i], (*h.nums)[j] = (*h.nums)[j], (*h.nums)[i] }

func (h *maxHeapInt) Push(x interface{}) {
    *h.nums = append(*h.nums, x.(int))
}

func (h *maxHeapInt) Pop() interface{} {
    old := *h.nums
    n := len(old)
    x := old[n-1]
    *h.nums = old[0 : n-1]
    return x
}
