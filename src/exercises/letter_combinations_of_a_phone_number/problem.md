You are given a string `digits` made up of digits from `2` through `9` inclusive.

Each digit (not including 1) is mapped to a set of characters as shown below:

A digit could represent any one of the characters it maps to.

Return all possible letter combinations that `digits` could represent. You may return the answer in **any order**.

<img
  src="https://imagedelivery.net/CLfkmk9Wzy8_9HRyug4EVA/d5eb2098-bd7f-47a1-554a-ad77a39f3100/public"
  alt="Phone keypad letter mapping"
  style="max-width: 300px; width: 100%; height: auto;"
/>

**Example 1:**

```java
Input: digits = "34"

Output: ["dg","dh","di","eg","eh","ei","fg","fh","fi"]
```

**Example 2:**

```java
Input: digits = ""

Output: []
```

**Constraints:**
* `0 <= digits.length <= 4`
* `2 <= digits[i] <= 9`
