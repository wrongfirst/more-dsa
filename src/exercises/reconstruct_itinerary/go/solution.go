func findItinerary(tickets [][]string) []string {
    adj := make(map[string][]string)
    for _, ticket := range tickets {
        src:= ticket[0]
        adj[src] = []string{}
    }
    res := []string{}

    for _, ticket := range tickets {
        src, dst := ticket[0], ticket[1]
        adj[src] = append(adj[src], dst)
    }

    for key := range adj {
        sort.Strings(adj[key])
    }

    var dfs func(adj map[string][]string, result []string, src string)
    dfs = func(adj map[string][]string, result []string, src string) {
        if destinations, exists := adj[src]; exists {
            destCopy := make([]string, len(destinations))
            copy(destCopy, destinations)

            for len(destCopy) > 0 {
                dest := destCopy[0]
                adj[src] = adj[src][1:]
                dfs(adj, res, dest)
                destCopy = make([]string, len(adj[src]))
                copy(destCopy, adj[src])
            }
        }
        res = append(res, src)
    }

    dfs(adj, res, "JFK")

    for i, j := 0, len(res)-1; i < j; i, j = i+1, j-1 {
        res[i], res[j] = res[j], res[i]
    }

    if len(res) != len(tickets)+1 {
        return []string{}
    }
    return res
}
