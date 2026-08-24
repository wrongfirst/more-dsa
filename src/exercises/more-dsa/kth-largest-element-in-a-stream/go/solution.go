type KthLargest struct {
    minHeap []int
    k       int
}

func Constructor(k int, nums []int) KthLargest {
    kl := KthLargest{
        minHeap: nums,
        k:       k,
    }
    heap.Init(&minHeap{&kl.minHeap})
    for len(kl.minHeap) > k {
        heap.Pop(&minHeap{&kl.minHeap})
    }
    return kl
}

func (this *KthLargest) Add(val int) int {
    heap.Push(&minHeap{&this.minHeap}, val)
    if len(this.minHeap) > this.k {
        heap.Pop(&minHeap{&this.minHeap})
    }
    return this.minHeap[0]
}


type minHeap struct {
    nums *[]int
}

func (h minHeap) Len() int           { return len(*h.nums) }
func (h minHeap) Less(i, j int) bool { return (*h.nums)[i] < (*h.nums)[j] }
func (h minHeap) Swap(i, j int)      { (*h.nums)[i], (*h.nums)[j] = (*h.nums)[j], (*h.nums)[i] }

func (h *minHeap) Push(x interface{}) {
    *h.nums = append(*h.nums, x.(int))
}

func (h *minHeap) Pop() interface{} {
    old := *h.nums
    n := len(old)
    x := old[n-1]
    *h.nums = old[0 : n-1]
    return x
}
