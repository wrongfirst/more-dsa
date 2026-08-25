You are given a 2-D grid of integers `matrix`, where each integer is greater than or equal to `0`.

Return the length of the longest strictly increasing path within `matrix`.

From each cell within the path, you can move either horizontally or vertically. You **may not** move **diagonally**.

**Example 1:**

![](https://imagedelivery.net/CLfkmk9Wzy8_9HRyug4EVA/c302dea2-1695-4edb-e1b3-91e96d9bb700/public)

```java
Input: matrix = [[5,5,3],[2,3,6],[1,1,1]]

Output: 4
```

Explanation: The longest increasing path is `[1, 2, 3, 6]` or `[1, 2, 3, 5]`.

**Example 2:**

![](https://imagedelivery.net/CLfkmk9Wzy8_9HRyug4EVA/72004ec9-6f68-464c-93c1-0ba0b384c700/public)

```java
Input: matrix = [[1,2,3],[2,1,4],[7,6,5]]

Output: 7
```

Explanation: The longest increasing path is `[1, 2, 3, 4, 5, 6, 7]`.

**Constraints:**
* `1 <= matrix.length, matrix[i].length <= 100`
