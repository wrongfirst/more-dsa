type Twitter struct {
    count     int
    tweetMap  map[int][][]int
    followMap map[int]map[int]bool
}


func Constructor() Twitter {
    return Twitter{
        count:     0,
        tweetMap:  make(map[int][][]int),
        followMap: make(map[int]map[int]bool),
    }
}


func (this *Twitter) PostTweet(userId int, tweetId int)  {
    this.tweetMap[userId] = append(this.tweetMap[userId], []int{this.count, tweetId})
    this.count--
}


func (this *Twitter) GetNewsFeed(userId int) []int {
    res := []int{}
    h := &minHeap{}
    heap.Init(h)
    
    if this.followMap[userId] == nil {
        this.followMap[userId] = make(map[int]bool)
    }
    this.followMap[userId][userId] = true
    
    for followeeId := range this.followMap[userId] {
        if tweets, ok := this.tweetMap[followeeId]; ok {
            index := len(tweets) - 1
            count, tweetId := tweets[index][0], tweets[index][1]
            heap.Push(h, []int{count, tweetId, followeeId, index - 1})
        }
    }
    
    for h.Len() > 0 && len(res) < 10 {
        tweet := heap.Pop(h).([]int)
        count, tweetId, followeeId, index := tweet[0], tweet[1], tweet[2], tweet[3]
        res = append(res, tweetId)
        
        if index >= 0 {
            count, tweetId = this.tweetMap[followeeId][index][0], this.tweetMap[followeeId][index][1]
            heap.Push(h, []int{count, tweetId, followeeId, index - 1})
        }
    }
    
    return res
}


func (this *Twitter) Follow(followerId int, followeeId int)  {
    if this.followMap[followerId] == nil {
        this.followMap[followerId] = make(map[int]bool)
    }
    this.followMap[followerId][followeeId] = true
}


func (this *Twitter) Unfollow(followerId int, followeeId int)  {
    if this.followMap[followerId] != nil {
        delete(this.followMap[followerId], followeeId)
    }
}

type minHeap [][]int

func (h minHeap) Len() int           { return len(h) }
func (h minHeap) Less(i, j int) bool { return h[i][0] < h[j][0] }
func (h minHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *minHeap) Push(x interface{}) {
    *h = append(*h, x.([]int))
}

func (h *minHeap) Pop() interface{} {
    old := *h
    n := len(old)
    x := old[n-1]
    *h = old[0 : n-1]
    return x
}
