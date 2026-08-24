type Node struct {
    key, val   int
    prev, next *Node
}

type LRUCache struct {
    cap   int
    cache map[int]*Node
    left  *Node
    right *Node
}

func Constructor(capacity int) LRUCache {
    lru := LRUCache{
        cap:   capacity,
        cache: make(map[int]*Node),
        left:  &Node{key: 0, val: 0},
        right: &Node{key: 0, val: 0},
    }
    lru.left.next = lru.right
    lru.right.prev = lru.left
    return lru
}

func (this *LRUCache) remove(node *Node) {
    prev, nxt := node.prev, node.next
    prev.next, nxt.prev = nxt, prev
}

func (this *LRUCache) insert(node *Node) {
    prev, nxt := this.right.prev, this.right
    prev.next = node
    nxt.prev = node
    node.next = nxt
    node.prev = prev
}

func (this *LRUCache) Get(key int) int {
    if node, exists := this.cache[key]; exists {
        this.remove(node)
        this.insert(node)
        return node.val
    }
    return -1
}

func (this *LRUCache) Put(key int, value int) {
    if node, exists := this.cache[key]; exists {
        this.remove(node)
    }
    
    this.cache[key] = &Node{key: key, val: value}
    this.insert(this.cache[key])
    
    if len(this.cache) > this.cap {
        lru := this.left.next
        this.remove(lru)
        delete(this.cache, lru.key)
    }
}
