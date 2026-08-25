You are given an `m x n` matrix `board` containing letters `'X'` and `'O'`, capture regions that are surrounded:

* **Connect:** A cell is connected to adjacent cells horizontally or vertically.

* **Region:** To form a region connect every `'O'` cell. Regions can have any shape; they do not need to be squares or rectangles.

* **Surround:** A region is surrounded if none of the `'O'` cells in that region are on the edge of the board. Such regions are **completely enclosed** by `'X'` cells.

To capture a **surrounded region**, replace all `'O'`s with `'X'`s **in-place** within the original board. You do not need to return anything.

**Example 1:**

![](https://imagedelivery.net/CLfkmk9Wzy8_9HRyug4EVA/1dcdb7cb-9299-440d-e96d-d55fe6c54700/public)

```java
Input: board = [
  ["X","X","X","X"],
  ["X","O","O","X"],
  ["X","X","O","X"],
  ["X","O","X","X"]
]

Output: [
  ["X","X","X","X"],
  ["X","X","X","X"],
  ["X","X","X","X"],
  ["X","O","X","X"]
]
```

Explanation: The bottom `'O'` region is not captured because it touches the edge of the board, so it cannot be surrounded.

**Example 2:**

```java
Input: board = [["X"]]

Output: [["X"]]
```

**Constraints:**
* `1 <= board.length, board[i].length <= 200`
* `board[i][j]` is `'X'` or `'O'`.
