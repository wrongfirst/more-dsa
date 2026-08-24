The **n-queens** puzzle is the problem of placing `n` queens on an `n x n` chessboard so that no two queens can attack each other.

A **queen** in a chessboard can attack horizontally, vertically, and diagonally.

Given an integer `n`, return all distinct solutions to the **n-queens puzzle**.

Each solution contains a unique board layout where the queen pieces are placed. `'Q'` indicates a queen and `'.'` indicates an empty space.

You may return the answer in **any order**.

**Example 1:**

![](https://imagedelivery.net/CLfkmk9Wzy8_9HRyug4EVA/cdf2b34d-7905-4388-db0b-9a120ebf4a00/public)

```java
Input: n = 4

Output: [[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
```

Explanation: There are two different solutions to the 4-queens puzzle.

**Example 2:**

```java
Input: n = 1

Output: [["Q"]]
```

**Constraints:**
* `1 <= n <= 8`


<br>
