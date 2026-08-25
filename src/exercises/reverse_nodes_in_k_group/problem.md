You are given the head of a singly linked list `head` and a positive integer `k`.

You must reverse the first `k` nodes in the linked list, and then reverse the next `k` nodes, and so on. If there are fewer than `k` nodes left, leave the nodes as they are.

Return the modified list after reversing the nodes in each group of `k`.

You are only allowed to modify the nodes' `next` pointers, not the values of the nodes.

**Example 1:**

![](https://imagedelivery.net/CLfkmk9Wzy8_9HRyug4EVA/67cf2fff-f20a-4558-6091-c3e857f56e00/public)

```java
Input: head = [1,2,3,4,5,6], k = 3

Output: [3,2,1,6,5,4]
```

**Example 2:**

![](https://imagedelivery.net/CLfkmk9Wzy8_9HRyug4EVA/af843e59-df12-4c55-652b-6ddab0a92900/public)

```java
Input: head = [1,2,3,4,5], k = 3

Output: [3,2,1,4,5]
```

**Constraints:**
* The length of the linked list is `n`.
* `1 <= k <= n <= 5000`
* `0 <= Node.val <= 100`
