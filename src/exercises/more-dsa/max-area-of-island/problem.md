You are given a matrix `grid` where `grid[i]` is either a `0` (representing water) or `1` (representing land).

An island is defined as a group of `1`'s connected horizontally or vertically. You may assume all four edges of the grid are surrounded by water.

The **area** of an island is defined as the number of cells within the island.

Return the maximum **area** of an island in `grid`. If no island exists, return `0`.

**Example 1:**

![](https://imagedelivery.net/CLfkmk9Wzy8_9HRyug4EVA/8eeb491c-c8ff-4ed6-78ed-ce4cf87d7200/public)

```java
Input: grid = [
  [0,1,1,0,1],
  [1,0,1,0,1],
  [0,1,1,0,1],
  [0,1,0,0,1]
]

Output: 6
```
Explanation: `1`'s cannot be connected diagonally, so the maximum area of the island is `6`.

**Constraints:**
* `1 <= grid.length, grid[i].length <= 50`


<br>
