You are given two **non-empty** linked lists, `l1` and `l2`, where each represents a non-negative integer.
    
The digits are stored in **reverse order**, e.g. the number 321 is represented as `1 -> 2 -> 3 ->` in the linked list.

Each of the nodes contains a single digit. You may assume the two numbers do not contain any leading zero, except the number `0` itself.

Return the sum of the two numbers as a linked list.

**Example 1:**

![](https://imagedelivery.net/CLfkmk9Wzy8_9HRyug4EVA/fee72e19-6a21-45a5-365e-3cb45aba9700/public)

```java
Input: l1 = [1,2,3], l2 = [4,5,6]

Output: [5,7,9]

Explanation: 321 + 654 = 975.
```

**Example 2:**

```java
Input: l1 = [9], l2 = [9]

Output: [8,1]
```

**Constraints:**
* `1 <= l1.length, l2.length <= 100`.
* `0 <= Node.val <= 9`


<br>
