func findOrder(numCourses int, prerequisites [][]int) []int {
    prereq := make(map[int][]int)
    for c := 0; c < numCourses; c++ {
        prereq[c] = []int{}
    }
    for _, pre := range prerequisites {
        crs, pre := pre[0], pre[1]
        prereq[crs] = append(prereq[crs], pre)
    }

    output := []int{}
    visit, cycle := make(map[int]bool), make(map[int]bool)

    var dfs func(crs int) bool
    dfs = func(crs int) bool {
        if cycle[crs] {
            return false
        }
        if visit[crs] {
            return true
        }

        cycle[crs] = true
        for _, pre := range prereq[crs] {
            if dfs(pre) == false {
                return false
            }
        }
        delete(cycle, crs)
        visit[crs] = true
        output = append(output, crs)
        return true
    }

    for c := 0; c < numCourses; c++ {
        if dfs(c) == false {
            return []int{}
        }
    }
    return output
}
