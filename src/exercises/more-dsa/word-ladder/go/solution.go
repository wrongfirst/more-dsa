func ladderLength(beginWord string, endWord string, wordList []string) int {
    endWordExists := false
    for _, word := range wordList {
        if word == endWord {
            endWordExists = true
            break
        }
    }
    if !endWordExists {
        return 0
    }

    nei := make(map[string][]string)
    wordList = append(wordList, beginWord)
    for _, word := range wordList {
        for j := 0; j < len(word); j++ {
            pattern := word[:j] + "*" + word[j+1:]
            nei[pattern] = append(nei[pattern], word)
        }
    }

    visit := make(map[string]bool)
    visit[beginWord] = true
    q := []string{beginWord}
    res := 1

    for len(q) > 0 {
        qLen := len(q)
        for i := 0; i < qLen; i++ {
            word := q[0]
            q = q[1:]

            if word == endWord {
                return res
            }
            for j := 0; j < len(word); j++ {
                pattern := word[:j] + "*" + word[j+1:]
                for _, neiWord := range nei[pattern] {
                    if !visit[neiWord] {
                        visit[neiWord] = true
                        q = append(q, neiWord)
                    }
                }
            }
        }
        res++
    }
    return 0
}
