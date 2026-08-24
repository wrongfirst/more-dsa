func findRedundantConnection(edges [][]int) []int {
    par := make([]int, len(edges)+1)
    for i := 0; i < len(par); i++ {
        par[i] = i
    }
    rank := make([]int, len(edges)+1)
    for i := 0; i < len(rank); i++ {
        rank[i] = 1
    }

    var find func(n int) int
    find = func(n int) int {
        p := par[n]
        for p != par[p] {
            par[p] = par[par[p]]
            p = par[p]
        }
        return p
    }

    var union func(n1, n2 int) bool
    union = func(n1, n2 int) bool {
        p1, p2 := find(n1), find(n2)

        if p1 == p2 {
            return false
        }
        if rank[p1] > rank[p2] {
            par[p2] = p1
            rank[p1] += rank[p2]
        } else {
            par[p1] = p2
            rank[p2] += rank[p1]
        }
        return true
    }

    for _, edge := range edges {
        n1, n2 := edge[0], edge[1]
        if !union(n1, n2) {
            return []int{n1, n2}
        }
    }
    return []int{}
}
