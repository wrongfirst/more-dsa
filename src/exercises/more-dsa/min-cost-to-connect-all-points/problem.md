You are given a 2-D integer array `points`, where `points[i] = [xi, yi]`. Each `points[i]` represents a distinct point on a 2-D plane.

The cost of connecting two points `[xi, yi]` and `[xj, yj]` is the **manhattan distance** between the two points, i.e. `|xi - xj| + |yi - yj|`.

Return the minimum cost to connect all points together, such that there exists exactly one path between each pair of points.

**Example 1:**

![](https://imagedelivery.net/CLfkmk9Wzy8_9HRyug4EVA/e0cd5270-73b5-42d4-3c3f-5451f795ca00/public)

```java
Input: points = [[0,0],[2,2],[3,3],[2,4],[4,2]]

Output: 10
```

**Constraints:**
* `1 <= points.length <= 1000`
* `-1,000,000 <= xi, yi <= 1,000,000`
* All pairs `(xi, yi)` are distinct.


<br>
