You are given a connected **undirected graph** with `n` nodes labeled from `1` to `n`. Initially, it contained no cycles and consisted of `n-1` edges.

We have now added one additional edge to the graph. The edge has two **different** vertices chosen from `1` to `n`, and was not an edge that previously existed in the graph.

The graph is represented as an array `edges` of length `n` where `edges[i] = [ai, bi]` represents an edge between nodes `ai` and `bi` in the graph.

Return an edge that can be removed so that the graph is still a connected non-cyclical graph. If there are multiple answers, return the edge that appears last in the input `edges`.

**Example 1:**

![](https://imagedelivery.net/CLfkmk9Wzy8_9HRyug4EVA/1a966522-e4d9-4215-18a1-4df7d26c3700/public)

```java
Input: edges = [[1,2],[1,3],[3,4],[2,4]]

Output: [2,4]
```

**Example 2:**

![](https://imagedelivery.net/CLfkmk9Wzy8_9HRyug4EVA/5cf17b17-8758-4f0a-8829-99cea143b100/public)

```java
Input: edges = [[1,2],[1,3],[1,4],[3,4],[4,5]]

Output: [3,4]
```

**Constraints:**
* `n == edges.length`
* `3 <= n <= 1000`
* `1 <= edges[i][0] < edges[i][1] <= edges.length`
* There are no repeated edges and no self-loops in the input.
